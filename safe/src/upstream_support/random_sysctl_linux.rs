extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
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
}
pub type size_t = usize;
pub type C2RustUnnamed = ::core::ffi::c_int;
pub const UV_ERRNO_MAX: C2RustUnnamed = -4096;
pub const UV_EUNATCH: C2RustUnnamed = -49;
pub const UV_ENODATA: C2RustUnnamed = -61;
pub const UV_ESOCKTNOSUPPORT: C2RustUnnamed = -94;
pub const UV_EILSEQ: C2RustUnnamed = -84;
pub const UV_EFTYPE: C2RustUnnamed = -4028;
pub const UV_ENOTTY: C2RustUnnamed = -25;
pub const UV_EREMOTEIO: C2RustUnnamed = -121;
pub const UV_EHOSTDOWN: C2RustUnnamed = -112;
pub const UV_EMLINK: C2RustUnnamed = -31;
pub const UV_ENXIO: C2RustUnnamed = -6;
pub const UV_EOF: C2RustUnnamed = -4095;
pub const UV_UNKNOWN: C2RustUnnamed = -4094;
pub const UV_EXDEV: C2RustUnnamed = -18;
pub const UV_ETXTBSY: C2RustUnnamed = -26;
pub const UV_ETIMEDOUT: C2RustUnnamed = -110;
pub const UV_ESRCH: C2RustUnnamed = -3;
pub const UV_ESPIPE: C2RustUnnamed = -29;
pub const UV_ESHUTDOWN: C2RustUnnamed = -108;
pub const UV_EROFS: C2RustUnnamed = -30;
pub const UV_ERANGE: C2RustUnnamed = -34;
pub const UV_EPROTOTYPE: C2RustUnnamed = -91;
pub const UV_EPROTONOSUPPORT: C2RustUnnamed = -93;
pub const UV_EPROTO: C2RustUnnamed = -71;
pub const UV_EPIPE: C2RustUnnamed = -32;
pub const UV_EPERM: C2RustUnnamed = -1;
pub const UV_EOVERFLOW: C2RustUnnamed = -75;
pub const UV_ENOTSUP: C2RustUnnamed = -95;
pub const UV_ENOTSOCK: C2RustUnnamed = -88;
pub const UV_ENOTEMPTY: C2RustUnnamed = -39;
pub const UV_ENOTDIR: C2RustUnnamed = -20;
pub const UV_ENOTCONN: C2RustUnnamed = -107;
pub const UV_ENOSYS: C2RustUnnamed = -38;
pub const UV_ENOSPC: C2RustUnnamed = -28;
pub const UV_ENOPROTOOPT: C2RustUnnamed = -92;
pub const UV_ENONET: C2RustUnnamed = -64;
pub const UV_ENOMEM: C2RustUnnamed = -12;
pub const UV_ENOENT: C2RustUnnamed = -2;
pub const UV_ENODEV: C2RustUnnamed = -19;
pub const UV_ENOBUFS: C2RustUnnamed = -105;
pub const UV_ENFILE: C2RustUnnamed = -23;
pub const UV_ENETUNREACH: C2RustUnnamed = -101;
pub const UV_ENETDOWN: C2RustUnnamed = -100;
pub const UV_ENAMETOOLONG: C2RustUnnamed = -36;
pub const UV_EMSGSIZE: C2RustUnnamed = -90;
pub const UV_EMFILE: C2RustUnnamed = -24;
pub const UV_ELOOP: C2RustUnnamed = -40;
pub const UV_EISDIR: C2RustUnnamed = -21;
pub const UV_EISCONN: C2RustUnnamed = -106;
pub const UV_EIO: C2RustUnnamed = -5;
pub const UV_EINVAL: C2RustUnnamed = -22;
pub const UV_EINTR: C2RustUnnamed = -4;
pub const UV_EHOSTUNREACH: C2RustUnnamed = -113;
pub const UV_EFBIG: C2RustUnnamed = -27;
pub const UV_EFAULT: C2RustUnnamed = -14;
pub const UV_EEXIST: C2RustUnnamed = -17;
pub const UV_EDESTADDRREQ: C2RustUnnamed = -89;
pub const UV_ECONNRESET: C2RustUnnamed = -104;
pub const UV_ECONNREFUSED: C2RustUnnamed = -111;
pub const UV_ECONNABORTED: C2RustUnnamed = -103;
pub const UV_ECHARSET: C2RustUnnamed = -4080;
pub const UV_ECANCELED: C2RustUnnamed = -125;
pub const UV_EBUSY: C2RustUnnamed = -16;
pub const UV_EBADF: C2RustUnnamed = -9;
pub const UV_EALREADY: C2RustUnnamed = -114;
pub const UV_EAI_SOCKTYPE: C2RustUnnamed = -3011;
pub const UV_EAI_SERVICE: C2RustUnnamed = -3010;
pub const UV_EAI_PROTOCOL: C2RustUnnamed = -3014;
pub const UV_EAI_OVERFLOW: C2RustUnnamed = -3009;
pub const UV_EAI_NONAME: C2RustUnnamed = -3008;
pub const UV_EAI_NODATA: C2RustUnnamed = -3007;
pub const UV_EAI_MEMORY: C2RustUnnamed = -3006;
pub const UV_EAI_FAMILY: C2RustUnnamed = -3005;
pub const UV_EAI_FAIL: C2RustUnnamed = -3004;
pub const UV_EAI_CANCELED: C2RustUnnamed = -3003;
pub const UV_EAI_BADHINTS: C2RustUnnamed = -3013;
pub const UV_EAI_BADFLAGS: C2RustUnnamed = -3002;
pub const UV_EAI_AGAIN: C2RustUnnamed = -3001;
pub const UV_EAI_ADDRFAMILY: C2RustUnnamed = -3000;
pub const UV_EAGAIN: C2RustUnnamed = -11;
pub const UV_EAFNOSUPPORT: C2RustUnnamed = -97;
pub const UV_EADDRNOTAVAIL: C2RustUnnamed = -99;
pub const UV_EADDRINUSE: C2RustUnnamed = -98;
pub const UV_EACCES: C2RustUnnamed = -13;
pub const UV_E2BIG: C2RustUnnamed = -7;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__sysctl_args {
    pub name: *mut ::core::ffi::c_int,
    pub nlen: ::core::ffi::c_int,
    pub oldval: *mut ::core::ffi::c_void,
    pub oldlenp: *mut size_t,
    pub newval: *mut ::core::ffi::c_void,
    pub newlen: size_t,
    pub unused: [::core::ffi::c_ulong; 4],
}
pub const __NR__sysctl: ::core::ffi::c_int = 156 as ::core::ffi::c_int;
pub const SYS__sysctl: ::core::ffi::c_int = __NR__sysctl;
#[no_mangle]
pub unsafe extern "C" fn uv__random_sysctl(
    mut buf: *mut ::core::ffi::c_void,
    mut buflen: size_t,
) -> ::core::ffi::c_int {
    static mut name: [::core::ffi::c_int; 3] = [
        1 as ::core::ffi::c_int,
        40 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
    ];
    let mut args: uv__sysctl_args = uv__sysctl_args {
        name: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        nlen: 0,
        oldval: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        oldlenp: ::core::ptr::null_mut::<size_t>(),
        newval: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        newlen: 0,
        unused: [0; 4],
    };
    let mut uuid: [::core::ffi::c_char; 16] = [0; 16];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pe: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: size_t = 0;
    p = buf as *mut ::core::ffi::c_char;
    pe = p.offset(buflen as isize);
    while p < pe {
        memset(
            &raw mut args as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uv__sysctl_args>() as size_t,
        );
        args.name = &raw mut name as *mut ::core::ffi::c_int;
        args.nlen = (::core::mem::size_of::<[::core::ffi::c_int; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
            as ::core::ffi::c_int;
        args.oldval = &raw mut uuid as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        args.oldlenp = &raw mut n;
        n = ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize as size_t;
        if syscall(SYS__sysctl as ::core::ffi::c_long, &raw mut args)
            == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
        {
            return -*__errno_location();
        }
        if n != ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as usize {
            return UV_EIO as ::core::ffi::c_int;
        }
        uuid[6 as ::core::ffi::c_int as usize] = uuid[14 as ::core::ffi::c_int as usize];
        uuid[8 as ::core::ffi::c_int as usize] = uuid[15 as ::core::ffi::c_int as usize];
        n = pe.offset_from(p) as ::core::ffi::c_long as size_t;
        if n > 14 as size_t {
            n = 14 as size_t;
        }
        memcpy(
            p as *mut ::core::ffi::c_void,
            &raw mut uuid as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            n,
        );
        p = p.offset(n as isize);
    }
    return 0 as ::core::ffi::c_int;
}
