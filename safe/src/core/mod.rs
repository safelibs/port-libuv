use crate::abi::linux_x86_64 as abi;
use std::sync::{Condvar, Mutex};

pub mod allocator;
pub mod default_loop;
pub mod error;
pub mod handle;
#[path = "loop.rs"]
pub mod loop_impl;
pub mod metrics;
pub mod queue;
pub mod request;
pub mod time;
pub mod timer;

pub use loop_impl as loop_;

pub(crate) const UV_HANDLE_CLOSING: u32 = 0x00000001;
pub(crate) const UV_HANDLE_CLOSED: u32 = 0x00000002;
pub(crate) const UV_HANDLE_ACTIVE: u32 = 0x00000004;
pub(crate) const UV_HANDLE_REF: u32 = 0x00000008;
pub(crate) const UV_HANDLE_INTERNAL: u32 = 0x00000010;

pub(crate) const UV_LOOP_BLOCK_SIGPROF: u64 = 0x1;
pub(crate) const UV_LOOP_REAP_CHILDREN: u64 = 0x2;
pub(crate) const UV_METRICS_IDLE_TIME_FLAG: u32 = 0x1;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HandleKind {
    Timer = 1,
    Prepare = 2,
    Idle = 3,
    Async = 4,
    Process = 5,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RequestKind {
    Work = 1,
}

#[repr(C)]
pub(crate) struct HandleRecord {
    pub next: *mut HandleRecord,
    pub handle: *mut abi::uv_handle_t,
    pub kind: u8,
    pub _reserved: [u8; 7],
    pub timer_generation: u64,
}

#[repr(C)]
pub(crate) struct RequestRecord {
    pub next: *mut RequestRecord,
    pub req: *mut abi::uv_req_t,
    pub kind: u8,
    pub _reserved: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct TimerHeapEntry {
    pub handle: *mut abi::uv_timer_t,
    pub timeout: u64,
    pub start_id: u64,
    pub generation: u64,
}

pub(crate) struct LoopStateInner {
    pub handle_records: *mut HandleRecord,
    pub request_records: *mut RequestRecord,
    pub timer_heap: *mut TimerHeapEntry,
    pub timer_len: usize,
    pub timer_cap: usize,
}

pub(crate) struct LoopState {
    pub inner: Mutex<LoopStateInner>,
    pub metrics: Mutex<metrics::LoopMetricsState>,
    pub wake: Condvar,
}

impl LoopState {
    pub(crate) fn new() -> Self {
        Self {
            inner: Mutex::new(LoopStateInner {
                handle_records: std::ptr::null_mut(),
                request_records: std::ptr::null_mut(),
                timer_heap: std::ptr::null_mut(),
                timer_len: 0,
                timer_cap: 0,
            }),
            metrics: Mutex::new(metrics::LoopMetricsState::new()),
            wake: Condvar::new(),
        }
    }
}

#[inline]
pub(crate) unsafe fn loop_state(loop_: *mut abi::uv_loop_t) -> *mut LoopState {
    unsafe { (*loop_).internal_fields.cast() }
}

pub(crate) unsafe fn find_handle_record(
    inner: &LoopStateInner,
    handle: *mut abi::uv_handle_t,
) -> *mut HandleRecord {
    let mut current = inner.handle_records;
    while !current.is_null() {
        if unsafe { (*current).handle == handle } {
            return current;
        }
        current = unsafe { (*current).next };
    }
    std::ptr::null_mut()
}

pub(crate) unsafe fn remove_handle_record(
    inner: &mut LoopStateInner,
    handle: *mut abi::uv_handle_t,
) -> *mut HandleRecord {
    let mut previous: *mut HandleRecord = std::ptr::null_mut();
    let mut current = inner.handle_records;
    while !current.is_null() {
        if unsafe { (*current).handle == handle } {
            let next = unsafe { (*current).next };
            if previous.is_null() {
                inner.handle_records = next;
            } else {
                unsafe {
                    (*previous).next = next;
                }
            }
            unsafe {
                (*current).next = std::ptr::null_mut();
            }
            return current;
        }
        previous = current;
        current = unsafe { (*current).next };
    }
    std::ptr::null_mut()
}

pub(crate) unsafe fn push_handle_record(inner: &mut LoopStateInner, record: *mut HandleRecord) {
    unsafe {
        (*record).next = inner.handle_records;
    }
    inner.handle_records = record;
}

pub(crate) unsafe fn find_request_record(
    inner: &LoopStateInner,
    req: *mut abi::uv_req_t,
) -> *mut RequestRecord {
    let mut current = inner.request_records;
    while !current.is_null() {
        if unsafe { (*current).req == req } {
            return current;
        }
        current = unsafe { (*current).next };
    }
    std::ptr::null_mut()
}

pub(crate) unsafe fn remove_request_record(
    inner: &mut LoopStateInner,
    req: *mut abi::uv_req_t,
) -> *mut RequestRecord {
    let mut previous: *mut RequestRecord = std::ptr::null_mut();
    let mut current = inner.request_records;
    while !current.is_null() {
        if unsafe { (*current).req == req } {
            let next = unsafe { (*current).next };
            if previous.is_null() {
                inner.request_records = next;
            } else {
                unsafe {
                    (*previous).next = next;
                }
            }
            unsafe {
                (*current).next = std::ptr::null_mut();
            }
            return current;
        }
        previous = current;
        current = unsafe { (*current).next };
    }
    std::ptr::null_mut()
}

pub(crate) unsafe fn push_request_record(inner: &mut LoopStateInner, record: *mut RequestRecord) {
    unsafe {
        (*record).next = inner.request_records;
    }
    inner.request_records = record;
}
