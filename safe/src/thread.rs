use crate::allocator::{self, uv_err, UV_EAGAIN, UV_EBUSY, UV_EINVAL, UV_ETIMEDOUT};
use crate::bindings::*;
use std::mem;

#[inline]
unsafe fn abort() -> ! {
    libc::abort()
}

#[inline]
unsafe fn as_mutex(mutex: *mut uv_mutex_t) -> *mut libc::pthread_mutex_t {
    mutex.cast()
}

#[inline]
unsafe fn as_rwlock(rwlock: *mut uv_rwlock_t) -> *mut libc::pthread_rwlock_t {
    rwlock.cast()
}

#[inline]
unsafe fn as_sem(sem: *mut uv_sem_t) -> *mut libc::sem_t {
    sem.cast()
}

#[inline]
unsafe fn as_cond(cond: *mut uv_cond_t) -> *mut libc::pthread_cond_t {
    cond.cast()
}

#[inline]
unsafe fn as_barrier(barrier: *mut uv_barrier_t) -> *mut libc::pthread_barrier_t {
    barrier.cast()
}

#[inline]
unsafe fn as_key(key: *mut uv_key_t) -> *mut libc::pthread_key_t {
    key.cast()
}

fn min_stack_size() -> usize {
    let min = 8192usize;
    min.max(libc::PTHREAD_STACK_MIN)
}

fn default_stack_size() -> usize {
    #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
    {
        4 << 20
    }
    #[cfg(not(any(target_arch = "powerpc", target_arch = "powerpc64")))]
    {
        2 << 20
    }
}

unsafe fn thread_stack_size() -> usize {
    let mut lim = mem::zeroed::<libc::rlimit>();
    if libc::getrlimit(libc::RLIMIT_STACK, &mut lim) != 0 {
        return default_stack_size();
    }
    if lim.rlim_cur == libc::RLIM_INFINITY {
        return default_stack_size();
    }

    let pagesize = libc::sysconf(libc::_SC_PAGESIZE);
    if pagesize <= 0 {
        return default_stack_size();
    }

    let pagesize = pagesize as libc::rlim_t;
    lim.rlim_cur -= lim.rlim_cur % pagesize;
    let aligned = lim.rlim_cur as usize;
    if aligned >= min_stack_size() {
        aligned
    } else {
        default_stack_size()
    }
}

unsafe fn monotonic_now_ns() -> u64 {
    let mut ts = mem::zeroed::<libc::timespec>();
    if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) != 0 {
        0
    } else {
        ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_create(
    tid: *mut uv_thread_t,
    entry: uv_thread_cb,
    arg: *mut libc::c_void,
) -> libc::c_int {
    let params = uv_thread_options_t {
        flags: uv_thread_create_flags_UV_THREAD_NO_FLAGS,
        stack_size: 0,
    };
    uv_thread_create_ex(tid, &params, entry, arg)
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_create_ex(
    tid: *mut uv_thread_t,
    params: *const uv_thread_options_t,
    entry: uv_thread_cb,
    arg: *mut libc::c_void,
) -> libc::c_int {
    let Some(entry) = entry else {
        return UV_EINVAL;
    };

    let params = &*params;
    let mut attr_storage = mem::zeroed::<libc::pthread_attr_t>();
    let mut attr_ptr = std::ptr::null::<libc::pthread_attr_t>();
    let mut stack_size = if params.flags & uv_thread_create_flags_UV_THREAD_HAS_STACK_SIZE != 0 {
        params.stack_size
    } else {
        0
    };

    if stack_size == 0 {
        stack_size = thread_stack_size();
    } else {
        let pagesize = libc::sysconf(libc::_SC_PAGESIZE) as usize;
        stack_size = (stack_size + pagesize - 1) & !(pagesize - 1);
        stack_size = stack_size.max(min_stack_size());
    }

    if stack_size > 0 {
        attr_ptr = &attr_storage;
        if libc::pthread_attr_init(&mut attr_storage) != 0 {
            abort();
        }
        if libc::pthread_attr_setstacksize(&mut attr_storage, stack_size) != 0 {
            abort();
        }
    }

    let start: extern "C" fn(*mut libc::c_void) -> *mut libc::c_void = mem::transmute(entry);
    let err = libc::pthread_create(tid.cast(), attr_ptr, start, arg);

    if !attr_ptr.is_null() {
        libc::pthread_attr_destroy(&mut attr_storage);
    }

    uv_err(err)
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_setaffinity(
    tid: *mut uv_thread_t,
    cpumask: *mut libc::c_char,
    oldmask: *mut libc::c_char,
    mask_size: usize,
) -> libc::c_int {
    let cpumasksize = crate::legacy::uv_cpumask_size();
    if cpumasksize < 0 {
        return cpumasksize;
    }
    if mask_size < cpumasksize as usize {
        return UV_EINVAL;
    }

    if !oldmask.is_null() {
        let r = uv_thread_getaffinity(tid, oldmask, mask_size);
        if r < 0 {
            return r;
        }
    }

    let mut cpuset = mem::zeroed::<libc::cpu_set_t>();
    libc::CPU_ZERO(&mut cpuset);
    for i in 0..cpumasksize as usize {
        if *cpumask.add(i) != 0 {
            libc::CPU_SET(i, &mut cpuset);
        }
    }

    uv_err(libc::pthread_setaffinity_np(
        *tid,
        mem::size_of::<libc::cpu_set_t>(),
        &cpuset,
    ))
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_getaffinity(
    tid: *mut uv_thread_t,
    cpumask: *mut libc::c_char,
    mask_size: usize,
) -> libc::c_int {
    let cpumasksize = crate::legacy::uv_cpumask_size();
    if cpumasksize < 0 {
        return cpumasksize;
    }
    if mask_size < cpumasksize as usize {
        return UV_EINVAL;
    }

    let mut cpuset = mem::zeroed::<libc::cpu_set_t>();
    libc::CPU_ZERO(&mut cpuset);
    let r = libc::pthread_getaffinity_np(*tid, mem::size_of::<libc::cpu_set_t>(), &mut cpuset);
    if r != 0 {
        return uv_err(r);
    }

    for i in 0..cpumasksize as usize {
        *cpumask.add(i) = libc::CPU_ISSET(i, &cpuset) as libc::c_char;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_getcpu() -> libc::c_int {
    let cpu = libc::sched_getcpu();
    if cpu < 0 {
        allocator::last_error()
    } else {
        cpu
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_self() -> uv_thread_t {
    libc::pthread_self()
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_join(tid: *mut uv_thread_t) -> libc::c_int {
    uv_err(libc::pthread_join(*tid, std::ptr::null_mut()))
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_equal(
    t1: *const uv_thread_t,
    t2: *const uv_thread_t,
) -> libc::c_int {
    libc::pthread_equal(*t1, *t2)
}

#[no_mangle]
pub unsafe extern "C" fn uv_mutex_init(mutex: *mut uv_mutex_t) -> libc::c_int {
    uv_err(libc::pthread_mutex_init(as_mutex(mutex), std::ptr::null()))
}

#[no_mangle]
pub unsafe extern "C" fn uv_mutex_init_recursive(mutex: *mut uv_mutex_t) -> libc::c_int {
    let mut attr = mem::zeroed::<libc::pthread_mutexattr_t>();
    if libc::pthread_mutexattr_init(&mut attr) != 0 {
        abort();
    }
    if libc::pthread_mutexattr_settype(&mut attr, libc::PTHREAD_MUTEX_RECURSIVE) != 0 {
        abort();
    }
    let err = libc::pthread_mutex_init(as_mutex(mutex), &attr);
    if libc::pthread_mutexattr_destroy(&mut attr) != 0 {
        abort();
    }
    uv_err(err)
}

#[no_mangle]
pub unsafe extern "C" fn uv_mutex_destroy(mutex: *mut uv_mutex_t) {
    if libc::pthread_mutex_destroy(as_mutex(mutex)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_mutex_lock(mutex: *mut uv_mutex_t) {
    if libc::pthread_mutex_lock(as_mutex(mutex)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_mutex_trylock(mutex: *mut uv_mutex_t) -> libc::c_int {
    let err = libc::pthread_mutex_trylock(as_mutex(mutex));
    if err == 0 {
        0
    } else if err == libc::EBUSY || err == libc::EAGAIN {
        UV_EBUSY
    } else {
        abort()
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_mutex_unlock(mutex: *mut uv_mutex_t) {
    if libc::pthread_mutex_unlock(as_mutex(mutex)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_init(rwlock: *mut uv_rwlock_t) -> libc::c_int {
    uv_err(libc::pthread_rwlock_init(
        as_rwlock(rwlock),
        std::ptr::null(),
    ))
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_destroy(rwlock: *mut uv_rwlock_t) {
    if libc::pthread_rwlock_destroy(as_rwlock(rwlock)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_rdlock(rwlock: *mut uv_rwlock_t) {
    if libc::pthread_rwlock_rdlock(as_rwlock(rwlock)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_tryrdlock(rwlock: *mut uv_rwlock_t) -> libc::c_int {
    let err = libc::pthread_rwlock_tryrdlock(as_rwlock(rwlock));
    if err == 0 {
        0
    } else if err == libc::EBUSY || err == libc::EAGAIN {
        UV_EBUSY
    } else {
        abort()
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_rdunlock(rwlock: *mut uv_rwlock_t) {
    if libc::pthread_rwlock_unlock(as_rwlock(rwlock)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_wrlock(rwlock: *mut uv_rwlock_t) {
    if libc::pthread_rwlock_wrlock(as_rwlock(rwlock)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_trywrlock(rwlock: *mut uv_rwlock_t) -> libc::c_int {
    let err = libc::pthread_rwlock_trywrlock(as_rwlock(rwlock));
    if err == 0 {
        0
    } else if err == libc::EBUSY || err == libc::EAGAIN {
        UV_EBUSY
    } else {
        abort()
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_rwlock_wrunlock(rwlock: *mut uv_rwlock_t) {
    if libc::pthread_rwlock_unlock(as_rwlock(rwlock)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_once(guard: *mut uv_once_t, callback: Option<unsafe extern "C" fn()>) {
    let Some(callback) = callback else {
        abort();
    };
    let callback: extern "C" fn() = mem::transmute(callback);
    if libc::pthread_once(guard.cast(), callback) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_sem_init(sem: *mut uv_sem_t, value: libc::c_uint) -> libc::c_int {
    if libc::sem_init(as_sem(sem), 0, value) != 0 {
        allocator::last_error()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_sem_destroy(sem: *mut uv_sem_t) {
    if libc::sem_destroy(as_sem(sem)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_sem_post(sem: *mut uv_sem_t) {
    if libc::sem_post(as_sem(sem)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_sem_wait(sem: *mut uv_sem_t) {
    loop {
        if libc::sem_wait(as_sem(sem)) == 0 {
            return;
        }
        if *allocator::errno_location() != libc::EINTR {
            abort();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_sem_trywait(sem: *mut uv_sem_t) -> libc::c_int {
    loop {
        if libc::sem_trywait(as_sem(sem)) == 0 {
            return 0;
        }
        let err = *allocator::errno_location();
        if err == libc::EINTR {
            continue;
        }
        if err == libc::EAGAIN {
            return UV_EAGAIN;
        }
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_barrier_init(
    barrier: *mut uv_barrier_t,
    count: libc::c_uint,
) -> libc::c_int {
    uv_err(libc::pthread_barrier_init(
        as_barrier(barrier),
        std::ptr::null(),
        count,
    ))
}

#[no_mangle]
pub unsafe extern "C" fn uv_barrier_wait(barrier: *mut uv_barrier_t) -> libc::c_int {
    let rc = libc::pthread_barrier_wait(as_barrier(barrier));
    if rc != 0 && rc != libc::PTHREAD_BARRIER_SERIAL_THREAD {
        abort();
    }
    (rc == libc::PTHREAD_BARRIER_SERIAL_THREAD) as libc::c_int
}

#[no_mangle]
pub unsafe extern "C" fn uv_barrier_destroy(barrier: *mut uv_barrier_t) {
    if libc::pthread_barrier_destroy(as_barrier(barrier)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_cond_init(cond: *mut uv_cond_t) -> libc::c_int {
    let mut attr = mem::zeroed::<libc::pthread_condattr_t>();
    let mut err = libc::pthread_condattr_init(&mut attr);
    if err != 0 {
        return uv_err(err);
    }

    err = libc::pthread_condattr_setclock(&mut attr, libc::CLOCK_MONOTONIC);
    if err != 0 {
        libc::pthread_condattr_destroy(&mut attr);
        return uv_err(err);
    }

    err = libc::pthread_cond_init(as_cond(cond), &attr);
    if err != 0 {
        libc::pthread_condattr_destroy(&mut attr);
        return uv_err(err);
    }

    err = libc::pthread_condattr_destroy(&mut attr);
    if err != 0 {
        libc::pthread_cond_destroy(as_cond(cond));
        return uv_err(err);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_cond_destroy(cond: *mut uv_cond_t) {
    if libc::pthread_cond_destroy(as_cond(cond)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_cond_signal(cond: *mut uv_cond_t) {
    if libc::pthread_cond_signal(as_cond(cond)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_cond_broadcast(cond: *mut uv_cond_t) {
    if libc::pthread_cond_broadcast(as_cond(cond)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_cond_wait(cond: *mut uv_cond_t, mutex: *mut uv_mutex_t) {
    if libc::pthread_cond_wait(as_cond(cond), as_mutex(mutex)) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_cond_timedwait(
    cond: *mut uv_cond_t,
    mutex: *mut uv_mutex_t,
    timeout: u64,
) -> libc::c_int {
    let deadline = monotonic_now_ns().saturating_add(timeout);
    let ts = libc::timespec {
        tv_sec: (deadline / 1_000_000_000) as libc::time_t,
        tv_nsec: (deadline % 1_000_000_000) as libc::c_long,
    };

    match libc::pthread_cond_timedwait(as_cond(cond), as_mutex(mutex), &ts) {
        0 => 0,
        libc::ETIMEDOUT => UV_ETIMEDOUT,
        _ => abort(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_key_create(key: *mut uv_key_t) -> libc::c_int {
    uv_err(libc::pthread_key_create(as_key(key), None))
}

#[no_mangle]
pub unsafe extern "C" fn uv_key_delete(key: *mut uv_key_t) {
    if libc::pthread_key_delete(*key) != 0 {
        abort();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_key_get(key: *mut uv_key_t) -> *mut libc::c_void {
    libc::pthread_getspecific(*key)
}

#[no_mangle]
pub unsafe extern "C" fn uv_key_set(key: *mut uv_key_t, value: *mut libc::c_void) {
    if libc::pthread_setspecific(*key, value) != 0 {
        abort();
    }
}
