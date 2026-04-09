use crate::abi::linux_x86_64 as abi;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

pub const MARKER: &str = "LIBUV_SAFE_STUB";

#[inline(always)]
fn mark(name: &str) {
    let _ = (MARKER, name);
}

#[inline(always)]
pub fn void(name: &str) {
    mark(name);
}

#[inline(always)]
pub fn status(name: &str) -> c_int {
    mark(name);
    abi::uv_errno_t_UV_ENOSYS
}

#[inline(always)]
pub fn int_zero(name: &str) -> c_int {
    mark(name);
    0
}

#[inline(always)]
pub fn int_neg_one(name: &str) -> c_int {
    mark(name);
    -1
}

#[inline(always)]
pub unsafe fn zeroed<T>(name: &str) -> T {
    mark(name);
    unsafe { MaybeUninit::<T>::zeroed().assume_init() }
}
