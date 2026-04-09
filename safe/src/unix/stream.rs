use crate::abi::linux_x86_64 as abi;
use std::os::raw::{c_int, c_uint};

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_accept"]
        pub fn uv_accept(server: *mut abi::uv_stream_t, client: *mut abi::uv_stream_t) -> c_int;
        #[link_name = "uv_internal_uv_is_readable"]
        pub fn uv_is_readable(handle: *const abi::uv_stream_t) -> c_int;
        #[link_name = "uv_internal_uv_is_writable"]
        pub fn uv_is_writable(handle: *const abi::uv_stream_t) -> c_int;
        #[link_name = "uv_internal_uv_listen"]
        pub fn uv_listen(
            stream: *mut abi::uv_stream_t,
            backlog: c_int,
            cb: abi::uv_connection_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_read_start"]
        pub fn uv_read_start(
            stream: *mut abi::uv_stream_t,
            alloc_cb: abi::uv_alloc_cb,
            read_cb: abi::uv_read_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_read_stop"]
        pub fn uv_read_stop(stream: *mut abi::uv_stream_t) -> c_int;
        #[link_name = "uv_internal_uv_shutdown"]
        pub fn uv_shutdown(
            req: *mut abi::uv_shutdown_t,
            handle: *mut abi::uv_stream_t,
            cb: abi::uv_shutdown_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_stream_set_blocking"]
        pub fn uv_stream_set_blocking(handle: *mut abi::uv_stream_t, blocking: c_int) -> c_int;
        #[link_name = "uv_internal_uv_try_write"]
        pub fn uv_try_write(
            handle: *mut abi::uv_stream_t,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
        ) -> c_int;
        #[link_name = "uv_internal_uv_try_write2"]
        pub fn uv_try_write2(
            handle: *mut abi::uv_stream_t,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
            send_handle: *mut abi::uv_stream_t,
        ) -> c_int;
        #[link_name = "uv_internal_uv_write"]
        pub fn uv_write(
            req: *mut abi::uv_write_t,
            handle: *mut abi::uv_stream_t,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
            cb: abi::uv_write_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_write2"]
        pub fn uv_write2(
            req: *mut abi::uv_write_t,
            handle: *mut abi::uv_stream_t,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
            send_handle: *mut abi::uv_stream_t,
            cb: abi::uv_write_cb,
        ) -> c_int;
    }
}

pub(crate) unsafe fn accept(
    server: *mut abi::uv_stream_t,
    client: *mut abi::uv_stream_t,
) -> c_int {
    unsafe { raw::uv_accept(server, client) }
}

pub(crate) unsafe fn is_readable(handle: *const abi::uv_stream_t) -> c_int {
    unsafe { raw::uv_is_readable(handle) }
}

pub(crate) unsafe fn is_writable(handle: *const abi::uv_stream_t) -> c_int {
    unsafe { raw::uv_is_writable(handle) }
}

pub(crate) unsafe fn listen(
    stream: *mut abi::uv_stream_t,
    backlog: c_int,
    cb: abi::uv_connection_cb,
) -> c_int {
    unsafe { raw::uv_listen(stream, backlog, cb) }
}

pub(crate) unsafe fn read_start(
    stream: *mut abi::uv_stream_t,
    alloc_cb: abi::uv_alloc_cb,
    read_cb: abi::uv_read_cb,
) -> c_int {
    unsafe { raw::uv_read_start(stream, alloc_cb, read_cb) }
}

pub(crate) unsafe fn read_stop(stream: *mut abi::uv_stream_t) -> c_int {
    unsafe { raw::uv_read_stop(stream) }
}

pub(crate) unsafe fn shutdown(
    req: *mut abi::uv_shutdown_t,
    handle: *mut abi::uv_stream_t,
    cb: abi::uv_shutdown_cb,
) -> c_int {
    unsafe { raw::uv_shutdown(req, handle, cb) }
}

pub(crate) unsafe fn set_blocking(handle: *mut abi::uv_stream_t, blocking: c_int) -> c_int {
    unsafe { raw::uv_stream_set_blocking(handle, blocking) }
}

pub(crate) unsafe fn try_write(
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
) -> c_int {
    unsafe { raw::uv_try_write(handle, bufs, nbufs) }
}

pub(crate) unsafe fn try_write2(
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    send_handle: *mut abi::uv_stream_t,
) -> c_int {
    unsafe { raw::uv_try_write2(handle, bufs, nbufs, send_handle) }
}

pub(crate) unsafe fn write(
    req: *mut abi::uv_write_t,
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    cb: abi::uv_write_cb,
) -> c_int {
    unsafe { raw::uv_write(req, handle, bufs, nbufs, cb) }
}

pub(crate) unsafe fn write2(
    req: *mut abi::uv_write_t,
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    send_handle: *mut abi::uv_stream_t,
    cb: abi::uv_write_cb,
) -> c_int {
    unsafe { raw::uv_write2(req, handle, bufs, nbufs, send_handle, cb) }
}
