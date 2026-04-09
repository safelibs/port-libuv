use crate::abi::linux_x86_64 as abi;
use std::os::raw::{c_char, c_uint};

const VERSION_STRING: &[u8] = b"1.48.0\0";

#[inline(always)]
pub fn version_hex() -> c_uint {
    abi::UV_VERSION_HEX
}

#[inline(always)]
pub fn version_string_ptr() -> *const c_char {
    VERSION_STRING.as_ptr().cast()
}
