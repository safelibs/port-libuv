use crate::abi::linux_x86_64 as abi;
use std::os::raw::c_int;

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_backend_fd"]
        pub fn uv_backend_fd(loop_: *const abi::uv_loop_t) -> c_int;
        #[link_name = "uv_internal_uv_backend_timeout"]
        pub fn uv_backend_timeout(loop_: *const abi::uv_loop_t) -> c_int;
        #[link_name = "uv_internal_uv_close"]
        pub fn uv_close(handle: *mut abi::uv_handle_t, close_cb: abi::uv_close_cb);
        #[link_name = "uv_internal_uv_default_loop"]
        pub fn uv_default_loop() -> *mut abi::uv_loop_t;
        #[link_name = "uv_internal_uv_is_active"]
        pub fn uv_is_active(handle: *const abi::uv_handle_t) -> c_int;
        #[link_name = "uv_internal_uv_is_closing"]
        pub fn uv_is_closing(handle: *const abi::uv_handle_t) -> c_int;
        #[link_name = "uv_internal_uv_loop_close"]
        pub fn uv_loop_close(loop_: *mut abi::uv_loop_t) -> c_int;
        #[link_name = "uv_internal_uv_loop_init"]
        pub fn uv_loop_init(loop_: *mut abi::uv_loop_t) -> c_int;
        #[link_name = "uv_internal_uv_ref"]
        pub fn uv_ref(handle: *mut abi::uv_handle_t);
        #[link_name = "uv_internal_uv_run"]
        pub fn uv_run(loop_: *mut abi::uv_loop_t, mode: abi::uv_run_mode) -> c_int;
        #[link_name = "uv_internal_uv_stop"]
        pub fn uv_stop(loop_: *mut abi::uv_loop_t);
        #[link_name = "uv_internal_uv_timer_again"]
        pub fn uv_timer_again(handle: *mut abi::uv_timer_t) -> c_int;
        #[link_name = "uv_internal_uv_timer_init"]
        pub fn uv_timer_init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_timer_t) -> c_int;
        #[link_name = "uv_internal_uv_timer_start"]
        pub fn uv_timer_start(
            handle: *mut abi::uv_timer_t,
            cb: abi::uv_timer_cb,
            timeout: u64,
            repeat: u64,
        ) -> c_int;
        #[link_name = "uv_internal_uv_timer_stop"]
        pub fn uv_timer_stop(handle: *mut abi::uv_timer_t) -> c_int;
        #[link_name = "uv_internal_uv_unref"]
        pub fn uv_unref(handle: *mut abi::uv_handle_t);
    }
}

pub(crate) unsafe fn backend_fd(loop_: *const abi::uv_loop_t) -> c_int {
    unsafe { raw::uv_backend_fd(loop_) }
}

pub(crate) unsafe fn backend_timeout(loop_: *const abi::uv_loop_t) -> c_int {
    unsafe { raw::uv_backend_timeout(loop_) }
}

pub(crate) unsafe fn close(handle: *mut abi::uv_handle_t, close_cb: abi::uv_close_cb) {
    unsafe { raw::uv_close(handle, close_cb) }
}

pub(crate) unsafe fn default_loop() -> *mut abi::uv_loop_t {
    unsafe { raw::uv_default_loop() }
}

pub(crate) unsafe fn is_active(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { raw::uv_is_active(handle) }
}

pub(crate) unsafe fn is_closing(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { raw::uv_is_closing(handle) }
}

pub(crate) unsafe fn loop_close(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe { raw::uv_loop_close(loop_) }
}

pub(crate) unsafe fn loop_init(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe { raw::uv_loop_init(loop_) }
}

pub(crate) unsafe fn handle_ref(handle: *mut abi::uv_handle_t) {
    unsafe { raw::uv_ref(handle) }
}

pub(crate) unsafe fn run(loop_: *mut abi::uv_loop_t, mode: abi::uv_run_mode) -> c_int {
    unsafe { raw::uv_run(loop_, mode) }
}

pub(crate) unsafe fn stop(loop_: *mut abi::uv_loop_t) {
    unsafe { raw::uv_stop(loop_) }
}

pub(crate) unsafe fn timer_again(handle: *mut abi::uv_timer_t) -> c_int {
    unsafe { raw::uv_timer_again(handle) }
}

pub(crate) unsafe fn timer_init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_timer_t) -> c_int {
    unsafe { raw::uv_timer_init(loop_, handle) }
}

pub(crate) unsafe fn timer_start(
    handle: *mut abi::uv_timer_t,
    cb: abi::uv_timer_cb,
    timeout: u64,
    repeat: u64,
) -> c_int {
    unsafe { raw::uv_timer_start(handle, cb, timeout, repeat) }
}

pub(crate) unsafe fn timer_stop(handle: *mut abi::uv_timer_t) -> c_int {
    unsafe { raw::uv_timer_stop(handle) }
}

pub(crate) unsafe fn handle_unref(handle: *mut abi::uv_handle_t) {
    unsafe { raw::uv_unref(handle) }
}
