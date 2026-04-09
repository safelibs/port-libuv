#[repr(C)]
pub struct sockaddr_x25 {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_un {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_ns {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_iso {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_ipx {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_inarp {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_eon {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_dl {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_ax25 {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct sockaddr_at {
    _unused: [u8; 0],
}
extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn socketpair(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
        __fds: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn getsockname(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn getpeername(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv_close(handle: *mut uv_handle_t, close_cb: uv_close_cb);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
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
    fn uv__nonblock_ioctl(fd: ::core::ffi::c_int, set: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__socket(
        domain: ::core::ffi::c_int,
        type_0: ::core::ffi::c_int,
        protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv__io_start(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_feed(loop_0: *mut uv_loop_t, w: *mut uv__io_t);
    fn uv__fd_exists(loop_0: *mut uv_loop_t, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__stream_init(loop_0: *mut uv_loop_t, stream: *mut uv_stream_t, type_0: uv_handle_type);
    fn uv__stream_open(
        _: *mut uv_stream_t,
        fd: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv__server_io(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__stream_close(handle: *mut uv_stream_t);
    fn uv__getsockpeername(
        handle: *const uv_handle_t,
        func: uv__peersockfunc,
        name: *mut sockaddr,
        namelen: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getifaddrs(__ifap: *mut *mut ifaddrs) -> ::core::ffi::c_int;
    fn freeifaddrs(__ifa: *mut ifaddrs);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__queue {
    pub next: *mut uv__queue,
    pub prev: *mut uv__queue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::core::ffi::c_uint,
    pub __writers: ::core::ffi::c_uint,
    pub __wrphase_futex: ::core::ffi::c_uint,
    pub __writers_futex: ::core::ffi::c_uint,
    pub __pad3: ::core::ffi::c_uint,
    pub __pad4: ::core::ffi::c_uint,
    pub __cur_writer: ::core::ffi::c_int,
    pub __shared: ::core::ffi::c_int,
    pub __rwelision: ::core::ffi::c_schar,
    pub __pad1: [::core::ffi::c_uchar; 7],
    pub __pad2: ::core::ffi::c_ulong,
    pub __flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linger {
    pub l_onoff: ::core::ffi::c_int,
    pub l_linger: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
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
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_0 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_0 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_0 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2RustUnnamed_6,
    pub internal_fields: *mut ::core::ffi::c_void,
    pub stop_flag: ::core::ffi::c_uint,
    pub flags: ::core::ffi::c_ulong,
    pub backend_fd: ::core::ffi::c_int,
    pub pending_queue: uv__queue,
    pub watcher_queue: uv__queue,
    pub watchers: *mut *mut uv__io_t,
    pub nwatchers: ::core::ffi::c_uint,
    pub nfds: ::core::ffi::c_uint,
    pub wq: uv__queue,
    pub wq_mutex: uv_mutex_t,
    pub wq_async: uv_async_t,
    pub cloexec_lock: uv_rwlock_t,
    pub closing_handles: *mut uv_handle_t,
    pub process_handles: uv__queue,
    pub prepare_handles: uv__queue,
    pub check_handles: uv__queue,
    pub idle_handles: uv__queue,
    pub async_handles: uv__queue,
    pub async_unused: Option<unsafe extern "C" fn() -> ()>,
    pub async_io_watcher: uv__io_t,
    pub async_wfd: ::core::ffi::c_int,
    pub timer_heap: C2RustUnnamed_4,
    pub timer_counter: uint64_t,
    pub time: uint64_t,
    pub signal_pipefd: [::core::ffi::c_int; 2],
    pub signal_io_watcher: uv__io_t,
    pub child_watcher: uv_signal_t,
    pub emfile_fd: ::core::ffi::c_int,
    pub inotify_read_watcher: uv__io_t,
    pub inotify_watchers: *mut ::core::ffi::c_void,
    pub inotify_fd: ::core::ffi::c_int,
}
pub type uv__io_t = uv__io_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_s {
    pub cb: uv__io_cb,
    pub pending_queue: uv__queue,
    pub watcher_queue: uv__queue,
    pub pevents: ::core::ffi::c_uint,
    pub events: ::core::ffi::c_uint,
    pub fd: ::core::ffi::c_int,
}
pub type uv__io_cb =
    Option<unsafe extern "C" fn(*mut uv_loop_s, *mut uv__io_s, ::core::ffi::c_uint) -> ()>;
pub type uv_signal_t = uv_signal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_signal_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_3,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2RustUnnamed_1,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub rbe_left: *mut uv_signal_s,
    pub rbe_right: *mut uv_signal_s,
    pub rbe_parent: *mut uv_signal_s,
    pub rbe_color: ::core::ffi::c_int,
}
pub type uv_signal_cb = Option<unsafe extern "C" fn(*mut uv_signal_t, ::core::ffi::c_int) -> ()>;
pub type uv_handle_t = uv_handle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_handle_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_2,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_close_cb = Option<unsafe extern "C" fn(*mut uv_handle_t) -> ()>;
pub type uv_handle_type = ::core::ffi::c_uint;
pub const UV_HANDLE_TYPE_MAX: uv_handle_type = 18;
pub const UV_FILE: uv_handle_type = 17;
pub const UV_SIGNAL: uv_handle_type = 16;
pub const UV_UDP: uv_handle_type = 15;
pub const UV_TTY: uv_handle_type = 14;
pub const UV_TIMER: uv_handle_type = 13;
pub const UV_TCP: uv_handle_type = 12;
pub const UV_STREAM: uv_handle_type = 11;
pub const UV_PROCESS: uv_handle_type = 10;
pub const UV_PREPARE: uv_handle_type = 9;
pub const UV_POLL: uv_handle_type = 8;
pub const UV_NAMED_PIPE: uv_handle_type = 7;
pub const UV_IDLE: uv_handle_type = 6;
pub const UV_HANDLE: uv_handle_type = 5;
pub const UV_FS_POLL: uv_handle_type = 4;
pub const UV_FS_EVENT: uv_handle_type = 3;
pub const UV_CHECK: uv_handle_type = 2;
pub const UV_ASYNC: uv_handle_type = 1;
pub const UV_UNKNOWN_HANDLE: uv_handle_type = 0;
pub type uv_loop_t = uv_loop_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub min: *mut ::core::ffi::c_void,
    pub nelts: ::core::ffi::c_uint,
}
pub type uv_rwlock_t = pthread_rwlock_t;
pub type uv_async_t = uv_async_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_async_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_5,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub unused: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_buf_t {
    pub base: *mut ::core::ffi::c_char,
    pub len: size_t,
}
pub type uv_os_sock_t = ::core::ffi::c_int;
pub type C2RustUnnamed_7 = ::core::ffi::c_int;
pub const UV_ERRNO_MAX: C2RustUnnamed_7 = -4096;
pub const UV_EUNATCH: C2RustUnnamed_7 = -49;
pub const UV_ENODATA: C2RustUnnamed_7 = -61;
pub const UV_ESOCKTNOSUPPORT: C2RustUnnamed_7 = -94;
pub const UV_EILSEQ: C2RustUnnamed_7 = -84;
pub const UV_EFTYPE: C2RustUnnamed_7 = -4028;
pub const UV_ENOTTY: C2RustUnnamed_7 = -25;
pub const UV_EREMOTEIO: C2RustUnnamed_7 = -121;
pub const UV_EHOSTDOWN: C2RustUnnamed_7 = -112;
pub const UV_EMLINK: C2RustUnnamed_7 = -31;
pub const UV_ENXIO: C2RustUnnamed_7 = -6;
pub const UV_EOF: C2RustUnnamed_7 = -4095;
pub const UV_UNKNOWN: C2RustUnnamed_7 = -4094;
pub const UV_EXDEV: C2RustUnnamed_7 = -18;
pub const UV_ETXTBSY: C2RustUnnamed_7 = -26;
pub const UV_ETIMEDOUT: C2RustUnnamed_7 = -110;
pub const UV_ESRCH: C2RustUnnamed_7 = -3;
pub const UV_ESPIPE: C2RustUnnamed_7 = -29;
pub const UV_ESHUTDOWN: C2RustUnnamed_7 = -108;
pub const UV_EROFS: C2RustUnnamed_7 = -30;
pub const UV_ERANGE: C2RustUnnamed_7 = -34;
pub const UV_EPROTOTYPE: C2RustUnnamed_7 = -91;
pub const UV_EPROTONOSUPPORT: C2RustUnnamed_7 = -93;
pub const UV_EPROTO: C2RustUnnamed_7 = -71;
pub const UV_EPIPE: C2RustUnnamed_7 = -32;
pub const UV_EPERM: C2RustUnnamed_7 = -1;
pub const UV_EOVERFLOW: C2RustUnnamed_7 = -75;
pub const UV_ENOTSUP: C2RustUnnamed_7 = -95;
pub const UV_ENOTSOCK: C2RustUnnamed_7 = -88;
pub const UV_ENOTEMPTY: C2RustUnnamed_7 = -39;
pub const UV_ENOTDIR: C2RustUnnamed_7 = -20;
pub const UV_ENOTCONN: C2RustUnnamed_7 = -107;
pub const UV_ENOSYS: C2RustUnnamed_7 = -38;
pub const UV_ENOSPC: C2RustUnnamed_7 = -28;
pub const UV_ENOPROTOOPT: C2RustUnnamed_7 = -92;
pub const UV_ENONET: C2RustUnnamed_7 = -64;
pub const UV_ENOMEM: C2RustUnnamed_7 = -12;
pub const UV_ENOENT: C2RustUnnamed_7 = -2;
pub const UV_ENODEV: C2RustUnnamed_7 = -19;
pub const UV_ENOBUFS: C2RustUnnamed_7 = -105;
pub const UV_ENFILE: C2RustUnnamed_7 = -23;
pub const UV_ENETUNREACH: C2RustUnnamed_7 = -101;
pub const UV_ENETDOWN: C2RustUnnamed_7 = -100;
pub const UV_ENAMETOOLONG: C2RustUnnamed_7 = -36;
pub const UV_EMSGSIZE: C2RustUnnamed_7 = -90;
pub const UV_EMFILE: C2RustUnnamed_7 = -24;
pub const UV_ELOOP: C2RustUnnamed_7 = -40;
pub const UV_EISDIR: C2RustUnnamed_7 = -21;
pub const UV_EISCONN: C2RustUnnamed_7 = -106;
pub const UV_EIO: C2RustUnnamed_7 = -5;
pub const UV_EINVAL: C2RustUnnamed_7 = -22;
pub const UV_EINTR: C2RustUnnamed_7 = -4;
pub const UV_EHOSTUNREACH: C2RustUnnamed_7 = -113;
pub const UV_EFBIG: C2RustUnnamed_7 = -27;
pub const UV_EFAULT: C2RustUnnamed_7 = -14;
pub const UV_EEXIST: C2RustUnnamed_7 = -17;
pub const UV_EDESTADDRREQ: C2RustUnnamed_7 = -89;
pub const UV_ECONNRESET: C2RustUnnamed_7 = -104;
pub const UV_ECONNREFUSED: C2RustUnnamed_7 = -111;
pub const UV_ECONNABORTED: C2RustUnnamed_7 = -103;
pub const UV_ECHARSET: C2RustUnnamed_7 = -4080;
pub const UV_ECANCELED: C2RustUnnamed_7 = -125;
pub const UV_EBUSY: C2RustUnnamed_7 = -16;
pub const UV_EBADF: C2RustUnnamed_7 = -9;
pub const UV_EALREADY: C2RustUnnamed_7 = -114;
pub const UV_EAI_SOCKTYPE: C2RustUnnamed_7 = -3011;
pub const UV_EAI_SERVICE: C2RustUnnamed_7 = -3010;
pub const UV_EAI_PROTOCOL: C2RustUnnamed_7 = -3014;
pub const UV_EAI_OVERFLOW: C2RustUnnamed_7 = -3009;
pub const UV_EAI_NONAME: C2RustUnnamed_7 = -3008;
pub const UV_EAI_NODATA: C2RustUnnamed_7 = -3007;
pub const UV_EAI_MEMORY: C2RustUnnamed_7 = -3006;
pub const UV_EAI_FAMILY: C2RustUnnamed_7 = -3005;
pub const UV_EAI_FAIL: C2RustUnnamed_7 = -3004;
pub const UV_EAI_CANCELED: C2RustUnnamed_7 = -3003;
pub const UV_EAI_BADHINTS: C2RustUnnamed_7 = -3013;
pub const UV_EAI_BADFLAGS: C2RustUnnamed_7 = -3002;
pub const UV_EAI_AGAIN: C2RustUnnamed_7 = -3001;
pub const UV_EAI_ADDRFAMILY: C2RustUnnamed_7 = -3000;
pub const UV_EAGAIN: C2RustUnnamed_7 = -11;
pub const UV_EAFNOSUPPORT: C2RustUnnamed_7 = -97;
pub const UV_EADDRNOTAVAIL: C2RustUnnamed_7 = -99;
pub const UV_EADDRINUSE: C2RustUnnamed_7 = -98;
pub const UV_EACCES: C2RustUnnamed_7 = -13;
pub const UV_E2BIG: C2RustUnnamed_7 = -7;
pub type uv_req_type = ::core::ffi::c_uint;
pub const UV_REQ_TYPE_MAX: uv_req_type = 11;
pub const UV_RANDOM: uv_req_type = 10;
pub const UV_GETNAMEINFO: uv_req_type = 9;
pub const UV_GETADDRINFO: uv_req_type = 8;
pub const UV_WORK: uv_req_type = 7;
pub const UV_FS: uv_req_type = 6;
pub const UV_UDP_SEND: uv_req_type = 5;
pub const UV_SHUTDOWN: uv_req_type = 4;
pub const UV_WRITE: uv_req_type = 3;
pub const UV_CONNECT: uv_req_type = 2;
pub const UV_REQ: uv_req_type = 1;
pub const UV_UNKNOWN_REQ: uv_req_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stream_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_8,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::core::ffi::c_int,
    pub accepted_fd: ::core::ffi::c_int,
    pub queued_fds: *mut ::core::ffi::c_void,
}
pub type uv_connection_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ::core::ffi::c_int) -> ()>;
pub type uv_stream_t = uv_stream_s;
pub type uv_shutdown_t = uv_shutdown_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_shutdown_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub handle: *mut uv_stream_t,
    pub cb: uv_shutdown_cb,
}
pub type uv_shutdown_cb =
    Option<unsafe extern "C" fn(*mut uv_shutdown_t, ::core::ffi::c_int) -> ()>;
pub type uv_connect_t = uv_connect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_connect_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub cb: uv_connect_cb,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
}
pub type uv_connect_cb = Option<unsafe extern "C" fn(*mut uv_connect_t, ::core::ffi::c_int) -> ()>;
pub type uv_read_cb =
    Option<unsafe extern "C" fn(*mut uv_stream_t, ssize_t, *const uv_buf_t) -> ()>;
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_tcp_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_9,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub write_queue_size: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub read_cb: uv_read_cb,
    pub connect_req: *mut uv_connect_t,
    pub shutdown_req: *mut uv_shutdown_t,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
    pub connection_cb: uv_connection_cb,
    pub delayed_error: ::core::ffi::c_int,
    pub accepted_fd: ::core::ffi::c_int,
    pub queued_fds: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tcp_t = uv_tcp_s;
pub const UV_NONBLOCK_PIPE: C2RustUnnamed_10 = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub union uv__sockaddr {
    pub in6: sockaddr_in6,
    pub in_0: sockaddr_in,
    pub addr: sockaddr,
}
pub const UV_HANDLE_BOUND: C2RustUnnamed_11 = 8192;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_11 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_11 = 16384;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_11 = 16777216;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_11 = 33554432;
pub type uv_tcp_flags = ::core::ffi::c_uint;
pub const UV_TCP_IPV6ONLY: uv_tcp_flags = 1;
pub type uv__peersockfunc = Option<
    unsafe extern "C" fn(::core::ffi::c_int, *mut sockaddr, *mut socklen_t) -> ::core::ffi::c_int,
>;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const UV_OVERLAPPED_PIPE: C2RustUnnamed_10 = 64;
pub const UV_WRITABLE_PIPE: C2RustUnnamed_10 = 32;
pub const UV_READABLE_PIPE: C2RustUnnamed_10 = 16;
pub const UV_INHERIT_STREAM: C2RustUnnamed_10 = 4;
pub const UV_INHERIT_FD: C2RustUnnamed_10 = 2;
pub const UV_CREATE_PIPE: C2RustUnnamed_10 = 1;
pub const UV_IGNORE: C2RustUnnamed_10 = 0;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_11 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_11 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_11 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_11 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_11 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_11 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_11 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_11 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_11 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_11 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_11 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_11 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_11 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_11 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_11 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_11 = 67108864;
pub const UV_HANDLE_IPV6: C2RustUnnamed_11 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_11 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_11 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_11 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_11 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_11 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_11 = 65536;
pub const UV_HANDLE_READING: C2RustUnnamed_11 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_11 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_11 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_11 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_11 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_11 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_11 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_11 = 16;
pub const UV_HANDLE_REF: C2RustUnnamed_11 = 8;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_11 = 4;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_11 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_11 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifaddrs {
    pub ifa_next: *mut ifaddrs,
    pub ifa_name: *mut ::core::ffi::c_char,
    pub ifa_flags: ::core::ffi::c_uint,
    pub ifa_addr: *mut sockaddr,
    pub ifa_netmask: *mut sockaddr,
    pub ifa_ifu: C2RustUnnamed_12,
    pub ifa_data: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ifu_broadaddr: *mut sockaddr,
    pub ifu_dstaddr: *mut sockaddr,
}
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const EADDRINUSE: ::core::ffi::c_int = 98 as ::core::ffi::c_int;
pub const ECONNREFUSED: ::core::ffi::c_int = 111 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SO_KEEPALIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SO_LINGER: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const IPV6_V6ONLY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const TCP_NODELAY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TCP_KEEPIDLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const TCP_KEEPINTVL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TCP_KEEPCNT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const UV_FS_O_NONBLOCK: ::core::ffi::c_int = O_NONBLOCK;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 102] = unsafe {
    ::core::mem::transmute::<
        [u8; 102],
        [::core::ffi::c_char; 102],
    >(
        *b"int uv__tcp_connect(uv_connect_t *, uv_tcp_t *, const struct sockaddr *, unsigned int, uv_connect_cb)\0",
    )
};
#[inline]
unsafe extern "C" fn uv__queue_init(mut q: *mut uv__queue) {
    (*q).next = q;
    (*q).prev = q;
}
#[inline]
unsafe extern "C" fn uv__queue_remove(mut q: *mut uv__queue) {
    (*(*q).prev).next = (*q).next;
    (*(*q).next).prev = (*q).prev;
}
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
unsafe extern "C" fn maybe_bind_socket(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut s: uv__sockaddr = uv__sockaddr {
        in6: sockaddr_in6 {
            sin6_family: 0,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: in6_addr {
                __in6_u: C2RustUnnamed {
                    __u6_addr8: [0; 16],
                },
            },
            sin6_scope_id: 0,
        },
    };
    let mut slen: socklen_t = 0;
    slen = ::core::mem::size_of::<uv__sockaddr>() as socklen_t;
    memset(
        &raw mut s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<uv__sockaddr>() as size_t,
    );
    if getsockname(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut s.addr,
        },
        &raw mut slen,
    ) != 0
    {
        return -*__errno_location();
    }
    if s.addr.sa_family as ::core::ffi::c_int == AF_INET {
        if s.in_0.sin_port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if s.addr.sa_family as ::core::ffi::c_int == AF_INET6 {
        if s.in6.sin6_port as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
    }
    if bind(
        fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut s.addr,
        },
        slen,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn new_socket(
    mut handle: *mut uv_tcp_t,
    mut domain: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut sockfd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    sockfd = uv__socket(
        domain,
        SOCK_STREAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if sockfd < 0 as ::core::ffi::c_int {
        return sockfd;
    }
    err = uv__stream_open(
        handle as *mut uv_stream_t,
        sockfd,
        flags as ::core::ffi::c_int,
    );
    if err != 0 {
        uv__close(sockfd);
        return err;
    }
    if flags & UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        return maybe_bind_socket(sockfd);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn maybe_new_socket(
    mut handle: *mut uv_tcp_t,
    mut domain: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut sockfd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    if !(domain == AF_UNSPEC) {
        sockfd = (*handle).io_watcher.fd;
        if sockfd == -(1 as ::core::ffi::c_int) {
            return new_socket(handle, domain, flags);
        }
        if !(flags & UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint == 0) {
            if !((*handle).flags & UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0)
            {
                err = maybe_bind_socket(sockfd);
                if err != 0 {
                    return err;
                }
            }
        }
    }
    (*handle).flags |= flags;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_tcp_init_ex(
    mut loop_0: *mut uv_loop_t,
    mut tcp: *mut uv_tcp_t,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut domain: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    domain = (flags & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if domain != AF_INET && domain != AF_INET6 && domain != AF_UNSPEC {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if flags & !(0xff as ::core::ffi::c_int) as ::core::ffi::c_uint != 0 {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    uv__stream_init(loop_0, tcp as *mut uv_stream_t, UV_TCP);
    if domain != AF_UNSPEC {
        err = new_socket(tcp, domain, 0 as ::core::ffi::c_uint);
        if err != 0 {
            uv__queue_remove(&raw mut (*tcp).handle_queue);
            if (*tcp).io_watcher.fd != -(1 as ::core::ffi::c_int) {
                uv__close((*tcp).io_watcher.fd);
            }
            (*tcp).io_watcher.fd = -(1 as ::core::ffi::c_int);
            return err;
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_tcp_init(
    mut loop_0: *mut uv_loop_t,
    mut tcp: *mut uv_tcp_t,
) -> ::core::ffi::c_int {
    return uv_tcp_init_ex(loop_0, tcp, AF_UNSPEC as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn uv__tcp_bind(
    mut tcp: *mut uv_tcp_t,
    mut addr: *const sockaddr,
    mut addrlen: ::core::ffi::c_uint,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut on: ::core::ffi::c_int = 0;
    if flags & UV_TCP_IPV6ONLY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && (*addr).sa_family as ::core::ffi::c_int != AF_INET6
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    err = maybe_new_socket(
        tcp,
        (*addr).sa_family as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
    );
    if err != 0 {
        return err;
    }
    on = 1 as ::core::ffi::c_int;
    if setsockopt(
        (*tcp).io_watcher.fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &raw mut on as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        on = (flags & UV_TCP_IPV6ONLY as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
        if setsockopt(
            (*tcp).io_watcher.fd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_V6ONLY,
            &raw mut on as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            return -*__errno_location();
        }
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    err = bind(
        (*tcp).io_watcher.fd,
        __CONST_SOCKADDR_ARG { __sockaddr__: addr },
        addrlen as socklen_t,
    );
    if err == -(1 as ::core::ffi::c_int) && *__errno_location() != EADDRINUSE {
        if *__errno_location() == EAFNOSUPPORT {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        return -*__errno_location();
    }
    (*tcp).delayed_error = if err == -(1 as ::core::ffi::c_int) {
        -*__errno_location()
    } else {
        0 as ::core::ffi::c_int
    };
    (*tcp).flags |= UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint;
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        (*tcp).flags |= UV_HANDLE_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__is_ipv6_link_local(mut addr: *const sockaddr) -> ::core::ffi::c_int {
    let mut a6: *const sockaddr_in6 = ::core::ptr::null::<sockaddr_in6>();
    let mut b: [uint8_t; 2] = [0; 2];
    if (*addr).sa_family as ::core::ffi::c_int != AF_INET6 {
        return 0 as ::core::ffi::c_int;
    }
    a6 = addr as *const sockaddr_in6;
    memcpy(
        &raw mut b as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw const (*a6).sin6_addr as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
    );
    return (b[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 0xfe as ::core::ffi::c_int
        && b[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == 0x80 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__ipv6_link_local_scope_id() -> ::core::ffi::c_int {
    let mut a6: *mut sockaddr_in6 = ::core::ptr::null_mut::<sockaddr_in6>();
    let mut rv: ::core::ffi::c_int = 0;
    let mut ifa: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
    let mut p: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
    if getifaddrs(&raw mut ifa) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    p = ifa;
    while !p.is_null() {
        if !(*p).ifa_addr.is_null() {
            if uv__is_ipv6_link_local((*p).ifa_addr) != 0 {
                break;
            }
        }
        p = (*p).ifa_next;
    }
    rv = 0 as ::core::ffi::c_int;
    if !p.is_null() {
        a6 = (*p).ifa_addr as *mut sockaddr_in6;
        rv = (*a6).sin6_scope_id as ::core::ffi::c_int;
    }
    freeifaddrs(ifa);
    return rv;
}
#[no_mangle]
pub unsafe extern "C" fn uv__tcp_connect(
    mut req: *mut uv_connect_t,
    mut handle: *mut uv_tcp_t,
    mut addr: *const sockaddr,
    mut addrlen: ::core::ffi::c_uint,
    mut cb: uv_connect_cb,
) -> ::core::ffi::c_int {
    let mut tmp6: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut err: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*handle).type_0 as ::core::ffi::c_uint
            == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"handle->type == UV_TCP\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/tcp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                280 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    if !(*handle).connect_req.is_null() {
        return UV_EALREADY as ::core::ffi::c_int;
    }
    if !((*handle).delayed_error != 0 as ::core::ffi::c_int) {
        err = maybe_new_socket(
            handle,
            (*addr).sa_family as ::core::ffi::c_int,
            (UV_HANDLE_READABLE as ::core::ffi::c_int | UV_HANDLE_WRITABLE as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
        if err != 0 {
            return err;
        }
        if uv__is_ipv6_link_local(addr) != 0 {
            memcpy(
                &raw mut tmp6 as *mut ::core::ffi::c_void,
                addr as *const ::core::ffi::c_void,
                ::core::mem::size_of::<sockaddr_in6>() as size_t,
            );
            if tmp6.sin6_scope_id == 0 as uint32_t {
                tmp6.sin6_scope_id = uv__ipv6_link_local_scope_id() as uint32_t;
                addr = &raw mut tmp6 as *mut ::core::ffi::c_void as *const sockaddr;
            }
        }
        loop {
            *__errno_location() = 0 as ::core::ffi::c_int;
            r = connect(
                (*handle).io_watcher.fd,
                __CONST_SOCKADDR_ARG { __sockaddr__: addr },
                addrlen as socklen_t,
            );
            if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if r == -(1 as ::core::ffi::c_int) && *__errno_location() != 0 as ::core::ffi::c_int {
            if !(*__errno_location() == EINPROGRESS) {
                if *__errno_location() == ECONNREFUSED {
                    (*handle).delayed_error = -(111 as ::core::ffi::c_int);
                } else {
                    return -*__errno_location();
                }
            }
        }
    }
    (*req).type_0 = UV_CONNECT;
    (*(*handle).loop_0).active_reqs.count = (*(*handle).loop_0).active_reqs.count.wrapping_add(1);
    (*req).cb = cb;
    (*req).handle = handle as *mut uv_stream_t;
    uv__queue_init(&raw mut (*req).queue);
    (*handle).connect_req = req;
    uv__io_start(
        (*handle).loop_0,
        &raw mut (*handle).io_watcher,
        POLLOUT as ::core::ffi::c_uint,
    );
    if (*handle).delayed_error != 0 {
        uv__io_feed((*handle).loop_0, &raw mut (*handle).io_watcher);
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_tcp_open(
    mut handle: *mut uv_tcp_t,
    mut sock: uv_os_sock_t,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    if uv__fd_exists((*handle).loop_0, sock as ::core::ffi::c_int) != 0 {
        return UV_EEXIST as ::core::ffi::c_int;
    }
    err = uv__nonblock_ioctl(sock as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    if err != 0 {
        return err;
    }
    return uv__stream_open(
        handle as *mut uv_stream_t,
        sock as ::core::ffi::c_int,
        UV_HANDLE_READABLE as ::core::ffi::c_int | UV_HANDLE_WRITABLE as ::core::ffi::c_int,
    );
}
pub(crate) unsafe fn uv_tcp_getsockname(
    mut handle: *const uv_tcp_t,
    mut name: *mut sockaddr,
    mut namelen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*handle).delayed_error != 0 {
        return (*handle).delayed_error;
    }
    return uv__getsockpeername(
        handle as *const uv_handle_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    __SOCKADDR_ARG,
                    *mut socklen_t,
                ) -> ::core::ffi::c_int,
            >,
            uv__peersockfunc,
        >(Some(
            getsockname
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    __SOCKADDR_ARG,
                    *mut socklen_t,
                ) -> ::core::ffi::c_int,
        )),
        name,
        namelen,
    );
}
pub(crate) unsafe fn uv_tcp_getpeername(
    mut handle: *const uv_tcp_t,
    mut name: *mut sockaddr,
    mut namelen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*handle).delayed_error != 0 {
        return (*handle).delayed_error;
    }
    return uv__getsockpeername(
        handle as *const uv_handle_t,
        ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    __SOCKADDR_ARG,
                    *mut socklen_t,
                ) -> ::core::ffi::c_int,
            >,
            uv__peersockfunc,
        >(Some(
            getpeername
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    __SOCKADDR_ARG,
                    *mut socklen_t,
                ) -> ::core::ffi::c_int,
        )),
        name,
        namelen,
    );
}
pub(crate) unsafe fn uv_tcp_close_reset(
    mut handle: *mut uv_tcp_t,
    mut close_cb: uv_close_cb,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut l: linger = linger {
        l_onoff: 1 as ::core::ffi::c_int,
        l_linger: 0 as ::core::ffi::c_int,
    };
    if !(*handle).shutdown_req.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    fd = (*handle).io_watcher.fd;
    if 0 as ::core::ffi::c_int
        != setsockopt(
            fd,
            SOL_SOCKET,
            SO_LINGER,
            &raw mut l as *const ::core::ffi::c_void,
            ::core::mem::size_of::<linger>() as socklen_t,
        )
    {
        if *__errno_location() == EINVAL {
            *__errno_location() = 0 as ::core::ffi::c_int;
        } else {
            return -*__errno_location();
        }
    }
    uv_close(handle as *mut uv_handle_t, close_cb);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__tcp_listen(
    mut tcp: *mut uv_tcp_t,
    mut backlog: ::core::ffi::c_int,
    mut cb: uv_connection_cb,
) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_uint = 0;
    let mut err: ::core::ffi::c_int = 0;
    if (*tcp).delayed_error != 0 {
        return (*tcp).delayed_error;
    }
    flags = 0 as ::core::ffi::c_uint;
    err = maybe_new_socket(tcp, AF_INET, flags);
    if err != 0 {
        return err;
    }
    if listen((*tcp).io_watcher.fd, backlog) != 0 {
        return -*__errno_location();
    }
    (*tcp).connection_cb = cb;
    (*tcp).flags |= UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint;
    (*tcp).io_watcher.cb = Some(
        uv__server_io
            as unsafe extern "C" fn(*mut uv_loop_t, *mut uv__io_t, ::core::ffi::c_uint) -> (),
    ) as uv__io_cb;
    uv__io_start(
        (*tcp).loop_0,
        &raw mut (*tcp).io_watcher,
        POLLIN as ::core::ffi::c_uint,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__tcp_nodelay(
    mut fd: ::core::ffi::c_int,
    mut on: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if setsockopt(
        fd,
        IPPROTO_TCP as ::core::ffi::c_int,
        TCP_NODELAY,
        &raw mut on as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__tcp_keepalive(
    mut fd: ::core::ffi::c_int,
    mut on: ::core::ffi::c_int,
    mut delay: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut idle: ::core::ffi::c_int = 0;
    let mut intvl: ::core::ffi::c_int = 0;
    let mut cnt: ::core::ffi::c_int = 0;
    &raw mut idle;
    &raw mut intvl;
    &raw mut cnt;
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_KEEPALIVE,
        &raw mut on as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    if on == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if delay == 0 as ::core::ffi::c_uint {
        return -(1 as ::core::ffi::c_int);
    }
    if setsockopt(
        fd,
        IPPROTO_TCP as ::core::ffi::c_int,
        TCP_KEEPIDLE,
        &raw mut delay as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_uint>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    intvl = 1 as ::core::ffi::c_int;
    if setsockopt(
        fd,
        IPPROTO_TCP as ::core::ffi::c_int,
        TCP_KEEPINTVL,
        &raw mut intvl as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    cnt = 10 as ::core::ffi::c_int;
    if setsockopt(
        fd,
        IPPROTO_TCP as ::core::ffi::c_int,
        TCP_KEEPCNT,
        &raw mut cnt as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_tcp_nodelay(
    mut handle: *mut uv_tcp_t,
    mut on: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    if (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int) {
        err = uv__tcp_nodelay((*handle).io_watcher.fd, on);
        if err != 0 {
            return err;
        }
    }
    if on != 0 {
        (*handle).flags |= UV_HANDLE_TCP_NODELAY as ::core::ffi::c_int as ::core::ffi::c_uint;
    } else {
        (*handle).flags &= !(UV_HANDLE_TCP_NODELAY as ::core::ffi::c_int) as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_tcp_keepalive(
    mut handle: *mut uv_tcp_t,
    mut on: ::core::ffi::c_int,
    mut delay: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    if (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int) {
        err = uv__tcp_keepalive((*handle).io_watcher.fd, on, delay);
        if err != 0 {
            return err;
        }
    }
    if on != 0 {
        (*handle).flags |= UV_HANDLE_TCP_KEEPALIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
    } else {
        (*handle).flags &= !(UV_HANDLE_TCP_KEEPALIVE as ::core::ffi::c_int) as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_tcp_simultaneous_accepts(
    mut handle: *mut uv_tcp_t,
    mut enable: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__tcp_close(mut handle: *mut uv_tcp_t) {
    uv__stream_close(handle as *mut uv_stream_t);
}
pub(crate) unsafe fn uv_socketpair(
    mut type_0: ::core::ffi::c_int,
    mut protocol: ::core::ffi::c_int,
    mut fds: *mut uv_os_sock_t,
    mut flags0: ::core::ffi::c_int,
    mut flags1: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut temp: [uv_os_sock_t; 2] = [0; 2];
    let mut err: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    flags = type_0 | SOCK_CLOEXEC as ::core::ffi::c_int;
    if flags0 & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0
        && flags1 & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0
    {
        flags |= SOCK_NONBLOCK as ::core::ffi::c_int;
    }
    if socketpair(
        AF_UNIX,
        flags,
        protocol,
        &raw mut temp as *mut ::core::ffi::c_int,
    ) != 0
    {
        return -*__errno_location();
    }
    if flags & UV_FS_O_NONBLOCK != 0 {
        *fds.offset(0 as ::core::ffi::c_int as isize) = temp[0 as ::core::ffi::c_int as usize];
        *fds.offset(1 as ::core::ffi::c_int as isize) = temp[1 as ::core::ffi::c_int as usize];
        return 0 as ::core::ffi::c_int;
    }
    if flags0 & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0 {
        err = uv__nonblock_ioctl(
            temp[0 as ::core::ffi::c_int as usize],
            1 as ::core::ffi::c_int,
        );
        if err != 0 {
            current_block = 3897503978071717076;
        } else {
            current_block = 1394248824506584008;
        }
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        1394248824506584008 => {
            if flags1 & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0 {
                err = uv__nonblock_ioctl(
                    temp[1 as ::core::ffi::c_int as usize],
                    1 as ::core::ffi::c_int,
                );
                if err != 0 {
                    current_block = 3897503978071717076;
                } else {
                    current_block = 7746791466490516765;
                }
            } else {
                current_block = 7746791466490516765;
            }
            match current_block {
                3897503978071717076 => {}
                _ => {
                    *fds.offset(0 as ::core::ffi::c_int as isize) =
                        temp[0 as ::core::ffi::c_int as usize];
                    *fds.offset(1 as ::core::ffi::c_int as isize) =
                        temp[1 as ::core::ffi::c_int as usize];
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        _ => {}
    }
    uv__close(temp[0 as ::core::ffi::c_int as usize]);
    uv__close(temp[1 as ::core::ffi::c_int as usize]);
    return err;
}

pub(crate) unsafe fn bind_tcp(
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    addr: *const crate::abi::linux_x86_64::sockaddr,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { crate::upstream_support::uv_common::uv_tcp_bind(handle.cast(), addr.cast(), flags) }
}

pub(crate) unsafe fn close_reset(
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    close_cb: crate::abi::linux_x86_64::uv_close_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        uv_tcp_close_reset(
            handle.cast(),
            std::mem::transmute::<_, uv_close_cb>(close_cb),
        )
    }
}

pub(crate) unsafe fn connect_tcp(
    req: *mut crate::abi::linux_x86_64::uv_connect_t,
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    addr: *const crate::abi::linux_x86_64::sockaddr,
    cb: crate::abi::linux_x86_64::uv_connect_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        crate::upstream_support::uv_common::uv_tcp_connect(
            req.cast(),
            handle.cast(),
            addr.cast(),
            std::mem::transmute::<_, crate::upstream_support::uv_common::uv_connect_cb>(cb),
        )
    }
}

pub(crate) unsafe fn getpeername_tcp(
    handle: *const crate::abi::linux_x86_64::uv_tcp_t,
    name: *mut crate::abi::linux_x86_64::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_getpeername(handle.cast(), name.cast(), namelen) }
}

pub(crate) unsafe fn getsockname_tcp(
    handle: *const crate::abi::linux_x86_64::uv_tcp_t,
    name: *mut crate::abi::linux_x86_64::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_getsockname(handle.cast(), name.cast(), namelen) }
}

pub(crate) unsafe fn init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_init(loop_.cast(), handle.cast()) }
}

pub(crate) unsafe fn init_ex(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_init_ex(loop_.cast(), handle.cast(), flags) }
}

pub(crate) unsafe fn keepalive(
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    enable: ::std::os::raw::c_int,
    delay: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_keepalive(handle.cast(), enable, delay) }
}

pub(crate) unsafe fn nodelay(
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_nodelay(handle.cast(), enable) }
}

pub(crate) unsafe fn open(
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    sock: crate::abi::linux_x86_64::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_open(handle.cast(), sock) }
}

pub(crate) unsafe fn simultaneous_accepts(
    handle: *mut crate::abi::linux_x86_64::uv_tcp_t,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_tcp_simultaneous_accepts(handle.cast(), enable) }
}
