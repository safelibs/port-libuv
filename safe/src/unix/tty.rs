use crate::abi::linux_x86_64 as abi;
use std::os::raw::c_int;

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_tty_get_vterm_state"]
        pub fn uv_tty_get_vterm_state(state: *mut abi::uv_tty_vtermstate_t) -> c_int;
        #[link_name = "uv_internal_uv_tty_get_winsize"]
        pub fn uv_tty_get_winsize(
            handle: *mut abi::uv_tty_t,
            width: *mut c_int,
            height: *mut c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tty_init"]
        pub fn uv_tty_init(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_tty_t,
            fd: abi::uv_file,
            readable: c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_tty_reset_mode"]
        pub fn uv_tty_reset_mode() -> c_int;
        #[link_name = "uv_internal_uv_tty_set_mode"]
        pub fn uv_tty_set_mode(handle: *mut abi::uv_tty_t, mode: abi::uv_tty_mode_t) -> c_int;
        #[link_name = "uv_internal_uv_tty_set_vterm_state"]
        pub fn uv_tty_set_vterm_state(state: abi::uv_tty_vtermstate_t);
    }
}

pub(crate) unsafe fn get_vterm_state(state: *mut abi::uv_tty_vtermstate_t) -> c_int {
    unsafe { raw::uv_tty_get_vterm_state(state) }
}

pub(crate) unsafe fn get_winsize(
    handle: *mut abi::uv_tty_t,
    width: *mut c_int,
    height: *mut c_int,
) -> c_int {
    unsafe { raw::uv_tty_get_winsize(handle, width, height) }
}

pub(crate) unsafe fn init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tty_t,
    fd: abi::uv_file,
    readable: c_int,
) -> c_int {
    unsafe { raw::uv_tty_init(loop_, handle, fd, readable) }
}

pub(crate) unsafe fn reset_mode() -> c_int {
    unsafe { raw::uv_tty_reset_mode() }
}

pub(crate) unsafe fn set_mode(handle: *mut abi::uv_tty_t, mode: abi::uv_tty_mode_t) -> c_int {
    unsafe { raw::uv_tty_set_mode(handle, mode) }
}

pub(crate) unsafe fn set_vterm_state(state: abi::uv_tty_vtermstate_t) {
    unsafe { raw::uv_tty_set_vterm_state(state) }
}
