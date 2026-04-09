use libc::{c_int, c_long};
use std::sync::atomic::{AtomicI32, AtomicIsize, AtomicU32, Ordering};

#[inline]
pub(crate) unsafe fn atomic_xchg_relaxed_i32(ptr: *mut c_int, value: c_int) -> c_int {
    unsafe { (&*(ptr.cast::<AtomicI32>())).swap(value, Ordering::Relaxed) }
}

#[inline]
pub(crate) unsafe fn atomic_load_acquire_u32(ptr: *mut u32) -> u32 {
    unsafe { (&*(ptr.cast::<AtomicU32>())).load(Ordering::Acquire) }
}

#[inline]
pub(crate) unsafe fn atomic_store_release_u32(ptr: *mut u32, value: u32) {
    unsafe {
        (&*(ptr.cast::<AtomicU32>() as *const AtomicU32)).store(value, Ordering::Release);
    }
}

#[inline]
pub(crate) unsafe fn atomic_load_relaxed_long(ptr: *mut c_long) -> c_long {
    unsafe { (&*(ptr.cast::<AtomicIsize>())).load(Ordering::Relaxed) as c_long }
}

#[inline]
pub(crate) unsafe fn atomic_store_relaxed_long(ptr: *mut c_long, value: c_long) {
    unsafe {
        (&*(ptr.cast::<AtomicIsize>() as *const AtomicIsize))
            .store(value as isize, Ordering::Relaxed);
    }
}
