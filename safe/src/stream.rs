use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_accept"]
        pub(super) fn uv_accept(server: *mut uv_stream_t, client: *mut uv_stream_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_is_readable"]
        pub(super) fn uv_is_readable(stream: *const uv_stream_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_is_writable"]
        pub(super) fn uv_is_writable(stream: *const uv_stream_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_listen"]
        pub(super) fn uv_listen(
            stream: *mut uv_stream_t,
            backlog: libc::c_int,
            cb: uv_connection_cb,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_read_stop"]
        pub(super) fn uv_read_stop(stream: *mut uv_stream_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_shutdown"]
        pub(super) fn uv_shutdown(
            req: *mut uv_shutdown_t,
            stream: *mut uv_stream_t,
            cb: uv_shutdown_cb,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_stream_set_blocking"]
        pub(super) fn uv_stream_set_blocking(
            handle: *mut uv_stream_t,
            blocking: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_try_write"]
        pub(super) fn uv_try_write(
            stream: *mut uv_stream_t,
            bufs: *const uv_buf_t,
            nbufs: libc::c_uint,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_try_write2"]
        pub(super) fn uv_try_write2(
            stream: *mut uv_stream_t,
            bufs: *const uv_buf_t,
            nbufs: libc::c_uint,
            send_handle: *mut uv_stream_t,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_write"]
        pub(super) fn uv_write(
            req: *mut uv_write_t,
            handle: *mut uv_stream_t,
            bufs: *const uv_buf_t,
            nbufs: libc::c_uint,
            cb: uv_write_cb,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_write2"]
        pub(super) fn uv_write2(
            req: *mut uv_write_t,
            handle: *mut uv_stream_t,
            bufs: *const uv_buf_t,
            nbufs: libc::c_uint,
            send_handle: *mut uv_stream_t,
            cb: uv_write_cb,
        ) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_accept(
    server: *mut uv_stream_t,
    client: *mut uv_stream_t,
) -> libc::c_int {
    private::uv_accept(server, client)
}

#[no_mangle]
pub unsafe extern "C" fn uv_listen(
    stream: *mut uv_stream_t,
    backlog: libc::c_int,
    cb: uv_connection_cb,
) -> libc::c_int {
    private::uv_listen(stream, backlog, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_read_start(
    stream: *mut uv_stream_t,
    alloc_cb: uv_alloc_cb,
    read_cb: uv_read_cb,
) -> libc::c_int {
    crate::private_support::uv_read_start(stream, alloc_cb, read_cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_read_stop(stream: *mut uv_stream_t) -> libc::c_int {
    private::uv_read_stop(stream)
}

#[no_mangle]
pub unsafe extern "C" fn uv_shutdown(
    req: *mut uv_shutdown_t,
    stream: *mut uv_stream_t,
    cb: uv_shutdown_cb,
) -> libc::c_int {
    private::uv_shutdown(req, stream, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_write(
    req: *mut uv_write_t,
    handle: *mut uv_stream_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    cb: uv_write_cb,
) -> libc::c_int {
    private::uv_write(req, handle, bufs, nbufs, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_write2(
    req: *mut uv_write_t,
    handle: *mut uv_stream_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    send_handle: *mut uv_stream_t,
    cb: uv_write_cb,
) -> libc::c_int {
    private::uv_write2(req, handle, bufs, nbufs, send_handle, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_try_write(
    stream: *mut uv_stream_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
) -> libc::c_int {
    private::uv_try_write(stream, bufs, nbufs)
}

#[no_mangle]
pub unsafe extern "C" fn uv_try_write2(
    stream: *mut uv_stream_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    send_handle: *mut uv_stream_t,
) -> libc::c_int {
    private::uv_try_write2(stream, bufs, nbufs, send_handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_is_readable(stream: *const uv_stream_t) -> libc::c_int {
    private::uv_is_readable(stream)
}

#[no_mangle]
pub unsafe extern "C" fn uv_is_writable(stream: *const uv_stream_t) -> libc::c_int {
    private::uv_is_writable(stream)
}

#[no_mangle]
pub unsafe extern "C" fn uv_stream_set_blocking(
    handle: *mut uv_stream_t,
    blocking: libc::c_int,
) -> libc::c_int {
    private::uv_stream_set_blocking(handle, blocking)
}
