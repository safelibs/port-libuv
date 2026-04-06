use crate::bindings::*;

#[no_mangle]
pub unsafe extern "C" fn uv_fs_event_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_fs_event_t,
) -> libc::c_int {
    crate::linux::inotify::uv_fs_event_init_impl(loop_, handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_event_start(
    handle: *mut uv_fs_event_t,
    cb: uv_fs_event_cb,
    path: *const libc::c_char,
    flags: libc::c_uint,
) -> libc::c_int {
    crate::linux::inotify::uv_fs_event_start_impl(handle, cb, path, flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_event_stop(handle: *mut uv_fs_event_t) -> libc::c_int {
    crate::linux::inotify::uv_fs_event_stop_impl(handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_event_getpath(
    handle: *mut uv_fs_event_t,
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    crate::linux::inotify::uv_fs_event_getpath_impl(handle, buffer, size)
}
