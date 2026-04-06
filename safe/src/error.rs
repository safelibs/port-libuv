use crate::allocator;
use crate::bindings::*;

unsafe fn lookup_error(err: libc::c_int) -> Option<(&'static [u8], &'static [u8])> {
    Some(match err {
        uv_errno_t_UV_E2BIG => (b"E2BIG\0", b"argument list too long\0"),
        uv_errno_t_UV_EACCES => (b"EACCES\0", b"permission denied\0"),
        uv_errno_t_UV_EADDRINUSE => (b"EADDRINUSE\0", b"address already in use\0"),
        uv_errno_t_UV_EADDRNOTAVAIL => (b"EADDRNOTAVAIL\0", b"address not available\0"),
        uv_errno_t_UV_EAFNOSUPPORT => (b"EAFNOSUPPORT\0", b"address family not supported\0"),
        uv_errno_t_UV_EAGAIN => (b"EAGAIN\0", b"resource temporarily unavailable\0"),
        UV__EAI_ADDRFAMILY => (b"EAI_ADDRFAMILY\0", b"address family not supported\0"),
        UV__EAI_AGAIN => (b"EAI_AGAIN\0", b"temporary failure\0"),
        UV__EAI_BADFLAGS => (b"EAI_BADFLAGS\0", b"bad ai_flags value\0"),
        UV__EAI_BADHINTS => (b"EAI_BADHINTS\0", b"invalid value for hints\0"),
        UV__EAI_CANCELED => (b"EAI_CANCELED\0", b"request canceled\0"),
        UV__EAI_FAIL => (b"EAI_FAIL\0", b"permanent failure\0"),
        UV__EAI_FAMILY => (b"EAI_FAMILY\0", b"ai_family not supported\0"),
        UV__EAI_MEMORY => (b"EAI_MEMORY\0", b"out of memory\0"),
        UV__EAI_NODATA => (b"EAI_NODATA\0", b"no address\0"),
        UV__EAI_NONAME => (b"EAI_NONAME\0", b"unknown node or service\0"),
        UV__EAI_OVERFLOW => (b"EAI_OVERFLOW\0", b"argument buffer overflow\0"),
        UV__EAI_PROTOCOL => (b"EAI_PROTOCOL\0", b"resolved protocol is unknown\0"),
        UV__EAI_SERVICE => (b"EAI_SERVICE\0", b"service not available for socket type\0"),
        UV__EAI_SOCKTYPE => (b"EAI_SOCKTYPE\0", b"socket type not supported\0"),
        uv_errno_t_UV_EALREADY => (b"EALREADY\0", b"connection already in progress\0"),
        uv_errno_t_UV_EBADF => (b"EBADF\0", b"bad file descriptor\0"),
        uv_errno_t_UV_EBUSY => (b"EBUSY\0", b"resource busy or locked\0"),
        uv_errno_t_UV_ECANCELED => (b"ECANCELED\0", b"operation canceled\0"),
        UV__ECHARSET => (b"ECHARSET\0", b"invalid Unicode character\0"),
        uv_errno_t_UV_ECONNABORTED => (b"ECONNABORTED\0", b"software caused connection abort\0"),
        uv_errno_t_UV_ECONNREFUSED => (b"ECONNREFUSED\0", b"connection refused\0"),
        uv_errno_t_UV_ECONNRESET => (b"ECONNRESET\0", b"connection reset by peer\0"),
        uv_errno_t_UV_EDESTADDRREQ => (b"EDESTADDRREQ\0", b"destination address required\0"),
        uv_errno_t_UV_EEXIST => (b"EEXIST\0", b"file already exists\0"),
        uv_errno_t_UV_EFAULT => (b"EFAULT\0", b"bad address in system call argument\0"),
        uv_errno_t_UV_EFBIG => (b"EFBIG\0", b"file too large\0"),
        uv_errno_t_UV_EHOSTUNREACH => (b"EHOSTUNREACH\0", b"host is unreachable\0"),
        uv_errno_t_UV_EINTR => (b"EINTR\0", b"interrupted system call\0"),
        uv_errno_t_UV_EINVAL => (b"EINVAL\0", b"invalid argument\0"),
        uv_errno_t_UV_EIO => (b"EIO\0", b"i/o error\0"),
        uv_errno_t_UV_EISCONN => (b"EISCONN\0", b"socket is already connected\0"),
        uv_errno_t_UV_EISDIR => (b"EISDIR\0", b"illegal operation on a directory\0"),
        uv_errno_t_UV_ELOOP => (b"ELOOP\0", b"too many symbolic links encountered\0"),
        uv_errno_t_UV_EMFILE => (b"EMFILE\0", b"too many open files\0"),
        uv_errno_t_UV_EMSGSIZE => (b"EMSGSIZE\0", b"message too long\0"),
        uv_errno_t_UV_ENAMETOOLONG => (b"ENAMETOOLONG\0", b"name too long\0"),
        uv_errno_t_UV_ENETDOWN => (b"ENETDOWN\0", b"network is down\0"),
        uv_errno_t_UV_ENETUNREACH => (b"ENETUNREACH\0", b"network is unreachable\0"),
        uv_errno_t_UV_ENFILE => (b"ENFILE\0", b"file table overflow\0"),
        uv_errno_t_UV_ENOBUFS => (b"ENOBUFS\0", b"no buffer space available\0"),
        uv_errno_t_UV_ENODEV => (b"ENODEV\0", b"no such device\0"),
        uv_errno_t_UV_ENOENT => (b"ENOENT\0", b"no such file or directory\0"),
        uv_errno_t_UV_ENOMEM => (b"ENOMEM\0", b"not enough memory\0"),
        uv_errno_t_UV_ENONET => (b"ENONET\0", b"machine is not on the network\0"),
        uv_errno_t_UV_ENOPROTOOPT => (b"ENOPROTOOPT\0", b"protocol not available\0"),
        uv_errno_t_UV_ENOSPC => (b"ENOSPC\0", b"no space left on device\0"),
        uv_errno_t_UV_ENOSYS => (b"ENOSYS\0", b"function not implemented\0"),
        uv_errno_t_UV_ENOTCONN => (b"ENOTCONN\0", b"socket is not connected\0"),
        uv_errno_t_UV_ENOTDIR => (b"ENOTDIR\0", b"not a directory\0"),
        uv_errno_t_UV_ENOTEMPTY => (b"ENOTEMPTY\0", b"directory not empty\0"),
        uv_errno_t_UV_ENOTSOCK => (b"ENOTSOCK\0", b"socket operation on non-socket\0"),
        uv_errno_t_UV_ENOTSUP => (b"ENOTSUP\0", b"operation not supported on socket\0"),
        uv_errno_t_UV_EOVERFLOW => (b"EOVERFLOW\0", b"value too large for defined data type\0"),
        uv_errno_t_UV_EPERM => (b"EPERM\0", b"operation not permitted\0"),
        uv_errno_t_UV_EPIPE => (b"EPIPE\0", b"broken pipe\0"),
        uv_errno_t_UV_EPROTO => (b"EPROTO\0", b"protocol error\0"),
        uv_errno_t_UV_EPROTONOSUPPORT => (b"EPROTONOSUPPORT\0", b"protocol not supported\0"),
        uv_errno_t_UV_EPROTOTYPE => (b"EPROTOTYPE\0", b"protocol wrong type for socket\0"),
        uv_errno_t_UV_ERANGE => (b"ERANGE\0", b"result too large\0"),
        uv_errno_t_UV_EROFS => (b"EROFS\0", b"read-only file system\0"),
        uv_errno_t_UV_ESHUTDOWN => (
            b"ESHUTDOWN\0",
            b"cannot send after transport endpoint shutdown\0",
        ),
        uv_errno_t_UV_ESPIPE => (b"ESPIPE\0", b"invalid seek\0"),
        uv_errno_t_UV_ESRCH => (b"ESRCH\0", b"no such process\0"),
        uv_errno_t_UV_ETIMEDOUT => (b"ETIMEDOUT\0", b"connection timed out\0"),
        uv_errno_t_UV_ETXTBSY => (b"ETXTBSY\0", b"text file is busy\0"),
        uv_errno_t_UV_EXDEV => (b"EXDEV\0", b"cross-device link not permitted\0"),
        uv_errno_t_UV_UNKNOWN => (b"UNKNOWN\0", b"unknown error\0"),
        uv_errno_t_UV_EOF => (b"EOF\0", b"end of file\0"),
        uv_errno_t_UV_ENXIO => (b"ENXIO\0", b"no such device or address\0"),
        uv_errno_t_UV_EMLINK => (b"EMLINK\0", b"too many links\0"),
        uv_errno_t_UV_EHOSTDOWN => (b"EHOSTDOWN\0", b"host is down\0"),
        uv_errno_t_UV_EREMOTEIO => (b"EREMOTEIO\0", b"remote I/O error\0"),
        uv_errno_t_UV_ENOTTY => (b"ENOTTY\0", b"inappropriate ioctl for device\0"),
        UV__EFTYPE => (b"EFTYPE\0", b"inappropriate file type or format\0"),
        uv_errno_t_UV_EILSEQ => (b"EILSEQ\0", b"illegal byte sequence\0"),
        uv_errno_t_UV_ESOCKTNOSUPPORT => (b"ESOCKTNOSUPPORT\0", b"socket type not supported\0"),
        uv_errno_t_UV_ENODATA => (b"ENODATA\0", b"no data available\0"),
        uv_errno_t_UV_EUNATCH => (b"EUNATCH\0", b"protocol driver not attached\0"),
        _ => return None,
    })
}

unsafe fn unknown_error_ptr(err: libc::c_int) -> *const libc::c_char {
    let mut buf = [0i8; 32];
    libc::snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        b"Unknown system error %d\0".as_ptr().cast(),
        err,
    );
    let copy = allocator::strdup(buf.as_ptr());
    if copy.is_null() {
        b"Unknown system error\0".as_ptr().cast()
    } else {
        copy.cast()
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_err_name(err: libc::c_int) -> *const libc::c_char {
    lookup_error(err)
        .map(|(name, _)| name.as_ptr().cast())
        .unwrap_or_else(|| unknown_error_ptr(err))
}

#[no_mangle]
pub unsafe extern "C" fn uv_err_name_r(
    err: libc::c_int,
    buf: *mut libc::c_char,
    buflen: usize,
) -> *mut libc::c_char {
    if let Some((name, _)) = lookup_error(err) {
        crate::strings::uv__strscpy(buf, name.as_ptr().cast(), buflen);
    } else {
        libc::snprintf(
            buf,
            buflen,
            b"Unknown system error %d\0".as_ptr().cast(),
            err,
        );
    }
    buf
}

#[no_mangle]
pub unsafe extern "C" fn uv_strerror(err: libc::c_int) -> *const libc::c_char {
    lookup_error(err)
        .map(|(_, message)| message.as_ptr().cast())
        .unwrap_or_else(|| unknown_error_ptr(err))
}

#[no_mangle]
pub unsafe extern "C" fn uv_strerror_r(
    err: libc::c_int,
    buf: *mut libc::c_char,
    buflen: usize,
) -> *mut libc::c_char {
    if let Some((_, message)) = lookup_error(err) {
        crate::strings::uv__strscpy(buf, message.as_ptr().cast(), buflen);
    } else {
        libc::snprintf(
            buf,
            buflen,
            b"Unknown system error %d\0".as_ptr().cast(),
            err,
        );
    }
    buf
}
