use crate::abi::linux_x86_64 as abi;
use crate::core::{handle, queue, HandleKind};
use crate::{threading, upstream_support};
use libc::{self, c_void};
use std::mem::offset_of;
use std::os::raw::{c_char, c_int, c_uint};

#[inline]
fn uv_err(err: c_int) -> c_int {
    if err == 0 {
        0
    } else {
        -err
    }
}

unsafe fn close_fd(fd: c_int) {
    if fd >= 0 {
        let _ = unsafe { libc::close(fd) };
    }
}

unsafe fn write_child_error(error_fd: c_int) -> ! {
    let err = uv_err(unsafe { *libc::__errno_location() });
    let err_ptr = std::ptr::addr_of!(err).cast::<c_void>();
    loop {
        let rc = unsafe { libc::write(error_fd, err_ptr, std::mem::size_of::<c_int>()) };
        if rc >= 0 || unsafe { *libc::__errno_location() } != libc::EINTR {
            break;
        }
    }
    unsafe { libc::_exit(127) }
}

unsafe fn clear_nonblock(fd: c_int) {
    loop {
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags >= 0 {
            let _ = unsafe { libc::fcntl(fd, libc::F_SETFL, flags & !libc::O_NONBLOCK) };
            return;
        }
        if unsafe { *libc::__errno_location() } != libc::EINTR {
            return;
        }
    }
}

unsafe fn init_spawn_stdio(
    container: *mut abi::uv_stdio_container_t,
    fds: &mut [c_int; 2],
) -> c_int {
    let flags = unsafe { (*container).flags };
    let mask = abi::uv_stdio_flags_UV_CREATE_PIPE
        | abi::uv_stdio_flags_UV_INHERIT_FD
        | abi::uv_stdio_flags_UV_INHERIT_STREAM;

    match flags & mask {
        abi::uv_stdio_flags_UV_IGNORE => 0,
        abi::uv_stdio_flags_UV_CREATE_PIPE => {
            let stream = unsafe { (*container).data.stream };
            if stream.is_null() || unsafe { (*stream).type_ } != abi::uv_handle_type_UV_NAMED_PIPE {
                return abi::uv_errno_t_UV_EINVAL;
            }
            unsafe {
                crate::unix::tcp::uv_socketpair(libc::SOCK_STREAM, 0, fds.as_mut_ptr().cast(), 0, 0)
            }
        }
        abi::uv_stdio_flags_UV_INHERIT_FD => {
            fds[1] = unsafe { (*container).data.fd };
            if fds[1] < 0 {
                abi::uv_errno_t_UV_EINVAL
            } else {
                0
            }
        }
        abi::uv_stdio_flags_UV_INHERIT_STREAM => {
            let stream = unsafe { (*container).data.stream };
            let mut fd: abi::uv_os_fd_t = -1;
            let rc = unsafe { upstream_support::unix_core::uv_fileno(stream.cast(), &mut fd) };
            if rc != 0 || fd < 0 {
                abi::uv_errno_t_UV_EINVAL
            } else {
                fds[1] = fd;
                0
            }
        }
        _ => abi::uv_errno_t_UV_EINVAL,
    }
}

unsafe fn cleanup_spawn_pipes(
    stdio: *mut abi::uv_stdio_container_t,
    requested_stdio: usize,
    pipes: &mut [[c_int; 2]],
) {
    for (idx, pair) in pipes.iter_mut().enumerate() {
        let inherited = if idx < requested_stdio && !stdio.is_null() {
            let flags = unsafe { (*stdio.add(idx)).flags };
            (flags & abi::uv_stdio_flags_UV_INHERIT_FD) != 0
                || (flags & abi::uv_stdio_flags_UV_INHERIT_STREAM) != 0
        } else {
            false
        };
        unsafe {
            close_fd(pair[0]);
            if !inherited {
                close_fd(pair[1]);
            }
        }
        pair[0] = -1;
        pair[1] = -1;
    }
}

unsafe fn open_parent_spawn_stream(
    container: *mut abi::uv_stdio_container_t,
    pair: &mut [c_int; 2],
) -> c_int {
    let flags = unsafe { (*container).flags };
    if (flags & abi::uv_stdio_flags_UV_CREATE_PIPE) == 0 || pair[0] < 0 {
        return 0;
    }

    unsafe {
        close_fd(pair[1]);
    }
    pair[1] = -1;

    let stream = unsafe { (*container).data.stream };
    let rc = unsafe { crate::unix::pipe::uv_pipe_open(stream.cast(), pair[0]) };
    if rc == 0 {
        pair[0] = -1;
    }
    rc
}

unsafe fn child_exec(
    options: *const abi::uv_process_options_t,
    stdio_count: usize,
    pipes: &[[c_int; 2]],
    error_fd: c_int,
) -> ! {
    if unsafe { (*options).flags & abi::uv_process_flags_UV_PROCESS_DETACHED } != 0
        && unsafe { libc::setsid() } < 0
    {
        unsafe { write_child_error(error_fd) };
    }

    for fd in 0..stdio_count {
        let mut use_fd = pipes[fd][1];
        if use_fd < 0 {
            if fd >= 3 {
                continue;
            }
            let mode = if fd == 0 {
                libc::O_RDONLY
            } else {
                libc::O_RDWR
            };
            use_fd = unsafe { libc::open(c"/dev/null".as_ptr(), mode) };
            if use_fd < 0 {
                unsafe { write_child_error(error_fd) };
            }
        }

        if use_fd != fd as c_int && unsafe { libc::dup2(use_fd, fd as c_int) } < 0 {
            unsafe { write_child_error(error_fd) };
        }

        if fd <= 2 {
            unsafe { clear_nonblock(fd as c_int) };
        }
    }

    for pair in pipes {
        unsafe {
            if pair[0] > 2 {
                close_fd(pair[0]);
            }
            if pair[1] > 2 {
                close_fd(pair[1]);
            }
        }
    }

    if unsafe { (*options).cwd }.is_null() == false && unsafe { libc::chdir((*options).cwd) } != 0 {
        unsafe { write_child_error(error_fd) };
    }

    if unsafe { (*options).flags & abi::uv_process_flags_UV_PROCESS_SETGID } != 0
        && unsafe { libc::setgid((*options).gid) } != 0
    {
        unsafe { write_child_error(error_fd) };
    }
    if unsafe { (*options).flags & abi::uv_process_flags_UV_PROCESS_SETUID } != 0
        && unsafe { libc::setuid((*options).uid) } != 0
    {
        unsafe { write_child_error(error_fd) };
    }

    if unsafe { !(*options).env.is_null() } {
        unsafe {
            libc::execve(
                (*options).file,
                (*options).args.cast(),
                (*options).env.cast(),
            );
        }
    } else {
        unsafe {
            libc::execvp((*options).file, (*options).args.cast());
        }
    }

    unsafe { write_child_error(error_fd) }
}

pub(crate) unsafe fn process_reap(loop_: *mut abi::uv_loop_t) {
    if loop_.is_null() {
        return;
    }

    let head = std::ptr::addr_of_mut!((*loop_).process_handles);
    let mut node = unsafe { (*head).next };
    let mut exited = Vec::<(*mut abi::uv_process_t, c_int)>::new();

    while !std::ptr::eq(node, head) {
        let process = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_process_t, queue))
                .cast::<abi::uv_process_t>()
        };
        node = unsafe { (*node).next };

        let mut status = 0;
        let pid = loop {
            let rc = unsafe { libc::waitpid((*process).pid, &mut status, libc::WNOHANG) };
            if rc < 0 && unsafe { *libc::__errno_location() } == libc::EINTR {
                continue;
            }
            break rc;
        };

        if pid == 0 {
            continue;
        }
        if pid < 0 {
            let err = unsafe { *libc::__errno_location() };
            if err != libc::ECHILD {
                continue;
            }
        }

        exited.push((process, status));
    }

    for (process, status) in exited {
        unsafe {
            (*process).status = status;
            queue::remove(std::ptr::addr_of_mut!((*process).queue));
            queue::init(std::ptr::addr_of_mut!((*process).queue));
            handle::handle_stop(process.cast());
        }

        if let Some(cb) = unsafe { (*process).exit_cb } {
            let exit_status = if libc::WIFEXITED(status) {
                libc::WEXITSTATUS(status)
            } else {
                0
            } as i64;
            let term_signal = if libc::WIFSIGNALED(status) {
                libc::WTERMSIG(status)
            } else {
                0
            };
            unsafe {
                cb(process, exit_status, term_signal);
            }
        }
    }

    if unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).process_handles)) } {
        unsafe {
            (*loop_).flags &= !(crate::upstream_support::unix_core::UV_LOOP_REAP_CHILDREN as u64);
        }
    }
}

pub(crate) unsafe fn disable_stdio_inheritance() {
    unsafe {
        upstream_support::unix_core::uv_disable_stdio_inheritance();
    }
}

pub(crate) unsafe fn exepath(buffer: *mut c_char, size: *mut usize) -> c_int {
    if buffer.is_null() || size.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let path = c"/proc/self/exe";
    let cap = unsafe { *size };
    if cap == 0 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let rc = unsafe { libc::readlink(path.as_ptr(), buffer.cast(), cap.saturating_sub(1)) };
    if rc < 0 {
        return -unsafe { *libc::__errno_location() };
    }

    let len = rc as usize;
    unsafe {
        *buffer.add(len) = 0;
        *size = len;
    }
    0
}

pub(crate) unsafe fn fs_read(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe { threading::threadpool::fs_read(loop_, req, file, bufs, nbufs as c_int, offset, cb) }
}

pub(crate) unsafe fn fs_req_cleanup(req: *mut abi::uv_fs_t) {
    unsafe {
        threading::threadpool::fs_req_cleanup(req);
    }
}

pub(crate) unsafe fn fs_unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe { threading::threadpool::fs_unlink(loop_, req, path, cb) }
}

pub(crate) unsafe fn fileno(handle: *const abi::uv_handle_t, fd: *mut abi::uv_os_fd_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_fileno(handle.cast(), fd.cast()) }
}

pub(crate) unsafe fn free_interface_addresses(
    addresses: *mut abi::uv_interface_address_t,
    count: c_int,
) {
    unsafe {
        crate::unix::epoll::uv_free_interface_addresses(addresses.cast(), count);
    }
}

pub(crate) unsafe fn guess_handle(file: abi::uv_file) -> abi::uv_handle_type {
    let fd = file;
    if fd < 0 {
        return abi::uv_handle_type_UV_UNKNOWN_HANDLE;
    }

    if unsafe { libc::isatty(fd) } == 1 {
        return abi::uv_handle_type_UV_TTY;
    }

    let mut st = std::mem::MaybeUninit::<libc::stat>::uninit();
    if unsafe { libc::fstat(fd, st.as_mut_ptr()) } != 0 {
        return abi::uv_handle_type_UV_UNKNOWN_HANDLE;
    }
    let st = unsafe { st.assume_init() };
    let mode = st.st_mode;

    if (mode & libc::S_IFMT) == libc::S_IFREG {
        return abi::uv_handle_type_UV_FILE;
    }

    if (mode & libc::S_IFMT) == libc::S_IFCHR {
        return abi::uv_handle_type_UV_FILE;
    }

    if (mode & libc::S_IFMT) == libc::S_IFIFO {
        return abi::uv_handle_type_UV_NAMED_PIPE;
    }

    if (mode & libc::S_IFMT) != libc::S_IFSOCK {
        return abi::uv_handle_type_UV_UNKNOWN_HANDLE;
    }

    let mut ss = std::mem::MaybeUninit::<libc::sockaddr_storage>::zeroed();
    let mut len = std::mem::size_of::<libc::sockaddr_storage>() as libc::socklen_t;
    if unsafe { libc::getsockname(fd, ss.as_mut_ptr().cast(), &mut len) } != 0 {
        return abi::uv_handle_type_UV_UNKNOWN_HANDLE;
    }
    let ss = unsafe { ss.assume_init() };

    let mut sock_type = 0;
    len = std::mem::size_of::<c_int>() as libc::socklen_t;
    if unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_TYPE,
            (&mut sock_type as *mut c_int).cast(),
            &mut len,
        )
    } != 0
    {
        return abi::uv_handle_type_UV_UNKNOWN_HANDLE;
    }

    if sock_type == libc::SOCK_DGRAM
        && (ss.ss_family as c_int == libc::AF_INET || ss.ss_family as c_int == libc::AF_INET6)
    {
        return abi::uv_handle_type_UV_UDP;
    }

    if sock_type == libc::SOCK_STREAM {
        if ss.ss_family as c_int == libc::AF_INET || ss.ss_family as c_int == libc::AF_INET6 {
            return abi::uv_handle_type_UV_TCP;
        }
        if ss.ss_family as c_int == libc::AF_UNIX {
            return abi::uv_handle_type_UV_NAMED_PIPE;
        }
    }

    abi::uv_handle_type_UV_UNKNOWN_HANDLE
}

pub(crate) unsafe fn inet_ntop(
    af: c_int,
    src: *const c_void,
    dst: *mut c_char,
    size: usize,
) -> c_int {
    unsafe { upstream_support::inet::uv_inet_ntop(af, src.cast(), dst, size) }
}

pub(crate) unsafe fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int {
    unsafe { upstream_support::inet::uv_inet_pton(af, src, dst.cast()) }
}

pub(crate) unsafe fn interface_addresses(
    addresses: *mut *mut abi::uv_interface_address_t,
    count: *mut c_int,
) -> c_int {
    unsafe { crate::unix::epoll::uv_interface_addresses(addresses.cast(), count) }
}

pub(crate) unsafe fn ip4_addr(
    ip: *const c_char,
    port: c_int,
    addr: *mut abi::sockaddr_in,
) -> c_int {
    unsafe { upstream_support::uv_common::uv_ip4_addr(ip, port, addr.cast()) }
}

pub(crate) unsafe fn ip4_name(
    src: *const abi::sockaddr_in,
    dst: *mut c_char,
    size: usize,
) -> c_int {
    unsafe { upstream_support::uv_common::uv_ip4_name(src.cast(), dst, size) }
}

pub(crate) unsafe fn ip6_addr(
    ip: *const c_char,
    port: c_int,
    addr: *mut abi::sockaddr_in6,
) -> c_int {
    unsafe { upstream_support::uv_common::uv_ip6_addr(ip, port, addr.cast()) }
}

pub(crate) unsafe fn ip6_name(
    src: *const abi::sockaddr_in6,
    dst: *mut c_char,
    size: usize,
) -> c_int {
    unsafe { upstream_support::uv_common::uv_ip6_name(src.cast(), dst, size) }
}

pub(crate) unsafe fn ip_name(src: *const abi::sockaddr, dst: *mut c_char, size: usize) -> c_int {
    unsafe { upstream_support::uv_common::uv_ip_name(src.cast(), dst, size) }
}

pub(crate) unsafe fn os_getpid() -> abi::uv_pid_t {
    unsafe { upstream_support::unix_core::uv_os_getpid() as abi::uv_pid_t }
}

pub(crate) unsafe fn process_kill(handle: *mut abi::uv_process_t, signum: c_int) -> c_int {
    if handle.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { libc::kill((*handle).pid, signum) } == 0 {
        0
    } else {
        uv_err(unsafe { *libc::__errno_location() })
    }
}

pub(crate) unsafe fn pipe(fds: *mut abi::uv_file, read_flags: c_int, write_flags: c_int) -> c_int {
    unsafe { crate::unix::pipe::uv_pipe(fds.cast(), read_flags, write_flags) }
}

pub(crate) unsafe fn socketpair(
    type_: c_int,
    protocol: c_int,
    socket_vector: *mut abi::uv_os_sock_t,
    flags0: c_int,
    flags1: c_int,
) -> c_int {
    unsafe {
        crate::unix::tcp::uv_socketpair(type_, protocol, socket_vector.cast(), flags0, flags1)
    }
}

pub(crate) unsafe fn spawn(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_process_t,
    options: *const abi::uv_process_options_t,
) -> c_int {
    if loop_.is_null() || handle.is_null() || options.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { (*options).file.is_null() } {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let supported_flags = abi::uv_process_flags_UV_PROCESS_DETACHED
        | abi::uv_process_flags_UV_PROCESS_SETUID
        | abi::uv_process_flags_UV_PROCESS_SETGID;
    if unsafe { (*options).flags & !supported_flags } != 0 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let requested_stdio = unsafe { (*options).stdio_count.max(0) as usize };
    if requested_stdio > 0 && unsafe { (*options).stdio.is_null() } {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let stdio_count = requested_stdio.max(3);
    let mut pipes = vec![[-1; 2]; stdio_count];
    for (idx, pair) in pipes.iter_mut().enumerate().take(requested_stdio) {
        let rc = unsafe { init_spawn_stdio((*options).stdio.add(idx), pair) };
        if rc != 0 {
            unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
            return rc;
        }
    }

    let mut error_pipe = [-1; 2];
    if unsafe { libc::pipe2(error_pipe.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        let err = uv_err(unsafe { *libc::__errno_location() });
        unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
        return err;
    }

    let pid = unsafe { libc::fork() };
    if pid < 0 {
        let err = uv_err(unsafe { *libc::__errno_location() });
        unsafe {
            close_fd(error_pipe[0]);
            close_fd(error_pipe[1]);
            cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes);
        }
        return err;
    }

    if pid == 0 {
        unsafe {
            close_fd(error_pipe[0]);
            child_exec(options, stdio_count, &pipes, error_pipe[1]);
        }
    }

    unsafe {
        close_fd(error_pipe[1]);
    }

    let mut exec_error = 0;
    let read_rc = loop {
        let rc = unsafe {
            libc::read(
                error_pipe[0],
                (&mut exec_error as *mut c_int).cast(),
                std::mem::size_of::<c_int>(),
            )
        };
        if rc < 0 && unsafe { *libc::__errno_location() } == libc::EINTR {
            continue;
        }
        break rc;
    };
    unsafe {
        close_fd(error_pipe[0]);
    }

    if read_rc > 0 {
        let mut status = 0;
        loop {
            let rc = unsafe { libc::waitpid(pid, &mut status, 0) };
            if rc == pid || (rc < 0 && unsafe { *libc::__errno_location() } != libc::EINTR) {
                break;
            }
        }
        unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
        return exec_error;
    }
    if read_rc < 0 {
        let err = uv_err(unsafe { *libc::__errno_location() });
        unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
        return err;
    }

    for (idx, pair) in pipes.iter_mut().enumerate().take(requested_stdio) {
        let rc = unsafe { open_parent_spawn_stream((*options).stdio.add(idx), pair) };
        if rc != 0 {
            let _ = unsafe { libc::kill(pid, libc::SIGKILL) };
            let mut status = 0;
            let _ = unsafe { libc::waitpid(pid, &mut status, 0) };
            unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
            return rc;
        }
    }

    let rc = unsafe {
        handle::handle_init(
            loop_,
            handle.cast(),
            abi::uv_handle_type_UV_PROCESS,
            HandleKind::Process,
        )
    };
    if rc != 0 {
        let _ = unsafe { libc::kill(pid, libc::SIGKILL) };
        let mut status = 0;
        let _ = unsafe { libc::waitpid(pid, &mut status, 0) };
        return rc;
    }

    unsafe {
        (*handle).exit_cb = (*options).exit_cb;
        (*handle).pid = pid;
        (*handle).status = 0;
        queue::init(std::ptr::addr_of_mut!((*handle).queue));
        queue::insert_tail(
            std::ptr::addr_of_mut!((*loop_).process_handles),
            std::ptr::addr_of_mut!((*handle).queue),
        );
        (*loop_).flags |= crate::upstream_support::unix_core::UV_LOOP_REAP_CHILDREN as u64;
        handle::handle_start(handle.cast());
    }

    0
}

pub(crate) unsafe fn translate_sys_error(sys_errno: c_int) -> c_int {
    unsafe { upstream_support::unix_core::uv_translate_sys_error(sys_errno) }
}

pub(crate) unsafe fn recv_buffer_size(handle: *mut abi::uv_handle_t, value: *mut c_int) -> c_int {
    unsafe { upstream_support::uv_common::uv_recv_buffer_size(handle.cast(), value) }
}

pub(crate) unsafe fn send_buffer_size(handle: *mut abi::uv_handle_t, value: *mut c_int) -> c_int {
    unsafe { upstream_support::uv_common::uv_send_buffer_size(handle.cast(), value) }
}
