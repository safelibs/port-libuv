use crate::abi::linux_x86_64 as abi;
use crate::core::{
    allocator, default_loop, handle, loop_state, queue, request, timer, LoopState,
    UV_LOOP_BLOCK_SIGPROF, UV_LOOP_REAP_CHILDREN, UV_METRICS_IDLE_TIME_FLAG,
};
use std::ffi::c_void;
use std::os::raw::c_int;
use std::time::Duration;

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

unsafe extern "C" {
    fn clock_gettime(clock_id: c_int, tp: *mut Timespec) -> c_int;
}

const CLOCK_MONOTONIC: c_int = 1;
const SIGPROF_LINUX_X86_64: c_int = 27;

fn monotonic_now_ms() -> u64 {
    let mut ts = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let rc = unsafe { clock_gettime(CLOCK_MONOTONIC, &mut ts) };
    if rc != 0 {
        return 0;
    }
    (ts.tv_sec as u64)
        .saturating_mul(1_000)
        .saturating_add((ts.tv_nsec as u64) / 1_000_000)
}

unsafe fn alloc_loop_state() -> *mut LoopState {
    unsafe { allocator::alloc_value(LoopState::new()) }
}

pub(crate) unsafe fn update_time(loop_: *mut abi::uv_loop_t) {
    if loop_.is_null() {
        return;
    }
    unsafe {
        (*loop_).time = monotonic_now_ms();
    }
}

pub(crate) unsafe fn now(loop_: *const abi::uv_loop_t) -> u64 {
    if loop_.is_null() {
        return 0;
    }
    unsafe { (*loop_).time }
}

pub(crate) unsafe fn backend_fd(loop_: *const abi::uv_loop_t) -> c_int {
    if loop_.is_null() {
        return -1;
    }
    unsafe { (*loop_).backend_fd }
}

pub(crate) unsafe fn loop_init(loop_: *mut abi::uv_loop_t) -> c_int {
    if loop_.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let saved_data = unsafe { (*loop_).data };
    unsafe {
        std::ptr::write_bytes(loop_, 0, 1);
        (*loop_).data = saved_data;
    }

    let state = unsafe { alloc_loop_state() };
    if state.is_null() {
        return abi::uv_errno_t_UV_ENOMEM;
    }

    unsafe {
        (*loop_).internal_fields = state.cast::<c_void>();
        queue::init(std::ptr::addr_of_mut!((*loop_).handle_queue));
        queue::init(std::ptr::addr_of_mut!((*loop_).pending_queue));
        queue::init(std::ptr::addr_of_mut!((*loop_).watcher_queue));
        queue::init(std::ptr::addr_of_mut!((*loop_).wq));
        queue::init(std::ptr::addr_of_mut!((*loop_).process_handles));
        queue::init(std::ptr::addr_of_mut!((*loop_).prepare_handles));
        queue::init(std::ptr::addr_of_mut!((*loop_).check_handles));
        queue::init(std::ptr::addr_of_mut!((*loop_).idle_handles));
        queue::init(std::ptr::addr_of_mut!((*loop_).async_handles));
        (*loop_).active_handles = 0;
        (*loop_).active_reqs.count = 0;
        (*loop_).watchers = std::ptr::null_mut();
        (*loop_).nwatchers = 0;
        (*loop_).nfds = 0;
        (*loop_).closing_handles = std::ptr::null_mut();
        (*loop_).backend_fd = -1;
        (*loop_).async_io_watcher.fd = -1;
        (*loop_).async_wfd = -1;
        (*loop_).signal_pipefd = [-1, -1];
        (*loop_).emfile_fd = -1;
        (*loop_).timer_heap.min = std::ptr::null_mut();
        (*loop_).timer_heap.nelts = 0;
        (*loop_).timer_counter = 0;
        (*loop_).flags = 0;
        (*loop_).stop_flag = 0;
    }
    unsafe {
        update_time(loop_);
    }
    0
}

unsafe fn free_loop_state(loop_: *mut abi::uv_loop_t) {
    let state_ptr = unsafe { loop_state(loop_) };
    if state_ptr.is_null() {
        return;
    }

    {
        let state = unsafe { &*state_ptr };
        let mut inner = state.inner.lock().unwrap();

        if !inner.timer_heap.is_null() {
            unsafe {
                allocator::free_bytes(inner.timer_heap.cast());
            }
            inner.timer_heap = std::ptr::null_mut();
            inner.timer_len = 0;
            inner.timer_cap = 0;
        }

        let mut handle_record = inner.handle_records;
        while !handle_record.is_null() {
            let next = unsafe { (*handle_record).next };
            unsafe {
                allocator::free_bytes(handle_record.cast());
            }
            handle_record = next;
        }
        inner.handle_records = std::ptr::null_mut();

        let mut request_record = inner.request_records;
        while !request_record.is_null() {
            let next = unsafe { (*request_record).next };
            unsafe {
                allocator::free_bytes(request_record.cast());
            }
            request_record = next;
        }
        inner.request_records = std::ptr::null_mut();
    }

    unsafe {
        allocator::free_value(state_ptr);
        (*loop_).internal_fields = std::ptr::null_mut();
        (*loop_).timer_heap.min = std::ptr::null_mut();
        (*loop_).timer_heap.nelts = 0;
    }
}

pub(crate) unsafe fn loop_close(loop_: *mut abi::uv_loop_t) -> c_int {
    if loop_.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    if unsafe { (*loop_).active_reqs.count } != 0 {
        return abi::uv_errno_t_UV_EBUSY;
    }

    let head = unsafe { std::ptr::addr_of_mut!((*loop_).handle_queue) };
    let mut node = unsafe { (*head).next };
    while !std::ptr::eq(node, head) {
        let handle_ptr = unsafe {
            node.cast::<u8>()
                .sub(std::mem::offset_of!(abi::uv_handle_t, handle_queue))
                .cast::<abi::uv_handle_t>()
        };
        if unsafe { (*handle_ptr).flags & crate::core::UV_HANDLE_INTERNAL } == 0 {
            return abi::uv_errno_t_UV_EBUSY;
        }
        node = unsafe { (*node).next };
    }

    unsafe {
        free_loop_state(loop_);
    }

    if unsafe { default_loop::is_default_loop(loop_) } {
        unsafe {
            default_loop::mark_closed();
        }
    }

    0
}

pub(crate) unsafe fn loop_new() -> *mut abi::uv_loop_t {
    let loop_ptr = unsafe { allocator::alloc_zeroed::<abi::uv_loop_t>() };
    if loop_ptr.is_null() {
        return std::ptr::null_mut();
    }
    if unsafe { loop_init(loop_ptr) } != 0 {
        unsafe {
            allocator::free_bytes(loop_ptr.cast());
        }
        return std::ptr::null_mut();
    }
    loop_ptr
}

pub(crate) unsafe fn loop_delete(loop_: *mut abi::uv_loop_t) {
    if loop_.is_null() {
        return;
    }
    let is_default = unsafe { default_loop::is_default_loop(loop_) };
    let rc = unsafe { loop_close(loop_) };
    debug_assert_eq!(rc, 0);
    if rc == 0 && !is_default {
        unsafe {
            allocator::free_bytes(loop_.cast());
        }
    }
}

pub(crate) unsafe fn loop_alive(loop_: *const abi::uv_loop_t) -> c_int {
    if loop_.is_null() {
        return 0;
    }

    let state = unsafe { &*loop_state(loop_.cast_mut()) };
    let _guard = state.inner.lock().unwrap();
    (unsafe { (*loop_).active_handles != 0 }
        || unsafe { (*loop_).active_reqs.count != 0 }
        || unsafe { !queue::is_empty(std::ptr::addr_of!((*loop_).pending_queue)) }
        || unsafe { !(*loop_).closing_handles.is_null() }) as c_int
}

pub(crate) unsafe fn loop_configure(
    loop_: *mut abi::uv_loop_t,
    option: abi::uv_loop_option,
    arg: c_int,
) -> c_int {
    if loop_.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let state = unsafe { &*loop_state(loop_) };
    let mut inner = state.inner.lock().unwrap();
    if option == abi::uv_loop_option_UV_METRICS_IDLE_TIME {
        inner.metrics_flags |= UV_METRICS_IDLE_TIME_FLAG;
        return 0;
    }
    drop(inner);

    if option != abi::uv_loop_option_UV_LOOP_BLOCK_SIGNAL {
        return abi::uv_errno_t_UV_ENOSYS;
    }
    if arg != SIGPROF_LINUX_X86_64 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    unsafe {
        (*loop_).flags |= UV_LOOP_BLOCK_SIGPROF;
    }
    0
}

fn pending_queue_empty(loop_: *mut abi::uv_loop_t) -> bool {
    let state = unsafe { &*loop_state(loop_) };
    let _guard = state.inner.lock().unwrap();
    unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).pending_queue)) }
}

unsafe fn backend_timeout_inner(loop_: *mut abi::uv_loop_t) -> c_int {
    if unsafe { (*loop_).stop_flag } == 0
        && (unsafe { (*loop_).active_handles != 0 } || unsafe { (*loop_).active_reqs.count != 0 })
        && pending_queue_empty(loop_)
        && unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).idle_handles)) }
        && unsafe { ((*loop_).flags & UV_LOOP_REAP_CHILDREN) == 0 }
        && unsafe { (*loop_).closing_handles.is_null() }
    {
        return unsafe { timer::next_timeout(loop_) };
    }
    0
}

pub(crate) unsafe fn backend_timeout(loop_: *const abi::uv_loop_t) -> c_int {
    if loop_.is_null() {
        return 0;
    }
    if unsafe { !queue::is_empty(std::ptr::addr_of!((*loop_).watcher_queue)) } {
        return 0;
    }
    unsafe { backend_timeout_inner(loop_.cast_mut()) }
}

unsafe fn wait_for_events(loop_: *mut abi::uv_loop_t, timeout_ms: c_int) {
    if timeout_ms == 0 {
        return;
    }

    let state = unsafe { &*loop_state(loop_) };
    let mut guard = state.inner.lock().unwrap();
    if timeout_ms < 0 {
        while unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).pending_queue)) }
            && unsafe { (*loop_).stop_flag == 0 }
        {
            guard = state.wake.wait(guard).unwrap();
        }
        return;
    }

    if unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).pending_queue)) }
        && unsafe { (*loop_).stop_flag == 0 }
    {
        let _ = state
            .wake
            .wait_timeout(guard, Duration::from_millis(timeout_ms as u64))
            .unwrap();
    }
}

pub(crate) unsafe fn stop(loop_: *mut abi::uv_loop_t) {
    if !loop_.is_null() {
        unsafe {
            (*loop_).stop_flag = 1;
        }
        let state_ptr = unsafe { loop_state(loop_) };
        if !state_ptr.is_null() {
            unsafe {
                (*state_ptr).wake.notify_all();
            }
        }
    }
}

pub(crate) unsafe fn run(loop_: *mut abi::uv_loop_t, mode: abi::uv_run_mode) -> c_int {
    if loop_.is_null() {
        return 0;
    }

    let mut alive = unsafe { loop_alive(loop_) };
    if alive == 0 {
        unsafe { update_time(loop_) };
    }

    if mode == abi::uv_run_mode_UV_RUN_DEFAULT && alive != 0 && unsafe { (*loop_).stop_flag } == 0
    {
        unsafe {
            update_time(loop_);
            timer::run_timers(loop_);
        }
    }

    while alive != 0 && unsafe { (*loop_).stop_flag } == 0 {
        let can_sleep = pending_queue_empty(loop_)
            && unsafe { queue::is_empty(std::ptr::addr_of!((*loop_).idle_handles)) };

        unsafe {
            request::run_pending(loop_);
            handle::run_idle(loop_);
            handle::run_prepare(loop_);
        }

        let mut timeout = 0;
        if (mode == abi::uv_run_mode_UV_RUN_ONCE && can_sleep)
            || mode == abi::uv_run_mode_UV_RUN_DEFAULT
        {
            timeout = unsafe { backend_timeout_inner(loop_) };
        }

        unsafe { wait_for_events(loop_, timeout) };

        for _ in 0..8 {
            if pending_queue_empty(loop_) {
                break;
            }
            unsafe {
                request::run_pending(loop_);
            }
        }

        unsafe {
            handle::run_closing_handles(loop_);
            update_time(loop_);
            timer::run_timers(loop_);
        }

        alive = unsafe { loop_alive(loop_) };
        if mode == abi::uv_run_mode_UV_RUN_ONCE || mode == abi::uv_run_mode_UV_RUN_NOWAIT {
            break;
        }
    }

    if unsafe { (*loop_).stop_flag } != 0 {
        unsafe {
            (*loop_).stop_flag = 0;
        }
    }

    alive
}

pub(crate) unsafe fn metrics_idle_time(_loop_: *mut abi::uv_loop_t) -> u64 {
    0
}

pub(crate) unsafe fn library_shutdown() {}
