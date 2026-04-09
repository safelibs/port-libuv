use crate::abi::linux_x86_64 as abi;
use std::os::raw::{c_char, c_int, c_uint};

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_pipe_bind"]
        pub fn uv_pipe_bind(handle: *mut abi::uv_pipe_t, name: *const c_char) -> c_int;
        #[link_name = "uv_internal_uv_pipe_bind2"]
        pub fn uv_pipe_bind2(
            handle: *mut abi::uv_pipe_t,
            name: *const c_char,
            namelen: usize,
            flags: c_uint,
        ) -> c_int;
        #[link_name = "uv_internal_uv_pipe_chmod"]
        pub fn uv_pipe_chmod(handle: *mut abi::uv_pipe_t, flags: c_int) -> c_int;
        #[link_name = "uv_internal_uv_pipe_connect"]
        pub fn uv_pipe_connect(
            req: *mut abi::uv_connect_t,
            handle: *mut abi::uv_pipe_t,
            name: *const c_char,
            cb: abi::uv_connect_cb,
        );
        #[link_name = "uv_internal_uv_pipe_connect2"]
        pub fn uv_pipe_connect2(
            req: *mut abi::uv_connect_t,
            handle: *mut abi::uv_pipe_t,
            name: *const c_char,
            namelen: usize,
            flags: c_uint,
            cb: abi::uv_connect_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_pipe_getpeername"]
        pub fn uv_pipe_getpeername(
            handle: *const abi::uv_pipe_t,
            buffer: *mut c_char,
            size: *mut usize,
        ) -> c_int;
        #[link_name = "uv_internal_uv_pipe_getsockname"]
        pub fn uv_pipe_getsockname(
            handle: *const abi::uv_pipe_t,
            buffer: *mut c_char,
            size: *mut usize,
        ) -> c_int;
        #[link_name = "uv_internal_uv_pipe_init"]
        pub fn uv_pipe_init(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_pipe_t,
            ipc: c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_pipe_open"]
        pub fn uv_pipe_open(handle: *mut abi::uv_pipe_t, file: abi::uv_file) -> c_int;
        #[link_name = "uv_internal_uv_pipe_pending_count"]
        pub fn uv_pipe_pending_count(handle: *mut abi::uv_pipe_t) -> c_int;
        #[link_name = "uv_internal_uv_pipe_pending_instances"]
        pub fn uv_pipe_pending_instances(handle: *mut abi::uv_pipe_t, count: c_int);
        #[link_name = "uv_internal_uv_pipe_pending_type"]
        pub fn uv_pipe_pending_type(handle: *mut abi::uv_pipe_t) -> abi::uv_handle_type;
    }
}

pub(crate) unsafe fn bind(handle: *mut abi::uv_pipe_t, name: *const c_char) -> c_int {
    unsafe { raw::uv_pipe_bind(handle, name) }
}

pub(crate) unsafe fn bind2(
    handle: *mut abi::uv_pipe_t,
    name: *const c_char,
    namelen: usize,
    flags: c_uint,
) -> c_int {
    unsafe { raw::uv_pipe_bind2(handle, name, namelen, flags) }
}

pub(crate) unsafe fn chmod(handle: *mut abi::uv_pipe_t, flags: c_int) -> c_int {
    unsafe { raw::uv_pipe_chmod(handle, flags) }
}

pub(crate) unsafe fn connect(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_pipe_t,
    name: *const c_char,
    cb: abi::uv_connect_cb,
) {
    unsafe { raw::uv_pipe_connect(req, handle, name, cb) }
}

pub(crate) unsafe fn connect2(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_pipe_t,
    name: *const c_char,
    namelen: usize,
    flags: c_uint,
    cb: abi::uv_connect_cb,
) -> c_int {
    unsafe { raw::uv_pipe_connect2(req, handle, name, namelen, flags, cb) }
}

pub(crate) unsafe fn getpeername(
    handle: *const abi::uv_pipe_t,
    buffer: *mut c_char,
    size: *mut usize,
) -> c_int {
    unsafe { raw::uv_pipe_getpeername(handle, buffer, size) }
}

pub(crate) unsafe fn getsockname(
    handle: *const abi::uv_pipe_t,
    buffer: *mut c_char,
    size: *mut usize,
) -> c_int {
    unsafe { raw::uv_pipe_getsockname(handle, buffer, size) }
}

pub(crate) unsafe fn init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_pipe_t,
    ipc: c_int,
) -> c_int {
    unsafe { raw::uv_pipe_init(loop_, handle, ipc) }
}

pub(crate) unsafe fn open(handle: *mut abi::uv_pipe_t, file: abi::uv_file) -> c_int {
    unsafe { raw::uv_pipe_open(handle, file) }
}

pub(crate) unsafe fn pending_count(handle: *mut abi::uv_pipe_t) -> c_int {
    unsafe { raw::uv_pipe_pending_count(handle) }
}

pub(crate) unsafe fn pending_instances(handle: *mut abi::uv_pipe_t, count: c_int) {
    unsafe { raw::uv_pipe_pending_instances(handle, count) }
}

pub(crate) unsafe fn pending_type(handle: *mut abi::uv_pipe_t) -> abi::uv_handle_type {
    unsafe { raw::uv_pipe_pending_type(handle) }
}
