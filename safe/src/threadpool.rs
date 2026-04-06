use crate::allocator::{self, UV_EBUSY, UV_EINVAL};
use crate::bindings::*;
use crate::handle::{
    queue_empty, queue_head, queue_init, queue_insert_tail, queue_move, queue_next, queue_remove,
};
use crate::request::{req_register, req_unregister};
use std::mem::{offset_of, MaybeUninit};

const DEFAULT_THREADPOOL_SIZE: usize = 2;
const MAX_THREADPOOL_SIZE: usize = 1024;
const THREADPOOL_STACK_SIZE: usize = 8 << 20;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum WorkKind {
    Cpu,
    FastIo,
    SlowIo,
}

static mut ONCE: uv_once_t = libc::PTHREAD_ONCE_INIT;
static mut COND: MaybeUninit<uv_cond_t> = MaybeUninit::uninit();
static mut MUTEX: MaybeUninit<uv_mutex_t> = MaybeUninit::uninit();
static mut IDLE_THREADS: u32 = 0;
static mut SLOW_IO_WORK_RUNNING: u32 = 0;
static mut NTHREADS: u32 = 0;
static mut THREADS: *mut uv_thread_t = std::ptr::null_mut();
static mut DEFAULT_THREADS: [uv_thread_t; DEFAULT_THREADPOOL_SIZE] =
    [0 as uv_thread_t; DEFAULT_THREADPOOL_SIZE];
static mut WQ: uv__queue = uv__queue {
    next: std::ptr::null_mut(),
    prev: std::ptr::null_mut(),
};
static mut RUN_SLOW_WORK_MESSAGE: uv__queue = uv__queue {
    next: std::ptr::null_mut(),
    prev: std::ptr::null_mut(),
};
static mut SLOW_IO_PENDING_WQ: uv__queue = uv__queue {
    next: std::ptr::null_mut(),
    prev: std::ptr::null_mut(),
};

#[inline]
unsafe fn abort() -> ! {
    libc::abort()
}

#[inline]
unsafe fn pool_cond() -> *mut uv_cond_t {
    COND.as_mut_ptr()
}

#[inline]
unsafe fn pool_mutex() -> *mut uv_mutex_t {
    MUTEX.as_mut_ptr()
}

#[inline]
unsafe fn work_queue() -> *mut uv__queue {
    std::ptr::addr_of_mut!(WQ)
}

#[inline]
unsafe fn run_slow_work_message() -> *mut uv__queue {
    std::ptr::addr_of_mut!(RUN_SLOW_WORK_MESSAGE)
}

#[inline]
unsafe fn slow_io_pending_queue() -> *mut uv__queue {
    std::ptr::addr_of_mut!(SLOW_IO_PENDING_WQ)
}

#[inline]
unsafe fn work_from_queue(q: *mut uv__queue) -> *mut uv__work {
    q.cast::<u8>()
        .sub(offset_of!(uv__work, wq))
        .cast::<uv__work>()
}

#[inline]
unsafe fn work_req_from_work(w: *mut uv__work) -> *mut uv_work_t {
    w.cast::<u8>()
        .sub(offset_of!(uv_work_t, work_req))
        .cast::<uv_work_t>()
}

#[inline]
unsafe fn slow_work_thread_threshold() -> u32 {
    (NTHREADS + 1) / 2
}

unsafe extern "C" fn uv__cancelled(_w: *mut uv__work) {
    abort()
}

unsafe extern "C" fn worker(arg: *mut libc::c_void) {
    let sem = arg.cast::<uv_sem_t>();
    uv_sem_post(sem);

    uv_mutex_lock(pool_mutex());
    loop {
        while queue_empty(work_queue())
            || (std::ptr::eq(queue_head(work_queue()), run_slow_work_message())
                && std::ptr::eq(queue_next(run_slow_work_message()), work_queue())
                && SLOW_IO_WORK_RUNNING >= slow_work_thread_threshold())
        {
            IDLE_THREADS += 1;
            uv_cond_wait(pool_cond(), pool_mutex());
            IDLE_THREADS -= 1;
        }

        let mut q = queue_head(work_queue());
        queue_remove(q);
        queue_init(q);

        let mut is_slow_work = false;
        if std::ptr::eq(q, run_slow_work_message()) {
            if SLOW_IO_WORK_RUNNING >= slow_work_thread_threshold() {
                queue_insert_tail(work_queue(), q);
                continue;
            }

            if queue_empty(slow_io_pending_queue()) {
                continue;
            }

            is_slow_work = true;
            SLOW_IO_WORK_RUNNING += 1;

            q = queue_head(slow_io_pending_queue());
            queue_remove(q);
            queue_init(q);

            if !queue_empty(slow_io_pending_queue()) {
                queue_insert_tail(work_queue(), run_slow_work_message());
                if IDLE_THREADS > 0 {
                    uv_cond_signal(pool_cond());
                }
            }
        }

        uv_mutex_unlock(pool_mutex());

        let w = work_from_queue(q);
        if let Some(work) = (*w).work {
            work(w);
        }

        uv_mutex_lock(std::ptr::addr_of_mut!((*(*w).loop_).wq_mutex));
        (*w).work = None;
        queue_insert_tail(std::ptr::addr_of_mut!((*(*w).loop_).wq), q);
        uv_async_send(std::ptr::addr_of_mut!((*(*w).loop_).wq_async));
        uv_mutex_unlock(std::ptr::addr_of_mut!((*(*w).loop_).wq_mutex));

        uv_mutex_lock(pool_mutex());
        if is_slow_work {
            SLOW_IO_WORK_RUNNING -= 1;
        }
    }
}

unsafe fn post(q: *mut uv__queue, kind: WorkKind) {
    uv_mutex_lock(pool_mutex());

    let mut q = q;
    if kind == WorkKind::SlowIo {
        queue_insert_tail(slow_io_pending_queue(), q);
        if !queue_empty(run_slow_work_message()) {
            uv_mutex_unlock(pool_mutex());
            return;
        }
        q = run_slow_work_message();
    }

    queue_insert_tail(work_queue(), q);
    if IDLE_THREADS > 0 {
        uv_cond_signal(pool_cond());
    }
    uv_mutex_unlock(pool_mutex());
}

unsafe extern "C" fn reset_once() {
    ONCE = libc::PTHREAD_ONCE_INIT;
}

unsafe fn init_threads() {
    let config = uv_thread_options_t {
        flags: uv_thread_create_flags_UV_THREAD_HAS_STACK_SIZE,
        stack_size: THREADPOOL_STACK_SIZE,
    };
    let mut sem: uv_sem_t = std::mem::zeroed();
    let env = libc::getenv(b"UV_THREADPOOL_SIZE\0".as_ptr().cast());

    NTHREADS = DEFAULT_THREADPOOL_SIZE as u32;
    if !env.is_null() {
        NTHREADS = libc::atoi(env).max(0) as u32;
    }
    if NTHREADS == 0 {
        NTHREADS = 1;
    }
    if NTHREADS as usize > MAX_THREADPOOL_SIZE {
        NTHREADS = MAX_THREADPOOL_SIZE as u32;
    }

    THREADS = DEFAULT_THREADS.as_mut_ptr();
    if NTHREADS as usize > DEFAULT_THREADPOOL_SIZE {
        THREADS = allocator::malloc(NTHREADS as usize * std::mem::size_of::<uv_thread_t>())
            .cast::<uv_thread_t>();
        if THREADS.is_null() {
            NTHREADS = DEFAULT_THREADPOOL_SIZE as u32;
            THREADS = DEFAULT_THREADS.as_mut_ptr();
        }
    }

    if uv_cond_init(pool_cond()) != 0 {
        abort();
    }
    if uv_mutex_init(pool_mutex()) != 0 {
        abort();
    }

    IDLE_THREADS = 0;
    SLOW_IO_WORK_RUNNING = 0;
    queue_init(work_queue());
    queue_init(slow_io_pending_queue());
    queue_init(run_slow_work_message());

    if uv_sem_init(std::ptr::addr_of_mut!(sem), 0) != 0 {
        abort();
    }

    for i in 0..NTHREADS as usize {
        if uv_thread_create_ex(
            THREADS.add(i),
            std::ptr::addr_of!(config),
            Some(worker),
            std::ptr::addr_of_mut!(sem).cast(),
        ) != 0
        {
            abort();
        }
    }

    for _ in 0..NTHREADS {
        uv_sem_wait(std::ptr::addr_of_mut!(sem));
    }
    uv_sem_destroy(std::ptr::addr_of_mut!(sem));
}

unsafe extern "C" fn init_once() {
    if libc::pthread_atfork(None, None, Some(reset_once)) != 0 {
        abort();
    }
    init_threads();
}

pub(crate) unsafe fn uv__work_submit(
    loop_: *mut uv_loop_t,
    w: *mut uv__work,
    kind: WorkKind,
    work: Option<unsafe extern "C" fn(*mut uv__work)>,
    done: Option<unsafe extern "C" fn(*mut uv__work, libc::c_int)>,
) {
    uv_once(std::ptr::addr_of_mut!(ONCE), Some(init_once));
    (*w).loop_ = loop_;
    (*w).work = work;
    (*w).done = done;
    post(std::ptr::addr_of_mut!((*w).wq), kind);
}

pub(crate) unsafe fn uv__work_cancel(loop_: *mut uv_loop_t, w: *mut uv__work) -> libc::c_int {
    uv_once(std::ptr::addr_of_mut!(ONCE), Some(init_once));
    uv_mutex_lock(pool_mutex());
    uv_mutex_lock(std::ptr::addr_of_mut!((*(*w).loop_).wq_mutex));

    let cancelled = !queue_empty(std::ptr::addr_of!((*w).wq)) && (*w).work.is_some();
    if cancelled {
        queue_remove(std::ptr::addr_of_mut!((*w).wq));
    }

    uv_mutex_unlock(std::ptr::addr_of_mut!((*(*w).loop_).wq_mutex));
    uv_mutex_unlock(pool_mutex());

    if !cancelled {
        return UV_EBUSY;
    }

    (*w).work = Some(uv__cancelled);
    uv_mutex_lock(std::ptr::addr_of_mut!((*loop_).wq_mutex));
    queue_insert_tail(
        std::ptr::addr_of_mut!((*loop_).wq),
        std::ptr::addr_of_mut!((*w).wq),
    );
    uv_async_send(std::ptr::addr_of_mut!((*loop_).wq_async));
    uv_mutex_unlock(std::ptr::addr_of_mut!((*loop_).wq_mutex));

    0
}

pub(crate) unsafe extern "C" fn uv__work_done_impl(handle: *mut uv_async_t) {
    let loop_ = (*handle).loop_;
    let mut wq = std::mem::zeroed::<uv__queue>();
    let mut nevents = 0;

    uv_mutex_lock(std::ptr::addr_of_mut!((*loop_).wq_mutex));
    queue_move(
        std::ptr::addr_of_mut!((*loop_).wq),
        std::ptr::addr_of_mut!(wq),
    );
    uv_mutex_unlock(std::ptr::addr_of_mut!((*loop_).wq_mutex));

    while !queue_empty(std::ptr::addr_of!(wq)) {
        let q = queue_head(std::ptr::addr_of!(wq));
        queue_remove(q);

        let w = work_from_queue(q);
        let err = if (*w).work == Some(uv__cancelled) {
            uv_errno_t_UV_ECANCELED
        } else {
            0
        };
        if let Some(done) = (*w).done {
            done(w, err);
        }
        nevents += 1;
    }

    if nevents > 1 {
        crate::r#loop::metrics_inc_events(loop_, (nevents - 1) as u64);
        if (*crate::state::loop_fields(loop_)).current_timeout == 0 {
            crate::r#loop::metrics_inc_events_waiting(loop_, (nevents - 1) as u64);
        }
    }
}

unsafe extern "C" fn uv__queue_work(w: *mut uv__work) {
    let req = work_req_from_work(w);
    if let Some(work_cb) = (*req).work_cb {
        work_cb(req);
    }
}

unsafe extern "C" fn uv__queue_done(w: *mut uv__work, err: libc::c_int) {
    let req = work_req_from_work(w);
    req_unregister((*req).loop_, req.cast());
    if let Some(after_work_cb) = (*req).after_work_cb {
        after_work_cb(req, err);
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_queue_work(
    loop_: *mut uv_loop_t,
    req: *mut uv_work_t,
    work_cb: uv_work_cb,
    after_work_cb: uv_after_work_cb,
) -> libc::c_int {
    if req.is_null() || loop_.is_null() || work_cb.is_none() {
        return UV_EINVAL;
    }

    (*req).type_ = uv_req_type_UV_WORK;
    (*req).loop_ = loop_;
    (*req).work_cb = work_cb;
    (*req).after_work_cb = after_work_cb;
    req_register(loop_, req.cast());
    uv__work_submit(
        loop_,
        std::ptr::addr_of_mut!((*req).work_req),
        WorkKind::Cpu,
        Some(uv__queue_work),
        Some(uv__queue_done),
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_cancel(req: *mut uv_req_t) -> libc::c_int {
    if req.is_null() {
        return UV_EINVAL;
    }

    match (*req).type_ {
        uv_req_type_UV_WORK => {
            let work = req.cast::<uv_work_t>();
            if (*work).loop_.is_null() {
                return UV_EINVAL;
            }
            uv__work_cancel((*work).loop_, std::ptr::addr_of_mut!((*work).work_req))
        }
        uv_req_type_UV_FS => {
            let fs = req.cast::<uv_fs_t>();
            if !crate::fs::is_rust_fs_request(fs) || (*fs).loop_.is_null() {
                return crate::private_support::uv_cancel(req);
            }
            uv__work_cancel((*fs).loop_, std::ptr::addr_of_mut!((*fs).work_req))
        }
        uv_req_type_UV_GETADDRINFO => {
            let gai = req.cast::<uv_getaddrinfo_t>();
            if (*gai).loop_.is_null() {
                return UV_EINVAL;
            }
            uv__work_cancel((*gai).loop_, std::ptr::addr_of_mut!((*gai).work_req))
        }
        uv_req_type_UV_RANDOM => {
            let random = req.cast::<uv_random_t>();
            if (*random).loop_.is_null() {
                return UV_EINVAL;
            }
            uv__work_cancel((*random).loop_, std::ptr::addr_of_mut!((*random).work_req))
        }
        uv_req_type_UV_GETNAMEINFO => {
            let gni = req.cast::<uv_getnameinfo_t>();
            if (*gni).loop_.is_null() {
                return UV_EINVAL;
            }
            uv__work_cancel((*gni).loop_, std::ptr::addr_of_mut!((*gni).work_req))
        }
        _ => UV_EINVAL,
    }
}
