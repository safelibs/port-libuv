use crate::allocator::UV_EINVAL;
use crate::bindings::*;
use crate::handle::{
    handle_init, handle_start, handle_stop, is_active, is_closing, queue_empty, queue_head,
    queue_init, queue_insert_tail, queue_remove,
};
use crate::state::{sync_timer_heap_fields, timers};
use std::mem::offset_of;

#[inline]
unsafe fn timer_from_queue(q: *mut uv__queue) -> *mut uv_timer_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_timer_t, node))
        .cast::<uv_timer_t>()
}

#[inline]
unsafe fn timer_less_than(a: *mut uv_timer_t, b: *mut uv_timer_t) -> bool {
    if (*a).timeout < (*b).timeout {
        return true;
    }
    if (*b).timeout < (*a).timeout {
        return false;
    }
    (*a).start_id < (*b).start_id
}

unsafe fn heap_sift_up(heap: &mut Vec<*mut uv_timer_t>, mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if !timer_less_than(heap[idx], heap[parent]) {
            break;
        }
        heap.swap(idx, parent);
        idx = parent;
    }
}

unsafe fn heap_sift_down(heap: &mut Vec<*mut uv_timer_t>, mut idx: usize) {
    loop {
        let left = idx * 2 + 1;
        if left >= heap.len() {
            break;
        }

        let mut smallest = left;
        let right = left + 1;
        if right < heap.len() && timer_less_than(heap[right], heap[left]) {
            smallest = right;
        }

        if !timer_less_than(heap[smallest], heap[idx]) {
            break;
        }

        heap.swap(idx, smallest);
        idx = smallest;
    }
}

unsafe fn heap_insert(loop_: *mut uv_loop_t, handle: *mut uv_timer_t) {
    let heap = &mut *timers(loop_);
    heap.push(handle);
    heap_sift_up(heap, heap.len() - 1);
    sync_timer_heap_fields(loop_);
}

unsafe fn heap_remove(loop_: *mut uv_loop_t, handle: *mut uv_timer_t) {
    let heap = &mut *timers(loop_);
    let Some(idx) = heap.iter().position(|&entry| entry == handle) else {
        return;
    };

    heap.swap_remove(idx);
    if idx < heap.len() {
        heap_sift_down(heap, idx);
        heap_sift_up(heap, idx);
    }
    sync_timer_heap_fields(loop_);
}

unsafe fn heap_min(loop_: *mut uv_loop_t) -> Option<*mut uv_timer_t> {
    (&*timers(loop_)).first().copied()
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_timer_t,
) -> libc::c_int {
    handle_init(loop_, handle.cast(), uv_handle_type_UV_TIMER);
    (*handle).timer_cb = None;
    (*handle).timeout = 0;
    (*handle).repeat = 0;
    (*handle).start_id = 0;
    queue_init(std::ptr::addr_of_mut!((*handle).node.queue));
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_start(
    handle: *mut uv_timer_t,
    cb: uv_timer_cb,
    timeout: u64,
    repeat: u64,
) -> libc::c_int {
    if is_closing(handle.cast()) || cb.is_none() {
        return UV_EINVAL;
    }

    uv_timer_stop(handle);

    let mut clamped_timeout = (*(*handle).loop_).time.wrapping_add(timeout);
    if clamped_timeout < timeout {
        clamped_timeout = u64::MAX;
    }

    (*handle).timer_cb = cb;
    (*handle).timeout = clamped_timeout;
    (*handle).repeat = repeat;
    (*handle).start_id = (*(*handle).loop_).timer_counter;
    (*(*handle).loop_).timer_counter += 1;

    heap_insert((*handle).loop_, handle);
    handle_start(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_stop(handle: *mut uv_timer_t) -> libc::c_int {
    if is_active(handle.cast()) {
        heap_remove((*handle).loop_, handle);
        handle_stop(handle.cast());
    } else {
        queue_remove(std::ptr::addr_of_mut!((*handle).node.queue));
    }

    queue_init(std::ptr::addr_of_mut!((*handle).node.queue));
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_again(handle: *mut uv_timer_t) -> libc::c_int {
    if (*handle).timer_cb.is_none() {
        return UV_EINVAL;
    }

    if (*handle).repeat != 0 {
        uv_timer_stop(handle);
        uv_timer_start(handle, (*handle).timer_cb, (*handle).repeat, (*handle).repeat);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_set_repeat(handle: *mut uv_timer_t, repeat: u64) {
    (*handle).repeat = repeat;
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_get_repeat(handle: *const uv_timer_t) -> u64 {
    (*handle).repeat
}

#[no_mangle]
pub unsafe extern "C" fn uv_timer_get_due_in(handle: *const uv_timer_t) -> u64 {
    if (*(*handle).loop_).time >= (*handle).timeout {
        return 0;
    }

    (*handle).timeout - (*(*handle).loop_).time
}

pub(crate) unsafe extern "C" fn uv__next_timeout_impl(loop_: *const uv_loop_t) -> libc::c_int {
    let Some(handle) = heap_min(loop_.cast_mut()) else {
        return -1;
    };

    if (*handle).timeout <= (*loop_).time {
        return 0;
    }

    let diff = ((*handle).timeout - (*loop_).time).min(libc::c_int::MAX as u64);
    diff as libc::c_int
}

pub(crate) unsafe extern "C" fn uv__run_timers_impl(loop_: *mut uv_loop_t) {
    let mut ready_queue = std::mem::zeroed::<uv__queue>();
    queue_init(std::ptr::addr_of_mut!(ready_queue));

    loop {
        let Some(handle) = heap_min(loop_) else {
            break;
        };
        if (*handle).timeout > (*loop_).time {
            break;
        }

        uv_timer_stop(handle);
        queue_insert_tail(
            std::ptr::addr_of_mut!(ready_queue),
            std::ptr::addr_of_mut!((*handle).node.queue),
        );
    }

    while !queue_empty(std::ptr::addr_of!(ready_queue)) {
        let q = queue_head(std::ptr::addr_of!(ready_queue));
        queue_remove(q);
        queue_init(q);

        let handle = timer_from_queue(q);
        uv_timer_again(handle);
        if let Some(cb) = (*handle).timer_cb {
            cb(handle);
        }
    }
}

pub(crate) unsafe extern "C" fn uv__timer_close_impl(handle: *mut uv_timer_t) {
    uv_timer_stop(handle);
}
