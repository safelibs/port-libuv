use crate::abi::linux_x86_64 as abi;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::sync::RwLock;

// SAFETY(syscall_ffi): these allocator symbols are provided by the platform C runtime.
unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn calloc(count: usize, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn __errno_location() -> *mut c_int;
}

#[derive(Clone, Copy)]
struct AllocatorFns {
    malloc: abi::uv_malloc_func,
    realloc: abi::uv_realloc_func,
    calloc: abi::uv_calloc_func,
    free: abi::uv_free_func,
}

const DEFAULT_ALLOCATOR: AllocatorFns = AllocatorFns {
    malloc: Some(malloc),
    realloc: Some(realloc),
    calloc: Some(calloc),
    free: Some(free),
};

static ALLOCATOR: RwLock<AllocatorFns> = RwLock::new(DEFAULT_ALLOCATOR);

pub(crate) fn replace_allocator(
    malloc_fn: abi::uv_malloc_func,
    realloc_fn: abi::uv_realloc_func,
    calloc_fn: abi::uv_calloc_func,
    free_fn: abi::uv_free_func,
) -> c_int {
    if malloc_fn.is_none() || realloc_fn.is_none() || calloc_fn.is_none() || free_fn.is_none() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let mut guard = ALLOCATOR.write().unwrap();
    *guard = AllocatorFns {
        malloc: malloc_fn,
        realloc: realloc_fn,
        calloc: calloc_fn,
        free: free_fn,
    };
    0
}

#[inline]
// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn malloc_bytes(size: usize) -> *mut c_void {
    unsafe {
        if size == 0 {
            return std::ptr::null_mut();
        }
        let guard = ALLOCATOR.read().unwrap();
        unsafe { guard.malloc.unwrap_unchecked()(size) }
    }
}

#[inline]
// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn calloc_bytes(count: usize, size: usize) -> *mut c_void {
    unsafe {
        let guard = ALLOCATOR.read().unwrap();
        unsafe { guard.calloc.unwrap_unchecked()(count, size) }
    }
}

#[inline]
// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn realloc_bytes(ptr: *mut c_void, size: usize) -> *mut c_void {
    unsafe {
        if size == 0 {
            unsafe {
                free_bytes(ptr);
            }
            return std::ptr::null_mut();
        }
        let guard = ALLOCATOR.read().unwrap();
        unsafe { guard.realloc.unwrap_unchecked()(ptr, size) }
    }
}

#[inline]
// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn free_bytes(ptr: *mut c_void) {
    unsafe {
        let errno_ptr = unsafe { __errno_location() };
        let saved_errno = if errno_ptr.is_null() {
            0
        } else {
            unsafe { *errno_ptr }
        };
        let guard = ALLOCATOR.read().unwrap();
        unsafe {
            guard.free.unwrap_unchecked()(ptr);
        }
        if !errno_ptr.is_null() {
            unsafe {
                *errno_ptr = saved_errno;
            }
        }
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn alloc_value<T>(value: T) -> *mut T {
    unsafe {
        let raw = unsafe { malloc_bytes(std::mem::size_of::<T>()) }.cast::<T>();
        if raw.is_null() {
            return raw;
        }
        unsafe {
            raw.write(value);
        }
        raw
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn alloc_zeroed<T>() -> *mut T {
    unsafe { unsafe { calloc_bytes(1, std::mem::size_of::<T>()) }.cast::<T>() }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn realloc_array<T>(ptr: *mut T, count: usize) -> *mut T {
    unsafe {
        let bytes = match count.checked_mul(std::mem::size_of::<T>()) {
            Some(bytes) => bytes,
            None => return std::ptr::null_mut(),
        };
        unsafe { realloc_bytes(ptr.cast(), bytes) }.cast::<T>()
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn free_value<T>(ptr: *mut T) {
    unsafe {
        if ptr.is_null() {
            return;
        }
        unsafe {
            std::ptr::drop_in_place(ptr);
            free_bytes(ptr.cast());
        }
    }
}
