use crate::abi::{uv__iou, uv__loop_internal_fields_s, uv__loop_metrics_s};
use crate::bindings::*;

#[repr(C)]
pub(crate) struct LoopState {
    pub(crate) compat: uv__loop_internal_fields_s,
    pub(crate) timers: Vec<*mut uv_timer_t>,
}

impl Default for LoopState {
    fn default() -> Self {
        Self {
            compat: uv__loop_internal_fields_s {
                flags: 0,
                // SAFETY: `uv__loop_metrics_s` is a plain C metrics record and
                // libuv initializes it from all-zero storage.
                loop_metrics: unsafe { std::mem::zeroed::<uv__loop_metrics_s>() },
                current_timeout: 0,
                ctl: uv__iou::default(),
                iou: uv__iou::default(),
                inv: std::ptr::null_mut(),
            },
            timers: Vec::new(),
        }
    }
}

#[inline]
pub(crate) unsafe fn loop_state(loop_: *const uv_loop_t) -> *mut LoopState {
    (*loop_).internal_fields.cast::<LoopState>()
}

#[inline]
pub(crate) unsafe fn loop_fields(loop_: *const uv_loop_t) -> *mut uv__loop_internal_fields_s {
    std::ptr::addr_of_mut!((*loop_state(loop_)).compat)
}

#[inline]
pub(crate) unsafe fn loop_metrics(loop_: *const uv_loop_t) -> *mut uv__loop_metrics_s {
    std::ptr::addr_of_mut!((*loop_fields(loop_)).loop_metrics)
}

#[inline]
pub(crate) unsafe fn timers(loop_: *mut uv_loop_t) -> *mut Vec<*mut uv_timer_t> {
    std::ptr::addr_of_mut!((*loop_state(loop_)).timers)
}

#[inline]
pub(crate) unsafe fn sync_timer_heap_fields(loop_: *mut uv_loop_t) {
    let timers = &*timers(loop_);
    (*loop_).timer_heap.nelts = timers.len() as libc::c_uint;
    (*loop_).timer_heap.min = timers
        .first()
        .copied()
        .map(|timer| std::ptr::addr_of_mut!((*timer).node.heap).cast())
        .unwrap_or(std::ptr::null_mut());
}
