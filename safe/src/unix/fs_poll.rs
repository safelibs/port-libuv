use crate::abi::linux_x86_64 as abi;
use crate::core::{allocator, handle, queue, UV_HANDLE_INTERNAL, UV_HANDLE_REF};
use crate::unix::{epoll, fs};
use std::mem::offset_of;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::{self, null_mut};

#[repr(C)]
struct PollCtx {
    parent_handle: *mut abi::uv_fs_poll_t,
    busy_polling: c_int,
    interval: c_uint,
    start_time: u64,
    loop_: *mut abi::uv_loop_t,
    poll_cb: abi::uv_fs_poll_cb,
    timer_handle: abi::uv_timer_t,
    fs_req: abi::uv_fs_t,
    statbuf: abi::uv_stat_t,
    previous: *mut PollCtx,
    path: [c_char; 1],
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn ctx_path(ctx: *mut PollCtx) -> *const c_char {
    unsafe { unsafe { ptr::addr_of!((*ctx).path).cast() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn poll_ctx_from_req(req: *mut abi::uv_fs_t) -> *mut PollCtx {
    unsafe {
        unsafe {
            req.cast::<u8>()
                .sub(offset_of!(PollCtx, fs_req))
                .cast::<PollCtx>()
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn poll_ctx_from_timer(timer: *mut abi::uv_timer_t) -> *mut PollCtx {
    unsafe {
        unsafe {
            timer
                .cast::<u8>()
                .sub(offset_of!(PollCtx, timer_handle))
                .cast::<PollCtx>()
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn free_ctx(ctx: *mut PollCtx) {
    unsafe {
        if ctx.is_null() {
            return;
        }
        unsafe {
            allocator::free_bytes(ctx.cast());
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn statbuf_eq(a: *const abi::uv_stat_t, b: *const abi::uv_stat_t) -> bool {
    unsafe {
        unsafe {
            (*a).st_ctim.tv_nsec == (*b).st_ctim.tv_nsec
                && (*a).st_mtim.tv_nsec == (*b).st_mtim.tv_nsec
                && (*a).st_birthtim.tv_nsec == (*b).st_birthtim.tv_nsec
                && (*a).st_ctim.tv_sec == (*b).st_ctim.tv_sec
                && (*a).st_mtim.tv_sec == (*b).st_mtim.tv_sec
                && (*a).st_birthtim.tv_sec == (*b).st_birthtim.tv_sec
                && (*a).st_size == (*b).st_size
                && (*a).st_mode == (*b).st_mode
                && (*a).st_uid == (*b).st_uid
                && (*a).st_gid == (*b).st_gid
                && (*a).st_ino == (*b).st_ino
                && (*a).st_dev == (*b).st_dev
                && (*a).st_flags == (*b).st_flags
                && (*a).st_gen == (*b).st_gen
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn timer_close_cb(timer: *mut abi::uv_handle_t) {
    unsafe {
        let ctx = unsafe { poll_ctx_from_timer(timer.cast()) };
        let handle = unsafe { (*ctx).parent_handle };
        if handle.is_null() {
            unsafe {
                free_ctx(ctx);
            }
            return;
        }

        if unsafe { (*handle).poll_ctx.cast::<PollCtx>() == ctx } {
            unsafe {
                (*handle).poll_ctx = (*ctx).previous.cast();
            }
            if unsafe { (*handle).poll_ctx.is_null() }
                && unsafe { handle::is_closing(handle.cast()) }
            {
                unsafe {
                    crate::upstream_support::unix_core::uv__make_close_pending(handle.cast());
                }
            }
        } else {
            let mut last = unsafe { (*handle).poll_ctx.cast::<PollCtx>() };
            while !last.is_null() {
                let next = unsafe { (*last).previous };
                if next == ctx {
                    unsafe {
                        (*last).previous = (*ctx).previous;
                    }
                    break;
                }
                last = next;
            }
        }

        unsafe {
            free_ctx(ctx);
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn poll_cb(req: *mut abi::uv_fs_t) {
    unsafe {
        let ctx = unsafe { poll_ctx_from_req(req) };
        let handle = unsafe { (*ctx).parent_handle };
        if handle.is_null() {
            unsafe {
                fs::req_cleanup(req);
                epoll::close(
                    ptr::addr_of_mut!((*ctx).timer_handle).cast(),
                    Some(timer_close_cb),
                );
            }
            return;
        }

        if unsafe { handle::is_active(handle.cast()) && !handle::is_closing(handle.cast()) } {
            if unsafe { (*req).result } != 0 {
                if unsafe { (*ctx).busy_polling } != unsafe { (*req).result as c_int } {
                    let zero = abi::uv_stat_t::default();
                    if let Some(cb) = unsafe { (*ctx).poll_cb } {
                        unsafe {
                            cb(
                                handle,
                                (*req).result as c_int,
                                ptr::addr_of!((*ctx).statbuf),
                                &zero,
                            );
                        }
                    }
                    unsafe {
                        (*ctx).busy_polling = (*req).result as c_int;
                    }
                }
            } else {
                let statbuf = unsafe { ptr::addr_of!((*req).statbuf) };
                if unsafe { (*ctx).busy_polling } != 0
                    && (unsafe { (*ctx).busy_polling } < 0
                        || !unsafe { statbuf_eq(ptr::addr_of!((*ctx).statbuf), statbuf) })
                {
                    if let Some(cb) = unsafe { (*ctx).poll_cb } {
                        unsafe {
                            cb(handle, 0, ptr::addr_of!((*ctx).statbuf), statbuf);
                        }
                    }
                }

                unsafe {
                    (*ctx).statbuf = *statbuf;
                    (*ctx).busy_polling = 1;
                }
            }
        }

        unsafe {
            fs::req_cleanup(req);
        }

        if !unsafe { handle::is_active(handle.cast()) }
            || unsafe { handle::is_closing(handle.cast()) }
        {
            unsafe {
                epoll::close(
                    ptr::addr_of_mut!((*ctx).timer_handle).cast(),
                    Some(timer_close_cb),
                );
            }
            return;
        }

        let now = unsafe { crate::upstream_support::uv_common::uv_now((*ctx).loop_.cast()) };
        let interval = unsafe {
            let interval = (*ctx).interval as u64;
            interval - ((now - (*ctx).start_time) % interval)
        };
        let rc = unsafe {
            epoll::timer_start(
                ptr::addr_of_mut!((*ctx).timer_handle),
                Some(timer_cb),
                interval,
                0,
            )
        };
        if rc != 0 {
            unsafe {
                libc::abort();
            }
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn timer_cb(timer: *mut abi::uv_timer_t) {
    unsafe {
        let ctx = unsafe { poll_ctx_from_timer(timer) };
        unsafe {
            (*ctx).start_time = crate::upstream_support::uv_common::uv_now((*ctx).loop_.cast());
        }
        let rc = unsafe {
            fs::stat(
                (*ctx).loop_,
                ptr::addr_of_mut!((*ctx).fs_req),
                ctx_path(ctx),
                Some(poll_cb),
            )
        };
        if rc != 0 {
            unsafe {
                libc::abort();
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init(loop_: *mut abi::uv_loop_t, handle_: *mut abi::uv_fs_poll_t) -> c_int {
    unsafe {
        if loop_.is_null() || handle_.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            (*handle_).loop_ = loop_;
            (*handle_).type_ = abi::uv_handle_type_UV_FS_POLL;
            (*handle_).flags = UV_HANDLE_REF;
            queue::insert_tail(
                ptr::addr_of_mut!((*loop_).handle_queue),
                ptr::addr_of_mut!((*handle_).handle_queue),
            );
            (*handle_).next_closing = null_mut();
            (*handle_).poll_ctx = null_mut();
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn start(
    handle_: *mut abi::uv_fs_poll_t,
    poll_cb_fn: abi::uv_fs_poll_cb,
    path: *const c_char,
    interval: c_uint,
) -> c_int {
    unsafe {
        if handle_.is_null() || path.is_null() || poll_cb_fn.is_none() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        if unsafe { handle::is_active(handle_.cast()) } {
            return 0;
        }

        let len = unsafe { libc::strlen(path) };
        let bytes = std::mem::size_of::<PollCtx>() + len;
        let ctx = unsafe { allocator::calloc_bytes(1, bytes) }.cast::<PollCtx>();
        if ctx.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        unsafe {
            (*ctx).loop_ = (*handle_).loop_;
            (*ctx).poll_cb = poll_cb_fn;
            (*ctx).interval = if interval == 0 { 1 } else { interval };
            (*ctx).start_time = crate::upstream_support::uv_common::uv_now((*ctx).loop_.cast());
            (*ctx).parent_handle = handle_;
            libc::memcpy(ptr::addr_of_mut!((*ctx).path).cast(), path.cast(), len + 1);
        }

        let mut rc =
            unsafe { epoll::timer_init((*ctx).loop_, ptr::addr_of_mut!((*ctx).timer_handle)) };
        if rc != 0 {
            unsafe {
                free_ctx(ctx);
            }
            return rc;
        }

        unsafe {
            (*ctx).timer_handle.flags |= UV_HANDLE_INTERNAL;
            epoll::handle_unref(ptr::addr_of_mut!((*ctx).timer_handle).cast());
        }

        rc = unsafe {
            fs::stat(
                (*ctx).loop_,
                ptr::addr_of_mut!((*ctx).fs_req),
                ctx_path(ctx),
                Some(poll_cb),
            )
        };
        if rc != 0 {
            unsafe {
                epoll::close(
                    ptr::addr_of_mut!((*ctx).timer_handle).cast(),
                    Some(timer_close_cb),
                );
                (*ctx).parent_handle = null_mut();
            }
            return rc;
        }

        unsafe {
            if !(*handle_).poll_ctx.is_null() {
                (*ctx).previous = (*handle_).poll_ctx.cast();
            }
            (*handle_).poll_ctx = ctx.cast();
            handle::handle_start(handle_.cast());
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn stop(handle_: *mut abi::uv_fs_poll_t) -> c_int {
    unsafe {
        if handle_.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        if !unsafe { handle::is_active(handle_.cast()) } {
            return 0;
        }

        let ctx = unsafe { (*handle_).poll_ctx.cast::<PollCtx>() };
        if ctx.is_null() {
            unsafe {
                handle::handle_stop(handle_.cast());
            }
            return 0;
        }

        if unsafe { handle::is_active(ptr::addr_of!((*ctx).timer_handle).cast()) } {
            unsafe {
                epoll::close(
                    ptr::addr_of_mut!((*ctx).timer_handle).cast(),
                    Some(timer_close_cb),
                );
            }
        }

        unsafe {
            handle::handle_stop(handle_.cast());
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn getpath(
    handle_: *mut abi::uv_fs_poll_t,
    buffer: *mut c_char,
    size: *mut usize,
) -> c_int {
    unsafe {
        if handle_.is_null() || size.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        if !unsafe { handle::is_active(handle_.cast()) } {
            unsafe {
                *size = 0;
            }
            return abi::uv_errno_t_UV_EINVAL;
        }

        let ctx = unsafe { (*handle_).poll_ctx.cast::<PollCtx>() };
        if ctx.is_null() {
            unsafe {
                *size = 0;
            }
            return abi::uv_errno_t_UV_EINVAL;
        }

        let required = unsafe { libc::strlen(ctx_path(ctx)) };
        if required >= unsafe { *size } {
            unsafe {
                *size = required + 1;
            }
            return abi::uv_errno_t_UV_ENOBUFS;
        }

        unsafe {
            libc::memcpy(buffer.cast(), ctx_path(ctx).cast(), required);
            *buffer.add(required) = 0;
            *size = required;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn close(handle_: *mut abi::uv_fs_poll_t) {
    unsafe {
        if handle_.is_null() {
            return;
        }

        unsafe {
            let _ = stop(handle_);
            if (*handle_).poll_ctx.is_null() {
                crate::upstream_support::unix_core::uv__make_close_pending(handle_.cast());
            }
        }
    }
}
