use crate::abi::linux_x86_64 as abi;
use crate::core::{queue, UV_HANDLE_ACTIVE, UV_HANDLE_INTERNAL, UV_HANDLE_REF};
use crate::threading::sync;
use crate::upstream_support;
use libc::{self, c_void};
use std::mem::offset_of;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    static mut environ: *mut *mut c_char;
}

unsafe extern "C" fn child_watcher_cb(_handle: *mut abi::uv_signal_t, _signum: c_int) {}

#[inline]
fn uv_err(err: c_int) -> c_int {
    if err == 0 {
        0
    } else {
        -err
    }
}

#[inline]
fn last_errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

unsafe fn close_fd(fd: c_int) {
    if fd >= 0 {
        let _ = unsafe { libc::close(fd) };
    }
}

unsafe fn set_cloexec(fd: c_int, enabled: bool) -> c_int {
    loop {
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
        if flags >= 0 {
            let new_flags = if enabled {
                flags | libc::FD_CLOEXEC
            } else {
                flags & !libc::FD_CLOEXEC
            };
            if unsafe { libc::fcntl(fd, libc::F_SETFD, new_flags) } == 0 {
                return 0;
            }
            return uv_err(last_errno());
        }

        if last_errno() != libc::EINTR {
            return uv_err(last_errno());
        }
    }
}

unsafe fn clear_nonblock(fd: c_int) {
    loop {
        let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
        if flags >= 0 {
            let _ = unsafe { libc::fcntl(fd, libc::F_SETFL, flags & !libc::O_NONBLOCK) };
            return;
        }
        if last_errno() != libc::EINTR {
            return;
        }
    }
}

unsafe fn write_child_error(error_fd: c_int) -> ! {
    let err = uv_err(last_errno());
    let err_ptr = std::ptr::addr_of!(err).cast::<c_void>();

    loop {
        let rc = unsafe { libc::write(error_fd, err_ptr, std::mem::size_of::<c_int>()) };
        if rc >= 0 || last_errno() != libc::EINTR {
            break;
        }
    }

    unsafe { libc::_exit(127) }
}

unsafe fn handle_start(handle: *mut abi::uv_handle_t) {
    if unsafe { (*handle).flags & UV_HANDLE_ACTIVE } != 0 {
        return;
    }

    unsafe {
        (*handle).flags |= UV_HANDLE_ACTIVE;
        if (*handle).flags & UV_HANDLE_REF != 0 {
            (*(*handle).loop_).active_handles += 1;
        }
    }
}

unsafe fn handle_stop(handle: *mut abi::uv_handle_t) {
    if unsafe { (*handle).flags & UV_HANDLE_ACTIVE } == 0 {
        return;
    }

    unsafe {
        (*handle).flags &= !UV_HANDLE_ACTIVE;
        if (*handle).flags & UV_HANDLE_REF != 0 {
            (*(*handle).loop_).active_handles -= 1;
        }
    }
}

unsafe fn init_handle(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_process_t) {
    unsafe {
        std::ptr::write_bytes(handle, 0, 1);
        (*handle).loop_ = loop_;
        (*handle).type_ = abi::uv_handle_type_UV_PROCESS;
        (*handle).close_cb = None;
        (*handle).next_closing = std::ptr::null_mut();
        (*handle).flags = UV_HANDLE_REF;
        queue::init(std::ptr::addr_of_mut!((*handle).handle_queue));
        queue::init(std::ptr::addr_of_mut!((*handle).queue));
        queue::insert_tail(
            std::ptr::addr_of_mut!((*loop_).handle_queue),
            std::ptr::addr_of_mut!((*handle).handle_queue),
        );
    }
}

unsafe fn init_spawn_stdio(
    container: *mut abi::uv_stdio_container_t,
    fds: &mut [c_int; 2],
) -> c_int {
    let flags = unsafe { (*container).flags };
    let mask = abi::uv_stdio_flags_UV_IGNORE
        | abi::uv_stdio_flags_UV_CREATE_PIPE
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
    let mut stream_flags = 0;
    if (flags & abi::uv_stdio_flags_UV_WRITABLE_PIPE) != 0 {
        stream_flags |= crate::upstream_support::unix_core::UV_HANDLE_READABLE as c_int;
    }
    if (flags & abi::uv_stdio_flags_UV_READABLE_PIPE) != 0 {
        stream_flags |= crate::upstream_support::unix_core::UV_HANDLE_WRITABLE as c_int;
    }

    let rc = unsafe {
        crate::upstream_support::unix_core::uv__nonblock_ioctl(pair[0], 1);
        crate::unix::stream::uv__stream_open(stream.cast(), pair[0], stream_flags)
    };
    if rc == 0 {
        pair[0] = -1;
    }
    rc
}

unsafe fn close_parent_spawn_stream(container: *mut abi::uv_stdio_container_t) {
    if container.is_null() {
        return;
    }

    if unsafe { ((*container).flags & abi::uv_stdio_flags_UV_CREATE_PIPE) == 0 } {
        return;
    }

    let stream = unsafe { (*container).data.stream };
    if stream.is_null() {
        return;
    }

    unsafe {
        crate::unix::stream::uv__stream_close(stream.cast());
    }
}

unsafe fn dup_stdio_fds(stdio_count: usize, pipes: &mut [[c_int; 2]], error_fd: c_int) {
    for fd in 0..stdio_count {
        let use_fd = pipes[fd][1];
        if use_fd < 0 || use_fd >= fd as c_int {
            continue;
        }

        let duplicated = unsafe { libc::fcntl(use_fd, libc::F_DUPFD_CLOEXEC, stdio_count as c_int) };

        let duplicated = if duplicated >= 0 {
            duplicated
        } else if last_errno() == libc::EINVAL {
            let fd = unsafe { libc::fcntl(use_fd, libc::F_DUPFD, stdio_count as c_int) };
            if fd < 0 {
                unsafe { write_child_error(error_fd) };
            }
            if unsafe { set_cloexec(fd, true) } != 0 {
                unsafe { write_child_error(error_fd) };
            }
            fd
        } else {
            unsafe { write_child_error(error_fd) };
        };

        pipes[fd][1] = duplicated;
    }
}

fn signal_numbers() -> impl Iterator<Item = c_int> {
    1..32
}

fn is_uncatchable_signal(signum: c_int) -> bool {
    signum == libc::SIGKILL || signum == libc::SIGSTOP
}

unsafe fn reset_signal_dispositions(error_fd: c_int) {
    for signum in signal_numbers() {
        if is_uncatchable_signal(signum) {
            continue;
        }

        let previous = unsafe { libc::signal(signum, libc::SIG_DFL) };
        if previous == libc::SIG_ERR {
            unsafe { write_child_error(error_fd) };
        }
    }
}

fn child_stdio_mode(fd: usize) -> c_int {
    if fd == 0 {
        libc::O_RDONLY
    } else {
        libc::O_RDWR
    }
}

unsafe fn configure_child_stdio(stdio_count: usize, pipes: &mut [[c_int; 2]], error_fd: c_int) {
    unsafe { dup_stdio_fds(stdio_count, pipes, error_fd) };

    for fd in 0..stdio_count {
        let mut close_after_dup = -1;
        let mut use_fd = pipes[fd][1];

        if use_fd < 0 {
            if fd >= 3 {
                continue;
            }

            use_fd = unsafe { libc::open(c"/dev/null".as_ptr(), child_stdio_mode(fd)) };
            if use_fd < 0 {
                unsafe { write_child_error(error_fd) };
            }
            close_after_dup = use_fd;
        }

        if use_fd == fd as c_int {
            if close_after_dup == -1 {
                let rc = unsafe { set_cloexec(use_fd, false) };
                if rc != 0 {
                    unsafe { write_child_error(error_fd) };
                }
            }
        } else if unsafe { libc::dup2(use_fd, fd as c_int) } < 0 {
            unsafe { write_child_error(error_fd) };
        }

        if fd <= 2 && close_after_dup == -1 {
            unsafe { clear_nonblock(fd as c_int) };
        }

        if close_after_dup >= stdio_count as c_int {
            unsafe { close_fd(close_after_dup) };
        }
    }
}

unsafe fn close_child_pipe_fds(pipes: &[[c_int; 2]]) {
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
}

unsafe fn reset_signal_mask(error_fd: c_int) {
    let mut mask = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
    if unsafe { libc::sigemptyset(mask.as_mut_ptr()) } != 0 {
        unsafe { write_child_error(error_fd) };
    }

    if unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, mask.as_ptr(), std::ptr::null_mut()) } != 0
    {
        unsafe { write_child_error(error_fd) };
    }
}

fn want_credential_drop(flags: c_int) -> bool {
    (flags & abi::uv_process_flags_UV_PROCESS_SETGID as c_int) != 0
        || (flags & abi::uv_process_flags_UV_PROCESS_SETUID as c_int) != 0
}

fn process_flags(options: *const abi::uv_process_options_t) -> c_int {
    unsafe { (*options).flags as c_int }
}

pub(crate) unsafe fn apply_credential_drop(
    options: *const abi::uv_process_options_t,
    error_fd: c_int,
) {
    let flags = process_flags(options);
    if !want_credential_drop(flags) {
        return;
    }

    let _ = unsafe { libc::setgroups(0, std::ptr::null()) };

    if (flags & abi::uv_process_flags_UV_PROCESS_SETGID as c_int) != 0
        && unsafe { libc::setgid((*options).gid) } != 0
    {
        unsafe { write_child_error(error_fd) };
    }

    if (flags & abi::uv_process_flags_UV_PROCESS_SETUID as c_int) != 0
        && unsafe { libc::setuid((*options).uid) } != 0
    {
        unsafe { write_child_error(error_fd) };
    }
}

unsafe fn child_exec(
    options: *const abi::uv_process_options_t,
    stdio_count: usize,
    pipes: &mut [[c_int; 2]],
    error_fd: c_int,
) -> ! {
    unsafe { reset_signal_dispositions(error_fd) };

    if (process_flags(options) & abi::uv_process_flags_UV_PROCESS_DETACHED as c_int) != 0
        && unsafe { libc::setsid() } < 0
    {
        unsafe { write_child_error(error_fd) };
    }

    unsafe { configure_child_stdio(stdio_count, pipes, error_fd) };
    unsafe { close_child_pipe_fds(pipes) };

    if unsafe { !(*options).cwd.is_null() } && unsafe { libc::chdir((*options).cwd) } != 0 {
        unsafe { write_child_error(error_fd) };
    }

    unsafe { apply_credential_drop(options, error_fd) };

    if unsafe { !(*options).env.is_null() } {
        unsafe {
            environ = (*options).env.cast();
        }
    }

    unsafe { reset_signal_mask(error_fd) };
    unsafe {
        libc::execvp((*options).file, (*options).args.cast());
    }

    unsafe { write_child_error(error_fd) }
}

unsafe fn read_exec_error(error_fd: c_int, pid: c_int) -> c_int {
    let mut exec_error = 0;
    let read_rc = loop {
        let rc = unsafe {
            libc::read(
                error_fd,
                (&mut exec_error as *mut c_int).cast(),
                std::mem::size_of::<c_int>(),
            )
        };
        if rc < 0 && last_errno() == libc::EINTR {
            continue;
        }
        break rc;
    };

    unsafe {
        close_fd(error_fd);
    }

    match read_rc {
        0 => 0,
        rc if rc as usize == std::mem::size_of::<c_int>() => {
            let mut status = 0;
            loop {
                let rc = unsafe { libc::waitpid(pid, &mut status, 0) };
                if rc == pid || (rc < 0 && last_errno() != libc::EINTR) {
                    break;
                }
            }
            exec_error
        }
        -1 if last_errno() == libc::EPIPE => {
            let mut status = 0;
            loop {
                let rc = unsafe { libc::waitpid(pid, &mut status, 0) };
                if rc == pid || (rc < 0 && last_errno() != libc::EINTR) {
                    break;
                }
            }
            abi::uv_errno_t_UV_EPIPE
        }
        -1 => uv_err(last_errno()),
        _ => abi::uv_errno_t_UV_EPIPE,
    }
}

pub(crate) unsafe fn loop_init(loop_: *mut abi::uv_loop_t) -> c_int {
    if loop_.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let rc = unsafe { crate::unix::signal::init(loop_, std::ptr::addr_of_mut!((*loop_).child_watcher)) };
    if rc != 0 {
        return rc;
    }

    unsafe {
        crate::upstream_support::uv_common::uv_unref(
            std::ptr::addr_of_mut!((*loop_).child_watcher).cast(),
        );
        (*loop_).child_watcher.flags |= UV_HANDLE_INTERNAL;
    }

    0
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
            if rc < 0 && last_errno() == libc::EINTR {
                continue;
            }
            break rc;
        };

        if pid == 0 {
            continue;
        }

        if pid < 0 {
            let err = last_errno();
            if err != libc::ECHILD {
                continue;
            }
            continue;
        }

        exited.push((process, status));
    }

    for (process, status) in exited {
        unsafe {
            (*process).status = status;
            queue::remove(std::ptr::addr_of_mut!((*process).queue));
            queue::init(std::ptr::addr_of_mut!((*process).queue));
            handle_stop(process.cast());
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

}

pub(crate) unsafe fn close(handle: *mut abi::uv_process_t) {
    if handle.is_null() {
        return;
    }

    unsafe {
        queue::remove(std::ptr::addr_of_mut!((*handle).queue));
        queue::init(std::ptr::addr_of_mut!((*handle).queue));
        handle_stop(handle.cast());
    }

    let loop_ = unsafe { (*handle).loop_ };
    if !loop_.is_null() && unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).process_handles)) } {
        let _ = unsafe { crate::unix::signal::stop(std::ptr::addr_of_mut!((*loop_).child_watcher)) };
    }
}

pub(crate) unsafe fn process_kill(handle: *mut abi::uv_process_t, signum: c_int) -> c_int {
    if handle.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    unsafe { kill((*handle).pid, signum) }
}

pub(crate) unsafe fn kill(pid: c_int, signum: c_int) -> c_int {
    if unsafe { libc::kill(pid, signum) } == 0 {
        0
    } else {
        uv_err(last_errno())
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
        | abi::uv_process_flags_UV_PROCESS_SETGID
        | abi::uv_process_flags_UV_PROCESS_WINDOWS_FILE_PATH_EXACT_NAME
        | abi::uv_process_flags_UV_PROCESS_WINDOWS_HIDE
        | abi::uv_process_flags_UV_PROCESS_WINDOWS_HIDE_CONSOLE
        | abi::uv_process_flags_UV_PROCESS_WINDOWS_HIDE_GUI
        | abi::uv_process_flags_UV_PROCESS_WINDOWS_VERBATIM_ARGUMENTS;
    if unsafe { (*options).flags & !supported_flags } != 0 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let requested_stdio = unsafe { (*options).stdio_count.max(0) as usize };
    if requested_stdio > 0 && unsafe { (*options).stdio.is_null() } {
        return abi::uv_errno_t_UV_EINVAL;
    }

    unsafe { init_handle(loop_, handle) };

    let stdio_count = requested_stdio.max(3);
    let mut pipes = vec![[-1; 2]; stdio_count];
    for (idx, pair) in pipes.iter_mut().enumerate().take(requested_stdio) {
        let rc = unsafe { init_spawn_stdio((*options).stdio.add(idx), pair) };
        if rc != 0 {
            unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
            return rc;
        }
    }

    let rc = unsafe {
        crate::unix::signal::start_regular(
            std::ptr::addr_of_mut!((*loop_).child_watcher),
            Some(child_watcher_cb),
            libc::SIGCHLD,
        )
    };
    if rc != 0 {
        unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
        return rc;
    }

    let mut error_pipe = [-1; 2];
    if unsafe { libc::pipe2(error_pipe.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        let err = uv_err(last_errno());
        unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
        return err;
    }

    unsafe { sync::rwlock_wrlock_raw(std::ptr::addr_of_mut!((*loop_).cloexec_lock)) };
    let pid = unsafe { libc::fork() };
    if pid == 0 {
        unsafe {
            sync::rwlock_wrunlock_raw(std::ptr::addr_of_mut!((*loop_).cloexec_lock));
            close_fd(error_pipe[0]);
            child_exec(options, stdio_count, &mut pipes, error_pipe[1]);
        }
    }
    unsafe { sync::rwlock_wrunlock_raw(std::ptr::addr_of_mut!((*loop_).cloexec_lock)) };

    if pid < 0 {
        let err = uv_err(last_errno());
        unsafe {
            close_fd(error_pipe[0]);
            close_fd(error_pipe[1]);
            cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes);
        }
        return err;
    }

    unsafe {
        close_fd(error_pipe[1]);
    }

    let exec_error = unsafe { read_exec_error(error_pipe[0], pid) };

    // Match upstream libuv: requested parent stdio is opened even when exec()
    // fails, and a successfully exec'd child remains reaped by the loop if a
    // later parent-side stdio open fails.
    if exec_error == 0 {
        unsafe {
            (*handle).exit_cb = (*options).exit_cb;
            (*handle).pid = pid;
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).process_handles),
                std::ptr::addr_of_mut!((*handle).queue),
            );
            handle_start(handle.cast());
        }
    }

    for (idx, pair) in pipes.iter_mut().enumerate().take(requested_stdio) {
        let rc = unsafe { open_parent_spawn_stream((*options).stdio.add(idx), pair) };
        if rc != 0 {
            let mut open_idx = idx;
            while open_idx > 0 {
                open_idx -= 1;
                unsafe {
                    close_parent_spawn_stream((*options).stdio.add(open_idx));
                }
            }
            unsafe { cleanup_spawn_pipes((*options).stdio, requested_stdio, &mut pipes) };
            return rc;
        }
    }

    exec_error
}
