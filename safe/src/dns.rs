use crate::allocator::{last_error, UV_EINVAL};
use crate::bindings::*;

unsafe extern "C" {
    fn getaddrinfo(
        node: *const libc::c_char,
        service: *const libc::c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> libc::c_int;
}

unsafe fn translate_eai_error(sys_err: libc::c_int) -> libc::c_int {
    match sys_err {
        0 => 0,
        x if x == libc::EAI_AGAIN => uv_errno_t_UV_EAI_AGAIN,
        x if x == libc::EAI_BADFLAGS => uv_errno_t_UV_EAI_BADFLAGS,
        x if x == libc::EAI_FAIL => uv_errno_t_UV_EAI_FAIL,
        x if x == libc::EAI_FAMILY => uv_errno_t_UV_EAI_FAMILY,
        x if x == libc::EAI_MEMORY => uv_errno_t_UV_EAI_MEMORY,
        #[cfg(any(target_os = "linux", target_os = "android"))]
        x if x == libc::EAI_NODATA => uv_errno_t_UV_EAI_NODATA,
        x if x == libc::EAI_NONAME => uv_errno_t_UV_EAI_NONAME,
        #[cfg(any(target_os = "linux", target_os = "android"))]
        x if x == libc::EAI_OVERFLOW => uv_errno_t_UV_EAI_OVERFLOW,
        x if x == libc::EAI_SERVICE => uv_errno_t_UV_EAI_SERVICE,
        x if x == libc::EAI_SOCKTYPE => uv_errno_t_UV_EAI_SOCKTYPE,
        x if x == libc::EAI_SYSTEM => last_error(),
        _ => last_error(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_getaddrinfo(
    loop_: *mut uv_loop_t,
    req: *mut uv_getaddrinfo_t,
    cb: uv_getaddrinfo_cb,
    node: *const libc::c_char,
    service: *const libc::c_char,
    hints: *const addrinfo,
) -> libc::c_int {
    if cb.is_some() || !crate::idna::is_long_ascii_hostname(node) {
        return crate::legacy::uv_getaddrinfo(loop_, req, cb, node, service, hints);
    }

    if req.is_null() || (node.is_null() && service.is_null()) {
        return UV_EINVAL;
    }

    (*req).type_ = uv_req_type_UV_GETADDRINFO;
    (*req).loop_ = loop_;
    (*req).cb = cb;
    (*req).hints = std::ptr::null_mut();
    (*req).hostname = std::ptr::null_mut();
    (*req).service = std::ptr::null_mut();
    (*req).addrinfo = std::ptr::null_mut();
    (*req).retcode = 0;

    let rc = getaddrinfo(node, service, hints, std::ptr::addr_of_mut!((*req).addrinfo));
    let translated = translate_eai_error(rc);
    (*req).retcode = translated;
    if translated != 0 {
        (*req).addrinfo = std::ptr::null_mut();
    }

    translated
}
