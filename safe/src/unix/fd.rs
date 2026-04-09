use crate::abi::linux_x86_64 as abi;
use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_uint};

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_disable_stdio_inheritance"]
        pub fn uv_disable_stdio_inheritance();
        #[link_name = "uv_internal_uv_exepath"]
        pub fn uv_exepath(buffer: *mut c_char, size: *mut usize) -> c_int;
        #[link_name = "uv_internal_uv_fs_read"]
        pub fn uv_fs_read(
            loop_: *mut abi::uv_loop_t,
            req: *mut abi::uv_fs_t,
            file: abi::uv_file,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
            offset: i64,
            cb: abi::uv_fs_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_fs_req_cleanup"]
        pub fn uv_fs_req_cleanup(req: *mut abi::uv_fs_t);
        #[link_name = "uv_internal_uv_fs_unlink"]
        pub fn uv_fs_unlink(
            loop_: *mut abi::uv_loop_t,
            req: *mut abi::uv_fs_t,
            path: *const c_char,
            cb: abi::uv_fs_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_fileno"]
        pub fn uv_fileno(handle: *const abi::uv_handle_t, fd: *mut abi::uv_os_fd_t) -> c_int;
        #[link_name = "uv_internal_uv_free_interface_addresses"]
        pub fn uv_free_interface_addresses(
            addresses: *mut abi::uv_interface_address_t,
            count: c_int,
        );
        #[link_name = "uv_internal_uv_guess_handle"]
        pub fn uv_guess_handle(file: abi::uv_file) -> abi::uv_handle_type;
        #[link_name = "uv_internal_uv_inet_ntop"]
        pub fn uv_inet_ntop(
            af: c_int,
            src: *const c_void,
            dst: *mut c_char,
            size: usize,
        ) -> c_int;
        #[link_name = "uv_internal_uv_inet_pton"]
        pub fn uv_inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
        #[link_name = "uv_internal_uv_interface_addresses"]
        pub fn uv_interface_addresses(
            addresses: *mut *mut abi::uv_interface_address_t,
            count: *mut c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_ip4_addr"]
        pub fn uv_ip4_addr(
            ip: *const c_char,
            port: c_int,
            addr: *mut abi::sockaddr_in,
        ) -> c_int;
        #[link_name = "uv_internal_uv_ip4_name"]
        pub fn uv_ip4_name(src: *const abi::sockaddr_in, dst: *mut c_char, size: usize) -> c_int;
        #[link_name = "uv_internal_uv_ip6_addr"]
        pub fn uv_ip6_addr(
            ip: *const c_char,
            port: c_int,
            addr: *mut abi::sockaddr_in6,
        ) -> c_int;
        #[link_name = "uv_internal_uv_ip6_name"]
        pub fn uv_ip6_name(
            src: *const abi::sockaddr_in6,
            dst: *mut c_char,
            size: usize,
        ) -> c_int;
        #[link_name = "uv_internal_uv_ip_name"]
        pub fn uv_ip_name(src: *const abi::sockaddr, dst: *mut c_char, size: usize) -> c_int;
        #[link_name = "uv_internal_uv_os_getpid"]
        pub fn uv_os_getpid() -> abi::uv_pid_t;
        #[link_name = "uv_internal_uv_process_kill"]
        pub fn uv_process_kill(handle: *mut abi::uv_process_t, signum: c_int) -> c_int;
        #[link_name = "uv_internal_uv_pipe"]
        pub fn uv_pipe(fds: *mut abi::uv_file, read_flags: c_int, write_flags: c_int) -> c_int;
        #[link_name = "uv_internal_uv_socketpair"]
        pub fn uv_socketpair(
            type_: c_int,
            protocol: c_int,
            socket_vector: *mut abi::uv_os_sock_t,
            flags0: c_int,
            flags1: c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_spawn"]
        pub fn uv_spawn(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_process_t,
            options: *const abi::uv_process_options_t,
        ) -> c_int;
    }
}

pub(crate) unsafe fn disable_stdio_inheritance() {
    unsafe { raw::uv_disable_stdio_inheritance() }
}

pub(crate) unsafe fn exepath(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { raw::uv_exepath(buffer, size) }
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
    unsafe { raw::uv_fs_read(loop_, req, file, bufs, nbufs, offset, cb) }
}

pub(crate) unsafe fn fs_req_cleanup(req: *mut abi::uv_fs_t) {
    unsafe { raw::uv_fs_req_cleanup(req) }
}

pub(crate) unsafe fn fs_unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe { raw::uv_fs_unlink(loop_, req, path, cb) }
}

pub(crate) unsafe fn fileno(handle: *const abi::uv_handle_t, fd: *mut abi::uv_os_fd_t) -> c_int {
    unsafe { raw::uv_fileno(handle, fd) }
}

pub(crate) unsafe fn free_interface_addresses(
    addresses: *mut abi::uv_interface_address_t,
    count: c_int,
) {
    unsafe { raw::uv_free_interface_addresses(addresses, count) }
}

pub(crate) unsafe fn guess_handle(file: abi::uv_file) -> abi::uv_handle_type {
    unsafe { raw::uv_guess_handle(file) }
}

pub(crate) unsafe fn inet_ntop(
    af: c_int,
    src: *const c_void,
    dst: *mut c_char,
    size: usize,
) -> c_int {
    unsafe { raw::uv_inet_ntop(af, src, dst, size) }
}

pub(crate) unsafe fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int {
    unsafe { raw::uv_inet_pton(af, src, dst) }
}

pub(crate) unsafe fn interface_addresses(
    addresses: *mut *mut abi::uv_interface_address_t,
    count: *mut c_int,
) -> c_int {
    unsafe { raw::uv_interface_addresses(addresses, count) }
}

pub(crate) unsafe fn ip4_addr(
    ip: *const c_char,
    port: c_int,
    addr: *mut abi::sockaddr_in,
) -> c_int {
    unsafe { raw::uv_ip4_addr(ip, port, addr) }
}

pub(crate) unsafe fn ip4_name(
    src: *const abi::sockaddr_in,
    dst: *mut c_char,
    size: usize,
) -> c_int {
    unsafe { raw::uv_ip4_name(src, dst, size) }
}

pub(crate) unsafe fn ip6_addr(
    ip: *const c_char,
    port: c_int,
    addr: *mut abi::sockaddr_in6,
) -> c_int {
    unsafe { raw::uv_ip6_addr(ip, port, addr) }
}

pub(crate) unsafe fn ip6_name(
    src: *const abi::sockaddr_in6,
    dst: *mut c_char,
    size: usize,
) -> c_int {
    unsafe { raw::uv_ip6_name(src, dst, size) }
}

pub(crate) unsafe fn ip_name(src: *const abi::sockaddr, dst: *mut c_char, size: usize) -> c_int {
    unsafe { raw::uv_ip_name(src, dst, size) }
}

pub(crate) unsafe fn os_getpid() -> abi::uv_pid_t {
    unsafe { raw::uv_os_getpid() }
}

pub(crate) unsafe fn process_kill(handle: *mut abi::uv_process_t, signum: c_int) -> c_int {
    unsafe { raw::uv_process_kill(handle, signum) }
}

pub(crate) unsafe fn pipe(fds: *mut abi::uv_file, read_flags: c_int, write_flags: c_int) -> c_int {
    unsafe { raw::uv_pipe(fds, read_flags, write_flags) }
}

pub(crate) unsafe fn socketpair(
    type_: c_int,
    protocol: c_int,
    socket_vector: *mut abi::uv_os_sock_t,
    flags0: c_int,
    flags1: c_int,
) -> c_int {
    unsafe { raw::uv_socketpair(type_, protocol, socket_vector, flags0, flags1) }
}

pub(crate) unsafe fn spawn(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_process_t,
    options: *const abi::uv_process_options_t,
) -> c_int {
    unsafe { raw::uv_spawn(loop_, handle, options) }
}
