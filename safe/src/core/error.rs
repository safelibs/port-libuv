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
        code: abi::uv_errno_t_UV_E2BIG,
        name: b"E2BIG\0",
        message: b"argument list too long\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EACCES,
        name: b"EACCES\0",
        message: b"permission denied\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EADDRINUSE,
        name: b"EADDRINUSE\0",
        message: b"address already in use\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EADDRNOTAVAIL,
        name: b"EADDRNOTAVAIL\0",
        message: b"address not available\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAFNOSUPPORT,
        name: b"EAFNOSUPPORT\0",
        message: b"address family not supported\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAGAIN,
        name: b"EAGAIN\0",
        message: b"resource temporarily unavailable\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_ADDRFAMILY,
        name: b"EAI_ADDRFAMILY\0",
        message: b"address family not supported\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_AGAIN,
        name: b"EAI_AGAIN\0",
        message: b"temporary failure\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_BADFLAGS,
        name: b"EAI_BADFLAGS\0",
        message: b"bad ai_flags value\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_BADHINTS,
        name: b"EAI_BADHINTS\0",
        message: b"invalid value for hints\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_CANCELED,
        name: b"EAI_CANCELED\0",
        message: b"request canceled\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_FAIL,
        name: b"EAI_FAIL\0",
        message: b"permanent failure\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_FAMILY,
        name: b"EAI_FAMILY\0",
        message: b"ai_family not supported\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_MEMORY,
        name: b"EAI_MEMORY\0",
        message: b"out of memory\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_NODATA,
        name: b"EAI_NODATA\0",
        message: b"no address\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_NONAME,
        name: b"EAI_NONAME\0",
        message: b"unknown node or service\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_OVERFLOW,
        name: b"EAI_OVERFLOW\0",
        message: b"argument buffer overflow\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_PROTOCOL,
        name: b"EAI_PROTOCOL\0",
        message: b"resolved protocol is unknown\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_SERVICE,
        name: b"EAI_SERVICE\0",
        message: b"service not available for socket type\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EAI_SOCKTYPE,
        name: b"EAI_SOCKTYPE\0",
        message: b"socket type not supported\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EALREADY,
        name: b"EALREADY\0",
        message: b"connection already in progress\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EBADF,
        name: b"EBADF\0",
        message: b"bad file descriptor\0",
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
        code: abi::uv_errno_t_UV_ECHARSET,
        name: b"ECHARSET\0",
        message: b"invalid Unicode character\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ECONNABORTED,
        name: b"ECONNABORTED\0",
        message: b"software caused connection abort\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ECONNREFUSED,
        name: b"ECONNREFUSED\0",
        message: b"connection refused\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ECONNRESET,
        name: b"ECONNRESET\0",
        message: b"connection reset by peer\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EDESTADDRREQ,
        name: b"EDESTADDRREQ\0",
        message: b"destination address required\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EEXIST,
        name: b"EEXIST\0",
        message: b"file already exists\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EFAULT,
        name: b"EFAULT\0",
        message: b"bad address in system call argument\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EFBIG,
        name: b"EFBIG\0",
        message: b"file too large\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EHOSTUNREACH,
        name: b"EHOSTUNREACH\0",
        message: b"host is unreachable\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EINTR,
        name: b"EINTR\0",
        message: b"interrupted system call\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EINVAL,
        name: b"EINVAL\0",
        message: b"invalid argument\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EIO,
        name: b"EIO\0",
        message: b"i/o error\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EISCONN,
        name: b"EISCONN\0",
        message: b"socket is already connected\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EISDIR,
        name: b"EISDIR\0",
        message: b"illegal operation on a directory\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ELOOP,
        name: b"ELOOP\0",
        message: b"too many symbolic links encountered\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EMFILE,
        name: b"EMFILE\0",
        message: b"too many open files\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EMSGSIZE,
        name: b"EMSGSIZE\0",
        message: b"message too long\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENAMETOOLONG,
        name: b"ENAMETOOLONG\0",
        message: b"name too long\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENETDOWN,
        name: b"ENETDOWN\0",
        message: b"network is down\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENETUNREACH,
        name: b"ENETUNREACH\0",
        message: b"network is unreachable\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENFILE,
        name: b"ENFILE\0",
        message: b"file table overflow\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOBUFS,
        name: b"ENOBUFS\0",
        message: b"no buffer space available\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENODEV,
        name: b"ENODEV\0",
        message: b"no such device\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOENT,
        name: b"ENOENT\0",
        message: b"no such file or directory\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOMEM,
        name: b"ENOMEM\0",
        message: b"not enough memory\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENONET,
        name: b"ENONET\0",
        message: b"machine is not on the network\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOPROTOOPT,
        name: b"ENOPROTOOPT\0",
        message: b"protocol not available\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOSPC,
        name: b"ENOSPC\0",
        message: b"no space left on device\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOSYS,
        name: b"ENOSYS\0",
        message: b"function not implemented\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOTCONN,
        name: b"ENOTCONN\0",
        message: b"socket is not connected\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOTDIR,
        name: b"ENOTDIR\0",
        message: b"not a directory\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOTEMPTY,
        name: b"ENOTEMPTY\0",
        message: b"directory not empty\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOTSOCK,
        name: b"ENOTSOCK\0",
        message: b"socket operation on non-socket\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOTSUP,
        name: b"ENOTSUP\0",
        message: b"operation not supported on socket\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EOVERFLOW,
        name: b"EOVERFLOW\0",
        message: b"value too large for defined data type\0",
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
        code: abi::uv_errno_t_UV_EPROTO,
        name: b"EPROTO\0",
        message: b"protocol error\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EPROTONOSUPPORT,
        name: b"EPROTONOSUPPORT\0",
        message: b"protocol not supported\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EPROTOTYPE,
        name: b"EPROTOTYPE\0",
        message: b"protocol wrong type for socket\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ERANGE,
        name: b"ERANGE\0",
        message: b"result too large\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EROFS,
        name: b"EROFS\0",
        message: b"read-only file system\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ESHUTDOWN,
        name: b"ESHUTDOWN\0",
        message: b"cannot send after transport endpoint shutdown\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ESPIPE,
        name: b"ESPIPE\0",
        message: b"invalid seek\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ESRCH,
        name: b"ESRCH\0",
        message: b"no such process\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ETIMEDOUT,
        name: b"ETIMEDOUT\0",
        message: b"connection timed out\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ETXTBSY,
        name: b"ETXTBSY\0",
        message: b"text file is busy\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EXDEV,
        name: b"EXDEV\0",
        message: b"cross-device link not permitted\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_UNKNOWN,
        name: b"UNKNOWN\0",
        message: b"unknown error\0",
    },
    ErrorEntry {
        code: abi::UV__EOF,
        name: b"EOF\0",
        message: b"end of file\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENXIO,
        name: b"ENXIO\0",
        message: b"no such device or address\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EMLINK,
        name: b"EMLINK\0",
        message: b"too many links\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EHOSTDOWN,
        name: b"EHOSTDOWN\0",
        message: b"host is down\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EREMOTEIO,
        name: b"EREMOTEIO\0",
        message: b"remote I/O error\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENOTTY,
        name: b"ENOTTY\0",
        message: b"inappropriate ioctl for device\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EFTYPE,
        name: b"EFTYPE\0",
        message: b"inappropriate file type or format\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EILSEQ,
        name: b"EILSEQ\0",
        message: b"illegal byte sequence\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ESOCKTNOSUPPORT,
        name: b"ESOCKTNOSUPPORT\0",
        message: b"socket type not supported\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_ENODATA,
        name: b"ENODATA\0",
        message: b"no data available\0",
    },
    ErrorEntry {
        code: abi::uv_errno_t_UV_EUNATCH,
        name: b"EUNATCH\0",
        message: b"protocol driver not attached\0",
    },
];

#[inline]
fn canonicalize(err: c_int) -> c_int {
    if err > 0 {
        -err
    } else {
        err
    }
}

fn lookup(err: c_int) -> Option<&'static ErrorEntry> {
    let err = canonicalize(err);
    ENTRIES.iter().find(|entry| entry.code == err)
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
fn copy_to_buffer(src: &[u8], buf: *mut c_char, buflen: usize) -> *mut c_char {
    unsafe {
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
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
fn alloc_unknown(err: c_int) -> *const c_char {
    unsafe {
        let text = format!("Unknown system error {err}");
        let bytes = text.as_bytes();
        let raw = unsafe { allocator::malloc_bytes(bytes.len() + 1) }.cast::<u8>();
        if raw.is_null() {
            return b"Unknown system error\0".as_ptr().cast();
        }

        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), raw, bytes.len());
            *raw.add(bytes.len()) = 0;
        }
        raw.cast()
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn err_name(err: c_int) -> *const c_char {
    unsafe {
        match lookup(err) {
            Some(entry) => entry.name.as_ptr().cast(),
            None => unsafe { alloc_unknown(err) },
        }
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn err_name_r(err: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char {
    unsafe {
        match lookup(err) {
            Some(entry) => unsafe { copy_to_buffer(entry.name, buf, buflen) },
            None => {
                let owned = format!("Unknown system error {err}\0");
                unsafe { copy_to_buffer(owned.as_bytes(), buf, buflen) }
            }
        }
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn strerror(err: c_int) -> *const c_char {
    unsafe {
        match lookup(err) {
            Some(entry) => entry.message.as_ptr().cast(),
            None => unsafe { alloc_unknown(err) },
        }
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn strerror_r(err: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char {
    unsafe {
        match lookup(err) {
            Some(entry) => unsafe { copy_to_buffer(entry.message, buf, buflen) },
            None => {
                let owned = format!("Unknown system error {err}\0");
                unsafe { copy_to_buffer(owned.as_bytes(), buf, buflen) }
            }
        }
    }
}
