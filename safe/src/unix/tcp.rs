use crate::abi::linux_x86_64 as abi;
use std::os::raw::{c_int, c_uint};

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_tcp_bind"]
        pub fn uv_tcp_bind(
            handle: *mut abi::uv_tcp_t,
            addr: *const abi::sockaddr,
            flags: c_uint,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tcp_close_reset"]
        pub fn uv_tcp_close_reset(handle: *mut abi::uv_tcp_t, close_cb: abi::uv_close_cb)
            -> c_int;
        #[link_name = "uv_internal_uv_tcp_connect"]
        pub fn uv_tcp_connect(
            req: *mut abi::uv_connect_t,
            handle: *mut abi::uv_tcp_t,
            addr: *const abi::sockaddr,
            cb: abi::uv_connect_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tcp_getpeername"]
        pub fn uv_tcp_getpeername(
            handle: *const abi::uv_tcp_t,
            name: *mut abi::sockaddr,
            namelen: *mut c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tcp_getsockname"]
        pub fn uv_tcp_getsockname(
            handle: *const abi::uv_tcp_t,
            name: *mut abi::sockaddr,
            namelen: *mut c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tcp_init"]
        pub fn uv_tcp_init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_tcp_t) -> c_int;
        #[link_name = "uv_internal_uv_tcp_init_ex"]
        pub fn uv_tcp_init_ex(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_tcp_t,
            flags: c_uint,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tcp_keepalive"]
        pub fn uv_tcp_keepalive(handle: *mut abi::uv_tcp_t, enable: c_int, delay: c_uint)
            -> c_int;
        #[link_name = "uv_internal_uv_tcp_nodelay"]
        pub fn uv_tcp_nodelay(handle: *mut abi::uv_tcp_t, enable: c_int) -> c_int;
        #[link_name = "uv_internal_uv_tcp_open"]
        pub fn uv_tcp_open(handle: *mut abi::uv_tcp_t, sock: abi::uv_os_sock_t) -> c_int;
        #[link_name = "uv_internal_uv_tcp_simultaneous_accepts"]
        pub fn uv_tcp_simultaneous_accepts(handle: *mut abi::uv_tcp_t, enable: c_int) -> c_int;
    }
}

pub(crate) unsafe fn bind(
    handle: *mut abi::uv_tcp_t,
    addr: *const abi::sockaddr,
    flags: c_uint,
) -> c_int {
    unsafe { raw::uv_tcp_bind(handle, addr, flags) }
}

pub(crate) unsafe fn close_reset(handle: *mut abi::uv_tcp_t, close_cb: abi::uv_close_cb) -> c_int {
    unsafe { raw::uv_tcp_close_reset(handle, close_cb) }
}

pub(crate) unsafe fn connect(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_tcp_t,
    addr: *const abi::sockaddr,
    cb: abi::uv_connect_cb,
) -> c_int {
    unsafe { raw::uv_tcp_connect(req, handle, addr, cb) }
}

pub(crate) unsafe fn getpeername(
    handle: *const abi::uv_tcp_t,
    name: *mut abi::sockaddr,
    namelen: *mut c_int,
) -> c_int {
    unsafe { raw::uv_tcp_getpeername(handle, name, namelen) }
}

pub(crate) unsafe fn getsockname(
    handle: *const abi::uv_tcp_t,
    name: *mut abi::sockaddr,
    namelen: *mut c_int,
) -> c_int {
    unsafe { raw::uv_tcp_getsockname(handle, name, namelen) }
}

pub(crate) unsafe fn init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_tcp_t) -> c_int {
    unsafe { raw::uv_tcp_init(loop_, handle) }
}

pub(crate) unsafe fn init_ex(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tcp_t,
    flags: c_uint,
) -> c_int {
    unsafe { raw::uv_tcp_init_ex(loop_, handle, flags) }
}

pub(crate) unsafe fn keepalive(handle: *mut abi::uv_tcp_t, enable: c_int, delay: c_uint) -> c_int {
    unsafe { raw::uv_tcp_keepalive(handle, enable, delay) }
}

pub(crate) unsafe fn nodelay(handle: *mut abi::uv_tcp_t, enable: c_int) -> c_int {
    unsafe { raw::uv_tcp_nodelay(handle, enable) }
}

pub(crate) unsafe fn open(handle: *mut abi::uv_tcp_t, sock: abi::uv_os_sock_t) -> c_int {
    unsafe { raw::uv_tcp_open(handle, sock) }
}

pub(crate) unsafe fn simultaneous_accepts(handle: *mut abi::uv_tcp_t, enable: c_int) -> c_int {
    unsafe { raw::uv_tcp_simultaneous_accepts(handle, enable) }
}
