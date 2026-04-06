use crate::bindings::*;

#[inline]
pub(crate) unsafe fn has_active_reqs(loop_: *const uv_loop_t) -> bool {
    (*loop_).active_reqs.count > 0
}

#[inline]
pub(crate) unsafe fn req_register(loop_: *mut uv_loop_t, _req: *mut uv_req_t) {
    (*loop_).active_reqs.count += 1;
}

#[inline]
pub(crate) unsafe fn req_unregister(loop_: *mut uv_loop_t, _req: *mut uv_req_t) {
    debug_assert!(has_active_reqs(loop_));
    (*loop_).active_reqs.count -= 1;
}
