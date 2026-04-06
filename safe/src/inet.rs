use crate::allocator::{self, last_error, UV_EAFNOSUPPORT, UV_EINVAL, UV_ENOSPC};
use crate::bindings::*;
use std::ptr;

unsafe extern "C" {
    fn inet_ntop(
        af: libc::c_int,
        src: *const libc::c_void,
        dst: *mut libc::c_char,
        size: libc::socklen_t,
    ) -> *const libc::c_char;
    fn inet_pton(af: libc::c_int, src: *const libc::c_char, dst: *mut libc::c_void) -> libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn uv_inet_ntop(
    af: libc::c_int,
    src: *const libc::c_void,
    dst: *mut libc::c_char,
    size: usize,
) -> libc::c_int {
    match af {
        libc::AF_INET | libc::AF_INET6 => {}
        _ => return UV_EAFNOSUPPORT,
    }

    if inet_ntop(af, src, dst, size as libc::socklen_t).is_null() {
        if *allocator::errno_location() == libc::ENOSPC {
            UV_ENOSPC
        } else {
            last_error()
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_inet_pton(
    af: libc::c_int,
    src: *const libc::c_char,
    dst: *mut libc::c_void,
) -> libc::c_int {
    if src.is_null() || dst.is_null() {
        return UV_EINVAL;
    }

    let mut scoped_buf = [0i8; 46];
    let actual_src = if af == libc::AF_INET6 {
        let percent = libc::strchr(src, '%' as libc::c_int);
        if percent.is_null() {
            src
        } else {
            let len = percent.offset_from(src) as usize;
            if len > scoped_buf.len() - 1 {
                return UV_EINVAL;
            }
            ptr::copy_nonoverlapping(src, scoped_buf.as_mut_ptr(), len);
            scoped_buf[len] = 0;
            scoped_buf.as_ptr()
        }
    } else {
        src
    };

    match af {
        libc::AF_INET | libc::AF_INET6 => {}
        _ => return UV_EAFNOSUPPORT,
    }

    match inet_pton(af, actual_src, dst) {
        1 => 0,
        0 => UV_EINVAL,
        _ => last_error(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_ip4_addr(
    ip: *const libc::c_char,
    port: libc::c_int,
    addr: *mut sockaddr_in,
) -> libc::c_int {
    ptr::write_bytes(addr, 0, 1);
    (*addr).sin_family = libc::AF_INET as sa_family_t;
    (*addr).sin_port = libc::htons(port as u16);
    uv_inet_pton(
        libc::AF_INET,
        ip,
        (&mut (*addr).sin_addr.s_addr as *mut in_addr_t).cast(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn uv_ip6_addr(
    ip: *const libc::c_char,
    port: libc::c_int,
    addr: *mut sockaddr_in6,
) -> libc::c_int {
    let mut address_part = [0i8; 40];

    ptr::write_bytes(addr, 0, 1);
    (*addr).sin6_family = libc::AF_INET6 as sa_family_t;
    (*addr).sin6_port = libc::htons(port as u16);

    let mut actual_ip = ip;
    let zone_index = libc::strchr(ip, '%' as libc::c_int);
    if !zone_index.is_null() {
        let mut address_part_size = zone_index.offset_from(ip) as usize;
        if address_part_size >= address_part.len() {
            address_part_size = address_part.len() - 1;
        }

        ptr::copy_nonoverlapping(ip, address_part.as_mut_ptr(), address_part_size);
        address_part[address_part_size] = 0;
        actual_ip = address_part.as_ptr();

        (*addr).sin6_scope_id = libc::if_nametoindex(zone_index.add(1));
    }

    uv_inet_pton(
        libc::AF_INET6,
        actual_ip,
        (&mut (*addr).sin6_addr as *mut in6_addr).cast(),
    )
}

#[no_mangle]
pub unsafe extern "C" fn uv_ip4_name(
    src: *const sockaddr_in,
    dst: *mut libc::c_char,
    size: usize,
) -> libc::c_int {
    uv_inet_ntop(
        libc::AF_INET,
        (&(*src).sin_addr as *const in_addr).cast(),
        dst,
        size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn uv_ip6_name(
    src: *const sockaddr_in6,
    dst: *mut libc::c_char,
    size: usize,
) -> libc::c_int {
    uv_inet_ntop(
        libc::AF_INET6,
        (&(*src).sin6_addr as *const in6_addr).cast(),
        dst,
        size,
    )
}

#[no_mangle]
pub unsafe extern "C" fn uv_ip_name(
    src: *const sockaddr,
    dst: *mut libc::c_char,
    size: usize,
) -> libc::c_int {
    match (*src).sa_family as libc::c_int {
        libc::AF_INET => uv_inet_ntop(
            libc::AF_INET,
            (&(*(src as *const sockaddr_in)).sin_addr as *const in_addr).cast(),
            dst,
            size,
        ),
        libc::AF_INET6 => uv_inet_ntop(
            libc::AF_INET6,
            (&(*(src as *const sockaddr_in6)).sin6_addr as *const in6_addr).cast(),
            dst,
            size,
        ),
        _ => UV_EAFNOSUPPORT,
    }
}
