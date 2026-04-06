use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_guess_handle"]
        pub(super) fn uv_guess_handle(file: uv_file) -> uv_handle_type;
        #[link_name = "uv_phase5_private_uv_tty_get_vterm_state"]
        pub(super) fn uv_tty_get_vterm_state(state: *mut uv_tty_vtermstate_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tty_get_winsize"]
        pub(super) fn uv_tty_get_winsize(
            tty: *mut uv_tty_t,
            width: *mut libc::c_int,
            height: *mut libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tty_init"]
        pub(super) fn uv_tty_init(
            loop_: *mut uv_loop_t,
            tty: *mut uv_tty_t,
            fd: uv_file,
            readable: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tty_reset_mode"]
        pub(super) fn uv_tty_reset_mode() -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tty_set_mode"]
        pub(super) fn uv_tty_set_mode(tty: *mut uv_tty_t, mode: uv_tty_mode_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_tty_set_vterm_state"]
        pub(super) fn uv_tty_set_vterm_state(state: uv_tty_vtermstate_t);
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_tty_init(
    loop_: *mut uv_loop_t,
    tty: *mut uv_tty_t,
    fd: uv_file,
    readable: libc::c_int,
) -> libc::c_int {
    private::uv_tty_init(loop_, tty, fd, readable)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tty_set_mode(tty: *mut uv_tty_t, mode: uv_tty_mode_t) -> libc::c_int {
    private::uv_tty_set_mode(tty, mode)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tty_reset_mode() -> libc::c_int {
    private::uv_tty_reset_mode()
}

#[no_mangle]
pub unsafe extern "C" fn uv_tty_get_winsize(
    tty: *mut uv_tty_t,
    width: *mut libc::c_int,
    height: *mut libc::c_int,
) -> libc::c_int {
    private::uv_tty_get_winsize(tty, width, height)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tty_set_vterm_state(state: uv_tty_vtermstate_t) {
    private::uv_tty_set_vterm_state(state)
}

#[no_mangle]
pub unsafe extern "C" fn uv_tty_get_vterm_state(state: *mut uv_tty_vtermstate_t) -> libc::c_int {
    private::uv_tty_get_vterm_state(state)
}

#[no_mangle]
pub unsafe extern "C" fn uv_guess_handle(file: uv_file) -> uv_handle_type {
    private::uv_guess_handle(file)
}
