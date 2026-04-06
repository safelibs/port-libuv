use crate::bindings::*;

pub(crate) const UV_HANDLE_CLOSING: libc::c_uint = 0x00000001;
pub(crate) const UV_HANDLE_CLOSED: libc::c_uint = 0x00000002;
pub(crate) const UV_HANDLE_ACTIVE: libc::c_uint = 0x00000004;
pub(crate) const UV_HANDLE_REF: libc::c_uint = 0x00000008;
pub(crate) const UV_HANDLE_INTERNAL: libc::c_uint = 0x00000010;

#[inline]
pub(crate) unsafe fn queue_init(q: *mut uv__queue) {
    (*q).next = q;
    (*q).prev = q;
}

#[inline]
pub(crate) unsafe fn queue_empty(q: *const uv__queue) -> bool {
    std::ptr::eq(q.cast_mut(), (*q).next)
}

#[inline]
pub(crate) unsafe fn queue_head(q: *const uv__queue) -> *mut uv__queue {
    (*q).next
}

#[inline]
pub(crate) unsafe fn queue_next(q: *const uv__queue) -> *mut uv__queue {
    (*q).next
}

#[inline]
pub(crate) unsafe fn queue_add(h: *mut uv__queue, n: *mut uv__queue) {
    (*(*h).prev).next = (*n).next;
    (*(*n).next).prev = (*h).prev;
    (*h).prev = (*n).prev;
    (*(*h).prev).next = h;
}

#[inline]
pub(crate) unsafe fn queue_split(h: *mut uv__queue, q: *mut uv__queue, n: *mut uv__queue) {
    (*n).prev = (*h).prev;
    (*(*n).prev).next = n;
    (*n).next = q;
    (*h).prev = (*q).prev;
    (*(*h).prev).next = h;
    (*q).prev = n;
}

#[inline]
pub(crate) unsafe fn queue_move(h: *mut uv__queue, n: *mut uv__queue) {
    if queue_empty(h) {
        queue_init(n);
    } else {
        queue_split(h, (*h).next, n);
    }
}

#[inline]
pub(crate) unsafe fn queue_insert_head(h: *mut uv__queue, q: *mut uv__queue) {
    (*q).next = (*h).next;
    (*q).prev = h;
    (*(*q).next).prev = q;
    (*h).next = q;
}

#[inline]
pub(crate) unsafe fn queue_insert_tail(h: *mut uv__queue, q: *mut uv__queue) {
    (*q).next = h;
    (*q).prev = (*h).prev;
    (*(*q).prev).next = q;
    (*h).prev = q;
}

#[inline]
pub(crate) unsafe fn queue_remove(q: *mut uv__queue) {
    (*(*q).prev).next = (*q).next;
    (*(*q).next).prev = (*q).prev;
}

#[inline]
pub(crate) unsafe fn has_active_handles(loop_: *const uv_loop_t) -> bool {
    (*loop_).active_handles > 0
}

#[inline]
pub(crate) unsafe fn is_active(handle: *const uv_handle_t) -> bool {
    ((*handle).flags & UV_HANDLE_ACTIVE) != 0
}

#[inline]
pub(crate) unsafe fn is_closing(handle: *const uv_handle_t) -> bool {
    ((*handle).flags & (UV_HANDLE_CLOSING | UV_HANDLE_CLOSED)) != 0
}

#[inline]
pub(crate) unsafe fn has_ref(handle: *const uv_handle_t) -> bool {
    ((*handle).flags & UV_HANDLE_REF) != 0
}

#[inline]
pub(crate) unsafe fn active_handle_add(handle: *mut uv_handle_t) {
    (*(*handle).loop_).active_handles += 1;
}

#[inline]
pub(crate) unsafe fn active_handle_rm(handle: *mut uv_handle_t) {
    (*(*handle).loop_).active_handles -= 1;
}

#[inline]
pub(crate) unsafe fn handle_start(handle: *mut uv_handle_t) {
    if ((*handle).flags & UV_HANDLE_ACTIVE) != 0 {
        return;
    }

    (*handle).flags |= UV_HANDLE_ACTIVE;
    if ((*handle).flags & UV_HANDLE_REF) != 0 {
        active_handle_add(handle);
    }
}

#[inline]
pub(crate) unsafe fn handle_stop(handle: *mut uv_handle_t) {
    if ((*handle).flags & UV_HANDLE_ACTIVE) == 0 {
        return;
    }

    (*handle).flags &= !UV_HANDLE_ACTIVE;
    if ((*handle).flags & UV_HANDLE_REF) != 0 {
        active_handle_rm(handle);
    }
}

#[inline]
pub(crate) unsafe fn handle_ref(handle: *mut uv_handle_t) {
    if ((*handle).flags & UV_HANDLE_REF) != 0 {
        return;
    }

    (*handle).flags |= UV_HANDLE_REF;
    if ((*handle).flags & UV_HANDLE_CLOSING) != 0 {
        return;
    }

    if ((*handle).flags & UV_HANDLE_ACTIVE) != 0 {
        active_handle_add(handle);
    }
}

#[inline]
pub(crate) unsafe fn handle_unref(handle: *mut uv_handle_t) {
    if ((*handle).flags & UV_HANDLE_REF) == 0 {
        return;
    }

    (*handle).flags &= !UV_HANDLE_REF;
    if ((*handle).flags & UV_HANDLE_CLOSING) != 0 {
        return;
    }

    if ((*handle).flags & UV_HANDLE_ACTIVE) != 0 {
        active_handle_rm(handle);
    }
}

#[inline]
pub(crate) unsafe fn handle_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_handle_t,
    type_: uv_handle_type,
) {
    (*handle).loop_ = loop_;
    (*handle).type_ = type_;
    (*handle).flags = UV_HANDLE_REF;
    queue_insert_tail(
        std::ptr::addr_of_mut!((*loop_).handle_queue),
        std::ptr::addr_of_mut!((*handle).handle_queue),
    );
    (*handle).next_closing = std::ptr::null_mut();
}
