use crate::abi::linux_x86_64 as abi;

#[inline]
pub(crate) unsafe fn init(queue: *mut abi::uv__queue) {
    unsafe {
        (*queue).next = queue;
        (*queue).prev = queue;
    }
}

#[inline]
pub(crate) unsafe fn is_empty(queue: *const abi::uv__queue) -> bool {
    unsafe { std::ptr::eq((*queue).next.cast_const(), queue) }
}

#[inline]
pub(crate) unsafe fn head(queue: *mut abi::uv__queue) -> *mut abi::uv__queue {
    unsafe { (*queue).next }
}

#[inline]
unsafe fn insert_between(
    prev: *mut abi::uv__queue,
    next: *mut abi::uv__queue,
    node: *mut abi::uv__queue,
) {
    unsafe {
        (*node).prev = prev;
        (*node).next = next;
        (*prev).next = node;
        (*next).prev = node;
    }
}

#[inline]
pub(crate) unsafe fn insert_head(queue: *mut abi::uv__queue, node: *mut abi::uv__queue) {
    unsafe {
        insert_between(queue, (*queue).next, node);
    }
}

#[inline]
pub(crate) unsafe fn insert_tail(queue: *mut abi::uv__queue, node: *mut abi::uv__queue) {
    unsafe {
        insert_between((*queue).prev, queue, node);
    }
}

#[inline]
pub(crate) unsafe fn remove(node: *mut abi::uv__queue) {
    unsafe {
        let prev = (*node).prev;
        let next = (*node).next;
        (*prev).next = next;
        (*next).prev = prev;
    }
}

pub(crate) unsafe fn move_queue(src: *mut abi::uv__queue, dst: *mut abi::uv__queue) {
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
