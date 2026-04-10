use crate::abi::linux_x86_64 as abi;
use crate::core::time;
use libc::{self, c_char, c_int, c_uint};
use std::ptr;
use std::sync::OnceLock;

#[repr(C)]
struct CustomSemaphore {
    mutex: abi::uv_mutex_t,
    cond: abi::uv_cond_t,
    value: c_uint,
}

#[inline]
fn uv_err(err: c_int) -> c_int {
    if err == 0 {
        0
    } else {
        -err
    }
}

#[cold]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn abort_now() -> ! {
    unsafe {
        unsafe {
            libc::abort();
        }
    }
}

#[cfg(target_env = "gnu")]
// SAFETY(syscall_ffi): glibc exposes this version query through its stable C ABI.
unsafe extern "C" {
    // SAFETY(syscall_ffi): glibc exports the version string through a stable process-global pointer.
    fn gnu_get_libc_version() -> *const c_char;
}

// SAFETY(syscall_ffi): glibc version probing and semaphore fallback selection use stable libc ABI entry points.
fn platform_needs_custom_semaphore() -> bool {
    static NEEDS_CUSTOM_SEMAPHORE: OnceLock<bool> = OnceLock::new();

    *NEEDS_CUSTOM_SEMAPHORE.get_or_init(|| {
        #[cfg(target_env = "gnu")]
        {
            let version_ptr = unsafe { gnu_get_libc_version() };
            if version_ptr.is_null() {
                return false;
            }

            let version = unsafe { std::ffi::CStr::from_ptr(version_ptr) }.to_bytes();
            let mut parts = version.split(|byte| *byte == b'.');
            let major = parts
                .next()
                .and_then(|part| std::str::from_utf8(part).ok())
                .and_then(|part| part.parse::<u32>().ok());
            let minor = parts
                .next()
                .and_then(|part| std::str::from_utf8(part).ok())
                .and_then(|part| part.parse::<u32>().ok());

            match (major, minor) {
                (Some(2), Some(minor)) => minor < 21,
                _ => false,
            }
        }
        #[cfg(not(target_env = "gnu"))]
        {
            false
        }
    })
}

#[inline]
fn custom_sem_storage(sem: *mut abi::uv_sem_t) -> *mut *mut CustomSemaphore {
    sem.cast()
}

// SAFETY(syscall_ffi): the custom semaphore stores a heap pointer inside uv_sem_t storage on affected glibc versions.
fn custom_sem_init_raw(sem: *mut abi::uv_sem_t, value: c_uint) -> c_int {
    if std::mem::size_of::<abi::uv_sem_t>() < std::mem::size_of::<*mut CustomSemaphore>() {
        return abi::uv_errno_t_UV_ENOSYS;
    }

    let mut state = Box::new(CustomSemaphore {
        mutex: unsafe { std::mem::zeroed() },
        cond: unsafe { std::mem::zeroed() },
        value,
    });

    let rc = mutex_init_raw(ptr::addr_of_mut!(state.mutex), false);
    if rc != 0 {
        return rc;
    }

    let rc = cond_init_raw(ptr::addr_of_mut!(state.cond));
    if rc != 0 {
        mutex_destroy_raw(ptr::addr_of_mut!(state.mutex));
        return rc;
    }

    let raw = Box::into_raw(state);
    unsafe {
        ptr::write_bytes(sem.cast::<u8>(), 0, std::mem::size_of::<abi::uv_sem_t>());
        custom_sem_storage(sem).write(raw);
    }
    0
}

// SAFETY(syscall_ffi): the custom semaphore fallback reconstructs the boxed state from uv_sem_t storage for teardown.
fn custom_sem_destroy_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        let state = custom_sem_storage(sem).read();
        if state.is_null() {
            return;
        }

        cond_destroy_raw(ptr::addr_of_mut!((*state).cond));
        mutex_destroy_raw(ptr::addr_of_mut!((*state).mutex));
        drop(Box::from_raw(state));
        custom_sem_storage(sem).write(ptr::null_mut());
    }
}

// SAFETY(syscall_ffi): the custom semaphore fallback mutates state referenced through uv_sem_t pointer storage.
fn custom_sem_post_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        let state = custom_sem_storage(sem).read();
        if state.is_null() {
            abort_now();
        }

        mutex_lock_raw(ptr::addr_of_mut!((*state).mutex));
        (*state).value = (*state).value.wrapping_add(1);
        if (*state).value == 1 {
            cond_signal_raw(ptr::addr_of_mut!((*state).cond));
        }
        mutex_unlock_raw(ptr::addr_of_mut!((*state).mutex));
    }
}

// SAFETY(syscall_ffi): the custom semaphore fallback waits on the boxed mutex and condvar stored behind uv_sem_t.
fn custom_sem_wait_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        let state = custom_sem_storage(sem).read();
        if state.is_null() {
            abort_now();
        }

        mutex_lock_raw(ptr::addr_of_mut!((*state).mutex));
        while (*state).value == 0 {
            cond_wait_raw(
                ptr::addr_of_mut!((*state).cond),
                ptr::addr_of_mut!((*state).mutex),
            );
        }
        (*state).value -= 1;
        mutex_unlock_raw(ptr::addr_of_mut!((*state).mutex));
    }
}

// SAFETY(syscall_ffi): the custom semaphore fallback performs a non-blocking check against boxed state stored behind uv_sem_t.
fn custom_sem_trywait_raw(sem: *mut abi::uv_sem_t) -> c_int {
    unsafe {
        let state = custom_sem_storage(sem).read();
        if state.is_null() {
            abort_now();
        }

        if mutex_trylock_raw(ptr::addr_of_mut!((*state).mutex)) != 0 {
            return abi::uv_errno_t_UV_EAGAIN;
        }

        if (*state).value == 0 {
            mutex_unlock_raw(ptr::addr_of_mut!((*state).mutex));
            return abi::uv_errno_t_UV_EAGAIN;
        }

        (*state).value -= 1;
        mutex_unlock_raw(ptr::addr_of_mut!((*state).mutex));
        0
    }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn assert_zero(err: c_int) {
    unsafe {
        if err != 0 {
            unsafe { abort_now() };
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_init_raw(mutex: *mut abi::uv_mutex_t, recursive: bool) -> c_int {
    unsafe {
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_destroy_raw(mutex: *mut abi::uv_mutex_t) {
    unsafe {
        if mutex.is_null() {
            return;
        }
        unsafe {
            assert_zero(libc::pthread_mutex_destroy(mutex.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_lock_raw(mutex: *mut abi::uv_mutex_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_mutex_lock(mutex.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_trylock_raw(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe {
        let err = unsafe { libc::pthread_mutex_trylock(mutex.cast()) };
        if err == 0 {
            return 0;
        }
        if err == libc::EBUSY || err == libc::EAGAIN {
            return abi::uv_errno_t_UV_EBUSY;
        }
        unsafe { abort_now() }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_unlock_raw(mutex: *mut abi::uv_mutex_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_mutex_unlock(mutex.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_init_raw(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe {
        if rwlock.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        uv_err(unsafe { libc::pthread_rwlock_init(rwlock.cast(), std::ptr::null()) })
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_destroy_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        if rwlock.is_null() {
            return;
        }
        unsafe {
            assert_zero(libc::pthread_rwlock_destroy(rwlock.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_rdlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_rwlock_rdlock(rwlock.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_tryrdlock_raw(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe {
        let err = unsafe { libc::pthread_rwlock_tryrdlock(rwlock.cast()) };
        if err == 0 {
            return 0;
        }
        if err == libc::EBUSY || err == libc::EAGAIN {
            return abi::uv_errno_t_UV_EBUSY;
        }
        unsafe { abort_now() }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_rdunlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_rwlock_unlock(rwlock.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_wrlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_rwlock_wrlock(rwlock.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_trywrlock_raw(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe {
        let err = unsafe { libc::pthread_rwlock_trywrlock(rwlock.cast()) };
        if err == 0 {
            return 0;
        }
        if err == libc::EBUSY || err == libc::EAGAIN {
            return abi::uv_errno_t_UV_EBUSY;
        }
        unsafe { abort_now() }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_wrunlock_raw(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_rwlock_unlock(rwlock.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_init_raw(sem: *mut abi::uv_sem_t, value: c_uint) -> c_int {
    unsafe {
        if sem.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        if platform_needs_custom_semaphore() {
            return custom_sem_init_raw(sem, value);
        }
        if unsafe { libc::sem_init(sem.cast(), 0, value) } != 0 {
            return -unsafe { *libc::__errno_location() };
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_destroy_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        if sem.is_null() {
            return;
        }
        if platform_needs_custom_semaphore() {
            custom_sem_destroy_raw(sem);
            return;
        }
        unsafe {
            assert_zero(libc::sem_destroy(sem.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_post_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        if platform_needs_custom_semaphore() {
            custom_sem_post_raw(sem);
            return;
        }
        unsafe {
            assert_zero(libc::sem_post(sem.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_wait_raw(sem: *mut abi::uv_sem_t) {
    unsafe {
        if platform_needs_custom_semaphore() {
            custom_sem_wait_raw(sem);
            return;
        }
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_trywait_raw(sem: *mut abi::uv_sem_t) -> c_int {
    unsafe {
        if platform_needs_custom_semaphore() {
            return custom_sem_trywait_raw(sem);
        }
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_init_raw(cond: *mut abi::uv_cond_t) -> c_int {
    unsafe {
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_destroy_raw(cond: *mut abi::uv_cond_t) {
    unsafe {
        if cond.is_null() {
            return;
        }
        unsafe {
            assert_zero(libc::pthread_cond_destroy(cond.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_signal_raw(cond: *mut abi::uv_cond_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_cond_signal(cond.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_broadcast_raw(cond: *mut abi::uv_cond_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_cond_broadcast(cond.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_wait_raw(cond: *mut abi::uv_cond_t, mutex: *mut abi::uv_mutex_t) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_cond_wait(cond.cast(), mutex.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_timedwait_raw(
    cond: *mut abi::uv_cond_t,
    mutex: *mut abi::uv_mutex_t,
    timeout: u64,
) -> c_int {
    unsafe {
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
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn barrier_init_raw(barrier: *mut abi::uv_barrier_t, count: c_uint) -> c_int {
    unsafe {
        if barrier.is_null() || count == 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }
        uv_err(unsafe { libc::pthread_barrier_init(barrier.cast(), std::ptr::null(), count) })
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn barrier_destroy_raw(barrier: *mut abi::uv_barrier_t) {
    unsafe {
        if barrier.is_null() {
            return;
        }
        unsafe {
            assert_zero(libc::pthread_barrier_destroy(barrier.cast()));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn barrier_wait_raw(barrier: *mut abi::uv_barrier_t) -> c_int {
    unsafe {
        let rc = unsafe { libc::pthread_barrier_wait(barrier.cast()) };
        if rc == 0 {
            return 0;
        }
        if rc == libc::PTHREAD_BARRIER_SERIAL_THREAD {
            return 1;
        }
        unsafe { abort_now() }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub(crate) fn once(guard: *mut abi::uv_once_t, callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let Some(callback) = callback else {
            return;
        };
        let callback: extern "C" fn() = unsafe { std::mem::transmute(callback) };
        unsafe {
            assert_zero(libc::pthread_once(guard.cast(), callback));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn key_create(key: *mut abi::uv_key_t) -> c_int {
    unsafe {
        if key.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        uv_err(unsafe { libc::pthread_key_create(key.cast(), None) })
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn key_delete(key: *mut abi::uv_key_t) {
    unsafe {
        if key.is_null() {
            return;
        }
        unsafe {
            assert_zero(libc::pthread_key_delete(*key));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn key_get(key: *mut abi::uv_key_t) -> *mut std::ffi::c_void {
    unsafe {
        if key.is_null() {
            return std::ptr::null_mut();
        }
        unsafe { libc::pthread_getspecific(*key) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn key_set(key: *mut abi::uv_key_t, value: *mut std::ffi::c_void) {
    unsafe {
        unsafe {
            assert_zero(libc::pthread_setspecific(*key, value));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_init(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe { unsafe { mutex_init_raw(mutex, false) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_init_recursive(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe { unsafe { mutex_init_raw(mutex, true) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_destroy(mutex: *mut abi::uv_mutex_t) {
    unsafe { unsafe { mutex_destroy_raw(mutex) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_lock(mutex: *mut abi::uv_mutex_t) {
    unsafe { unsafe { mutex_lock_raw(mutex) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_trylock(mutex: *mut abi::uv_mutex_t) -> c_int {
    unsafe { unsafe { mutex_trylock_raw(mutex) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mutex_unlock(mutex: *mut abi::uv_mutex_t) {
    unsafe { unsafe { mutex_unlock_raw(mutex) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_init(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe { unsafe { rwlock_init_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_destroy(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { unsafe { rwlock_destroy_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_rdlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { unsafe { rwlock_rdlock_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_tryrdlock(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe { unsafe { rwlock_tryrdlock_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_rdunlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { unsafe { rwlock_rdunlock_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_wrlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { unsafe { rwlock_wrlock_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_trywrlock(rwlock: *mut abi::uv_rwlock_t) -> c_int {
    unsafe { unsafe { rwlock_trywrlock_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rwlock_wrunlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe { unsafe { rwlock_wrunlock_raw(rwlock) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_init(sem: *mut abi::uv_sem_t, value: c_uint) -> c_int {
    unsafe { unsafe { sem_init_raw(sem, value) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_destroy(sem: *mut abi::uv_sem_t) {
    unsafe { unsafe { sem_destroy_raw(sem) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_post(sem: *mut abi::uv_sem_t) {
    unsafe { unsafe { sem_post_raw(sem) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_wait(sem: *mut abi::uv_sem_t) {
    unsafe { unsafe { sem_wait_raw(sem) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sem_trywait(sem: *mut abi::uv_sem_t) -> c_int {
    unsafe { unsafe { sem_trywait_raw(sem) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_init(cond: *mut abi::uv_cond_t) -> c_int {
    unsafe { unsafe { cond_init_raw(cond) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_destroy(cond: *mut abi::uv_cond_t) {
    unsafe { unsafe { cond_destroy_raw(cond) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_signal(cond: *mut abi::uv_cond_t) {
    unsafe { unsafe { cond_signal_raw(cond) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_broadcast(cond: *mut abi::uv_cond_t) {
    unsafe { unsafe { cond_broadcast_raw(cond) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_wait(cond: *mut abi::uv_cond_t, mutex: *mut abi::uv_mutex_t) {
    unsafe { unsafe { cond_wait_raw(cond, mutex) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cond_timedwait(
    cond: *mut abi::uv_cond_t,
    mutex: *mut abi::uv_mutex_t,
    timeout: u64,
) -> c_int {
    unsafe { unsafe { cond_timedwait_raw(cond, mutex, timeout) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn barrier_init(barrier: *mut abi::uv_barrier_t, count: c_uint) -> c_int {
    unsafe { unsafe { barrier_init_raw(barrier, count) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn barrier_destroy(barrier: *mut abi::uv_barrier_t) {
    unsafe { unsafe { barrier_destroy_raw(barrier) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn barrier_wait(barrier: *mut abi::uv_barrier_t) -> c_int {
    unsafe { unsafe { barrier_wait_raw(barrier) } }
}
