use crate::abi::linux_x86_64 as abi;
use crate::core::handle;
use std::os::raw::{c_char, c_int, c_uint};

pub(crate) unsafe fn init(loop_: *mut abi::uv_loop_t, handle_: *mut abi::uv_fs_event_t) -> c_int {
    unsafe { crate::unix::epoll::uv_fs_event_init(loop_.cast(), handle_.cast()) }
}

pub(crate) unsafe fn start(
    handle_: *mut abi::uv_fs_event_t,
    cb: abi::uv_fs_event_cb,
    path: *const c_char,
    flags: c_uint,
) -> c_int {
    unsafe {
        crate::unix::epoll::uv_fs_event_start(
            handle_.cast(),
            std::mem::transmute(cb),
            path,
            flags,
        )
    }
}

pub(crate) unsafe fn stop(handle_: *mut abi::uv_fs_event_t) -> c_int {
    unsafe { crate::unix::epoll::uv_fs_event_stop(handle_.cast()) }
}

pub(crate) unsafe fn getpath(
    handle_: *mut abi::uv_fs_event_t,
    buffer: *mut c_char,
    size: *mut usize,
) -> c_int {
    if handle_.is_null() || size.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if !unsafe { handle::is_active(handle_.cast()) } {
        unsafe {
            *size = 0;
        }
        return abi::uv_errno_t_UV_EINVAL;
    }

    let required = unsafe { libc::strlen((*handle_).path) };
    if required >= unsafe { *size } {
        unsafe {
            *size = required + 1;
        }
        return abi::uv_errno_t_UV_ENOBUFS;
    }

    unsafe {
        libc::memcpy(buffer.cast(), (*handle_).path.cast(), required);
        *buffer.add(required) = 0;
        *size = required;
    }
    0
}
