use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_signal_init"]
        pub(super) fn uv_signal_init(
            loop_: *mut uv_loop_t,
            handle: *mut uv_signal_t,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_signal_start"]
        pub(super) fn uv_signal_start(
            handle: *mut uv_signal_t,
            signal_cb: uv_signal_cb,
            signum: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_signal_start_oneshot"]
        pub(super) fn uv_signal_start_oneshot(
            handle: *mut uv_signal_t,
            signal_cb: uv_signal_cb,
            signum: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_signal_stop"]
        pub(super) fn uv_signal_stop(handle: *mut uv_signal_t) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_signal_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_signal_t,
) -> libc::c_int {
    private::uv_signal_init(loop_, handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_signal_start(
    handle: *mut uv_signal_t,
    signal_cb: uv_signal_cb,
    signum: libc::c_int,
) -> libc::c_int {
    private::uv_signal_start(handle, signal_cb, signum)
}

#[no_mangle]
pub unsafe extern "C" fn uv_signal_start_oneshot(
    handle: *mut uv_signal_t,
    signal_cb: uv_signal_cb,
    signum: libc::c_int,
) -> libc::c_int {
    private::uv_signal_start_oneshot(handle, signal_cb, signum)
}

#[no_mangle]
pub unsafe extern "C" fn uv_signal_stop(handle: *mut uv_signal_t) -> libc::c_int {
    private::uv_signal_stop(handle)
}
