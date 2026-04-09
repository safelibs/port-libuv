use crate::abi::linux_x86_64 as abi;
use crate::core::time;
use libc::{self, c_int, c_uint};

#[inline]
fn uv_err(err: c_int) -> c_int {
    if err == 0 {
        0
    } else {
        -err
    }
}

#[cold]
unsafe fn abort_now() -> ! {
    unsafe {
        libc::abort();
    }
}

#[inline]
unsafe fn assert_zero(err: c_int) {
    if err != 0 {
        unsafe { abort_now() };
    }
}

pub(crate) unsafe fn mutex_init_raw(mutex: *mut abi::uv_mutex_t, recursive: bool) -> c_int {
    if mutex.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    if !recursive {
        return uv_err(unsafe { libc::pthread_mutex_init(mutex.cast(), std::ptr::null()) });
    }

    let mut attr: libc::pthread_mutexattr_t = unsafe { std::mem::zeroed() };
    unsafe {
        assert_zero(libc::pthread_mutexattr_init(&mut attr));
        assert_zero(libc::pthread_mutexattr_settype(
            &mut attr,
            libc::PTHREAD_MUTEX_RECURSIVE,
        ));
    }
    let rc = unsafe { libc::pthread_mutex_init(mutex.cast(), &attr) };
    unsafe {
        assert_zero(libc::pthread_mutexattr_destroy(&mut attr));
    }
    uv_err(rc)
}

pub(crate) unsafe fn mutex_destroy_raw(mutex: *mut abi::uv_mutex_t) {
    if mutex.is_null() {
        return;
    }
    unsafe {
        assert_zero(libc::pthread_mutex_destroy(mutex.cast()));
    }
}

pub(crate) unsafe fn mutex_lock_raw(mutex: *mut abi::uv_mutex_t) {
    unsafe {
        assert_zero(libc::pthread_mutex_lock(mutex.cast()));
    }
}

pub(crate) unsafe fn mutex_trylock_raw(mutex: *mut abi::uv_mutex_t) -> c_int {
    let err = unsafe { libc::pthread_mutex_trylock(mutex.cast()) };
    if err == 0 {
        return 0;
    }
    if err == libc::EBUSY || err == libc::EAGAIN {
        return abi::uv_errno_t_UV_EBUSY;
    }
    unsafe { abort_now() }
}

pub(crate) unsafe fn mutex_unlock_raw(mutex: *mut abi::uv_mutex_t) {
    unsafe {
        assert_zero(libc::pthread_mutex_unlock(mutex.cast()));
    }
}

pub(crate) unsafe fn rwlock_init_raw(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    if rwlock.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    uv_err(unsafe { libc::pthread_rwlock_init(rwlock.cast(), std::ptr::null()) })
}

pub(crate) unsafe fn rwlock_destroy_raw(rwlock: *mut abi::uv_rwlock_t) {
    if rwlock.is_null() {
        return;
    }
    unsafe {
        assert_zero(libc::pthread_rwlock_destroy(rwlock.cast()));
    }
}

pub(crate) unsafe fn rwlock_rdlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        assert_zero(libc::pthread_rwlock_rdlock(rwlock.cast()));
    }
}

pub(crate) unsafe fn rwlock_tryrdlock_raw(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    let err = unsafe { libc::pthread_rwlock_tryrdlock(rwlock.cast()) };
    if err == 0 {
        return 0;
    }
    if err == libc::EBUSY || err == libc::EAGAIN {
        return abi::uv_errno_t_UV_EBUSY;
    }
    unsafe { abort_now() }
}

pub(crate) unsafe fn rwlock_rdunlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        assert_zero(libc::pthread_rwlock_unlock(rwlock.cast()));
    }
}

pub(crate) unsafe fn rwlock_wrlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        assert_zero(libc::pthread_rwlock_wrlock(rwlock.cast()));
    }
}

pub(crate) unsafe fn rwlock_trywrlock_raw(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    let err = unsafe { libc::pthread_rwlock_trywrlock(rwlock.cast()) };
    if err == 0 {
        return 0;
    }
    if err == libc::EBUSY || err == libc::EAGAIN {
        return abi::uv_errno_t_UV_EBUSY;
    }
    unsafe { abort_now() }
}

pub(crate) unsafe fn rwlock_wrunlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        assert_zero(libc::pthread_rwlock_unlock(rwlock.cast()));
    }
}

pub(crate) unsafe fn sem_init_raw(sem: *mut abi::uv_sem_t, value: c_uint) -> c_int {
    if sem.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { libc::sem_init(sem.cast(), 0, value) } != 0 {
        return -unsafe { *libc::__errno_location() };
    }
    0
}

pub(crate) unsafe fn sem_destroy_raw(sem: *mut abi::uv_sem_t) {
    if sem.is_null() {
        return;
    }
    unsafe {
        assert_zero(libc::sem_destroy(sem.cast()));
    }
}

pub(crate) unsafe fn sem_post_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        assert_zero(libc::sem_post(sem.cast()));
    }
}

pub(crate) unsafe fn sem_wait_raw(sem: *mut abi::uv_sem_t) {
    loop {
        if unsafe { libc::sem_wait(sem.cast()) } == 0 {
            return;
        }
        let err = unsafe { *libc::__errno_location() };
        if err != libc::EINTR {
            unsafe { abort_now() };
        }
    }
}

pub(crate) unsafe fn sem_trywait_raw(sem: *mut abi::uv_sem_t) -> c_int {
    loop {
        if unsafe { libc::sem_trywait(sem.cast()) } == 0 {
            return 0;
        }
        let err = unsafe { *libc::__errno_location() };
        if err == libc::EINTR {
            continue;
        }
        if err == libc::EAGAIN {
            return abi::uv_errno_t_UV_EAGAIN;
        }
        unsafe { abort_now() }
    }
}

pub(crate) unsafe fn cond_init_raw(cond: *mut abi::uv_cond_t) -> c_int {
    if cond.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let mut attr: libc::pthread_condattr_t = unsafe { std::mem::zeroed() };
    let mut err = unsafe { libc::pthread_condattr_init(&mut attr) };
    if err != 0 {
        return uv_err(err);
    }

    err = unsafe { libc::pthread_condattr_setclock(&mut attr, libc::CLOCK_MONOTONIC) };
    if err != 0 {
        unsafe {
            assert_zero(libc::pthread_condattr_destroy(&mut attr));
        }
        return uv_err(err);
    }

    err = unsafe { libc::pthread_cond_init(cond.cast(), &attr) };
    unsafe {
        assert_zero(libc::pthread_condattr_destroy(&mut attr));
    }
    uv_err(err)
}

pub(crate) unsafe fn cond_destroy_raw(cond: *mut abi::uv_cond_t) {
    if cond.is_null() {
        return;
    }
    unsafe {
        assert_zero(libc::pthread_cond_destroy(cond.cast()));
    }
}

pub(crate) unsafe fn cond_signal_raw(cond: *mut abi::uv_cond_t) {
    unsafe {
        assert_zero(libc::pthread_cond_signal(cond.cast()));
    }
}

pub(crate) unsafe fn cond_broadcast_raw(cond: *mut abi::uv_cond_t) {
    unsafe {
        assert_zero(libc::pthread_cond_broadcast(cond.cast()));
    }
}

pub(crate) unsafe fn cond_wait_raw(cond: *mut abi::uv_cond_t, mutex: *mut abi::uv_mutex_t) {
    unsafe {
        assert_zero(libc::pthread_cond_wait(cond.cast(), mutex.cast()));
    }
}

pub(crate) unsafe fn cond_timedwait_raw(
    cond: *mut abi::uv_cond_t,
    mutex: *mut abi::uv_mutex_t,
    timeout: u64,
) -> c_int {
    let deadline = time::hrtime_precise_ns().saturating_add(timeout);
    let ts = libc::timespec {
        tv_sec: (deadline / 1_000_000_000) as libc::time_t,
        tv_nsec: (deadline % 1_000_000_000) as libc::c_long,
    };
    let err = unsafe { libc::pthread_cond_timedwait(cond.cast(), mutex.cast(), &ts) };
    if err == 0 {
        return 0;
    }
    if err == libc::ETIMEDOUT {
        return abi::uv_errno_t_UV_ETIMEDOUT;
    }
    unsafe { abort_now() }
}

pub(crate) unsafe fn barrier_init_raw(barrier: *mut abi::uv_barrier_t, count: c_uint) -> c_int {
    if barrier.is_null() || count == 0 {
        return abi::uv_errno_t_UV_EINVAL;
    }
    uv_err(unsafe { libc::pthread_barrier_init(barrier.cast(), std::ptr::null(), count) })
}

pub(crate) unsafe fn barrier_destroy_raw(barrier: *mut abi::uv_barrier_t) {
    if barrier.is_null() {
        return;
    }
    unsafe {
        assert_zero(libc::pthread_barrier_destroy(barrier.cast()));
    }
}

pub(crate) unsafe fn barrier_wait_raw(barrier: *mut abi::uv_barrier_t) -> c_int {
    let rc = unsafe { libc::pthread_barrier_wait(barrier.cast()) };
    if rc == 0 {
        return 0;
    }
    if rc == libc::PTHREAD_BARRIER_SERIAL_THREAD {
        return 1;
    }
    unsafe { abort_now() }
}

pub(crate) unsafe fn once(guard: *mut abi::uv_once_t, callback: Option<unsafe extern "C" fn()>) {
    let Some(callback) = callback else {
        return;
    };
    let callback: extern "C" fn() = unsafe { std::mem::transmute(callback) };
    unsafe {
        assert_zero(libc::pthread_once(guard.cast(), callback));
    }
}

pub(crate) unsafe fn key_create(key: *mut abi::uv_key_t) -> c_int {
    if key.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    uv_err(unsafe { libc::pthread_key_create(key.cast(), None) })
}

pub(crate) unsafe fn key_delete(key: *mut abi::uv_key_t) {
    if key.is_null() {
        return;
    }
    unsafe {
        assert_zero(libc::pthread_key_delete(*key));
    }
}

pub(crate) unsafe fn key_get(key: *mut abi::uv_key_t) -> *mut std::ffi::c_void {
    if key.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { libc::pthread_getspecific(*key) }
}

pub(crate) unsafe fn key_set(key: *mut abi::uv_key_t, value: *mut std::ffi::c_void) {
    unsafe {
        assert_zero(libc::pthread_setspecific(*key, value));
    }
}

pub(crate) unsafe fn mutex_init(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe { mutex_init_raw(mutex, false) }
}

pub(crate) unsafe fn mutex_init_recursive(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe { mutex_init_raw(mutex, true) }
}

pub(crate) unsafe fn mutex_destroy(mutex: *mut abi::uv_mutex_t) {
    unsafe { mutex_destroy_raw(mutex) }
}

pub(crate) unsafe fn mutex_lock(mutex: *mut abi::uv_mutex_t) {
    unsafe { mutex_lock_raw(mutex) }
}

pub(crate) unsafe fn mutex_trylock(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe { mutex_trylock_raw(mutex) }
}

pub(crate) unsafe fn mutex_unlock(mutex: *mut abi::uv_mutex_t) {
    unsafe { mutex_unlock_raw(mutex) }
}

pub(crate) unsafe fn rwlock_init(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe { rwlock_init_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_destroy(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { rwlock_destroy_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_rdlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { rwlock_rdlock_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_tryrdlock(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe { rwlock_tryrdlock_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_rdunlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { rwlock_rdunlock_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_wrlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { rwlock_wrlock_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_trywrlock(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe { rwlock_trywrlock_raw(rwlock) }
}

pub(crate) unsafe fn rwlock_wrunlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { rwlock_wrunlock_raw(rwlock) }
}

pub(crate) unsafe fn sem_init(sem: *mut abi::uv_sem_t, value: c_uint) -> c_int {
    unsafe { sem_init_raw(sem, value) }
}

pub(crate) unsafe fn sem_destroy(sem: *mut abi::uv_sem_t) {
    unsafe { sem_destroy_raw(sem) }
}

pub(crate) unsafe fn sem_post(sem: *mut abi::uv_sem_t) {
    unsafe { sem_post_raw(sem) }
}

pub(crate) unsafe fn sem_wait(sem: *mut abi::uv_sem_t) {
    unsafe { sem_wait_raw(sem) }
}

pub(crate) unsafe fn sem_trywait(sem: *mut abi::uv_sem_t) -> c_int {
    unsafe { sem_trywait_raw(sem) }
}

pub(crate) unsafe fn cond_init(cond: *mut abi::uv_cond_t) -> c_int {
    unsafe { cond_init_raw(cond) }
}

pub(crate) unsafe fn cond_destroy(cond: *mut abi::uv_cond_t) {
    unsafe { cond_destroy_raw(cond) }
}

pub(crate) unsafe fn cond_signal(cond: *mut abi::uv_cond_t) {
    unsafe { cond_signal_raw(cond) }
}

pub(crate) unsafe fn cond_broadcast(cond: *mut abi::uv_cond_t) {
    unsafe { cond_broadcast_raw(cond) }
}

pub(crate) unsafe fn cond_wait(cond: *mut abi::uv_cond_t, mutex: *mut abi::uv_mutex_t) {
    unsafe { cond_wait_raw(cond, mutex) }
}

pub(crate) unsafe fn cond_timedwait(
    cond: *mut abi::uv_cond_t,
    mutex: *mut abi::uv_mutex_t,
    timeout: u64,
) -> c_int {
    unsafe { cond_timedwait_raw(cond, mutex, timeout) }
}

pub(crate) unsafe fn barrier_init(barrier: *mut abi::uv_barrier_t, count: c_uint) -> c_int {
    unsafe { barrier_init_raw(barrier, count) }
}

pub(crate) unsafe fn barrier_destroy(barrier: *mut abi::uv_barrier_t) {
    unsafe { barrier_destroy_raw(barrier) }
}

pub(crate) unsafe fn barrier_wait(barrier: *mut abi::uv_barrier_t) -> c_int {
    unsafe { barrier_wait_raw(barrier) }
}
