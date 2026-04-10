use crate::abi::linux_x86_64 as abi;
use crate::core::allocator;
use std::os::raw::{c_char, c_int};

struct ErrorEntry {
    code: c_int,
    name: &'static [u8],
    message: &'static [u8],
}

const ENTRIES: &[ErrorEntry] = &[
    ErrorEntry {
        code: 0,
        name: b"OK\0",
        message: b"Success\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAGAIN,
        name: b"EAGAIN\0",
        message: b"resource temporarily unavailable\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EALREADY,
        name: b"EALREADY\0",
        message: b"connection already in progress\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EBUSY,
        name: b"EBUSY\0",
        message: b"resource busy or locked\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ECANCELED,
        name: b"ECANCELED\0",
        message: b"operation canceled\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EINVAL,
        name: b"EINVAL\0",
        message: b"invalid argument\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOMEM,
        name: b"ENOMEM\0",
        message: b"not enough memory\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOSYS,
        name: b"ENOSYS\0",
        message: b"function not implemented\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EPERM,
        name: b"EPERM\0",
        message: b"operation not permitted\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EPIPE,
        name: b"EPIPE\0",
        message: b"broken pipe\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ETIMEDOUT,
        name: b"ETIMEDOUT\0",
        message: b"connection timed out\0",
    },
    ErrorEntry {
        code: abi::UV__EOF,
        name: b"EOF\0",
        message: b"end of file\0",
    },
];

#[inline]
fn canonicalize(err: c_int) -> c_int {
    if err > 0 { -err } else { err }
}

fn lookup(err: c_int) -> Option<&'static ErrorEntry> {
    let err = canonicalize(err);
    ENTRIES.iter().find(|entry| entry.code == err)
}

unsafe fn copy_to_buffer(src: &[u8], buf: *mut c_char, buflen: usize) -> *mut c_char {
    if buf.is_null() || buflen == 0 {
        return buf;
    }

    let copy_len = src
        .iter()
        .position(|byte| *byte == 0)
        .unwrap_or(src.len())
        .min(buflen.saturating_sub(1));

    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), buf.cast::<u8>(), copy_len);
        *buf.cast::<u8>().add(copy_len) = 0;
    }
    buf
}

unsafe fn alloc_unknown(prefix: &[u8], err: c_int) -> *const c_char {
    let text = if prefix == b"name" {
        format!("Unknown system error {err}")
    } else {
        "Unknown error".to_owned()
    };
    let bytes = text.as_bytes();
    let raw = unsafe { allocator::malloc_bytes(bytes.len() + 1) }.cast::<u8>();
    if raw.is_null() {
        return if prefix == b"name" {
            b"UNKNOWN\0".as_ptr().cast()
        } else {
            b"Unknown system error\0".as_ptr().cast()
        };
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), raw, bytes.len());
        *raw.add(bytes.len()) = 0;
    }
    raw.cast()
}

pub(crate) unsafe fn err_name(err: c_int) -> *const c_char {
    match lookup(err) {
        Some(entry) => entry.name.as_ptr().cast(),
        None => unsafe { alloc_unknown(b"name", err) },
    }
}

pub(crate) unsafe fn err_name_r(err: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char {
    let text = match lookup(err) {
        Some(entry) => entry.name,
        None => format!("Unknown system error {err}\0").into_bytes().leak(),
    };
    unsafe { copy_to_buffer(text, buf, buflen) }
}

pub(crate) unsafe fn strerror(err: c_int) -> *const c_char {
    match lookup(err) {
        Some(entry) => entry.message.as_ptr().cast(),
        None => unsafe { alloc_unknown(b"message", canonicalize(err)) },
    }
}

pub(crate) unsafe fn strerror_r(err: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char {
    let err = canonicalize(err);
    match lookup(err) {
        Some(entry) => unsafe { copy_to_buffer(entry.message, buf, buflen) },
        None => {
            let owned = format!("Unknown system error {err}\0");
            unsafe { copy_to_buffer(owned.as_bytes(), buf, buflen) }
        }
    }
}
