use crate::abi::linux_x86_64 as abi;

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn init(queue: *mut abi::uv__queue) {
    unsafe {
        unsafe {
            (*queue).next = queue;
            (*queue).prev = queue;
        }
    }
}

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn is_empty(queue: *const abi::uv__queue) -> bool {
    unsafe { unsafe { std::ptr::eq((*queue).next.cast_const(), queue) } }
}

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn head(queue: *mut abi::uv__queue) -> *mut abi::uv__queue {
    unsafe { unsafe { (*queue).next } }
}

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
fn insert_between(prev: *mut abi::uv__queue, next: *mut abi::uv__queue, node: *mut abi::uv__queue) {
    unsafe {
        unsafe {
            (*node).prev = prev;
            (*node).next = next;
            (*prev).next = node;
            (*next).prev = node;
        }
    }
}

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn insert_head(queue: *mut abi::uv__queue, node: *mut abi::uv__queue) {
    unsafe {
        unsafe {
            insert_between(queue, (*queue).next, node);
        }
    }
}

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn insert_tail(queue: *mut abi::uv__queue, node: *mut abi::uv__queue) {
    unsafe {
        unsafe {
            insert_between((*queue).prev, queue, node);
        }
    }
}

#[inline]
// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn remove(node: *mut abi::uv__queue) {
    unsafe {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;
            (*prev).next = next;
            (*next).prev = prev;
        }
    }
}

// SAFETY(intrusive_queue): manipulates libuv's intrusive queue links in place.
pub(crate) fn move_queue(src: *mut abi::uv__queue, dst: *mut abi::uv__queue) {
    unsafe {
        if unsafe { is_empty(src) } {
            unsafe { init(dst) };
            return;
        }

        unsafe {
            *dst = *src;
            (*(*dst).next).prev = dst;
            (*(*dst).prev).next = dst;
            init(src);
        }
    }
}
