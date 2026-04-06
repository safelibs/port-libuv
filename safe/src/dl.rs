use crate::allocator;
use crate::bindings::*;

static NO_ERROR: &[u8] = b"no error\0";

unsafe fn update_dlerror(lib: *mut uv_lib_t) -> libc::c_int {
    allocator::free((*lib).errmsg.cast());

    let errmsg = libc::dlerror();
    if errmsg.is_null() {
        (*lib).errmsg = std::ptr::null_mut();
        0
    } else {
        (*lib).errmsg = allocator::strdup(errmsg);
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_dlopen(
    filename: *const libc::c_char,
    lib: *mut uv_lib_t,
) -> libc::c_int {
    libc::dlerror();
    (*lib).errmsg = std::ptr::null_mut();
    (*lib).handle = libc::dlopen(filename, libc::RTLD_LAZY);
    if (*lib).handle.is_null() {
        update_dlerror(lib)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_dlclose(lib: *mut uv_lib_t) {
    allocator::free((*lib).errmsg.cast());
    (*lib).errmsg = std::ptr::null_mut();

    if !(*lib).handle.is_null() {
        libc::dlclose((*lib).handle);
        (*lib).handle = std::ptr::null_mut();
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_dlsym(
    lib: *mut uv_lib_t,
    name: *const libc::c_char,
    ptr: *mut *mut libc::c_void,
) -> libc::c_int {
    libc::dlerror();
    *ptr = libc::dlsym((*lib).handle, name);
    if (*ptr).is_null() {
        update_dlerror(lib)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_dlerror(lib: *const uv_lib_t) -> *const libc::c_char {
    if (*lib).errmsg.is_null() {
        NO_ERROR.as_ptr().cast()
    } else {
        (*lib).errmsg.cast()
    }
}
