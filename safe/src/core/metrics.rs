use crate::abi::linux_x86_64 as abi;
use crate::core::loop_state;
use crate::core::time;
use std::os::raw::c_int;

#[derive(Default)]
pub(crate) struct LoopMetricsState {
    pub metrics: abi::uv_metrics_t,
    pub provider_entry_time: u64,
    pub provider_idle_time: u64,
    pub current_timeout: c_int,
    pub enabled: bool,
}

impl LoopMetricsState {
    pub(crate) const fn new() -> Self {
        Self {
            metrics: abi::uv_metrics_t {
                loop_count: 0,
                events: 0,
                events_waiting: 0,
                reserved: [std::ptr::null_mut(); 13],
            },
            provider_entry_time: 0,
            provider_idle_time: 0,
            current_timeout: 0,
            enabled: false,
        }
    }
}

pub(crate) unsafe fn enable_idle_time(loop_: *mut abi::uv_loop_t) {
    if loop_.is_null() {
        return;
    }
    let state = unsafe { &*loop_state(loop_) };
    let mut metrics = state.metrics.lock().unwrap();
    metrics.enabled = true;
}

pub(crate) unsafe fn increment_loop_count(loop_: *mut abi::uv_loop_t) {
    if loop_.is_null() {
        return;
    }
    let state = unsafe { &*loop_state(loop_) };
    let mut metrics = state.metrics.lock().unwrap();
    metrics.metrics.loop_count = metrics.metrics.loop_count.wrapping_add(1);
}

pub(crate) unsafe fn set_current_timeout(loop_: *mut abi::uv_loop_t, timeout: c_int) {
    if loop_.is_null() {
        return;
    }
    let state = unsafe { &*loop_state(loop_) };
    let mut metrics = state.metrics.lock().unwrap();
    metrics.current_timeout = timeout;
    if metrics.enabled && timeout != 0 {
        metrics.provider_entry_time = time::hrtime_precise_ns();
    } else {
        metrics.provider_entry_time = 0;
    }
}

pub(crate) unsafe fn update_idle_time(loop_: *mut abi::uv_loop_t) {
    if loop_.is_null() {
        return;
    }
    let state = unsafe { &*loop_state(loop_) };
    let mut metrics = state.metrics.lock().unwrap();
    if !metrics.enabled || metrics.provider_entry_time == 0 {
        return;
    }
    let exit_time = time::hrtime_precise_ns();
    let entry_time = metrics.provider_entry_time;
    metrics.provider_entry_time = 0;
    metrics.provider_idle_time = metrics
        .provider_idle_time
        .wrapping_add(exit_time.saturating_sub(entry_time));
}

pub(crate) unsafe fn record_event(loop_: *mut abi::uv_loop_t, count: u64) {
    if loop_.is_null() || count == 0 {
        return;
    }
    let state = unsafe { &*loop_state(loop_) };
    let mut metrics = state.metrics.lock().unwrap();
    metrics.metrics.events = metrics.metrics.events.wrapping_add(count);
    if metrics.current_timeout == 0 {
        metrics.metrics.events_waiting = metrics.metrics.events_waiting.wrapping_add(count);
    }
}

pub(crate) unsafe fn metrics_info(
    loop_: *mut abi::uv_loop_t,
    out: *mut abi::uv_metrics_t,
) -> c_int {
    if loop_.is_null() || out.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    let state = unsafe { &*loop_state(loop_) };
    let metrics = state.metrics.lock().unwrap();
    unsafe {
        *out = metrics.metrics;
    }
    0
}

pub(crate) unsafe fn metrics_idle_time(loop_: *mut abi::uv_loop_t) -> u64 {
    if loop_.is_null() {
        return 0;
    }
    let state = unsafe { &*loop_state(loop_) };
    let metrics = state.metrics.lock().unwrap();
    let mut idle_time = metrics.provider_idle_time;
    if metrics.provider_entry_time > 0 {
        idle_time = idle_time.wrapping_add(time::hrtime_precise_ns() - metrics.provider_entry_time);
    }
    idle_time
}
