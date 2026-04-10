use crate::upstream_support;
use std::os::raw::{c_char, c_int, c_void};

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn inet_ntop(af: c_int, src: *const c_void, dst: *mut c_char, size: usize) -> c_int {
    unsafe { unsafe { upstream_support::inet::uv_inet_ntop(af, src.cast(), dst, size) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int {
    unsafe { unsafe { upstream_support::inet::uv_inet_pton(af, src, dst.cast()) } }
}
