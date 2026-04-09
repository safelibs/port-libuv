use crate::abi::linux_x86_64 as abi;
use crate::core::allocator;
use crate::util::idna;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::null_mut;

fn clone_bytes(bytes: &[u8]) -> Result<*mut c_char, c_int> {
    let ptr = unsafe { allocator::malloc_bytes(bytes.len()) }.cast::<c_char>();
    if ptr.is_null() {
        return Err(abi::uv_errno_t_UV_ENOMEM);
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr().cast::<c_char>(), ptr, bytes.len());
    }
    Ok(ptr)
}

pub(crate) unsafe fn prepare_hostname(node: *const c_char) -> Result<*mut c_char, c_int> {
    if node.is_null() {
        return Ok(null_mut());
    }

    let ascii = idna::toascii_host(unsafe { CStr::from_ptr(node) })?;
    clone_bytes(ascii.as_bytes_with_nul())
}

pub(crate) unsafe fn if_indextoname(
    ifindex: c_uint,
    buffer: *mut c_char,
    size: *mut usize,
) -> c_int {
    let mut ifname = [0 as c_char; libc::IF_NAMESIZE];
    if buffer.is_null() || size.is_null() || unsafe { *size } == 0 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    if unsafe { libc::if_indextoname(ifindex, ifname.as_mut_ptr()) }.is_null() {
        return -unsafe { *libc::__errno_location() };
    }

    let len = unsafe { libc::strnlen(ifname.as_ptr(), ifname.len()) };
    if unsafe { *size } <= len {
        unsafe {
            *size = len + 1;
        }
        return abi::uv_errno_t_UV_ENOBUFS;
    }

    unsafe {
        libc::memcpy(buffer.cast(), ifname.as_ptr().cast(), len);
        *buffer.add(len) = 0;
        *size = len;
    }
    0
}

pub(crate) unsafe fn if_indextoiid(
    ifindex: c_uint,
    buffer: *mut c_char,
    size: *mut usize,
) -> c_int {
    unsafe { if_indextoname(ifindex, buffer, size) }
}
