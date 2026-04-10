use crate::abi::linux_x86_64 as abi;
use crate::{threading, upstream_support};
use libc::{self, c_void};
use std::os::raw::{c_char, c_int, c_uint};

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn process_reap(loop_: *mut abi::uv_loop_t) {
    unsafe { unsafe { crate::unix::process::process_reap(loop_) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn disable_stdio_inheritance() {
    unsafe {
        unsafe {
            upstream_support::unix_core::uv_disable_stdio_inheritance();
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn exepath(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe {
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fs_read(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        unsafe {
            threading::threadpool::fs_read(loop_, req, file, bufs, nbufs as c_int, offset, cb)
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fs_req_cleanup(req: *mut abi::uv_fs_t) {
    unsafe {
        unsafe {
            threading::threadpool::fs_req_cleanup(req);
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fs_unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe { unsafe { threading::threadpool::fs_unlink(loop_, req, path, cb) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fileno(handle: *const abi::uv_handle_t, fd: *mut abi::uv_os_fd_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_fileno(handle.cast(), fd.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn free_interface_addresses(addresses: *mut abi::uv_interface_address_t, count: c_int) {
    unsafe {
        unsafe {
            crate::unix::epoll::uv_free_interface_addresses(addresses.cast(), count);
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn guess_handle(file: abi::uv_file) -> abi::uv_handle_type {
    unsafe {
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: usize) -> c_int {
    unsafe { unsafe { upstream_support::inet::uv_inet_ntop(af, src.cast(), dst, size) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int {
    unsafe { unsafe { upstream_support::inet::uv_inet_pton(af, src, dst.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn interface_addresses(
    addresses: *mut *mut abi::uv_interface_address_t,
    count: *mut c_int,
) -> c_int {
    unsafe { unsafe { crate::unix::epoll::uv_interface_addresses(addresses.cast(), count) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ip4_addr(ip: *const c_char, port: c_int, addr: *mut abi::sockaddr_in) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_ip4_addr(ip, port, addr.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ip4_name(src: *const abi::sockaddr_in, dst: *mut c_char, size: usize) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_ip4_name(src.cast(), dst, size) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ip6_addr(ip: *const c_char, port: c_int, addr: *mut abi::sockaddr_in6) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_ip6_addr(ip, port, addr.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ip6_name(src: *const abi::sockaddr_in6, dst: *mut c_char, size: usize) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_ip6_name(src.cast(), dst, size) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ip_name(src: *const abi::sockaddr, dst: *mut c_char, size: usize) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_ip_name(src.cast(), dst, size) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_getpid() -> abi::uv_pid_t {
    unsafe { unsafe { upstream_support::unix_core::uv_os_getpid() as abi::uv_pid_t } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn process_kill(handle: *mut abi::uv_process_t, signum: c_int) -> c_int {
    unsafe { unsafe { crate::unix::process::process_kill(handle, signum) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn pipe(fds: *mut abi::uv_file, read_flags: c_int, write_flags: c_int) -> c_int {
    unsafe { unsafe { crate::unix::pipe::uv_pipe(fds.cast(), read_flags, write_flags) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn socketpair(
    type_: c_int,
    protocol: c_int,
    socket_vector: *mut abi::uv_os_sock_t,
    flags0: c_int,
    flags1: c_int,
) -> c_int {
    unsafe {
        unsafe {
            crate::unix::tcp::uv_socketpair(type_, protocol, socket_vector.cast(), flags0, flags1)
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn spawn(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_process_t,
    options: *const abi::uv_process_options_t,
) -> c_int {
    unsafe { unsafe { crate::unix::process::spawn(loop_, handle, options) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn translate_sys_error(sys_errno: c_int) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_translate_sys_error(sys_errno) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn recv_buffer_size(handle: *mut abi::uv_handle_t, value: *mut c_int) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_recv_buffer_size(handle.cast(), value) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn send_buffer_size(handle: *mut abi::uv_handle_t, value: *mut c_int) -> c_int {
    unsafe { unsafe { upstream_support::uv_common::uv_send_buffer_size(handle.cast(), value) } }
}
