use crate::allocator::UV_EINVAL;
use crate::bindings::*;
use crate::handle::{
    handle_init, handle_start, handle_stop, is_active, queue_empty, queue_head, queue_init,
    queue_insert_head, queue_insert_tail, queue_move, queue_remove,
};
use std::mem::offset_of;

#[inline]
unsafe fn prepare_from_queue(q: *mut uv__queue) -> *mut uv_prepare_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_prepare_t, queue))
        .cast::<uv_prepare_t>()
}

#[inline]
unsafe fn check_from_queue(q: *mut uv__queue) -> *mut uv_check_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_check_t, queue))
        .cast::<uv_check_t>()
}

#[inline]
unsafe fn idle_from_queue(q: *mut uv__queue) -> *mut uv_idle_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_idle_t, queue))
        .cast::<uv_idle_t>()
}

#[no_mangle]
pub unsafe extern "C" fn uv_prepare_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_prepare_t,
) -> libc::c_int {
    handle_init(loop_, handle.cast(), uv_handle_type_UV_PREPARE);
    (*handle).prepare_cb = None;
    queue_init(std::ptr::addr_of_mut!((*handle).queue));
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_prepare_start(
    handle: *mut uv_prepare_t,
    cb: uv_prepare_cb,
) -> libc::c_int {
    if is_active(handle.cast()) {
        return 0;
    }
    if cb.is_none() {
        return UV_EINVAL;
    }

    queue_insert_head(
        std::ptr::addr_of_mut!((*(*handle).loop_).prepare_handles),
        std::ptr::addr_of_mut!((*handle).queue),
    );
    (*handle).prepare_cb = cb;
    handle_start(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_prepare_stop(handle: *mut uv_prepare_t) -> libc::c_int {
    if !is_active(handle.cast()) {
        return 0;
    }

    queue_remove(std::ptr::addr_of_mut!((*handle).queue));
    handle_stop(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_check_init(loop_: *mut uv_loop_t, handle: *mut uv_check_t) -> libc::c_int {
    handle_init(loop_, handle.cast(), uv_handle_type_UV_CHECK);
    (*handle).check_cb = None;
    queue_init(std::ptr::addr_of_mut!((*handle).queue));
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_check_start(handle: *mut uv_check_t, cb: uv_check_cb) -> libc::c_int {
    if is_active(handle.cast()) {
        return 0;
    }
    if cb.is_none() {
        return UV_EINVAL;
    }

    queue_insert_head(
        std::ptr::addr_of_mut!((*(*handle).loop_).check_handles),
        std::ptr::addr_of_mut!((*handle).queue),
    );
    (*handle).check_cb = cb;
    handle_start(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_check_stop(handle: *mut uv_check_t) -> libc::c_int {
    if !is_active(handle.cast()) {
        return 0;
    }

    queue_remove(std::ptr::addr_of_mut!((*handle).queue));
    handle_stop(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_idle_init(loop_: *mut uv_loop_t, handle: *mut uv_idle_t) -> libc::c_int {
    handle_init(loop_, handle.cast(), uv_handle_type_UV_IDLE);
    (*handle).idle_cb = None;
    queue_init(std::ptr::addr_of_mut!((*handle).queue));
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_idle_start(handle: *mut uv_idle_t, cb: uv_idle_cb) -> libc::c_int {
    if is_active(handle.cast()) {
        return 0;
    }
    if cb.is_none() {
        return UV_EINVAL;
    }

    queue_insert_head(
        std::ptr::addr_of_mut!((*(*handle).loop_).idle_handles),
        std::ptr::addr_of_mut!((*handle).queue),
    );
    (*handle).idle_cb = cb;
    handle_start(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_idle_stop(handle: *mut uv_idle_t) -> libc::c_int {
    if !is_active(handle.cast()) {
        return 0;
    }

    queue_remove(std::ptr::addr_of_mut!((*handle).queue));
    handle_stop(handle.cast());
    0
}

pub(crate) unsafe extern "C" fn uv__run_prepare_impl(loop_: *mut uv_loop_t) {
    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).prepare_handles),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = prepare_from_queue(q);
        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).prepare_handles), q);
        if let Some(cb) = (*handle).prepare_cb {
            cb(handle);
        }
    }
}

pub(crate) unsafe extern "C" fn uv__run_check_impl(loop_: *mut uv_loop_t) {
    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).check_handles),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = check_from_queue(q);
        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).check_handles), q);
        if let Some(cb) = (*handle).check_cb {
            cb(handle);
        }
    }
}

pub(crate) unsafe extern "C" fn uv__run_idle_impl(loop_: *mut uv_loop_t) {
    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).idle_handles),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = idle_from_queue(q);
        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).idle_handles), q);
        if let Some(cb) = (*handle).idle_cb {
            cb(handle);
        }
    }
}

pub(crate) unsafe extern "C" fn uv__prepare_close_impl(handle: *mut uv_prepare_t) {
    uv_prepare_stop(handle);
}

pub(crate) unsafe extern "C" fn uv__check_close_impl(handle: *mut uv_check_t) {
    uv_check_stop(handle);
}

pub(crate) unsafe extern "C" fn uv__idle_close_impl(handle: *mut uv_idle_t) {
    uv_idle_stop(handle);
}
