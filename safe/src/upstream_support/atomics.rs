use libc::{c_int, c_long};
use std::sync::atomic::{AtomicI32, AtomicIsize, AtomicU32, Ordering};

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn atomic_xchg_relaxed_i32(ptr: *mut c_int, value: c_int) -> c_int {
    unsafe { unsafe { (&*(ptr.cast::<AtomicI32>())).swap(value, Ordering::Relaxed) } }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn atomic_load_acquire_u32(ptr: *mut u32) -> u32 {
    unsafe { unsafe { (&*(ptr.cast::<AtomicU32>())).load(Ordering::Acquire) } }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn atomic_store_release_u32(ptr: *mut u32, value: u32) {
    unsafe {
        unsafe {
            (&*(ptr.cast::<AtomicU32>() as *const AtomicU32)).store(value, Ordering::Release);
        }
    }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn atomic_load_relaxed_long(ptr: *mut c_long) -> c_long {
    unsafe { unsafe { (&*(ptr.cast::<AtomicIsize>())).load(Ordering::Relaxed) as c_long } }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn atomic_store_relaxed_long(ptr: *mut c_long, value: c_long) {
    unsafe {
        unsafe {
            (&*(ptr.cast::<AtomicIsize>() as *const AtomicIsize))
                .store(value as isize, Ordering::Relaxed);
        }
    }
}
