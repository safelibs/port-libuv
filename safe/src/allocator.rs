use crate::bindings::*;
use std::ptr;
use std::sync::RwLock;

pub(crate) const UV_E2BIG: libc::c_int = uv_errno_t_UV_E2BIG;
pub(crate) const UV_EAGAIN: libc::c_int = uv_errno_t_UV_EAGAIN;
pub(crate) const UV_EAFNOSUPPORT: libc::c_int = uv_errno_t_UV_EAFNOSUPPORT;
pub(crate) const UV_EBUSY: libc::c_int = uv_errno_t_UV_EBUSY;
pub(crate) const UV_EFAULT: libc::c_int = uv_errno_t_UV_EFAULT;
pub(crate) const UV_EINVAL: libc::c_int = uv_errno_t_UV_EINVAL;
pub(crate) const UV_ENOBUFS: libc::c_int = uv_errno_t_UV_ENOBUFS;
pub(crate) const UV_ENOENT: libc::c_int = uv_errno_t_UV_ENOENT;
pub(crate) const UV_ENOMEM: libc::c_int = uv_errno_t_UV_ENOMEM;
pub(crate) const UV_ENOSPC: libc::c_int = uv_errno_t_UV_ENOSPC;
pub(crate) const UV_ENOTSUP: libc::c_int = uv_errno_t_UV_ENOTSUP;
pub(crate) const UV_ETIMEDOUT: libc::c_int = uv_errno_t_UV_ETIMEDOUT;

#[derive(Clone, Copy)]
struct AllocatorHooks {
    malloc: uv_malloc_func,
    realloc: uv_realloc_func,
    calloc: uv_calloc_func,
    free: uv_free_func,
}

static ALLOCATOR_HOOKS: RwLock<AllocatorHooks> = RwLock::new(AllocatorHooks {
    malloc: Some(libc::malloc),
    realloc: Some(libc::realloc),
    calloc: Some(libc::calloc),
    free: Some(libc::free),
});

#[inline]
pub(crate) unsafe fn errno_location() -> *mut libc::c_int {
    libc::__errno_location()
}

#[inline]
pub(crate) unsafe fn last_error() -> libc::c_int {
    -(*errno_location())
}

#[inline]
pub(crate) fn uv_err(code: libc::c_int) -> libc::c_int {
    -code
}

fn hooks() -> AllocatorHooks {
    *ALLOCATOR_HOOKS.read().expect("allocator hooks poisoned")
}

pub(crate) unsafe fn malloc(size: usize) -> *mut libc::c_void {
    if size == 0 {
        return ptr::null_mut();
    }

    hooks().malloc.expect("malloc hook missing")(size)
}

pub(crate) unsafe fn calloc(count: usize, size: usize) -> *mut libc::c_void {
    hooks().calloc.expect("calloc hook missing")(count, size)
}

pub(crate) unsafe fn realloc(ptr: *mut libc::c_void, size: usize) -> *mut libc::c_void {
    if size == 0 {
        free(ptr);
        return ptr::null_mut();
    }

    hooks().realloc.expect("realloc hook missing")(ptr, size)
}

pub(crate) unsafe fn reallocf(ptr: *mut libc::c_void, size: usize) -> *mut libc::c_void {
    let new_ptr = realloc(ptr, size);
    if new_ptr.is_null() && size > 0 {
        free(ptr);
    }
    new_ptr
}

pub(crate) unsafe fn free(ptr: *mut libc::c_void) {
    let saved_errno = *errno_location();
    hooks().free.expect("free hook missing")(ptr);
    *errno_location() = saved_errno;
}

pub(crate) unsafe fn strdup(s: *const libc::c_char) -> *mut libc::c_char {
    if s.is_null() {
        return ptr::null_mut();
    }

    let len = libc::strlen(s) + 1;
    let dst = malloc(len).cast::<libc::c_char>();
    if dst.is_null() {
        return ptr::null_mut();
    }

    ptr::copy_nonoverlapping(s, dst, len);
    dst
}

pub(crate) unsafe fn strndup(s: *const libc::c_char, n: usize) -> *mut libc::c_char {
    if s.is_null() {
        return ptr::null_mut();
    }

    let len = libc::strlen(s).min(n);
    let dst = malloc(len + 1).cast::<libc::c_char>();
    if dst.is_null() {
        return ptr::null_mut();
    }

    ptr::copy_nonoverlapping(s, dst, len);
    *dst.add(len) = 0;
    dst
}

#[no_mangle]
pub unsafe extern "C" fn uv_replace_allocator(
    malloc_func: uv_malloc_func,
    realloc_func: uv_realloc_func,
    calloc_func: uv_calloc_func,
    free_func: uv_free_func,
) -> libc::c_int {
    let Some(malloc_func) = malloc_func else {
        return UV_EINVAL;
    };
    let Some(realloc_func) = realloc_func else {
        return UV_EINVAL;
    };
    let Some(calloc_func) = calloc_func else {
        return UV_EINVAL;
    };
    let Some(free_func) = free_func else {
        return UV_EINVAL;
    };

    let rc = crate::private_support::uv_replace_allocator(
        Some(malloc_func),
        Some(realloc_func),
        Some(calloc_func),
        Some(free_func),
    );
    if rc != 0 {
        return rc;
    }

    *ALLOCATOR_HOOKS.write().expect("allocator hooks poisoned") = AllocatorHooks {
        malloc: Some(malloc_func),
        realloc: Some(realloc_func),
        calloc: Some(calloc_func),
        free: Some(free_func),
    };

    0
}
