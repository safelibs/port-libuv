use crate::bindings::*;

static VERSION_STRING: &[u8] = b"1.48.0\0";

#[no_mangle]
pub unsafe extern "C" fn uv_version() -> libc::c_uint {
    UV_VERSION_HEX
}

#[no_mangle]
pub unsafe extern "C" fn uv_version_string() -> *const libc::c_char {
    VERSION_STRING.as_ptr().cast()
}
