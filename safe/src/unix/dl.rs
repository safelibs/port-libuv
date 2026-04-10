use crate::abi::linux_x86_64 as abi;
use crate::core::allocator;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// SAFETY(syscall_ffi): these loader entry points are raw libc interfaces on the target platform.
unsafe extern "C" {
    #[link_name = "dlopen"]
    fn c_dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    #[link_name = "dlclose"]
    fn c_dlclose(handle: *mut c_void) -> c_int;
    #[link_name = "dlsym"]
    fn c_dlsym(handle: *mut c_void, name: *const c_char) -> *mut c_void;
    #[link_name = "dlerror"]
    fn c_dlerror() -> *const c_char;
}

fn no_error_ptr() -> *const c_char {
    static NO_ERROR: &[u8] = b"no error\0";
    NO_ERROR.as_ptr().cast()
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn free_error(lib: *mut abi::uv_lib_t) {
    unsafe {
        if unsafe { (*lib).errmsg.is_null() } {
            return;
        }

        unsafe {
            allocator::free_bytes((*lib).errmsg.cast());
            (*lib).errmsg = ptr::null_mut();
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn store_error(lib: *mut abi::uv_lib_t) -> c_int {
    unsafe {
        unsafe { free_error(lib) };

        let err = unsafe { c_dlerror() };
        if err.is_null() {
            unsafe {
                (*lib).errmsg = ptr::null_mut();
            }
            return 0;
        }

        let bytes = unsafe { CStr::from_ptr(err).to_bytes_with_nul() };
        let copy = unsafe { allocator::malloc_bytes(bytes.len()) }.cast::<c_char>();
        if copy.is_null() {
            unsafe {
                (*lib).errmsg = ptr::null_mut();
            }
            return -1;
        }

        unsafe {
            ptr::copy_nonoverlapping(err, copy, bytes.len());
            (*lib).errmsg = copy;
        }

        -1
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn dlopen(filename: *const c_char, lib: *mut abi::uv_lib_t) -> c_int {
    unsafe {
        if filename.is_null() || lib.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            c_dlerror();
            (*lib).errmsg = ptr::null_mut();
            (*lib).handle = c_dlopen(filename, libc::RTLD_LAZY);
        }

        if unsafe { (*lib).handle.is_null() } {
            unsafe { store_error(lib) }
        } else {
            0
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn dlclose(lib: *mut abi::uv_lib_t) {
    unsafe {
        if lib.is_null() {
            return;
        }

        unsafe { free_error(lib) };
        if unsafe { (*lib).handle.is_null() } {
            return;
        }

        unsafe {
            let _ = c_dlclose((*lib).handle);
            (*lib).handle = ptr::null_mut();
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn dlsym(
    lib: *mut abi::uv_lib_t,
    name: *const c_char,
    ptr_out: *mut *mut c_void,
) -> c_int {
    unsafe {
        if lib.is_null() || name.is_null() || ptr_out.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            c_dlerror();
            *ptr_out = c_dlsym((*lib).handle, name);
        }

        if unsafe { (*ptr_out).is_null() } {
            unsafe { store_error(lib) }
        } else {
            0
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn dlerror(lib: *const abi::uv_lib_t) -> *const c_char {
    unsafe {
        if lib.is_null() || unsafe { (*lib).errmsg.is_null() } {
            no_error_ptr()
        } else {
            unsafe { (*lib).errmsg.cast_const() }
        }
    }
}
