use crate::abi::linux_x86_64 as abi;
use crate::core::{
    allocator, find_handle_record, handle, loop_state, queue, HandleKind, LoopStateInner,
    TimerHeapEntry,
};
use std::cmp::Ordering;
use std::mem::offset_of;
use std::os::raw::c_int;

#[inline]
fn timer_less_than(a: &TimerHeapEntry, b: &TimerHeapEntry) -> bool {
    match a.timeout.cmp(&b.timeout) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => a.start_id < b.start_id,
    }
}

unsafe fn heap_swap(inner: &mut LoopStateInner, left: usize, right: usize) {
    unsafe {
        let ptr = inner.timer_heap;
        let left_value = ptr.add(left).read();
        let right_value = ptr.add(right).read();
        ptr.add(left).write(right_value);
        ptr.add(right).write(left_value);
    }
}

unsafe fn heap_sift_up(inner: &mut LoopStateInner, mut index: usize) {
    while index > 0 {
        let parent = (index - 1) / 2;
        let should_swap = unsafe {
            timer_less_than(&*inner.timer_heap.add(index), &*inner.timer_heap.add(parent))
        };
        if !should_swap {
            break;
        }
        unsafe {
            heap_swap(inner, index, parent);
        }
        index = parent;
    }
}

unsafe fn heap_sift_down(inner: &mut LoopStateInner, mut index: usize) {
    loop {
        let left = index * 2 + 1;
        let right = left + 1;
        let mut smallest = index;

        if left < inner.timer_len
            && unsafe {
                timer_less_than(
                    &*inner.timer_heap.add(left),
                    &*inner.timer_heap.add(smallest),
                )
            }
        {
            smallest = left;
        }

        if right < inner.timer_len
            && unsafe {
                timer_less_than(
                    &*inner.timer_heap.add(right),
                    &*inner.timer_heap.add(smallest),
                )
            }
        {
            smallest = right;
        }

        if smallest == index {
            break;
        }

        unsafe {
            heap_swap(inner, index, smallest);
        }
        index = smallest;
    }
}

unsafe fn ensure_timer_capacity(inner: &mut LoopStateInner, loop_: *mut abi::uv_loop_t) -> bool {
    if inner.timer_len < inner.timer_cap {
        return true;
    }

    let next_cap = if inner.timer_cap == 0 {
        8
    } else {
        inner.timer_cap.saturating_mul(2)
    };
    let next_ptr = unsafe { allocator::realloc_array(inner.timer_heap, next_cap) };
    if next_ptr.is_null() {
        return false;
    }

    inner.timer_heap = next_ptr;
    inner.timer_cap = next_cap;
    unsafe {
        (*loop_).timer_heap.min = inner.timer_heap.cast();
    }
    true
}

unsafe fn update_public_heap(loop_: *mut abi::uv_loop_t, inner: &LoopStateInner) {
    unsafe {
        (*loop_).timer_heap.min = inner.timer_heap.cast();
        (*loop_).timer_heap.nelts = inner.timer_len as u32;
    }
}

unsafe fn heap_insert(loop_: *mut abi::uv_loop_t, entry: TimerHeapEntry) -> bool {
    let state = unsafe { &*loop_state(loop_) };
    let mut inner = state.inner.lock().unwrap();
    if !unsafe { ensure_timer_capacity(&mut inner, loop_) } {
        return false;
    }

    unsafe {
        inner.timer_heap.add(inner.timer_len).write(entry);
        inner.timer_len += 1;
        let index = inner.timer_len - 1;
        heap_sift_up(&mut inner, index);
        update_public_heap(loop_, &inner);
    }
    true
}

unsafe fn heap_remove_active(loop_: *mut abi::uv_loop_t, handle_ptr: *mut abi::uv_timer_t) {
    let state = unsafe { &*loop_state(loop_) };
    let mut inner = state.inner.lock().unwrap();
    let record = unsafe { find_handle_record(&inner, handle_ptr.cast()) };
    if record.is_null() {
        return;
    }
    let generation = unsafe { (*record).timer_generation };

    let mut index = 0;
    while index < inner.timer_len {
        let entry = unsafe { *inner.timer_heap.add(index) };
        if entry.handle == handle_ptr && entry.generation == generation {
            let last_index = inner.timer_len - 1;
            unsafe {
                let last = inner.timer_heap.add(last_index).read();
                inner.timer_len -= 1;
                if index < inner.timer_len {
                    inner.timer_heap.add(index).write(last);
                    heap_sift_down(&mut inner, index);
                    heap_sift_up(&mut inner, index);
                }
                update_public_heap(loop_, &inner);
            }
            return;
        }
        index += 1;
    }
}

unsafe fn heap_pop_min(loop_: *mut abi::uv_loop_t) -> Option<TimerHeapEntry> {
    let state = unsafe { &*loop_state(loop_) };
    let mut inner = state.inner.lock().unwrap();
    if inner.timer_len == 0 {
        return None;
    }

    let first = unsafe { inner.timer_heap.read() };
    let last_index = inner.timer_len - 1;
    unsafe {
        let last = inner.timer_heap.add(last_index).read();
        inner.timer_len -= 1;
        if inner.timer_len > 0 {
            inner.timer_heap.write(last);
            heap_sift_down(&mut inner, 0);
        }
        update_public_heap(loop_, &inner);
    }
    Some(first)
}

unsafe fn heap_min(loop_: *mut abi::uv_loop_t) -> Option<TimerHeapEntry> {
    let state = unsafe { &*loop_state(loop_) };
    let inner = state.inner.lock().unwrap();
    if inner.timer_len == 0 {
        None
    } else {
        Some(unsafe { *inner.timer_heap })
    }
}

pub(crate) unsafe fn timer_init(loop_: *mut abi::uv_loop_t, handle_ptr: *mut abi::uv_timer_t) -> c_int {
    let rc = unsafe {
        handle::handle_init(
            loop_,
            handle_ptr.cast(),
            abi::uv_handle_type_UV_TIMER,
            HandleKind::Timer,
        )
    };
    if rc != 0 {
        return rc;
    }

    unsafe {
        (*handle_ptr).timer_cb = None;
        (*handle_ptr).timeout = 0;
        (*handle_ptr).repeat = 0;
        (*handle_ptr).start_id = 0;
        queue::init(std::ptr::addr_of_mut!((*handle_ptr).node.queue));
    }
    0
}

pub(crate) unsafe fn timer_start(
    handle_ptr: *mut abi::uv_timer_t,
    cb: abi::uv_timer_cb,
    timeout: u64,
    repeat: u64,
) -> c_int {
    if handle_ptr.is_null() || cb.is_none() || unsafe { handle::is_closing(handle_ptr.cast()) } {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let loop_ = unsafe { (*handle_ptr).loop_ };
    let _ = unsafe { timer_stop(handle_ptr) };

    let mut clamped_timeout = unsafe { (*loop_).time }.wrapping_add(timeout);
    if clamped_timeout < timeout {
        clamped_timeout = u64::MAX;
    }

    let state = unsafe { &*loop_state(loop_) };
    let inner = state.inner.lock().unwrap();
    let record = unsafe { find_handle_record(&inner, handle_ptr.cast()) };
    if record.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    unsafe {
        (*record).timer_generation = (*record).timer_generation.wrapping_add(1);
        (*handle_ptr).timer_cb = cb;
        (*handle_ptr).timeout = clamped_timeout;
        (*handle_ptr).repeat = repeat;
        (*handle_ptr).start_id = (*loop_).timer_counter;
        (*loop_).timer_counter = (*loop_).timer_counter.wrapping_add(1);
    }

    let entry = TimerHeapEntry {
        handle: handle_ptr,
        timeout: unsafe { (*handle_ptr).timeout },
        start_id: unsafe { (*handle_ptr).start_id },
        generation: unsafe { (*record).timer_generation },
    };
    drop(inner);

    if !unsafe { heap_insert(loop_, entry) } {
        return abi::uv_errno_t_UV_ENOMEM;
    }

    unsafe {
        handle::handle_start(handle_ptr.cast());
    }
    0
}

pub(crate) unsafe fn timer_stop(handle_ptr: *mut abi::uv_timer_t) -> c_int {
    if handle_ptr.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let loop_ = unsafe { (*handle_ptr).loop_ };
    if unsafe { handle::is_active(handle_ptr.cast()) } {
        unsafe {
            heap_remove_active(loop_, handle_ptr);
            handle::handle_stop(handle_ptr.cast());
        }
    } else {
        let node = unsafe { std::ptr::addr_of_mut!((*handle_ptr).node.queue) };
        if unsafe { !std::ptr::eq((*node).next, node) } {
            unsafe {
                queue::remove(node);
            }
        }
    }

    unsafe {
        queue::init(std::ptr::addr_of_mut!((*handle_ptr).node.queue));
    }
    0
}

pub(crate) unsafe fn timer_again(handle_ptr: *mut abi::uv_timer_t) -> c_int {
    if handle_ptr.is_null() || unsafe { (*handle_ptr).timer_cb.is_none() } {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { (*handle_ptr).repeat } != 0 {
        let repeat = unsafe { (*handle_ptr).repeat };
        let cb = unsafe { (*handle_ptr).timer_cb };
        let _ = unsafe { timer_stop(handle_ptr) };
        return unsafe { timer_start(handle_ptr, cb, repeat, repeat) };
    }
    0
}

pub(crate) unsafe fn timer_get_due_in(handle_ptr: *const abi::uv_timer_t) -> u64 {
    if handle_ptr.is_null() {
        return 0;
    }
    let loop_ = unsafe { (*handle_ptr).loop_ };
    if loop_.is_null() {
        return 0;
    }
    let now = unsafe { (*loop_).time };
    let timeout = unsafe { (*handle_ptr).timeout };
    timeout.saturating_sub(now)
}

pub(crate) unsafe fn next_timeout(loop_: *mut abi::uv_loop_t) -> c_int {
    let Some(entry) = (unsafe { heap_min(loop_) }) else {
        return -1;
    };

    if entry.timeout <= unsafe { (*loop_).time } {
        return 0;
    }

    let diff = entry.timeout - unsafe { (*loop_).time };
    if diff > c_int::MAX as u64 {
        c_int::MAX
    } else {
        diff as c_int
    }
}

pub(crate) unsafe fn run_timers(loop_: *mut abi::uv_loop_t) {
    let mut ready = abi::uv__queue::default();
    unsafe {
        queue::init(std::ptr::addr_of_mut!(ready));
    }

    loop {
        let Some(entry) = (unsafe { heap_min(loop_) }) else {
            break;
        };
        if entry.timeout > unsafe { (*loop_).time } {
            break;
        }

        let entry = unsafe { heap_pop_min(loop_) }.unwrap();
        let handle_ptr = entry.handle;
        let state = unsafe { &*loop_state(loop_) };
        let inner = state.inner.lock().unwrap();
        let record = unsafe { find_handle_record(&inner, handle_ptr.cast()) };
        let stale = record.is_null() || unsafe { (*record).timer_generation != entry.generation };
        drop(inner);
        if stale {
            continue;
        }

        unsafe {
            handle::handle_stop(handle_ptr.cast());
            queue::insert_tail(
                std::ptr::addr_of_mut!(ready),
                std::ptr::addr_of_mut!((*handle_ptr).node.queue),
            );
        }
    }

    while unsafe { !queue::is_empty(std::ptr::addr_of!(ready)) } {
        let node = unsafe { queue::head(std::ptr::addr_of_mut!(ready)) };
        let handle_ptr = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_timer_t, node))
                .cast::<abi::uv_timer_t>()
        };

        unsafe {
            queue::remove(node);
            queue::init(std::ptr::addr_of_mut!((*handle_ptr).node.queue));
        }

        let _ = unsafe { timer_again(handle_ptr) };
        unsafe {
            if let Some(cb) = (*handle_ptr).timer_cb {
                cb(handle_ptr);
            }
        }
    }
}

pub(crate) unsafe fn timer_close(handle_ptr: *mut abi::uv_timer_t) {
    let _ = unsafe { timer_stop(handle_ptr) };
}
