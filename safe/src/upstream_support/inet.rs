extern "C" {
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn uv__strscpy(
        d: *mut ::core::ffi::c_char,
        s: *const ::core::ffi::c_char,
        n: size_t,
    ) -> ssize_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __ssize_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub type C2RustUnnamed_0 = ::core::ffi::c_int;
pub const UV_ERRNO_MAX: C2RustUnnamed_0 = -4096;
pub const UV_EUNATCH: C2RustUnnamed_0 = -49;
pub const UV_ENODATA: C2RustUnnamed_0 = -61;
pub const UV_ESOCKTNOSUPPORT: C2RustUnnamed_0 = -94;
pub const UV_EILSEQ: C2RustUnnamed_0 = -84;
pub const UV_EFTYPE: C2RustUnnamed_0 = -4028;
pub const UV_ENOTTY: C2RustUnnamed_0 = -25;
pub const UV_EREMOTEIO: C2RustUnnamed_0 = -121;
pub const UV_EHOSTDOWN: C2RustUnnamed_0 = -112;
pub const UV_EMLINK: C2RustUnnamed_0 = -31;
pub const UV_ENXIO: C2RustUnnamed_0 = -6;
pub const UV_EOF: C2RustUnnamed_0 = -4095;
pub const UV_UNKNOWN: C2RustUnnamed_0 = -4094;
pub const UV_EXDEV: C2RustUnnamed_0 = -18;
pub const UV_ETXTBSY: C2RustUnnamed_0 = -26;
pub const UV_ETIMEDOUT: C2RustUnnamed_0 = -110;
pub const UV_ESRCH: C2RustUnnamed_0 = -3;
pub const UV_ESPIPE: C2RustUnnamed_0 = -29;
pub const UV_ESHUTDOWN: C2RustUnnamed_0 = -108;
pub const UV_EROFS: C2RustUnnamed_0 = -30;
pub const UV_ERANGE: C2RustUnnamed_0 = -34;
pub const UV_EPROTOTYPE: C2RustUnnamed_0 = -91;
pub const UV_EPROTONOSUPPORT: C2RustUnnamed_0 = -93;
pub const UV_EPROTO: C2RustUnnamed_0 = -71;
pub const UV_EPIPE: C2RustUnnamed_0 = -32;
pub const UV_EPERM: C2RustUnnamed_0 = -1;
pub const UV_EOVERFLOW: C2RustUnnamed_0 = -75;
pub const UV_ENOTSUP: C2RustUnnamed_0 = -95;
pub const UV_ENOTSOCK: C2RustUnnamed_0 = -88;
pub const UV_ENOTEMPTY: C2RustUnnamed_0 = -39;
pub const UV_ENOTDIR: C2RustUnnamed_0 = -20;
pub const UV_ENOTCONN: C2RustUnnamed_0 = -107;
pub const UV_ENOSYS: C2RustUnnamed_0 = -38;
pub const UV_ENOSPC: C2RustUnnamed_0 = -28;
pub const UV_ENOPROTOOPT: C2RustUnnamed_0 = -92;
pub const UV_ENONET: C2RustUnnamed_0 = -64;
pub const UV_ENOMEM: C2RustUnnamed_0 = -12;
pub const UV_ENOENT: C2RustUnnamed_0 = -2;
pub const UV_ENODEV: C2RustUnnamed_0 = -19;
pub const UV_ENOBUFS: C2RustUnnamed_0 = -105;
pub const UV_ENFILE: C2RustUnnamed_0 = -23;
pub const UV_ENETUNREACH: C2RustUnnamed_0 = -101;
pub const UV_ENETDOWN: C2RustUnnamed_0 = -100;
pub const UV_ENAMETOOLONG: C2RustUnnamed_0 = -36;
pub const UV_EMSGSIZE: C2RustUnnamed_0 = -90;
pub const UV_EMFILE: C2RustUnnamed_0 = -24;
pub const UV_ELOOP: C2RustUnnamed_0 = -40;
pub const UV_EISDIR: C2RustUnnamed_0 = -21;
pub const UV_EISCONN: C2RustUnnamed_0 = -106;
pub const UV_EIO: C2RustUnnamed_0 = -5;
pub const UV_EINVAL: C2RustUnnamed_0 = -22;
pub const UV_EINTR: C2RustUnnamed_0 = -4;
pub const UV_EHOSTUNREACH: C2RustUnnamed_0 = -113;
pub const UV_EFBIG: C2RustUnnamed_0 = -27;
pub const UV_EFAULT: C2RustUnnamed_0 = -14;
pub const UV_EEXIST: C2RustUnnamed_0 = -17;
pub const UV_EDESTADDRREQ: C2RustUnnamed_0 = -89;
pub const UV_ECONNRESET: C2RustUnnamed_0 = -104;
pub const UV_ECONNREFUSED: C2RustUnnamed_0 = -111;
pub const UV_ECONNABORTED: C2RustUnnamed_0 = -103;
pub const UV_ECHARSET: C2RustUnnamed_0 = -4080;
pub const UV_ECANCELED: C2RustUnnamed_0 = -125;
pub const UV_EBUSY: C2RustUnnamed_0 = -16;
pub const UV_EBADF: C2RustUnnamed_0 = -9;
pub const UV_EALREADY: C2RustUnnamed_0 = -114;
pub const UV_EAI_SOCKTYPE: C2RustUnnamed_0 = -3011;
pub const UV_EAI_SERVICE: C2RustUnnamed_0 = -3010;
pub const UV_EAI_PROTOCOL: C2RustUnnamed_0 = -3014;
pub const UV_EAI_OVERFLOW: C2RustUnnamed_0 = -3009;
pub const UV_EAI_NONAME: C2RustUnnamed_0 = -3008;
pub const UV_EAI_NODATA: C2RustUnnamed_0 = -3007;
pub const UV_EAI_MEMORY: C2RustUnnamed_0 = -3006;
pub const UV_EAI_FAMILY: C2RustUnnamed_0 = -3005;
pub const UV_EAI_FAIL: C2RustUnnamed_0 = -3004;
pub const UV_EAI_CANCELED: C2RustUnnamed_0 = -3003;
pub const UV_EAI_BADHINTS: C2RustUnnamed_0 = -3013;
pub const UV_EAI_BADFLAGS: C2RustUnnamed_0 = -3002;
pub const UV_EAI_AGAIN: C2RustUnnamed_0 = -3001;
pub const UV_EAI_ADDRFAMILY: C2RustUnnamed_0 = -3000;
pub const UV_EAGAIN: C2RustUnnamed_0 = -11;
pub const UV_EAFNOSUPPORT: C2RustUnnamed_0 = -97;
pub const UV_EADDRNOTAVAIL: C2RustUnnamed_0 = -99;
pub const UV_EADDRINUSE: C2RustUnnamed_0 = -98;
pub const UV_EACCES: C2RustUnnamed_0 = -13;
pub const UV_E2BIG: C2RustUnnamed_0 = -7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub base: ::core::ffi::c_int,
    pub len: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PF_INET: ::core::ffi::c_int = 2;
pub const PF_INET6: ::core::ffi::c_int = 10;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const UV__INET6_ADDRSTRLEN: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub(crate) unsafe fn uv_inet_ntop(
    mut af: ::core::ffi::c_int,
    mut src: *const ::core::ffi::c_void,
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    match af {
        AF_INET => return inet_ntop4(src as *const ::core::ffi::c_uchar, dst, size),
        AF_INET6 => return inet_ntop6(src as *const ::core::ffi::c_uchar, dst, size),
        _ => return UV_EAFNOSUPPORT as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn inet_ntop4(
    mut src: *const ::core::ffi::c_uchar,
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    static mut fmt: [::core::ffi::c_char; 12] =
        unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"%u.%u.%u.%u\0") };
    let mut tmp: [::core::ffi::c_char; 16] = [0; 16];
    let mut l: ::core::ffi::c_int = 0;
    l = snprintf(
        &raw mut tmp as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
        &raw const fmt as *const ::core::ffi::c_char,
        *src.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *src.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *src.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        *src.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    );
    if l <= 0 as ::core::ffi::c_int || l as size_t >= size {
        return UV_ENOSPC as ::core::ffi::c_int;
    }
    uv__strscpy(dst, &raw mut tmp as *mut ::core::ffi::c_char, size);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn inet_ntop6(
    mut src: *const ::core::ffi::c_uchar,
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    let mut tmp: [::core::ffi::c_char; 46] = [0; 46];
    let mut tp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut best: C2RustUnnamed_1 = C2RustUnnamed_1 { base: 0, len: 0 };
    let mut cur: C2RustUnnamed_1 = C2RustUnnamed_1 { base: 0, len: 0 };
    let mut words: [::core::ffi::c_uint; 8] = [0; 8];
    let mut i: ::core::ffi::c_int = 0;
    memset(
        &raw mut words as *mut ::core::ffi::c_uint as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<[::core::ffi::c_uint; 8]>() as size_t,
    );
    i = 0 as ::core::ffi::c_int;
    while i < ::core::mem::size_of::<in6_addr>() as ::core::ffi::c_int {
        words[(i / 2 as ::core::ffi::c_int) as usize] |= ((*src.offset(i as isize)
            as ::core::ffi::c_int)
            << ((1 as ::core::ffi::c_int - i % 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int))
            as ::core::ffi::c_uint;
        i += 1;
    }
    best.base = -(1 as ::core::ffi::c_int);
    best.len = 0 as ::core::ffi::c_int;
    cur.base = -(1 as ::core::ffi::c_int);
    cur.len = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[::core::ffi::c_uint; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_uint>() as usize)
            as ::core::ffi::c_int
    {
        if words[i as usize] == 0 as ::core::ffi::c_uint {
            if cur.base == -(1 as ::core::ffi::c_int) {
                cur.base = i;
                cur.len = 1 as ::core::ffi::c_int;
            } else {
                cur.len += 1;
            }
        } else if cur.base != -(1 as ::core::ffi::c_int) {
            if best.base == -(1 as ::core::ffi::c_int) || cur.len > best.len {
                best = cur;
            }
            cur.base = -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    if cur.base != -(1 as ::core::ffi::c_int) {
        if best.base == -(1 as ::core::ffi::c_int) || cur.len > best.len {
            best = cur;
        }
    }
    if best.base != -(1 as ::core::ffi::c_int) && best.len < 2 as ::core::ffi::c_int {
        best.base = -(1 as ::core::ffi::c_int);
    }
    tp = &raw mut tmp as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i
        < (::core::mem::size_of::<[::core::ffi::c_uint; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_uint>() as usize)
            as ::core::ffi::c_int
    {
        if best.base != -(1 as ::core::ffi::c_int) && i >= best.base && i < best.base + best.len {
            if i == best.base {
                let fresh0 = tp;
                tp = tp.offset(1);
                *fresh0 = ':' as i32 as ::core::ffi::c_char;
            }
        } else {
            if i != 0 as ::core::ffi::c_int {
                let fresh1 = tp;
                tp = tp.offset(1);
                *fresh1 = ':' as i32 as ::core::ffi::c_char;
            }
            if i == 6 as ::core::ffi::c_int
                && best.base == 0 as ::core::ffi::c_int
                && (best.len == 6 as ::core::ffi::c_int
                    || best.len == 7 as ::core::ffi::c_int
                        && words[7 as ::core::ffi::c_int as usize] != 0x1 as ::core::ffi::c_uint
                    || best.len == 5 as ::core::ffi::c_int
                        && words[5 as ::core::ffi::c_int as usize] == 0xffff as ::core::ffi::c_uint)
            {
                let mut err: ::core::ffi::c_int = inet_ntop4(
                    src.offset(12 as ::core::ffi::c_int as isize),
                    tp,
                    (::core::mem::size_of::<[::core::ffi::c_char; 46]>() as size_t)
                        .wrapping_sub(tp.offset_from(&raw mut tmp as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long as size_t),
                );
                if err != 0 {
                    return err;
                }
                tp = tp.offset(strlen(tp) as isize);
                break;
            } else {
                tp = tp.offset(snprintf(
                    tp,
                    (::core::mem::size_of::<[::core::ffi::c_char; 46]>() as size_t)
                        .wrapping_sub(tp.offset_from(&raw mut tmp as *mut ::core::ffi::c_char)
                            as ::core::ffi::c_long as size_t),
                    b"%x\0" as *const u8 as *const ::core::ffi::c_char,
                    words[i as usize],
                ) as isize);
            }
        }
        i += 1;
    }
    if best.base != -(1 as ::core::ffi::c_int)
        && (best.base + best.len) as usize
            == (::core::mem::size_of::<[::core::ffi::c_uint; 8]>() as usize)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_uint>() as usize)
    {
        let fresh2 = tp;
        tp = tp.offset(1);
        *fresh2 = ':' as i32 as ::core::ffi::c_char;
    }
    let fresh3 = tp;
    tp = tp.offset(1);
    *fresh3 = '\0' as i32 as ::core::ffi::c_char;
    if tp.offset_from(&raw mut tmp as *mut ::core::ffi::c_char) as ::core::ffi::c_long as size_t
        > size
    {
        return UV_ENOSPC as ::core::ffi::c_int;
    }
    uv__strscpy(dst, &raw mut tmp as *mut ::core::ffi::c_char, size);
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_inet_pton(
    mut af: ::core::ffi::c_int,
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if src.is_null() || dst.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    match af {
        AF_INET => return inet_pton4(src, dst as *mut ::core::ffi::c_uchar),
        AF_INET6 => {
            let mut len: ::core::ffi::c_int = 0;
            let mut tmp: [::core::ffi::c_char; 46] = [0; 46];
            let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            s = src as *mut ::core::ffi::c_char;
            p = strchr(src, '%' as i32);
            if !p.is_null() {
                s = &raw mut tmp as *mut ::core::ffi::c_char;
                len = p.offset_from(src) as ::core::ffi::c_long as ::core::ffi::c_int;
                if len > UV__INET6_ADDRSTRLEN - 1 as ::core::ffi::c_int {
                    return UV_EINVAL as ::core::ffi::c_int;
                }
                memcpy(
                    s as *mut ::core::ffi::c_void,
                    src as *const ::core::ffi::c_void,
                    len as size_t,
                );
                *s.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
            }
            return inet_pton6(s, dst as *mut ::core::ffi::c_uchar);
        }
        _ => return UV_EAFNOSUPPORT as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn inet_pton4(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    static mut digits: [::core::ffi::c_char; 11] =
        unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"0123456789\0") };
    let mut saw_digit: ::core::ffi::c_int = 0;
    let mut octets: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut tmp: [::core::ffi::c_uchar; 4] = [0; 4];
    let mut tp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    saw_digit = 0 as ::core::ffi::c_int;
    octets = 0 as ::core::ffi::c_int;
    tp = &raw mut tmp as *mut ::core::ffi::c_uchar;
    *tp = 0 as ::core::ffi::c_uchar;
    loop {
        let fresh9 = src;
        src = src.offset(1);
        ch = *fresh9 as ::core::ffi::c_int;
        if !(ch != '\0' as i32) {
            break;
        }
        let mut pch: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        pch = strchr(&raw const digits as *const ::core::ffi::c_char, ch);
        if !pch.is_null() {
            let mut nw: ::core::ffi::c_uint =
                ((*tp as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as ::core::ffi::c_long
                    + pch.offset_from(&raw const digits as *const ::core::ffi::c_char)
                        as ::core::ffi::c_long) as ::core::ffi::c_uint;
            if saw_digit != 0 && *tp as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return UV_EINVAL as ::core::ffi::c_int;
            }
            if nw > 255 as ::core::ffi::c_uint {
                return UV_EINVAL as ::core::ffi::c_int;
            }
            *tp = nw as ::core::ffi::c_uchar;
            if saw_digit == 0 {
                octets += 1;
                if octets > 4 as ::core::ffi::c_int {
                    return UV_EINVAL as ::core::ffi::c_int;
                }
                saw_digit = 1 as ::core::ffi::c_int;
            }
        } else if ch == '.' as i32 && saw_digit != 0 {
            if octets == 4 as ::core::ffi::c_int {
                return UV_EINVAL as ::core::ffi::c_int;
            }
            tp = tp.offset(1);
            *tp = 0 as ::core::ffi::c_uchar;
            saw_digit = 0 as ::core::ffi::c_int;
        } else {
            return UV_EINVAL as ::core::ffi::c_int;
        }
    }
    if octets < 4 as ::core::ffi::c_int {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw mut tmp as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        ::core::mem::size_of::<in_addr>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn inet_pton6(
    mut src: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    static mut xdigits_l: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789abcdef\0")
    };
    static mut xdigits_u: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789ABCDEF\0")
    };
    let mut tmp: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut tp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut endp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut colonp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut xdigits: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut curtok: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ch: ::core::ffi::c_int = 0;
    let mut seen_xdigits: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_uint = 0;
    tp = &raw mut tmp as *mut ::core::ffi::c_uchar;
    memset(
        tp as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as size_t,
    );
    endp = tp.offset(::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as usize as isize);
    colonp = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if *src as ::core::ffi::c_int == ':' as i32 {
        src = src.offset(1);
        if *src as ::core::ffi::c_int != ':' as i32 {
            return UV_EINVAL as ::core::ffi::c_int;
        }
    }
    curtok = src;
    seen_xdigits = 0 as ::core::ffi::c_int;
    val = 0 as ::core::ffi::c_uint;
    loop {
        let fresh4 = src;
        src = src.offset(1);
        ch = *fresh4 as ::core::ffi::c_int;
        if !(ch != '\0' as i32) {
            break;
        }
        let mut pch: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        xdigits = &raw const xdigits_l as *const ::core::ffi::c_char;
        pch = strchr(xdigits, ch);
        if pch.is_null() {
            xdigits = &raw const xdigits_u as *const ::core::ffi::c_char;
            pch = strchr(xdigits, ch);
        }
        if !pch.is_null() {
            val <<= 4 as ::core::ffi::c_int;
            val = (val as ::core::ffi::c_long | pch.offset_from(xdigits) as ::core::ffi::c_long)
                as ::core::ffi::c_uint;
            seen_xdigits += 1;
            if seen_xdigits > 4 as ::core::ffi::c_int {
                return UV_EINVAL as ::core::ffi::c_int;
            }
        } else if ch == ':' as i32 {
            curtok = src;
            if seen_xdigits == 0 {
                if !colonp.is_null() {
                    return UV_EINVAL as ::core::ffi::c_int;
                }
                colonp = tp;
            } else {
                if *src as ::core::ffi::c_int == '\0' as i32 {
                    return UV_EINVAL as ::core::ffi::c_int;
                }
                if tp.offset(::core::mem::size_of::<uint16_t>() as usize as isize) > endp {
                    return UV_EINVAL as ::core::ffi::c_int;
                }
                let fresh5 = tp;
                tp = tp.offset(1);
                *fresh5 = ((val >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar
                    as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                let fresh6 = tp;
                tp = tp.offset(1);
                *fresh6 = (val as ::core::ffi::c_uchar as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
                seen_xdigits = 0 as ::core::ffi::c_int;
                val = 0 as ::core::ffi::c_uint;
            }
        } else {
            if ch == '.' as i32
                && tp.offset(::core::mem::size_of::<in_addr>() as usize as isize) <= endp
            {
                let mut err: ::core::ffi::c_int = inet_pton4(curtok, tp);
                if err == 0 as ::core::ffi::c_int {
                    tp = tp.offset(::core::mem::size_of::<in_addr>() as usize as isize);
                    seen_xdigits = 0 as ::core::ffi::c_int;
                    break;
                }
            }
            return UV_EINVAL as ::core::ffi::c_int;
        }
    }
    if seen_xdigits != 0 {
        if tp.offset(::core::mem::size_of::<uint16_t>() as usize as isize) > endp {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        let fresh7 = tp;
        tp = tp.offset(1);
        *fresh7 = ((val >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        let fresh8 = tp;
        tp = tp.offset(1);
        *fresh8 = (val as ::core::ffi::c_uchar as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
    }
    if !colonp.is_null() {
        let n: ::core::ffi::c_int =
            tp.offset_from(colonp) as ::core::ffi::c_long as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0;
        if tp == endp {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        i = 1 as ::core::ffi::c_int;
        while i <= n {
            *endp.offset(-i as isize) = *colonp.offset((n - i) as isize);
            *colonp.offset((n - i) as isize) = 0 as ::core::ffi::c_uchar;
            i += 1;
        }
        tp = endp;
    }
    if tp != endp {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    memcpy(
        dst as *mut ::core::ffi::c_void,
        &raw mut tmp as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 16]>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
