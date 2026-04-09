use crate::abi::linux_x86_64 as abi;
use std::os::raw::c_int;

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_check_init"]
        pub fn uv_check_init(loop_: *mut abi::uv_loop_t, check: *mut abi::uv_check_t) -> c_int;
        #[link_name = "uv_internal_uv_check_start"]
        pub fn uv_check_start(check: *mut abi::uv_check_t, cb: abi::uv_check_cb) -> c_int;
        #[link_name = "uv_internal_uv_check_stop"]
        pub fn uv_check_stop(check: *mut abi::uv_check_t) -> c_int;
        #[link_name = "uv_internal_uv_idle_init"]
        pub fn uv_idle_init(loop_: *mut abi::uv_loop_t, idle: *mut abi::uv_idle_t) -> c_int;
        #[link_name = "uv_internal_uv_idle_start"]
        pub fn uv_idle_start(idle: *mut abi::uv_idle_t, cb: abi::uv_idle_cb) -> c_int;
        #[link_name = "uv_internal_uv_idle_stop"]
        pub fn uv_idle_stop(idle: *mut abi::uv_idle_t) -> c_int;
        #[link_name = "uv_internal_uv_prepare_init"]
        pub fn uv_prepare_init(
            loop_: *mut abi::uv_loop_t,
            prepare: *mut abi::uv_prepare_t,
        ) -> c_int;
        #[link_name = "uv_internal_uv_prepare_start"]
        pub fn uv_prepare_start(
            prepare: *mut abi::uv_prepare_t,
            cb: abi::uv_prepare_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_prepare_stop"]
        pub fn uv_prepare_stop(prepare: *mut abi::uv_prepare_t) -> c_int;
    }
}

pub(crate) unsafe fn check_init(loop_: *mut abi::uv_loop_t, check: *mut abi::uv_check_t) -> c_int {
    unsafe { raw::uv_check_init(loop_, check) }
}

pub(crate) unsafe fn check_start(check: *mut abi::uv_check_t, cb: abi::uv_check_cb) -> c_int {
    unsafe { raw::uv_check_start(check, cb) }
}

pub(crate) unsafe fn check_stop(check: *mut abi::uv_check_t) -> c_int {
    unsafe { raw::uv_check_stop(check) }
}

pub(crate) unsafe fn idle_init(loop_: *mut abi::uv_loop_t, idle: *mut abi::uv_idle_t) -> c_int {
    unsafe { raw::uv_idle_init(loop_, idle) }
}

pub(crate) unsafe fn idle_start(idle: *mut abi::uv_idle_t, cb: abi::uv_idle_cb) -> c_int {
    unsafe { raw::uv_idle_start(idle, cb) }
}

pub(crate) unsafe fn idle_stop(idle: *mut abi::uv_idle_t) -> c_int {
    unsafe { raw::uv_idle_stop(idle) }
}

pub(crate) unsafe fn prepare_init(
    loop_: *mut abi::uv_loop_t,
    prepare: *mut abi::uv_prepare_t,
) -> c_int {
    unsafe { raw::uv_prepare_init(loop_, prepare) }
}

pub(crate) unsafe fn prepare_start(
    prepare: *mut abi::uv_prepare_t,
    cb: abi::uv_prepare_cb,
) -> c_int {
    unsafe { raw::uv_prepare_start(prepare, cb) }
}

pub(crate) unsafe fn prepare_stop(prepare: *mut abi::uv_prepare_t) -> c_int {
    unsafe { raw::uv_prepare_stop(prepare) }
}
