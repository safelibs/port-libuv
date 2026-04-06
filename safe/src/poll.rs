use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_poll_init"]
        pub(super) fn uv_poll_init(
            loop_: *mut uv_loop_t,
            handle: *mut uv_poll_t,
            fd: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_poll_init_socket"]
        pub(super) fn uv_poll_init_socket(
            loop_: *mut uv_loop_t,
            handle: *mut uv_poll_t,
            socket: uv_os_sock_t,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_poll_start"]
        pub(super) fn uv_poll_start(
            handle: *mut uv_poll_t,
            events: libc::c_int,
            cb: uv_poll_cb,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_poll_stop"]
        pub(super) fn uv_poll_stop(handle: *mut uv_poll_t) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_poll_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_poll_t,
    fd: libc::c_int,
) -> libc::c_int {
    private::uv_poll_init(loop_, handle, fd)
}

#[no_mangle]
pub unsafe extern "C" fn uv_poll_init_socket(
    loop_: *mut uv_loop_t,
    handle: *mut uv_poll_t,
    socket: uv_os_sock_t,
) -> libc::c_int {
    private::uv_poll_init_socket(loop_, handle, socket)
}

#[no_mangle]
pub unsafe extern "C" fn uv_poll_start(
    handle: *mut uv_poll_t,
    events: libc::c_int,
    cb: uv_poll_cb,
) -> libc::c_int {
    private::uv_poll_start(handle, events, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_poll_stop(handle: *mut uv_poll_t) -> libc::c_int {
    private::uv_poll_stop(handle)
}
