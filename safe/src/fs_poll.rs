use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_fs_poll_getpath"]
        pub(super) fn uv_fs_poll_getpath(
            handle: *mut uv_fs_poll_t,
            buffer: *mut libc::c_char,
            size: *mut usize,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_fs_poll_init"]
        pub(super) fn uv_fs_poll_init(
            loop_: *mut uv_loop_t,
            handle: *mut uv_fs_poll_t,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_fs_poll_start"]
        pub(super) fn uv_fs_poll_start(
            handle: *mut uv_fs_poll_t,
            poll_cb: uv_fs_poll_cb,
            path: *const libc::c_char,
            interval: libc::c_uint,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_fs_poll_stop"]
        pub(super) fn uv_fs_poll_stop(handle: *mut uv_fs_poll_t) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_poll_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_fs_poll_t,
) -> libc::c_int {
    private::uv_fs_poll_init(loop_, handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_poll_start(
    handle: *mut uv_fs_poll_t,
    poll_cb: uv_fs_poll_cb,
    path: *const libc::c_char,
    interval: libc::c_uint,
) -> libc::c_int {
    private::uv_fs_poll_start(handle, poll_cb, path, interval)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_poll_stop(handle: *mut uv_fs_poll_t) -> libc::c_int {
    private::uv_fs_poll_stop(handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_poll_getpath(
    handle: *mut uv_fs_poll_t,
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    private::uv_fs_poll_getpath(handle, buffer, size)
}
