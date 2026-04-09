use crate::abi::linux_x86_64 as abi;
use crate::core::handle;
use libc::{c_char, ssize_t};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__strscpy(d: *mut c_char, s: *const c_char, n: usize) -> ssize_t {
    let mut i = 0usize;

    while i < n {
        let ch = unsafe { *s.add(i) };
        unsafe {
            *d.add(i) = ch;
        }
        if ch == 0 {
            return if i > isize::MAX as usize {
                abi::uv_errno_t_UV_E2BIG as ssize_t
            } else {
                i as ssize_t
            };
        }
        i += 1;
    }

    if i != 0 {
        unsafe {
            *d.add(i - 1) = 0;
        }
    }

    abi::uv_errno_t_UV_E2BIG as ssize_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__strtok(
    str_: *mut c_char,
    sep: *const c_char,
    itr: *mut *mut c_char,
) -> *mut c_char {
    let start = if str_.is_null() {
        unsafe { *itr }
    } else {
        str_
    };
    let mut cur = start;

    if cur.is_null() {
        return std::ptr::null_mut();
    }

    while unsafe { *cur } != 0 {
        let mut sep_cur = sep;
        while unsafe { *sep_cur } != 0 {
            if unsafe { *cur } == unsafe { *sep_cur } {
                unsafe {
                    *itr = cur.add(1);
                    *cur = 0;
                }
                return start;
            }
            sep_cur = unsafe { sep_cur.add(1) };
        }
        cur = unsafe { cur.add(1) };
    }

    unsafe {
        *itr = std::ptr::null_mut();
    }
    start
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__work_done(handle: *mut abi::uv_async_t) {
    unsafe { crate::threading::threadpool::loop_wq_async_cb(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__async_stop(loop_: *mut abi::uv_loop_t) {
    unsafe { crate::unix_async::shutdown(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__async_fork(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { crate::unix_async::fork(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__async_close(handle: *mut abi::uv_async_t) {
    unsafe { crate::unix_async::close(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__signal_global_once_init() {
    unsafe { crate::unix::signal::global_once_init() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__signal_loop_cleanup(loop_: *mut abi::uv_loop_t) {
    unsafe { crate::unix::signal::loop_cleanup(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__signal_loop_fork(
    loop_: *mut abi::uv_loop_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::unix::signal::loop_fork(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__signal_cleanup() {
    unsafe { crate::unix::signal::cleanup_global() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__signal_close(handle: *mut abi::uv_signal_t) {
    unsafe { crate::unix::signal::close(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__process_init(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { crate::unix::process::loop_init(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__process_close(handle: *mut abi::uv_process_t) {
    unsafe { crate::unix::process::close(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__process_title_cleanup() {
    unsafe { crate::unix::process_title::cleanup() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__fs_poll_close(handle: *mut abi::uv_fs_poll_t) {
    unsafe { crate::unix::fs_poll::close(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__fs_post(_loop_: *mut abi::uv_loop_t, _req: *mut abi::uv_fs_t) {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv__threadpool_cleanup() {
    crate::threading::threadpool::cleanup();
}
