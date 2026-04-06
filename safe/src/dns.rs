use crate::allocator::{last_error, UV_EINVAL, UV_ENOMEM};
use crate::bindings::*;
use crate::request::{req_register, req_unregister};
use crate::threadpool::{self, WorkKind};
use std::mem::offset_of;

unsafe extern "C" {
    fn getaddrinfo(
        node: *const libc::c_char,
        service: *const libc::c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn getnameinfo(
        addr: *const sockaddr,
        addrlen: libc::socklen_t,
        host: *mut libc::c_char,
        hostlen: libc::socklen_t,
        service: *mut libc::c_char,
        servicelen: libc::socklen_t,
        flags: libc::c_int,
    ) -> libc::c_int;
}

#[inline]
unsafe fn gai_req_from_work(w: *mut uv__work) -> *mut uv_getaddrinfo_t {
    w.cast::<u8>()
        .sub(offset_of!(uv_getaddrinfo_t, work_req))
        .cast::<uv_getaddrinfo_t>()
}

#[inline]
unsafe fn gni_req_from_work(w: *mut uv__work) -> *mut uv_getnameinfo_t {
    w.cast::<u8>()
        .sub(offset_of!(uv_getnameinfo_t, work_req))
        .cast::<uv_getnameinfo_t>()
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

unsafe fn initialize_request(
    loop_: *mut uv_loop_t,
    req: *mut uv_getaddrinfo_t,
    cb: uv_getaddrinfo_cb,
    hostname: Option<&std::ffi::CString>,
    service: *const libc::c_char,
    hints: *const addrinfo,
) -> libc::c_int {
    let hostname_len = hostname
        .as_ref()
        .map(|value| value.as_bytes_with_nul().len())
        .unwrap_or(0);
    let service_len = if service.is_null() {
        0
    } else {
        libc::strlen(service) + 1
    };
    let hints_len = if hints.is_null() {
        0
    } else {
        std::mem::size_of::<addrinfo>()
    };
    let total_len = hostname_len + service_len + hints_len;
    let buf = if total_len == 0 {
        std::ptr::null_mut()
    } else {
        crate::allocator::malloc(total_len).cast::<u8>()
    };

    if total_len != 0 && buf.is_null() {
        return UV_ENOMEM;
    }

    (*req).type_ = uv_req_type_UV_GETADDRINFO;
    (*req).loop_ = loop_;
    (*req).cb = cb;
    (*req).addrinfo = std::ptr::null_mut();
    (*req).hints = std::ptr::null_mut();
    (*req).service = std::ptr::null_mut();
    (*req).hostname = std::ptr::null_mut();
    (*req).retcode = 0;

    let mut len = 0usize;
    if !hints.is_null() {
        (*req).hints = buf.add(len).cast::<addrinfo>();
        std::ptr::copy_nonoverlapping(hints, (*req).hints, 1);
        len += std::mem::size_of::<addrinfo>();
    }
    if !service.is_null() {
        (*req).service = buf.add(len).cast::<libc::c_char>();
        std::ptr::copy_nonoverlapping(
            service.cast::<u8>(),
            (*req).service.cast::<u8>(),
            service_len,
        );
        len += service_len;
    }
    if let Some(hostname) = hostname {
        (*req).hostname = buf.add(len).cast::<libc::c_char>();
        std::ptr::copy_nonoverlapping(
            hostname.as_ptr().cast::<u8>(),
            (*req).hostname.cast::<u8>(),
            hostname_len,
        );
    }

    0
}

unsafe extern "C" fn uv__getaddrinfo_work(w: *mut uv__work) {
    let req = gai_req_from_work(w);
    let rc = getaddrinfo(
        (*req).hostname,
        (*req).service,
        (*req).hints,
        std::ptr::addr_of_mut!((*req).addrinfo),
    );
    (*req).retcode = translate_eai_error(rc);
    if (*req).retcode != 0 {
        (*req).addrinfo = std::ptr::null_mut();
    }
}

unsafe extern "C" fn uv__getaddrinfo_done(w: *mut uv__work, status: libc::c_int) {
    let req = gai_req_from_work(w);
    req_unregister((*req).loop_, req.cast());

    if !(*req).hints.is_null() {
        crate::allocator::free((*req).hints.cast());
    } else if !(*req).service.is_null() {
        crate::allocator::free((*req).service.cast());
    } else if !(*req).hostname.is_null() {
        crate::allocator::free((*req).hostname.cast());
    }

    (*req).hints = std::ptr::null_mut();
    (*req).service = std::ptr::null_mut();
    (*req).hostname = std::ptr::null_mut();

    if status == uv_errno_t_UV_ECANCELED {
        (*req).retcode = uv_errno_t_UV_EAI_CANCELED;
        (*req).addrinfo = std::ptr::null_mut();
    }

    if let Some(cb) = (*req).cb {
        cb(req, (*req).retcode, (*req).addrinfo);
    }
}

unsafe extern "C" fn uv__getnameinfo_work(w: *mut uv__work) {
    let req = gni_req_from_work(w);
    let salen = match (*req).storage.ss_family as libc::c_int {
        libc::AF_INET => std::mem::size_of::<sockaddr_in>() as libc::socklen_t,
        libc::AF_INET6 => std::mem::size_of::<sockaddr_in6>() as libc::socklen_t,
        _ => libc::abort(),
    };

    let rc = getnameinfo(
        std::ptr::addr_of!((*req).storage).cast(),
        salen,
        (*req).host.as_mut_ptr(),
        (*req).host.len() as libc::socklen_t,
        (*req).service.as_mut_ptr(),
        (*req).service.len() as libc::socklen_t,
        (*req).flags,
    );
    (*req).retcode = translate_eai_error(rc);
}

unsafe extern "C" fn uv__getnameinfo_done(w: *mut uv__work, status: libc::c_int) {
    let req = gni_req_from_work(w);
    req_unregister((*req).loop_, req.cast());

    let mut host = std::ptr::null();
    let mut service = std::ptr::null();

    if status == uv_errno_t_UV_ECANCELED {
        (*req).retcode = uv_errno_t_UV_EAI_CANCELED;
    } else if (*req).retcode == 0 {
        host = (*req).host.as_ptr();
        service = (*req).service.as_ptr();
    }

    if let Some(cb) = (*req).getnameinfo_cb {
        cb(req, (*req).retcode, host, service);
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
    if req.is_null() || (node.is_null() && service.is_null()) {
        return UV_EINVAL;
    }
    if loop_.is_null() {
        return UV_EINVAL;
    }

    let hostname = match crate::idna::to_ascii_hostname(node) {
        Ok(hostname) => hostname,
        Err(err) => return err,
    };

    let rc = initialize_request(loop_, req, cb, hostname.as_ref(), service, hints);
    if rc != 0 {
        return rc;
    }

    req_register(loop_, req.cast());
    if cb.is_some() {
        threadpool::uv__work_submit(
            loop_,
            std::ptr::addr_of_mut!((*req).work_req),
            WorkKind::SlowIo,
            Some(uv__getaddrinfo_work),
            Some(uv__getaddrinfo_done),
        );
        0
    } else {
        uv__getaddrinfo_work(std::ptr::addr_of_mut!((*req).work_req));
        uv__getaddrinfo_done(std::ptr::addr_of_mut!((*req).work_req), 0);
        (*req).retcode
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_getnameinfo(
    loop_: *mut uv_loop_t,
    req: *mut uv_getnameinfo_t,
    cb: uv_getnameinfo_cb,
    addr: *const sockaddr,
    flags: libc::c_int,
) -> libc::c_int {
    if req.is_null() || addr.is_null() {
        return UV_EINVAL;
    }
    if cb.is_some() && loop_.is_null() {
        return UV_EINVAL;
    }

    std::ptr::write_bytes((*req).host.as_mut_ptr(), 0, (*req).host.len());
    std::ptr::write_bytes((*req).service.as_mut_ptr(), 0, (*req).service.len());
    match (*addr).sa_family as libc::c_int {
        libc::AF_INET => {
            std::ptr::copy_nonoverlapping(
                addr.cast::<sockaddr_in>(),
                std::ptr::addr_of_mut!((*req).storage).cast::<sockaddr_in>(),
                1,
            );
        }
        libc::AF_INET6 => {
            std::ptr::copy_nonoverlapping(
                addr.cast::<sockaddr_in6>(),
                std::ptr::addr_of_mut!((*req).storage).cast::<sockaddr_in6>(),
                1,
            );
        }
        _ => return UV_EINVAL,
    }

    (*req).type_ = uv_req_type_UV_GETNAMEINFO;
    (*req).loop_ = loop_;
    (*req).getnameinfo_cb = cb;
    (*req).flags = flags;
    (*req).retcode = 0;

    if cb.is_some() {
        req_register(loop_, req.cast());
        threadpool::uv__work_submit(
            loop_,
            std::ptr::addr_of_mut!((*req).work_req),
            WorkKind::SlowIo,
            Some(uv__getnameinfo_work),
            Some(uv__getnameinfo_done),
        );
        0
    } else {
        uv__getnameinfo_work(std::ptr::addr_of_mut!((*req).work_req));
        (*req).retcode
    }
}
