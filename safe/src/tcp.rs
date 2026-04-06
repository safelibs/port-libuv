use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_tcp_close_reset"]
        pub(super) fn uv_tcp_close_reset(
            handle: *mut uv_tcp_t,
            close_cb: uv_close_cb,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_getpeername"]
        pub(super) fn uv_tcp_getpeername(
            handle: *const uv_tcp_t,
            name: *mut sockaddr,
            namelen: *mut libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_getsockname"]
        pub(super) fn uv_tcp_getsockname(
            handle: *const uv_tcp_t,
            name: *mut sockaddr,
            namelen: *mut libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_init"]
        pub(super) fn uv_tcp_init(loop_: *mut uv_loop_t, handle: *mut uv_tcp_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_init_ex"]
        pub(super) fn uv_tcp_init_ex(
            loop_: *mut uv_loop_t,
            handle: *mut uv_tcp_t,
            flags: libc::c_uint,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_keepalive"]
        pub(super) fn uv_tcp_keepalive(
            handle: *mut uv_tcp_t,
            enable: libc::c_int,
            delay: libc::c_uint,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_nodelay"]
        pub(super) fn uv_tcp_nodelay(handle: *mut uv_tcp_t, enable: libc::c_int) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_open"]
        pub(super) fn uv_tcp_open(handle: *mut uv_tcp_t, sock: uv_os_sock_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tcp_simultaneous_accepts"]
        pub(super) fn uv_tcp_simultaneous_accepts(
            handle: *mut uv_tcp_t,
            enable: libc::c_int,
        ) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_init(loop_: *mut uv_loop_t, handle: *mut uv_tcp_t) -> libc::c_int {
    private::uv_tcp_init(loop_, handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_init_ex(
    loop_: *mut uv_loop_t,
    handle: *mut uv_tcp_t,
    flags: libc::c_uint,
) -> libc::c_int {
    private::uv_tcp_init_ex(loop_, handle, flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_open(handle: *mut uv_tcp_t, sock: uv_os_sock_t) -> libc::c_int {
    private::uv_tcp_open(handle, sock)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_nodelay(handle: *mut uv_tcp_t, enable: libc::c_int) -> libc::c_int {
    private::uv_tcp_nodelay(handle, enable)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_keepalive(
    handle: *mut uv_tcp_t,
    enable: libc::c_int,
    delay: libc::c_uint,
) -> libc::c_int {
    private::uv_tcp_keepalive(handle, enable, delay)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_simultaneous_accepts(
    handle: *mut uv_tcp_t,
    enable: libc::c_int,
) -> libc::c_int {
    private::uv_tcp_simultaneous_accepts(handle, enable)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_bind(
    handle: *mut uv_tcp_t,
    addr: *const sockaddr,
    flags: libc::c_uint,
) -> libc::c_int {
    crate::private_support::uv_tcp_bind(handle, addr, flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_getsockname(
    handle: *const uv_tcp_t,
    name: *mut sockaddr,
    namelen: *mut libc::c_int,
) -> libc::c_int {
    private::uv_tcp_getsockname(handle, name, namelen)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_getpeername(
    handle: *const uv_tcp_t,
    name: *mut sockaddr,
    namelen: *mut libc::c_int,
) -> libc::c_int {
    private::uv_tcp_getpeername(handle, name, namelen)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_close_reset(
    handle: *mut uv_tcp_t,
    close_cb: uv_close_cb,
) -> libc::c_int {
    private::uv_tcp_close_reset(handle, close_cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tcp_connect(
    req: *mut uv_connect_t,
    handle: *mut uv_tcp_t,
    addr: *const sockaddr,
    cb: uv_connect_cb,
) -> libc::c_int {
    crate::private_support::uv_tcp_connect(req, handle, addr, cb)
}
