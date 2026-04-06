use std::ffi::CStr;

pub(crate) unsafe fn is_long_ascii_hostname(hostname: *const libc::c_char) -> bool {
    if hostname.is_null() {
        return false;
    }

    let bytes = CStr::from_ptr(hostname).to_bytes();
    bytes.len() >= 256 && bytes.is_ascii()
}
