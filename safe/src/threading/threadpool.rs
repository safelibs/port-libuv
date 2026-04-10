use crate::abi::linux_x86_64 as abi;
use crate::core::{allocator, queue};
use crate::threading::sync;
use crate::unix_async;
use libc::{self, c_char, c_int};
use std::collections::VecDeque;
use std::ffi::CStr;
use std::mem::{offset_of, zeroed, MaybeUninit};
use std::ptr::{self, null_mut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, Once};

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum TaskClass {
    Cpu,
    SlowIo,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TaskStatus {
    Pending,
    Running,
    CompletedPending,
    CancelledPending,
}

#[repr(C)]
struct TaskState {
    queue: abi::uv__queue,
    req: *mut abi::uv_req_t,
    work: *mut abi::uv__work,
    loop_: *mut abi::uv_loop_t,
    class: TaskClass,
    status: TaskStatus,
}

unsafe impl Send for TaskState {}
unsafe impl Sync for TaskState {}

#[derive(Clone, Copy)]
enum QueueItem {
    Task(*mut TaskState),
    RunSlow,
}

struct Inner {
    queue: VecDeque<QueueItem>,
    slow_pending: VecDeque<*mut TaskState>,
    run_slow_enqueued: bool,
    idle_threads: usize,
    slow_running: usize,
    shutdown: bool,
}

unsafe impl Send for Inner {}

struct Shared {
    inner: Mutex<Inner>,
    cond: Condvar,
    nthreads: usize,
}

struct Pool {
    shared: Arc<Shared>,
    workers: Mutex<Vec<std::thread::JoinHandle<()>>>,
    cleaned: AtomicBool,
}

#[derive(Clone, Copy)]
struct PoolControl {
    pid: libc::pid_t,
    pool: *mut Pool,
}

struct PoolControlCell(std::cell::UnsafeCell<PoolControl>);

unsafe impl Sync for PoolControlCell {}

static THREADPOOL_CONTROL: PoolControlCell = PoolControlCell(std::cell::UnsafeCell::new(
    PoolControl {
        pid: 0,
        pool: null_mut(),
    },
));
static INIT_LOCK_ONCE: Once = Once::new();
static mut INIT_LOCK: MaybeUninit<abi::uv_mutex_t> = MaybeUninit::uninit();

unsafe extern "C" {
    #[link_name = "getaddrinfo"]
    fn c_getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const abi::addrinfo,
        res: *mut *mut abi::addrinfo,
    ) -> c_int;
    #[link_name = "freeaddrinfo"]
    fn c_freeaddrinfo(ai: *mut abi::addrinfo);
    #[link_name = "getnameinfo"]
    fn c_getnameinfo(
        sa: *const abi::sockaddr,
        salen: libc::socklen_t,
        host: *mut c_char,
        hostlen: libc::socklen_t,
        serv: *mut c_char,
        servlen: libc::socklen_t,
        flags: c_int,
    ) -> c_int;
}

#[inline]
fn req_state(req: *mut abi::uv_req_t) -> *mut TaskState {
    unsafe { (*req).reserved[0].cast() }
}

#[inline]
unsafe fn set_req_state(req: *mut abi::uv_req_t, state: *mut TaskState) {
    unsafe {
        (*req).reserved[0] = state.cast();
    }
}

#[inline]
fn uv_err(errno: c_int) -> c_int {
    if errno == 0 {
        0
    } else {
        -errno
    }
}

#[inline]
fn last_errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

fn slow_threshold(nthreads: usize) -> usize {
    (nthreads + 1) / 2
}

fn configured_thread_count() -> usize {
    let mut nthreads = 4usize;
    if let Ok(value) = std::env::var("UV_THREADPOOL_SIZE") {
        if let Ok(parsed) = value.parse::<usize>() {
            nthreads = parsed;
        }
    }
    nthreads.clamp(1, 1024)
}

fn current_pid() -> libc::pid_t {
    unsafe { libc::getpid() }
}

unsafe fn init_lock() -> *mut abi::uv_mutex_t {
    INIT_LOCK_ONCE.call_once(|| unsafe {
        let lock = INIT_LOCK.as_mut_ptr();
        std::ptr::write_bytes(lock, 0, 1);
        let rc = sync::mutex_init_raw(lock, false);
        if rc != 0 {
            panic!("failed to initialize threadpool lock: {rc}");
        }
    });

    unsafe { INIT_LOCK.as_mut_ptr() }
}

unsafe fn reset_init_lock_after_fork() {
    unsafe {
        let lock = INIT_LOCK.as_mut_ptr();
        std::ptr::write_bytes(lock, 0, 1);
        let rc = sync::mutex_init_raw(lock, false);
        if rc != 0 {
            panic!("failed to reset threadpool lock after fork: {rc}");
        }
    }
}

fn build_pool() -> Box<Pool> {
    let nthreads = configured_thread_count();
    let shared = Arc::new(Shared {
        inner: Mutex::new(Inner {
            queue: VecDeque::new(),
            slow_pending: VecDeque::new(),
            run_slow_enqueued: false,
            idle_threads: 0,
            slow_running: 0,
            shutdown: false,
        }),
        cond: Condvar::new(),
        nthreads,
    });

    let mut workers = Vec::with_capacity(nthreads);
    for _ in 0..nthreads {
        let worker_shared = Arc::clone(&shared);
        let handle = std::thread::Builder::new()
            .stack_size(8 << 20)
            .spawn(move || worker_main(worker_shared))
            .unwrap_or_else(|_| panic!("failed to create threadpool worker"));
        workers.push(handle);
    }

    Box::new(Pool {
        shared,
        workers: Mutex::new(workers),
        cleaned: AtomicBool::new(false),
    })
}

fn pool() -> &'static Pool {
    let pid = current_pid();

    unsafe {
        let control = &mut *THREADPOOL_CONTROL.0.get();
        if control.pid == pid
            && !control.pool.is_null()
            && !(*control.pool).cleaned.load(Ordering::Acquire)
        {
            return &*control.pool;
        }

        if control.pid != 0 && control.pid != pid {
            reset_init_lock_after_fork();
            let pool = Box::into_raw(build_pool());
            control.pid = pid;
            control.pool = pool;
            return &*pool;
        }

        let lock = init_lock();
        sync::mutex_lock_raw(lock);

        let control = &mut *THREADPOOL_CONTROL.0.get();
        if control.pid != pid
            || control.pool.is_null()
            || (*control.pool).cleaned.load(Ordering::Acquire)
        {
            let pool = Box::into_raw(build_pool());
            control.pid = pid;
            control.pool = pool;
        }

        let pool = &*control.pool;
        sync::mutex_unlock_raw(lock);
        pool
    }
}

fn worker_main(shared: Arc<Shared>) {
    loop {
        let (state_ptr, slow) = {
            let mut inner = shared.inner.lock().unwrap();
            loop {
                let only_slow = matches!(inner.queue.front(), Some(QueueItem::RunSlow))
                    && inner.queue.len() == 1
                    && inner.slow_running >= slow_threshold(shared.nthreads);

                if inner.shutdown {
                    return;
                }

                if inner.queue.is_empty() || only_slow {
                    inner.idle_threads += 1;
                    inner = shared.cond.wait(inner).unwrap();
                    inner.idle_threads -= 1;
                    continue;
                }

                match inner.queue.pop_front().unwrap() {
                    QueueItem::Task(task) => {
                        unsafe {
                            (*task).status = TaskStatus::Running;
                        }
                        break (task, false);
                    }
                    QueueItem::RunSlow => {
                        if inner.slow_running >= slow_threshold(shared.nthreads) {
                            inner.queue.push_back(QueueItem::RunSlow);
                            continue;
                        }

                        let Some(task) = inner.slow_pending.pop_front() else {
                            inner.run_slow_enqueued = false;
                            continue;
                        };

                        inner.slow_running += 1;
                        if !inner.slow_pending.is_empty() {
                            inner.queue.push_back(QueueItem::RunSlow);
                            inner.run_slow_enqueued = true;
                            if inner.idle_threads > 0 {
                                shared.cond.notify_one();
                            }
                        } else {
                            inner.run_slow_enqueued = false;
                        }

                        unsafe {
                            (*task).status = TaskStatus::Running;
                        }
                        break (task, true);
                    }
                }
            }
        };

        unsafe {
            if let Some(work_cb) = (*(*state_ptr).work).work {
                work_cb((*state_ptr).work);
            }
        }

        {
            let mut inner = shared.inner.lock().unwrap();
            if slow {
                inner.slow_running -= 1;
            }
            unsafe {
                if (*state_ptr).status == TaskStatus::Running {
                    (*state_ptr).status = TaskStatus::CompletedPending;
                }
            }
        }

        unsafe {
            queue_completion(state_ptr);
        }
    }
}

unsafe fn queue_completion(state_ptr: *mut TaskState) {
    let loop_ = unsafe { (*state_ptr).loop_ };
    unsafe {
        sync::mutex_lock_raw(std::ptr::addr_of_mut!((*loop_).wq_mutex));
        queue::insert_tail(
            std::ptr::addr_of_mut!((*loop_).wq),
            std::ptr::addr_of_mut!((*state_ptr).queue),
        );
        sync::mutex_unlock_raw(std::ptr::addr_of_mut!((*loop_).wq_mutex));
    }
    let _ = unsafe { unix_async::send(std::ptr::addr_of_mut!((*loop_).wq_async)) };
}

pub(crate) unsafe fn submit(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_req_t,
    work: *mut abi::uv__work,
    class: TaskClass,
) -> c_int {
    if loop_.is_null() || req.is_null() || work.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if !req_state(req).is_null() {
        return abi::uv_errno_t_UV_EBUSY;
    }

    let pool = pool();
    let mut state = Box::new(TaskState {
        queue: abi::uv__queue::default(),
        req,
        work,
        loop_,
        class,
        status: TaskStatus::Pending,
    });
    unsafe {
        queue::init(std::ptr::addr_of_mut!(state.queue));
    }
    let raw = Box::into_raw(state);

    unsafe {
        set_req_state(req, raw);
        (*work).loop_ = loop_;
        (*loop_).active_reqs.count += 1;
    }

    let mut inner = pool.shared.inner.lock().unwrap();
    match class {
        TaskClass::Cpu => inner.queue.push_back(QueueItem::Task(raw)),
        TaskClass::SlowIo => {
            inner.slow_pending.push_back(raw);
            if !inner.run_slow_enqueued {
                inner.run_slow_enqueued = true;
                inner.queue.push_back(QueueItem::RunSlow);
            }
        }
    }
    if inner.idle_threads > 0 {
        pool.shared.cond.notify_one();
    }
    0
}

unsafe fn drain_loop_queue(loop_: *mut abi::uv_loop_t) {
    let mut local = abi::uv__queue::default();
    unsafe {
        queue::init(std::ptr::addr_of_mut!(local));
        sync::mutex_lock_raw(std::ptr::addr_of_mut!((*loop_).wq_mutex));
        queue::move_queue(
            std::ptr::addr_of_mut!((*loop_).wq),
            std::ptr::addr_of_mut!(local),
        );
        sync::mutex_unlock_raw(std::ptr::addr_of_mut!((*loop_).wq_mutex));
    }

    let mut nevents = 0u64;
    while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
        let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
        let state_ptr = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(TaskState, queue))
                .cast::<TaskState>()
        };
        unsafe {
            queue::remove(node);
            queue::init(node);
        }

        let err = unsafe {
            if (*state_ptr).status == TaskStatus::CancelledPending {
                abi::uv_errno_t_UV_ECANCELED
            } else {
                0
            }
        };

        let req = unsafe { (*state_ptr).req };
        unsafe {
            set_req_state(req, null_mut());
            (*loop_).active_reqs.count -= 1;
        }

        if let Some(done_cb) = unsafe { (*(*state_ptr).work).done } {
            unsafe {
                done_cb((*state_ptr).work, err);
            }
        }

        unsafe {
            drop(Box::from_raw(state_ptr));
        }
        nevents += 1;
    }

    if nevents > 1 {
        unsafe {
            crate::unix::epoll::record_work_queue_events(loop_, nevents - 1);
        }
    }
}

pub(crate) unsafe extern "C" fn loop_wq_async_cb(handle: *mut abi::uv_async_t) {
    if handle.is_null() {
        return;
    }
    unsafe {
        drain_loop_queue((*handle).loop_);
    }
}

pub(crate) fn cleanup() {
    let pid = current_pid();

    unsafe {
        let control = &mut *THREADPOOL_CONTROL.0.get();
        if control.pid != pid || control.pool.is_null() {
            return;
        }

        let lock = init_lock();
        sync::mutex_lock_raw(lock);

        let control = &mut *THREADPOOL_CONTROL.0.get();
        if control.pid != pid || control.pool.is_null() {
            sync::mutex_unlock_raw(lock);
            return;
        }

        let pool_ptr = control.pool;
        if (*pool_ptr).cleaned.swap(true, Ordering::AcqRel) {
            sync::mutex_unlock_raw(lock);
            return;
        }

        control.pool = null_mut();
        sync::mutex_unlock_raw(lock);

        {
            let mut inner = (&(*pool_ptr).shared).inner.lock().unwrap();
            inner.shutdown = true;
        }
        (&(*pool_ptr).shared).cond.notify_all();

        let mut workers = (*pool_ptr).workers.lock().unwrap();
        let handles: Vec<_> = workers.drain(..).collect();
        drop(workers);

        for handle in handles {
            let _ = handle.join();
        }

        drop(Box::from_raw(pool_ptr));
    }
}

pub(crate) unsafe fn queue_work(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_work_t,
    work_cb: abi::uv_work_cb,
    after_work_cb: abi::uv_after_work_cb,
) -> c_int {
    if loop_.is_null() || req.is_null() || work_cb.is_none() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let data = unsafe { (*req).data };
    unsafe {
        std::ptr::write_bytes(req, 0, 1);
        (*req).data = data;
        (*req).type_ = abi::uv_req_type_UV_WORK;
        (*req).loop_ = loop_;
        (*req).work_cb = work_cb;
        (*req).after_work_cb = after_work_cb;
        (*req).work_req.work = Some(work_cb_trampoline);
        (*req).work_req.done = Some(work_done_trampoline);
        queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
    }

    unsafe {
        submit(
            loop_,
            req.cast(),
            std::ptr::addr_of_mut!((*req).work_req),
            TaskClass::Cpu,
        )
    }
}

unsafe extern "C" fn work_cb_trampoline(work: *mut abi::uv__work) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_work_t, work_req))
            .cast::<abi::uv_work_t>()
    };
    if let Some(cb) = unsafe { (*req).work_cb } {
        unsafe {
            cb(req);
        }
    }
}

unsafe extern "C" fn work_done_trampoline(work: *mut abi::uv__work, status: c_int) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_work_t, work_req))
            .cast::<abi::uv_work_t>()
    };
    if let Some(cb) = unsafe { (*req).after_work_cb } {
        unsafe {
            cb(req, status);
        }
    }
}

pub(crate) unsafe fn cancel(req: *mut abi::uv_req_t) -> c_int {
    if req.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    match unsafe { (*req).type_ } {
        abi::uv_req_type_UV_FS
        | abi::uv_req_type_UV_GETADDRINFO
        | abi::uv_req_type_UV_GETNAMEINFO
        | abi::uv_req_type_UV_RANDOM
        | abi::uv_req_type_UV_WORK => {}
        _ => return abi::uv_errno_t_UV_EINVAL,
    }

    let state_ptr = req_state(req);
    if state_ptr.is_null() {
        return abi::uv_errno_t_UV_EBUSY;
    }

    let pool = pool();
    let mut inner = pool.shared.inner.lock().unwrap();
    let cancelled = unsafe { (*state_ptr).status == TaskStatus::Pending };
    if !cancelled {
        return abi::uv_errno_t_UV_EBUSY;
    }

    match unsafe { (*state_ptr).class } {
        TaskClass::Cpu => {
            if let Some(idx) = inner
                .queue
                .iter()
                .position(|item| matches!(item, QueueItem::Task(task) if *task == state_ptr))
            {
                inner.queue.remove(idx);
            }
        }
        TaskClass::SlowIo => {
            if let Some(idx) = inner
                .slow_pending
                .iter()
                .position(|task| *task == state_ptr)
            {
                inner.slow_pending.remove(idx);
            }
        }
    }

    unsafe {
        (*state_ptr).status = TaskStatus::CancelledPending;
    }
    drop(inner);

    unsafe {
        queue_completion(state_ptr);
    }
    0
}

fn clone_c_string(value: *const c_char) -> *mut c_char {
    if value.is_null() {
        return null_mut();
    }

    let bytes = unsafe { CStr::from_ptr(value).to_bytes_with_nul() };
    let ptr = unsafe { allocator::malloc_bytes(bytes.len()) }.cast::<c_char>();
    if ptr.is_null() {
        return null_mut();
    }
    unsafe {
        ptr::copy_nonoverlapping(value, ptr, bytes.len());
    }
    ptr
}

fn translate_eai_error(err: c_int) -> c_int {
    const EAI_ADDRFAMILY_GNU: c_int = -9;
    const EAI_CANCELED_GNU: c_int = -101;

    match err {
        0 => 0,
        x if x == EAI_ADDRFAMILY_GNU => abi::uv_errno_t_UV_EAI_ADDRFAMILY,
        x if x == libc::EAI_AGAIN => abi::uv_errno_t_UV_EAI_AGAIN,
        x if x == libc::EAI_BADFLAGS => abi::uv_errno_t_UV_EAI_BADFLAGS,
        x if x == EAI_CANCELED_GNU => abi::uv_errno_t_UV_EAI_CANCELED,
        x if x == libc::EAI_FAIL => abi::uv_errno_t_UV_EAI_FAIL,
        x if x == libc::EAI_FAMILY => abi::uv_errno_t_UV_EAI_FAMILY,
        x if x == libc::EAI_MEMORY => abi::uv_errno_t_UV_EAI_MEMORY,
        #[allow(unreachable_patterns)]
        x if x == libc::EAI_NODATA => abi::uv_errno_t_UV_EAI_NODATA,
        x if x == libc::EAI_NONAME => abi::uv_errno_t_UV_EAI_NONAME,
        x if x == libc::EAI_OVERFLOW => abi::uv_errno_t_UV_EAI_OVERFLOW,
        x if x == libc::EAI_SERVICE => abi::uv_errno_t_UV_EAI_SERVICE,
        x if x == libc::EAI_SOCKTYPE => abi::uv_errno_t_UV_EAI_SOCKTYPE,
        x if x == libc::EAI_SYSTEM => -last_errno(),
        _ => abi::uv_errno_t_UV_EAI_FAIL,
    }
}

unsafe extern "C" fn getaddrinfo_work(work: *mut abi::uv__work) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_getaddrinfo_t, work_req))
            .cast::<abi::uv_getaddrinfo_t>()
    };
    let mut result: *mut abi::addrinfo = null_mut();
    let err = unsafe { c_getaddrinfo((*req).hostname, (*req).service, (*req).hints, &mut result) };
    unsafe {
        (*req).addrinfo = result;
        (*req).retcode = translate_eai_error(err);
    }
}

unsafe extern "C" fn getaddrinfo_done(work: *mut abi::uv__work, status: c_int) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_getaddrinfo_t, work_req))
            .cast::<abi::uv_getaddrinfo_t>()
    };

    if !unsafe { (*req).hints.is_null() } {
        unsafe { allocator::free_bytes((*req).hints.cast()) };
    }
    if !unsafe { (*req).service.is_null() } {
        unsafe { allocator::free_bytes((*req).service.cast()) };
    }
    if !unsafe { (*req).hostname.is_null() } {
        unsafe { allocator::free_bytes((*req).hostname.cast()) };
    }

    unsafe {
        (*req).hints = null_mut();
        (*req).service = null_mut();
        (*req).hostname = null_mut();
        if status == abi::uv_errno_t_UV_ECANCELED {
            (*req).retcode = abi::uv_errno_t_UV_EAI_CANCELED;
            (*req).addrinfo = null_mut();
        }
        if let Some(cb) = (*req).cb {
            cb(req, (*req).retcode, (*req).addrinfo);
        }
    }
}

pub(crate) unsafe fn getaddrinfo(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getaddrinfo_t,
    cb: abi::uv_getaddrinfo_cb,
    node: *const c_char,
    service: *const c_char,
    hints: *const abi::addrinfo,
) -> c_int {
    if req.is_null() || (node.is_null() && service.is_null()) {
        return abi::uv_errno_t_UV_EINVAL;
    }

    if cb.is_none() {
        let prepared_node = match unsafe { crate::unix::getaddrinfo::prepare_hostname(node) } {
            Ok(node) => node,
            Err(err) => return err,
        };
        let mut result: *mut abi::addrinfo = null_mut();
        let rc = unsafe { c_getaddrinfo(prepared_node, service, hints, &mut result) };
        unsafe {
            allocator::free_bytes(prepared_node.cast());
        }
        unsafe {
            (*req).type_ = abi::uv_req_type_UV_GETADDRINFO;
            (*req).loop_ = loop_;
            (*req).cb = cb;
            (*req).addrinfo = result;
            (*req).retcode = translate_eai_error(rc);
        }
        return unsafe { (*req).retcode };
    }

    let cloned_hints = if hints.is_null() {
        null_mut()
    } else {
        let out = unsafe { allocator::alloc_zeroed::<abi::addrinfo>() };
        if out.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }
        unsafe {
            *out = *hints;
        }
        out
    };
    let cloned_service = clone_c_string(service);
    if !service.is_null() && cloned_service.is_null() {
        unsafe {
            allocator::free_bytes(cloned_hints.cast());
        }
        return abi::uv_errno_t_UV_ENOMEM;
    }
    let cloned_node = match unsafe { crate::unix::getaddrinfo::prepare_hostname(node) } {
        Ok(node) => node,
        Err(err) => {
            unsafe {
                allocator::free_bytes(cloned_hints.cast());
                allocator::free_bytes(cloned_service.cast());
            }
            return err;
        }
    };
    if !node.is_null() && cloned_node.is_null() {
        unsafe {
            allocator::free_bytes(cloned_hints.cast());
            allocator::free_bytes(cloned_service.cast());
        }
        return abi::uv_errno_t_UV_ENOMEM;
    }

    let data = unsafe { (*req).data };
    unsafe {
        std::ptr::write_bytes(req, 0, 1);
        (*req).data = data;
        (*req).type_ = abi::uv_req_type_UV_GETADDRINFO;
        (*req).loop_ = loop_;
        (*req).cb = cb;
        (*req).addrinfo = null_mut();
        (*req).retcode = 0;
        (*req).hints = cloned_hints;
        (*req).service = cloned_service;
        (*req).hostname = cloned_node;
        (*req).work_req.work = Some(getaddrinfo_work);
        (*req).work_req.done = Some(getaddrinfo_done);
        queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
    }

    unsafe {
        submit(
            loop_,
            req.cast(),
            std::ptr::addr_of_mut!((*req).work_req),
            TaskClass::SlowIo,
        )
    }
}

pub(crate) unsafe fn freeaddrinfo(ai: *mut abi::addrinfo) {
    if !ai.is_null() {
        unsafe {
            c_freeaddrinfo(ai);
        }
    }
}

unsafe extern "C" fn getnameinfo_work(work: *mut abi::uv__work) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_getnameinfo_t, work_req))
            .cast::<abi::uv_getnameinfo_t>()
    };

    let salen = match unsafe { (*req).storage.ss_family as c_int } {
        libc::AF_INET => std::mem::size_of::<abi::sockaddr_in>() as libc::socklen_t,
        libc::AF_INET6 => std::mem::size_of::<abi::sockaddr_in6>() as libc::socklen_t,
        _ => {
            unsafe {
                (*req).retcode = abi::uv_errno_t_UV_EINVAL;
            }
            return;
        }
    };

    let rc = unsafe {
        c_getnameinfo(
            std::ptr::addr_of!((*req).storage).cast(),
            salen,
            (*req).host.as_mut_ptr(),
            (*req).host.len() as libc::socklen_t,
            (*req).service.as_mut_ptr(),
            (*req).service.len() as libc::socklen_t,
            (*req).flags,
        )
    };
    unsafe {
        (*req).retcode = translate_eai_error(rc);
    }
}

unsafe extern "C" fn getnameinfo_done(work: *mut abi::uv__work, status: c_int) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_getnameinfo_t, work_req))
            .cast::<abi::uv_getnameinfo_t>()
    };
    let (host, service) = unsafe {
        if status == abi::uv_errno_t_UV_ECANCELED {
            (*req).retcode = abi::uv_errno_t_UV_EAI_CANCELED;
            (null_mut(), null_mut())
        } else if (*req).retcode == 0 {
            ((*req).host.as_mut_ptr(), (*req).service.as_mut_ptr())
        } else {
            (null_mut(), null_mut())
        }
    };

    unsafe {
        if let Some(cb) = (*req).getnameinfo_cb {
            cb(req, (*req).retcode, host, service);
        }
    }
}

pub(crate) unsafe fn getnameinfo(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getnameinfo_t,
    cb: abi::uv_getnameinfo_cb,
    addr: *const abi::sockaddr,
    flags: c_int,
) -> c_int {
    if req.is_null() || addr.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let family = unsafe { (*addr).sa_family as c_int };
    if family != libc::AF_INET && family != libc::AF_INET6 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    unsafe {
        std::ptr::write_bytes(req, 0, 1);
        (*req).type_ = abi::uv_req_type_UV_GETNAMEINFO;
        (*req).loop_ = loop_;
        (*req).getnameinfo_cb = cb;
        (*req).flags = flags;
    }

    if family == libc::AF_INET {
        unsafe {
            ptr::copy_nonoverlapping(
                addr.cast::<u8>(),
                std::ptr::addr_of_mut!((*req).storage).cast(),
                std::mem::size_of::<abi::sockaddr_in>(),
            );
        }
    } else {
        unsafe {
            ptr::copy_nonoverlapping(
                addr.cast::<u8>(),
                std::ptr::addr_of_mut!((*req).storage).cast(),
                std::mem::size_of::<abi::sockaddr_in6>(),
            );
        }
    }

    if cb.is_none() {
        unsafe {
            getnameinfo_work(std::ptr::addr_of_mut!((*req).work_req));
        }
        return unsafe { (*req).retcode };
    }

    unsafe {
        (*req).work_req.work = Some(getnameinfo_work);
        (*req).work_req.done = Some(getnameinfo_done);
        queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
    }
    unsafe {
        submit(
            loop_,
            req.cast(),
            std::ptr::addr_of_mut!((*req).work_req),
            TaskClass::SlowIo,
        )
    }
}

unsafe fn fill_uv_stat(out: *mut abi::uv_stat_t, st: *const abi::stat) {
    unsafe {
        (*out).st_dev = (*st).st_dev;
        (*out).st_mode = (*st).st_mode as u64;
        (*out).st_nlink = (*st).st_nlink;
        (*out).st_uid = (*st).st_uid as u64;
        (*out).st_gid = (*st).st_gid as u64;
        (*out).st_rdev = (*st).st_rdev;
        (*out).st_ino = (*st).st_ino;
        (*out).st_size = (*st).st_size as u64;
        (*out).st_blksize = (*st).st_blksize as u64;
        (*out).st_blocks = (*st).st_blocks as u64;
        (*out).st_flags = 0;
        (*out).st_gen = 0;
        (*out).st_atim = abi::uv_timespec_t {
            tv_sec: (*st).st_atim.tv_sec as libc::c_long,
            tv_nsec: (*st).st_atim.tv_nsec as libc::c_long,
        };
        (*out).st_mtim = abi::uv_timespec_t {
            tv_sec: (*st).st_mtim.tv_sec as libc::c_long,
            tv_nsec: (*st).st_mtim.tv_nsec as libc::c_long,
        };
        (*out).st_ctim = abi::uv_timespec_t {
            tv_sec: (*st).st_ctim.tv_sec as libc::c_long,
            tv_nsec: (*st).st_ctim.tv_nsec as libc::c_long,
        };
        (*out).st_birthtim = abi::uv_timespec_t::default();
    }
}

unsafe fn cleanup_fs_allocations(req: *mut abi::uv_fs_t) {
    for idx in 1..=3 {
        let slot = unsafe { (*req).reserved[idx] };
        if !slot.is_null() {
            unsafe {
                allocator::free_bytes(slot);
                (*req).reserved[idx] = null_mut();
            }
        }
    }
}

unsafe fn fs_req_init(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    fs_type: abi::uv_fs_type,
    cb: abi::uv_fs_cb,
) {
    let data = unsafe { (*req).data };
    unsafe {
        std::ptr::write_bytes(req, 0, 1);
        (*req).data = data;
        (*req).type_ = abi::uv_req_type_UV_FS;
        (*req).fs_type = fs_type;
        (*req).loop_ = loop_;
        (*req).cb = cb;
    }
}

unsafe fn clone_path_slot(req: *mut abi::uv_fs_t, path: *const c_char, slot: usize) -> c_int {
    let cloned = clone_c_string(path);
    if path.is_null() || !cloned.is_null() {
        unsafe {
            (*req).path = cloned;
            (*req).reserved[slot] = cloned.cast();
        }
        return 0;
    }
    abi::uv_errno_t_UV_ENOMEM
}

unsafe fn copy_bufs(
    req: *mut abi::uv_fs_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_int,
    stable: bool,
) -> c_int {
    if nbufs <= 0 || bufs.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    unsafe {
        (*req).nbufs = nbufs as u32;
    }

    if !stable {
        unsafe {
            (*req).bufs = bufs.cast_mut();
        }
        return 0;
    }

    if nbufs as usize <= unsafe { (*req).bufsml.len() } {
        unsafe {
            ptr::copy_nonoverlapping(bufs, (*req).bufsml.as_mut_ptr(), nbufs as usize);
            (*req).bufs = (*req).bufsml.as_mut_ptr();
        }
        return 0;
    }

    let bytes = (nbufs as usize) * std::mem::size_of::<abi::uv_buf_t>();
    let storage = unsafe { allocator::malloc_bytes(bytes) }.cast::<abi::uv_buf_t>();
    if storage.is_null() {
        return abi::uv_errno_t_UV_ENOMEM;
    }
    unsafe {
        ptr::copy_nonoverlapping(bufs, storage, nbufs as usize);
        (*req).bufs = storage;
        (*req).reserved[3] = storage.cast();
    }
    0
}

unsafe extern "C" fn fs_work(work: *mut abi::uv__work) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_fs_t, work_req))
            .cast::<abi::uv_fs_t>()
    };

    match unsafe { (*req).fs_type } {
        abi::uv_fs_type_UV_FS_READ => {
            let bufs = unsafe { (*req).bufs.cast::<libc::iovec>() };
            let nbufs = unsafe { (*req).nbufs as c_int };
            let off = unsafe { (*req).off as i64 };
            let rc = if off < 0 {
                if nbufs == 1 {
                    let buf = unsafe { *(*req).bufs };
                    unsafe { libc::read((*req).file, buf.base.cast(), buf.len) }
                } else {
                    unsafe { libc::readv((*req).file, bufs, nbufs) }
                }
            } else if nbufs == 1 {
                let buf = unsafe { *(*req).bufs };
                unsafe { libc::pread((*req).file, buf.base.cast(), buf.len, off as libc::off_t) }
            } else {
                unsafe { libc::preadv((*req).file, bufs, nbufs, off as libc::off_t) }
            };
            unsafe {
                (*req).result = if rc >= 0 {
                    rc as isize
                } else {
                    uv_err(last_errno()) as isize
                };
            }
        }
        abi::uv_fs_type_UV_FS_OPEN => {
            let rc = unsafe { libc::open((*req).path, (*req).flags, (*req).mode as libc::mode_t) };
            unsafe {
                if rc >= 0 {
                    (*req).result = rc as isize;
                    (*req).file = rc;
                } else {
                    (*req).result = uv_err(last_errno()) as isize;
                }
            }
        }
        abi::uv_fs_type_UV_FS_CLOSE => {
            let rc = unsafe { libc::close((*req).file) };
            unsafe {
                (*req).result = if rc == 0 {
                    0
                } else {
                    let err = last_errno();
                    if err == libc::EINTR || err == libc::EINPROGRESS {
                        0
                    } else {
                        uv_err(err) as isize
                    }
                };
            }
        }
        abi::uv_fs_type_UV_FS_WRITE => {
            let bufs = unsafe { (*req).bufs.cast::<libc::iovec>() };
            let nbufs = unsafe { (*req).nbufs as c_int };
            let off = unsafe { (*req).off as i64 };
            let rc = if off < 0 {
                if nbufs == 1 {
                    let buf = unsafe { *(*req).bufs };
                    unsafe { libc::write((*req).file, buf.base.cast(), buf.len) }
                } else {
                    unsafe { libc::writev((*req).file, bufs, nbufs) }
                }
            } else if nbufs == 1 {
                let buf = unsafe { *(*req).bufs };
                unsafe { libc::pwrite((*req).file, buf.base.cast(), buf.len, off as libc::off_t) }
            } else {
                unsafe { libc::pwritev((*req).file, bufs, nbufs, off as libc::off_t) }
            };
            unsafe {
                (*req).result = if rc >= 0 {
                    rc as isize
                } else {
                    uv_err(last_errno()) as isize
                };
            }
        }
        abi::uv_fs_type_UV_FS_STAT => {
            let mut st: abi::stat = unsafe { zeroed() };
            let rc = unsafe { libc::stat((*req).path, (&mut st as *mut abi::stat).cast()) };
            unsafe {
                if rc == 0 {
                    (*req).result = 0;
                    fill_uv_stat(std::ptr::addr_of_mut!((*req).statbuf), &st);
                } else {
                    (*req).result = uv_err(last_errno()) as isize;
                }
            }
        }
        abi::uv_fs_type_UV_FS_UNLINK => {
            let rc = unsafe { libc::unlink((*req).path) };
            unsafe {
                (*req).result = if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                };
            }
        }
        _ => unsafe {
            (*req).result = abi::uv_errno_t_UV_ENOSYS as isize;
        },
    }
}

unsafe extern "C" fn fs_done(work: *mut abi::uv__work, status: c_int) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_fs_t, work_req))
            .cast::<abi::uv_fs_t>()
    };
    if status == abi::uv_errno_t_UV_ECANCELED {
        unsafe {
            (*req).result = status as isize;
        }
    }
    if let Some(cb) = unsafe { (*req).cb } {
        unsafe {
            cb(req);
        }
    }
}

pub(crate) unsafe fn fs_req_cleanup(req: *mut abi::uv_fs_t) {
    if req.is_null() {
        return;
    }
    unsafe {
        cleanup_fs_allocations(req);
        (*req).path = null_mut();
        (*req).new_path = null_mut();
        (*req).bufs = null_mut();
        (*req).ptr = null_mut();
    }
}

pub(crate) unsafe fn fs_open(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    flags: c_int,
    mode: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    if req.is_null() || path.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    unsafe {
        fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_OPEN, cb);
        (*req).flags = flags;
        (*req).mode = mode as abi::mode_t;
    }

    if cb.is_none() {
        unsafe {
            (*req).path = path;
            fs_work(std::ptr::addr_of_mut!((*req).work_req));
            (*req).result as c_int
        }
    } else {
        let rc = unsafe { clone_path_slot(req, path, 1) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).work_req.work = Some(fs_work);
            (*req).work_req.done = Some(fs_done);
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            submit(
                loop_,
                req.cast(),
                std::ptr::addr_of_mut!((*req).work_req),
                TaskClass::SlowIo,
            )
        }
    }
}

pub(crate) unsafe fn fs_close(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> c_int {
    if req.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    unsafe {
        fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_CLOSE, cb);
        (*req).file = file;
    }

    if cb.is_none() {
        unsafe {
            fs_work(std::ptr::addr_of_mut!((*req).work_req));
            (*req).result as c_int
        }
    } else {
        unsafe {
            (*req).work_req.work = Some(fs_work);
            (*req).work_req.done = Some(fs_done);
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            submit(
                loop_,
                req.cast(),
                std::ptr::addr_of_mut!((*req).work_req),
                TaskClass::SlowIo,
            )
        }
    }
}

pub(crate) unsafe fn fs_read(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: c_int,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    if req.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    unsafe {
        fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_READ, cb);
        (*req).file = file;
        (*req).off = offset as abi::off_t;
    }
    let rc = unsafe { copy_bufs(req, bufs, nbufs, cb.is_some()) };
    if rc != 0 {
        return rc;
    }

    if cb.is_none() {
        unsafe {
            fs_work(std::ptr::addr_of_mut!((*req).work_req));
            (*req).result as c_int
        }
    } else {
        unsafe {
            (*req).work_req.work = Some(fs_work);
            (*req).work_req.done = Some(fs_done);
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            submit(
                loop_,
                req.cast(),
                std::ptr::addr_of_mut!((*req).work_req),
                TaskClass::SlowIo,
            )
        }
    }
}

pub(crate) unsafe fn fs_write(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: c_int,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    if req.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    unsafe {
        fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_WRITE, cb);
        (*req).file = file;
        (*req).off = offset as abi::off_t;
    }
    let rc = unsafe { copy_bufs(req, bufs, nbufs, cb.is_some()) };
    if rc != 0 {
        return rc;
    }

    if cb.is_none() {
        unsafe {
            fs_work(std::ptr::addr_of_mut!((*req).work_req));
            (*req).result as c_int
        }
    } else {
        unsafe {
            (*req).work_req.work = Some(fs_work);
            (*req).work_req.done = Some(fs_done);
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            submit(
                loop_,
                req.cast(),
                std::ptr::addr_of_mut!((*req).work_req),
                TaskClass::SlowIo,
            )
        }
    }
}

pub(crate) unsafe fn fs_stat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    if req.is_null() || path.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    unsafe {
        fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_STAT, cb);
    }

    if cb.is_none() {
        unsafe {
            (*req).path = path;
            fs_work(std::ptr::addr_of_mut!((*req).work_req));
            (*req).result as c_int
        }
    } else {
        let rc = unsafe { clone_path_slot(req, path, 1) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).work_req.work = Some(fs_work);
            (*req).work_req.done = Some(fs_done);
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            submit(
                loop_,
                req.cast(),
                std::ptr::addr_of_mut!((*req).work_req),
                TaskClass::SlowIo,
            )
        }
    }
}

pub(crate) unsafe fn fs_unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    if req.is_null() || path.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    unsafe {
        fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_UNLINK, cb);
    }

    if cb.is_none() {
        unsafe {
            (*req).path = path;
            fs_work(std::ptr::addr_of_mut!((*req).work_req));
            (*req).result as c_int
        }
    } else {
        let rc = unsafe { clone_path_slot(req, path, 1) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).work_req.work = Some(fs_work);
            (*req).work_req.done = Some(fs_done);
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            submit(
                loop_,
                req.cast(),
                std::ptr::addr_of_mut!((*req).work_req),
                TaskClass::SlowIo,
            )
        }
    }
}
