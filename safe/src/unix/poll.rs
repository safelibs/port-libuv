use crate::abi::linux_x86_64 as abi;
use std::os::raw::c_int;

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_poll_init"]
        pub fn uv_poll_init(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_poll_t,
            fd: c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_poll_init_socket"]
        pub fn uv_poll_init_socket(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_poll_t,
            socket: abi::uv_os_sock_t,
        ) -> c_int;
        #[link_name = "uv_internal_uv_poll_start"]
        pub fn uv_poll_start(
            handle: *mut abi::uv_poll_t,
            events: c_int,
            cb: abi::uv_poll_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_poll_stop"]
        pub fn uv_poll_stop(handle: *mut abi::uv_poll_t) -> c_int;
    }
}

pub(crate) unsafe fn init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_poll_t,
    fd: c_int,
) -> c_int {
    unsafe { raw::uv_poll_init(loop_, handle, fd) }
}

pub(crate) unsafe fn init_socket(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_poll_t,
    socket: abi::uv_os_sock_t,
) -> c_int {
    unsafe { raw::uv_poll_init_socket(loop_, handle, socket) }
}

pub(crate) unsafe fn start(
    handle: *mut abi::uv_poll_t,
    events: c_int,
    cb: abi::uv_poll_cb,
) -> c_int {
    unsafe { raw::uv_poll_start(handle, events, cb) }
}

pub(crate) unsafe fn stop(handle: *mut abi::uv_poll_t) -> c_int {
    unsafe { raw::uv_poll_stop(handle) }
}
