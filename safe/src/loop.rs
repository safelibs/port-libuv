use crate::abi::{
    self, UV_LOOP_REAP_CHILDREN, UV_METRICS_IDLE_TIME_FLAG,
};
use crate::allocator::{self, UV_EBUSY};
use crate::bindings::*;
use crate::handle::{
    handle_ref, handle_unref, has_active_handles, has_ref, is_active, is_closing, queue_empty,
    queue_head, queue_init, queue_insert_tail, queue_move, queue_next, queue_remove,
    UV_HANDLE_CLOSED, UV_HANDLE_CLOSING, UV_HANDLE_INTERNAL,
};
use crate::request::has_active_reqs;
use crate::state::{loop_fields, loop_metrics, loop_state, sync_timer_heap_fields, LoopState};
use crate::{r#async, timer, watchers};
use std::mem::{offset_of, MaybeUninit};

static mut DEFAULT_LOOP_PTR: *mut uv_loop_t = std::ptr::null_mut();
static mut DEFAULT_LOOP_STORAGE: MaybeUninit<uv_loop_t> = MaybeUninit::uninit();

#[inline]
unsafe fn metrics_enabled(loop_: *const uv_loop_t) -> bool {
    ((*loop_fields(loop_)).flags & UV_METRICS_IDLE_TIME_FLAG) != 0
}

#[inline]
pub(crate) unsafe fn metrics_inc_loop_count(loop_: *mut uv_loop_t) {
    (*loop_metrics(loop_)).metrics.loop_count += 1;
}

#[inline]
pub(crate) unsafe fn metrics_inc_events(loop_: *mut uv_loop_t, count: u64) {
    (*loop_metrics(loop_)).metrics.events += count;
}

#[inline]
pub(crate) unsafe fn metrics_inc_events_waiting(loop_: *mut uv_loop_t, count: u64) {
    (*loop_metrics(loop_)).metrics.events_waiting += count;
}

#[inline]
unsafe fn update_time_internal(loop_: *mut uv_loop_t) {
    (*loop_).time = uv_hrtime() / 1_000_000;
}

unsafe fn loop_alive_internal(loop_: *const uv_loop_t) -> bool {
    has_active_handles(loop_)
        || has_active_reqs(loop_)
        || !queue_empty(std::ptr::addr_of!((*loop_).pending_queue))
        || !(*loop_).closing_handles.is_null()
}

unsafe fn backend_timeout_internal(loop_: *const uv_loop_t) -> libc::c_int {
    if (*loop_).stop_flag == 0
        && (has_active_handles(loop_) || has_active_reqs(loop_))
        && queue_empty(std::ptr::addr_of!((*loop_).pending_queue))
        && queue_empty(std::ptr::addr_of!((*loop_).idle_handles))
        && ((*loop_).flags & UV_LOOP_REAP_CHILDREN) == 0
        && (*loop_).closing_handles.is_null()
    {
        return timer::uv__next_timeout_impl(loop_);
    }
    0
}

unsafe fn io_from_pending_queue(q: *mut uv__queue) -> *mut uv__io_t {
    q.cast::<u8>()
        .sub(offset_of!(uv__io_t, pending_queue))
        .cast::<uv__io_t>()
}

unsafe fn handle_from_handle_queue(q: *mut uv__queue) -> *mut uv_handle_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_handle_t, handle_queue))
        .cast::<uv_handle_t>()
}

unsafe fn run_pending(loop_: *mut uv_loop_t) {
    let mut pending = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).pending_queue),
        std::ptr::addr_of_mut!(pending),
    );

    while !queue_empty(std::ptr::addr_of!(pending)) {
        let q = queue_head(std::ptr::addr_of!(pending));
        queue_remove(q);
        queue_init(q);
        let watcher = io_from_pending_queue(q);
        if let Some(cb) = (*watcher).cb {
            cb(loop_, watcher, libc::POLLOUT as libc::c_uint);
        }
    }
}

unsafe fn make_close_pending(handle: *mut uv_handle_t) {
    (*handle).next_closing = (*(*handle).loop_).closing_handles;
    (*(*handle).loop_).closing_handles = handle;
}

unsafe fn finish_close(handle: *mut uv_handle_t) {
    debug_assert!(((*handle).flags & UV_HANDLE_CLOSING) != 0);
    debug_assert!(((*handle).flags & UV_HANDLE_CLOSED) == 0);
    (*handle).flags |= UV_HANDLE_CLOSED;

    match (*handle).type_ {
        uv_handle_type_UV_PREPARE
        | uv_handle_type_UV_CHECK
        | uv_handle_type_UV_IDLE
        | uv_handle_type_UV_ASYNC
        | uv_handle_type_UV_TIMER
        | uv_handle_type_UV_PROCESS
        | uv_handle_type_UV_FS_EVENT
        | uv_handle_type_UV_FS_POLL
        | uv_handle_type_UV_POLL => {}
        uv_handle_type_UV_SIGNAL => {
            let signal = handle.cast::<uv_signal_t>();
            if (*signal).caught_signals > (*signal).dispatched_signals {
                (*handle).flags ^= UV_HANDLE_CLOSED;
                make_close_pending(handle);
                return;
            }
        }
        uv_handle_type_UV_NAMED_PIPE | uv_handle_type_UV_TCP | uv_handle_type_UV_TTY => {
            abi::uv__stream_destroy(handle.cast::<uv_stream_t>());
        }
        uv_handle_type_UV_UDP => {
            abi::uv__udp_finish_close(handle.cast::<uv_udp_t>());
        }
        _ => libc::abort(),
    }

    handle_unref(handle);
    queue_remove(std::ptr::addr_of_mut!((*handle).handle_queue));

    if let Some(cb) = (*handle).close_cb {
        cb(handle);
    }
}

unsafe fn run_closing_handles(loop_: *mut uv_loop_t) {
    let mut current = (*loop_).closing_handles;
    (*loop_).closing_handles = std::ptr::null_mut();

    while !current.is_null() {
        let next = (*current).next_closing;
        finish_close(current);
        current = next;
    }
}

unsafe fn loop_close_internal(loop_: *mut uv_loop_t) {
    abi::uv__signal_loop_cleanup(loop_);
    crate::linux::epoll::uv__platform_loop_delete_impl(loop_);
    r#async::uv__async_stop_impl(loop_);

    if (*loop_).emfile_fd != -1 {
        abi::uv__close((*loop_).emfile_fd);
        (*loop_).emfile_fd = -1;
    }

    if (*loop_).backend_fd != -1 {
        abi::uv__close((*loop_).backend_fd);
        (*loop_).backend_fd = -1;
    }

    uv_mutex_destroy(std::ptr::addr_of_mut!((*loop_).wq_mutex));
    uv_rwlock_destroy(std::ptr::addr_of_mut!((*loop_).cloexec_lock));

    allocator::free((*loop_).watchers.cast());
    (*loop_).watchers = std::ptr::null_mut();
    (*loop_).nwatchers = 0;
    (*loop_).nfds = 0;
    sync_timer_heap_fields(loop_);

    uv_mutex_destroy(std::ptr::addr_of_mut!((*loop_metrics(loop_)).lock));
    drop(Box::from_raw(loop_state(loop_)));
    (*loop_).internal_fields = std::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_size() -> usize {
    std::mem::size_of::<uv_loop_t>()
}

#[no_mangle]
pub unsafe extern "C" fn uv_default_loop() -> *mut uv_loop_t {
    if !DEFAULT_LOOP_PTR.is_null() {
        return DEFAULT_LOOP_PTR;
    }

    let loop_ = DEFAULT_LOOP_STORAGE.as_mut_ptr();
    if uv_loop_init(loop_) != 0 {
        return std::ptr::null_mut();
    }

    DEFAULT_LOOP_PTR = loop_;
    DEFAULT_LOOP_PTR
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_new() -> *mut uv_loop_t {
    let loop_ = allocator::malloc(std::mem::size_of::<uv_loop_t>()).cast::<uv_loop_t>();
    if loop_.is_null() {
        return std::ptr::null_mut();
    }

    if uv_loop_init(loop_) != 0 {
        allocator::free(loop_.cast());
        return std::ptr::null_mut();
    }

    loop_
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_init(loop_: *mut uv_loop_t) -> libc::c_int {
    let saved_data = (*loop_).data;
    std::ptr::write_bytes(loop_, 0, 1);
    (*loop_).data = saved_data;

    let state = Box::new(LoopState::default());
    let state_ptr = Box::into_raw(state);
    (*loop_).internal_fields = state_ptr.cast();

    let err = uv_mutex_init(std::ptr::addr_of_mut!((*state_ptr).compat.loop_metrics.lock));
    if err != 0 {
        drop(Box::from_raw(state_ptr));
        (*loop_).internal_fields = std::ptr::null_mut();
        return err;
    }

    queue_init(std::ptr::addr_of_mut!((*loop_).wq));
    queue_init(std::ptr::addr_of_mut!((*loop_).idle_handles));
    queue_init(std::ptr::addr_of_mut!((*loop_).async_handles));
    queue_init(std::ptr::addr_of_mut!((*loop_).check_handles));
    queue_init(std::ptr::addr_of_mut!((*loop_).prepare_handles));
    queue_init(std::ptr::addr_of_mut!((*loop_).handle_queue));
    queue_init(std::ptr::addr_of_mut!((*loop_).pending_queue));
    queue_init(std::ptr::addr_of_mut!((*loop_).watcher_queue));
    queue_init(std::ptr::addr_of_mut!((*loop_).process_handles));

    (*loop_).active_handles = 0;
    (*loop_).active_reqs.count = 0;
    (*loop_).watchers = std::ptr::null_mut();
    (*loop_).nwatchers = 0;
    (*loop_).nfds = 0;
    (*loop_).closing_handles = std::ptr::null_mut();
    (*loop_).async_io_watcher.fd = -1;
    (*loop_).async_wfd = -1;
    (*loop_).signal_pipefd = [-1, -1];
    (*loop_).backend_fd = -1;
    (*loop_).emfile_fd = -1;
    (*loop_).timer_heap.min = std::ptr::null_mut();
    (*loop_).timer_heap.nelts = 0;
    (*loop_).timer_counter = 0;
    (*loop_).stop_flag = 0;
    (*loop_).flags = 0;
    update_time_internal(loop_);

    let err = crate::linux::epoll::uv__platform_loop_init_impl(loop_);
    if err != 0 {
        uv_mutex_destroy(std::ptr::addr_of_mut!((*state_ptr).compat.loop_metrics.lock));
        drop(Box::from_raw(state_ptr));
        (*loop_).internal_fields = std::ptr::null_mut();
        return err;
    }

    abi::uv__signal_global_once_init();
    let err = abi::uv__process_init(loop_);
    if err != 0 {
        crate::linux::epoll::uv__platform_loop_delete_impl(loop_);
        uv_mutex_destroy(std::ptr::addr_of_mut!((*state_ptr).compat.loop_metrics.lock));
        drop(Box::from_raw(state_ptr));
        (*loop_).internal_fields = std::ptr::null_mut();
        return err;
    }

    let err = uv_rwlock_init(std::ptr::addr_of_mut!((*loop_).cloexec_lock));
    if err != 0 {
        abi::uv__signal_loop_cleanup(loop_);
        crate::linux::epoll::uv__platform_loop_delete_impl(loop_);
        uv_mutex_destroy(std::ptr::addr_of_mut!((*state_ptr).compat.loop_metrics.lock));
        drop(Box::from_raw(state_ptr));
        (*loop_).internal_fields = std::ptr::null_mut();
        return err;
    }

    let err = uv_mutex_init(std::ptr::addr_of_mut!((*loop_).wq_mutex));
    if err != 0 {
        uv_rwlock_destroy(std::ptr::addr_of_mut!((*loop_).cloexec_lock));
        abi::uv__signal_loop_cleanup(loop_);
        crate::linux::epoll::uv__platform_loop_delete_impl(loop_);
        uv_mutex_destroy(std::ptr::addr_of_mut!((*state_ptr).compat.loop_metrics.lock));
        drop(Box::from_raw(state_ptr));
        (*loop_).internal_fields = std::ptr::null_mut();
        return err;
    }

    let err = r#async::uv_async_init(
        loop_,
        std::ptr::addr_of_mut!((*loop_).wq_async),
        Some(abi::uv__work_done),
    );
    if err != 0 {
        uv_mutex_destroy(std::ptr::addr_of_mut!((*loop_).wq_mutex));
        uv_rwlock_destroy(std::ptr::addr_of_mut!((*loop_).cloexec_lock));
        abi::uv__signal_loop_cleanup(loop_);
        crate::linux::epoll::uv__platform_loop_delete_impl(loop_);
        uv_mutex_destroy(std::ptr::addr_of_mut!((*state_ptr).compat.loop_metrics.lock));
        drop(Box::from_raw(state_ptr));
        (*loop_).internal_fields = std::ptr::null_mut();
        return err;
    }

    handle_unref(std::ptr::addr_of_mut!((*loop_).wq_async).cast());
    (*loop_).wq_async.flags |= UV_HANDLE_INTERNAL;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_close(loop_: *mut uv_loop_t) -> libc::c_int {
    if has_active_reqs(loop_) {
        return UV_EBUSY;
    }

    let head = std::ptr::addr_of_mut!((*loop_).handle_queue);
    let mut q = queue_head(head);
    while !std::ptr::eq(q, head) {
        let handle = handle_from_handle_queue(q);
        if ((*handle).flags & UV_HANDLE_INTERNAL) == 0 {
            return UV_EBUSY;
        }
        q = queue_next(q);
    }

    loop_close_internal(loop_);
    if std::ptr::eq(loop_, DEFAULT_LOOP_PTR) {
        DEFAULT_LOOP_PTR = std::ptr::null_mut();
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_delete(loop_: *mut uv_loop_t) {
    let default_loop = DEFAULT_LOOP_PTR;
    let err = uv_loop_close(loop_);
    debug_assert_eq!(err, 0);
    if !std::ptr::eq(loop_, default_loop) {
        allocator::free(loop_.cast());
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_fork(loop_: *mut uv_loop_t) -> libc::c_int {
    let err = crate::linux::epoll::uv__io_fork_impl(loop_);
    if err != 0 {
        return err;
    }

    let err = r#async::uv__async_fork_impl(loop_);
    if err != 0 {
        return err;
    }

    let err = abi::uv__signal_loop_fork(loop_);
    if err != 0 {
        return err;
    }

    for i in 0..(*loop_).nwatchers {
        let watcher = *(*loop_).watchers.add(i as usize);
        if watcher.is_null() {
            continue;
        }

        if (*watcher).pevents != 0 && queue_empty(std::ptr::addr_of!((*watcher).watcher_queue)) {
            (*watcher).events = 0;
            queue_insert_tail(
                std::ptr::addr_of_mut!((*loop_).watcher_queue),
                std::ptr::addr_of_mut!((*watcher).watcher_queue),
            );
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_backend_fd(loop_: *const uv_loop_t) -> libc::c_int {
    (*loop_).backend_fd
}

#[no_mangle]
pub unsafe extern "C" fn uv_backend_timeout(loop_: *const uv_loop_t) -> libc::c_int {
    if queue_empty(std::ptr::addr_of!((*loop_).watcher_queue)) {
        return backend_timeout_internal(loop_);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_alive(loop_: *const uv_loop_t) -> libc::c_int {
    loop_alive_internal(loop_) as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn uv_run(loop_: *mut uv_loop_t, mode: uv_run_mode) -> libc::c_int {
    let mut r = loop_alive_internal(loop_);
    if !r {
        update_time_internal(loop_);
    }

    if mode == uv_run_mode_UV_RUN_DEFAULT && r && (*loop_).stop_flag == 0 {
        update_time_internal(loop_);
        timer::uv__run_timers_impl(loop_);
    }

    while r && (*loop_).stop_flag == 0 {
        let can_sleep = queue_empty(std::ptr::addr_of!((*loop_).pending_queue))
            && queue_empty(std::ptr::addr_of!((*loop_).idle_handles));

        run_pending(loop_);
        watchers::uv__run_idle_impl(loop_);
        watchers::uv__run_prepare_impl(loop_);

        let mut timeout = 0;
        if (mode == uv_run_mode_UV_RUN_ONCE && can_sleep) || mode == uv_run_mode_UV_RUN_DEFAULT {
            timeout = backend_timeout_internal(loop_);
        }

        metrics_inc_loop_count(loop_);
        crate::linux::epoll::uv__io_poll_impl(loop_, timeout);

        let mut i = 0;
        while i < 8 && !queue_empty(std::ptr::addr_of!((*loop_).pending_queue)) {
            run_pending(loop_);
            i += 1;
        }

        uv__metrics_update_idle_time_impl(loop_);
        watchers::uv__run_check_impl(loop_);
        run_closing_handles(loop_);
        update_time_internal(loop_);
        timer::uv__run_timers_impl(loop_);

        r = loop_alive_internal(loop_);
        if mode == uv_run_mode_UV_RUN_ONCE || mode == uv_run_mode_UV_RUN_NOWAIT {
            break;
        }
    }

    if (*loop_).stop_flag != 0 {
        (*loop_).stop_flag = 0;
    }

    r as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn uv_update_time(loop_: *mut uv_loop_t) {
    update_time_internal(loop_);
}

#[no_mangle]
pub unsafe extern "C" fn uv_now(loop_: *const uv_loop_t) -> u64 {
    (*loop_).time
}

#[no_mangle]
pub unsafe extern "C" fn uv_stop(loop_: *mut uv_loop_t) {
    (*loop_).stop_flag = 1;
}

#[no_mangle]
pub unsafe extern "C" fn uv_ref(handle: *mut uv_handle_t) {
    handle_ref(handle);
}

#[no_mangle]
pub unsafe extern "C" fn uv_unref(handle: *mut uv_handle_t) {
    handle_unref(handle);
}

#[no_mangle]
pub unsafe extern "C" fn uv_has_ref(handle: *const uv_handle_t) -> libc::c_int {
    has_ref(handle) as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn uv_is_active(handle: *const uv_handle_t) -> libc::c_int {
    is_active(handle) as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn uv_is_closing(handle: *const uv_handle_t) -> libc::c_int {
    is_closing(handle) as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn uv_walk(
    loop_: *mut uv_loop_t,
    walk_cb: uv_walk_cb,
    arg: *mut libc::c_void,
) {
    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).handle_queue),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = handle_from_handle_queue(q);

        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).handle_queue), q);

        if ((*handle).flags & UV_HANDLE_INTERNAL) != 0 {
            continue;
        }

        if let Some(cb) = walk_cb {
            cb(handle, arg);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb) {
    debug_assert!(!is_closing(handle));

    (*handle).flags |= UV_HANDLE_CLOSING;
    (*handle).close_cb = close_cb;

    match (*handle).type_ {
        uv_handle_type_UV_NAMED_PIPE => abi::uv__pipe_close(handle.cast::<uv_pipe_t>()),
        uv_handle_type_UV_TTY => abi::uv__stream_close(handle.cast::<uv_stream_t>()),
        uv_handle_type_UV_TCP => abi::uv__tcp_close(handle.cast::<uv_tcp_t>()),
        uv_handle_type_UV_UDP => abi::uv__udp_close(handle.cast::<uv_udp_t>()),
        uv_handle_type_UV_PREPARE => {
            watchers::uv__prepare_close_impl(handle.cast::<uv_prepare_t>())
        }
        uv_handle_type_UV_CHECK => watchers::uv__check_close_impl(handle.cast::<uv_check_t>()),
        uv_handle_type_UV_IDLE => watchers::uv__idle_close_impl(handle.cast::<uv_idle_t>()),
        uv_handle_type_UV_ASYNC => r#async::uv__async_close_impl(handle.cast::<uv_async_t>()),
        uv_handle_type_UV_TIMER => timer::uv__timer_close_impl(handle.cast::<uv_timer_t>()),
        uv_handle_type_UV_PROCESS => abi::uv__process_close(handle.cast::<uv_process_t>()),
        uv_handle_type_UV_FS_EVENT => abi::uv__fs_event_close(handle.cast::<uv_fs_event_t>()),
        uv_handle_type_UV_POLL => abi::uv__poll_close(handle.cast::<uv_poll_t>()),
        uv_handle_type_UV_FS_POLL => {
            abi::uv__fs_poll_close(handle.cast::<uv_fs_poll_t>());
            return;
        }
        uv_handle_type_UV_SIGNAL => abi::uv__signal_close(handle.cast::<uv_signal_t>()),
        _ => libc::abort(),
    }

    make_close_pending(handle);
}

#[no_mangle]
pub unsafe extern "C" fn uv_metrics_info(
    loop_: *mut uv_loop_t,
    metrics: *mut uv_metrics_t,
) -> libc::c_int {
    std::ptr::copy_nonoverlapping(
        std::ptr::addr_of!((*loop_metrics(loop_)).metrics),
        metrics,
        1,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_metrics_idle_time(loop_: *mut uv_loop_t) -> u64 {
    let metrics_state = loop_metrics(loop_);
    uv_mutex_lock(std::ptr::addr_of_mut!((*metrics_state).lock));
    let idle_time = (*metrics_state).provider_idle_time;
    let entry_time = (*metrics_state).provider_entry_time;
    uv_mutex_unlock(std::ptr::addr_of_mut!((*metrics_state).lock));

    if entry_time > 0 {
        idle_time + uv_hrtime() - entry_time
    } else {
        idle_time
    }
}

pub(crate) unsafe extern "C" fn uv__metrics_update_idle_time_impl(loop_: *mut uv_loop_t) {
    if !metrics_enabled(loop_) {
        return;
    }

    let metrics_state = loop_metrics(loop_);
    if (*metrics_state).provider_entry_time == 0 {
        return;
    }

    let exit_time = uv_hrtime();
    uv_mutex_lock(std::ptr::addr_of_mut!((*metrics_state).lock));
    let entry_time = (*metrics_state).provider_entry_time;
    (*metrics_state).provider_entry_time = 0;
    (*metrics_state).provider_idle_time += exit_time - entry_time;
    uv_mutex_unlock(std::ptr::addr_of_mut!((*metrics_state).lock));
}

pub(crate) unsafe extern "C" fn uv__metrics_set_provider_entry_time_impl(
    loop_: *mut uv_loop_t,
) {
    if !metrics_enabled(loop_) {
        return;
    }

    let now = uv_hrtime();
    let metrics_state = loop_metrics(loop_);
    uv_mutex_lock(std::ptr::addr_of_mut!((*metrics_state).lock));
    (*metrics_state).provider_entry_time = now;
    uv_mutex_unlock(std::ptr::addr_of_mut!((*metrics_state).lock));
}
