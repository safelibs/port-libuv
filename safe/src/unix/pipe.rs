#[repr(C)]
pub struct sockaddr_x25 {
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
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn chmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
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
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe2(__pipedes: *mut ::core::ffi::c_int, __flags: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn uv_guess_handle(file: uv_file) -> uv_handle_type;
    fn uv__malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__free(ptr: *mut ::core::ffi::c_void);
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
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
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
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2RustUnnamed_5,
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
    pub timer_heap: C2RustUnnamed_3,
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
    pub u: C2RustUnnamed_2,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2RustUnnamed_0,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
    pub u: C2RustUnnamed_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub union C2RustUnnamed_2 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
    pub u: C2RustUnnamed_4,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub unused: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_buf_t {
    pub base: *mut ::core::ffi::c_char,
    pub len: size_t,
}
pub type uv_file = ::core::ffi::c_int;
pub type uv_os_fd_t = ::core::ffi::c_int;
pub type C2RustUnnamed_6 = ::core::ffi::c_int;
pub const UV_ERRNO_MAX: C2RustUnnamed_6 = -4096;
pub const UV_EUNATCH: C2RustUnnamed_6 = -49;
pub const UV_ENODATA: C2RustUnnamed_6 = -61;
pub const UV_ESOCKTNOSUPPORT: C2RustUnnamed_6 = -94;
pub const UV_EILSEQ: C2RustUnnamed_6 = -84;
pub const UV_EFTYPE: C2RustUnnamed_6 = -4028;
pub const UV_ENOTTY: C2RustUnnamed_6 = -25;
pub const UV_EREMOTEIO: C2RustUnnamed_6 = -121;
pub const UV_EHOSTDOWN: C2RustUnnamed_6 = -112;
pub const UV_EMLINK: C2RustUnnamed_6 = -31;
pub const UV_ENXIO: C2RustUnnamed_6 = -6;
pub const UV_EOF: C2RustUnnamed_6 = -4095;
pub const UV_UNKNOWN: C2RustUnnamed_6 = -4094;
pub const UV_EXDEV: C2RustUnnamed_6 = -18;
pub const UV_ETXTBSY: C2RustUnnamed_6 = -26;
pub const UV_ETIMEDOUT: C2RustUnnamed_6 = -110;
pub const UV_ESRCH: C2RustUnnamed_6 = -3;
pub const UV_ESPIPE: C2RustUnnamed_6 = -29;
pub const UV_ESHUTDOWN: C2RustUnnamed_6 = -108;
pub const UV_EROFS: C2RustUnnamed_6 = -30;
pub const UV_ERANGE: C2RustUnnamed_6 = -34;
pub const UV_EPROTOTYPE: C2RustUnnamed_6 = -91;
pub const UV_EPROTONOSUPPORT: C2RustUnnamed_6 = -93;
pub const UV_EPROTO: C2RustUnnamed_6 = -71;
pub const UV_EPIPE: C2RustUnnamed_6 = -32;
pub const UV_EPERM: C2RustUnnamed_6 = -1;
pub const UV_EOVERFLOW: C2RustUnnamed_6 = -75;
pub const UV_ENOTSUP: C2RustUnnamed_6 = -95;
pub const UV_ENOTSOCK: C2RustUnnamed_6 = -88;
pub const UV_ENOTEMPTY: C2RustUnnamed_6 = -39;
pub const UV_ENOTDIR: C2RustUnnamed_6 = -20;
pub const UV_ENOTCONN: C2RustUnnamed_6 = -107;
pub const UV_ENOSYS: C2RustUnnamed_6 = -38;
pub const UV_ENOSPC: C2RustUnnamed_6 = -28;
pub const UV_ENOPROTOOPT: C2RustUnnamed_6 = -92;
pub const UV_ENONET: C2RustUnnamed_6 = -64;
pub const UV_ENOMEM: C2RustUnnamed_6 = -12;
pub const UV_ENOENT: C2RustUnnamed_6 = -2;
pub const UV_ENODEV: C2RustUnnamed_6 = -19;
pub const UV_ENOBUFS: C2RustUnnamed_6 = -105;
pub const UV_ENFILE: C2RustUnnamed_6 = -23;
pub const UV_ENETUNREACH: C2RustUnnamed_6 = -101;
pub const UV_ENETDOWN: C2RustUnnamed_6 = -100;
pub const UV_ENAMETOOLONG: C2RustUnnamed_6 = -36;
pub const UV_EMSGSIZE: C2RustUnnamed_6 = -90;
pub const UV_EMFILE: C2RustUnnamed_6 = -24;
pub const UV_ELOOP: C2RustUnnamed_6 = -40;
pub const UV_EISDIR: C2RustUnnamed_6 = -21;
pub const UV_EISCONN: C2RustUnnamed_6 = -106;
pub const UV_EIO: C2RustUnnamed_6 = -5;
pub const UV_EINVAL: C2RustUnnamed_6 = -22;
pub const UV_EINTR: C2RustUnnamed_6 = -4;
pub const UV_EHOSTUNREACH: C2RustUnnamed_6 = -113;
pub const UV_EFBIG: C2RustUnnamed_6 = -27;
pub const UV_EFAULT: C2RustUnnamed_6 = -14;
pub const UV_EEXIST: C2RustUnnamed_6 = -17;
pub const UV_EDESTADDRREQ: C2RustUnnamed_6 = -89;
pub const UV_ECONNRESET: C2RustUnnamed_6 = -104;
pub const UV_ECONNREFUSED: C2RustUnnamed_6 = -111;
pub const UV_ECONNABORTED: C2RustUnnamed_6 = -103;
pub const UV_ECHARSET: C2RustUnnamed_6 = -4080;
pub const UV_ECANCELED: C2RustUnnamed_6 = -125;
pub const UV_EBUSY: C2RustUnnamed_6 = -16;
pub const UV_EBADF: C2RustUnnamed_6 = -9;
pub const UV_EALREADY: C2RustUnnamed_6 = -114;
pub const UV_EAI_SOCKTYPE: C2RustUnnamed_6 = -3011;
pub const UV_EAI_SERVICE: C2RustUnnamed_6 = -3010;
pub const UV_EAI_PROTOCOL: C2RustUnnamed_6 = -3014;
pub const UV_EAI_OVERFLOW: C2RustUnnamed_6 = -3009;
pub const UV_EAI_NONAME: C2RustUnnamed_6 = -3008;
pub const UV_EAI_NODATA: C2RustUnnamed_6 = -3007;
pub const UV_EAI_MEMORY: C2RustUnnamed_6 = -3006;
pub const UV_EAI_FAMILY: C2RustUnnamed_6 = -3005;
pub const UV_EAI_FAIL: C2RustUnnamed_6 = -3004;
pub const UV_EAI_CANCELED: C2RustUnnamed_6 = -3003;
pub const UV_EAI_BADHINTS: C2RustUnnamed_6 = -3013;
pub const UV_EAI_BADFLAGS: C2RustUnnamed_6 = -3002;
pub const UV_EAI_AGAIN: C2RustUnnamed_6 = -3001;
pub const UV_EAI_ADDRFAMILY: C2RustUnnamed_6 = -3000;
pub const UV_EAGAIN: C2RustUnnamed_6 = -11;
pub const UV_EAFNOSUPPORT: C2RustUnnamed_6 = -97;
pub const UV_EADDRNOTAVAIL: C2RustUnnamed_6 = -99;
pub const UV_EADDRINUSE: C2RustUnnamed_6 = -98;
pub const UV_EACCES: C2RustUnnamed_6 = -13;
pub const UV_E2BIG: C2RustUnnamed_6 = -7;
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
    pub u: C2RustUnnamed_7,
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
pub union C2RustUnnamed_7 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_pipe_s {
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
    pub ipc: ::core::ffi::c_int,
    pub pipe_fname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_pipe_t = uv_pipe_s;
pub const UV_NONBLOCK_PIPE: C2RustUnnamed_10 = 64;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const UV_PIPE_NO_TRUNCATE: C2RustUnnamed_9 = 1;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_11 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_11 = 16384;
pub const UV_HANDLE_BOUND: C2RustUnnamed_11 = 8192;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_11 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_11 = 1;
pub type uv__peersockfunc = Option<
    unsafe extern "C" fn(::core::ffi::c_int, *mut sockaddr, *mut socklen_t) -> ::core::ffi::c_int,
>;
pub type uv__stream_queued_fds_t = uv__stream_queued_fds_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__stream_queued_fds_s {
    pub size: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub fds: [::core::ffi::c_int; 1],
}
pub const UV_WRITABLE: uv_poll_event = 2;
pub const UV_READABLE: uv_poll_event = 1;
pub type uv_poll_event = ::core::ffi::c_uint;
pub const UV_PRIORITIZED: uv_poll_event = 8;
pub const UV_DISCONNECT: uv_poll_event = 4;
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
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_11 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_11 = 16777216;
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
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EINPROGRESS: ::core::ffi::c_int = 115 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const O_ACCMODE: ::core::ffi::c_int = 0o3 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IWGRP: ::core::ffi::c_int = S_IWUSR >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IWOTH: ::core::ffi::c_int = S_IWGRP >> 3 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const UV_FS_O_NONBLOCK: ::core::ffi::c_int = O_NONBLOCK;
#[inline]
unsafe extern "C" fn uv__queue_init(mut q: *mut uv__queue) {
    (*q).next = q;
    (*q).prev = q;
}
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
unsafe extern "C" fn uv__stat(
    mut path: *const ::core::ffi::c_char,
    mut s: *mut stat,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0;
    rc = stat(path, s);
    rc >= 0 as ::core::ffi::c_int;
    return rc;
}
unsafe extern "C" fn includes_nul(
    mut s: *const ::core::ffi::c_char,
    mut n: size_t,
) -> ::core::ffi::c_int {
    if n == 0 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    s = s.offset(1);
    n = n.wrapping_sub(1);
    return (NULL != memchr(s as *const ::core::ffi::c_void, '\0' as i32, n)) as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_pipe_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_pipe_t,
    mut ipc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    uv__stream_init(loop_0, handle as *mut uv_stream_t, UV_NAMED_PIPE);
    (*handle).shutdown_req = ::core::ptr::null_mut::<uv_shutdown_t>();
    (*handle).connect_req = ::core::ptr::null_mut::<uv_connect_t>();
    (*handle).pipe_fname = ::core::ptr::null::<::core::ffi::c_char>();
    (*handle).ipc = ipc;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_pipe_bind(
    mut handle: *mut uv_pipe_t,
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return uv_pipe_bind2(handle, name, strlen(name), 0 as ::core::ffi::c_uint);
}
pub(crate) unsafe fn uv_pipe_bind2(
    mut handle: *mut uv_pipe_t,
    mut name: *const ::core::ffi::c_char,
    mut namelen: size_t,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut saddr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut pipe_fname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sockfd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut addrlen: socklen_t = 0;
    pipe_fname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if flags & !(UV_PIPE_NO_TRUNCATE as ::core::ffi::c_int) as ::core::ffi::c_uint != 0 {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if name.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if namelen == 0 as size_t {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if includes_nul(name, namelen) != 0 {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if flags & UV_PIPE_NO_TRUNCATE as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        if namelen > ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize {
            return UV_EINVAL as ::core::ffi::c_int;
        }
    }
    if namelen > ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize {
        namelen = ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize as size_t;
    }
    if (*handle).io_watcher.fd >= 0 as ::core::ffi::c_int {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if (*handle).flags
        & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
            as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if *name as ::core::ffi::c_int == '\0' as i32 {
        addrlen = (2 as size_t).wrapping_add(namelen) as socklen_t;
    } else {
        pipe_fname = uv__malloc(namelen.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if pipe_fname.is_null() {
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        memcpy(
            pipe_fname as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            namelen,
        );
        *pipe_fname.offset(namelen as isize) = '\0' as i32 as ::core::ffi::c_char;
        addrlen = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
    }
    err = uv__socket(
        AF_UNIX,
        SOCK_STREAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if !(err < 0 as ::core::ffi::c_int) {
        sockfd = err;
        memset(
            &raw mut saddr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_un>() as size_t,
        );
        memcpy(
            &raw mut saddr.sun_path as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            namelen,
        );
        saddr.sun_family = AF_UNIX as sa_family_t;
        if bind(
            sockfd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut saddr as *mut sockaddr,
            },
            addrlen,
        ) != 0
        {
            err = -*__errno_location();
            if err == UV_ENOENT as ::core::ffi::c_int {
                err = UV_EACCES as ::core::ffi::c_int;
            }
            uv__close(sockfd);
        } else {
            (*handle).flags |= UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint;
            (*handle).pipe_fname = pipe_fname;
            (*handle).io_watcher.fd = sockfd;
            return 0 as ::core::ffi::c_int;
        }
    }
    uv__free(pipe_fname as *mut ::core::ffi::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn uv__pipe_listen(
    mut handle: *mut uv_pipe_t,
    mut backlog: ::core::ffi::c_int,
    mut cb: uv_connection_cb,
) -> ::core::ffi::c_int {
    if (*handle).io_watcher.fd == -(1 as ::core::ffi::c_int) {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if (*handle).ipc != 0 {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if listen((*handle).io_watcher.fd, backlog) != 0 {
        return -*__errno_location();
    }
    (*handle).connection_cb = cb;
    (*handle).io_watcher.cb = Some(
        uv__server_io
            as unsafe extern "C" fn(*mut uv_loop_t, *mut uv__io_t, ::core::ffi::c_uint) -> (),
    ) as uv__io_cb;
    uv__io_start(
        (*handle).loop_0,
        &raw mut (*handle).io_watcher,
        POLLIN as ::core::ffi::c_uint,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__pipe_close(mut handle: *mut uv_pipe_t) {
    if !(*handle).pipe_fname.is_null() {
        unlink((*handle).pipe_fname);
        uv__free((*handle).pipe_fname as *mut ::core::ffi::c_void);
        (*handle).pipe_fname = ::core::ptr::null::<::core::ffi::c_char>();
    }
    uv__stream_close(handle as *mut uv_stream_t);
}
pub(crate) unsafe fn uv_pipe_open(
    mut handle: *mut uv_pipe_t,
    mut fd: uv_file,
) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0;
    let mut mode: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    flags = 0 as ::core::ffi::c_int;
    if uv__fd_exists((*handle).loop_0, fd as ::core::ffi::c_int) != 0 {
        return UV_EEXIST as ::core::ffi::c_int;
    }
    loop {
        mode = fcntl(fd as ::core::ffi::c_int, F_GETFL);
        if !(mode == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if mode == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    err = uv__nonblock_ioctl(fd as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    if err != 0 {
        return err;
    }
    mode &= O_ACCMODE;
    if mode != O_WRONLY {
        flags |= UV_HANDLE_READABLE as ::core::ffi::c_int;
    }
    if mode != O_RDONLY {
        flags |= UV_HANDLE_WRITABLE as ::core::ffi::c_int;
    }
    return uv__stream_open(handle as *mut uv_stream_t, fd as ::core::ffi::c_int, flags);
}
pub(crate) unsafe fn uv_pipe_connect(
    mut req: *mut uv_connect_t,
    mut handle: *mut uv_pipe_t,
    mut name: *const ::core::ffi::c_char,
    mut cb: uv_connect_cb,
) {
    let mut err: ::core::ffi::c_int = 0;
    err = uv_pipe_connect2(
        req,
        handle,
        name,
        strlen(name),
        0 as ::core::ffi::c_uint,
        cb,
    );
    if err != 0 {
        (*handle).delayed_error = err;
        (*handle).connect_req = req;
        (*req).type_0 = UV_CONNECT;
        (*(*handle).loop_0).active_reqs.count =
            (*(*handle).loop_0).active_reqs.count.wrapping_add(1);
        (*req).handle = handle as *mut uv_stream_t;
        (*req).cb = cb;
        uv__queue_init(&raw mut (*req).queue);
        uv__io_feed((*handle).loop_0, &raw mut (*handle).io_watcher);
    }
}
pub(crate) unsafe fn uv_pipe_connect2(
    mut req: *mut uv_connect_t,
    mut handle: *mut uv_pipe_t,
    mut name: *const ::core::ffi::c_char,
    mut namelen: size_t,
    mut flags: ::core::ffi::c_uint,
    mut cb: uv_connect_cb,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut saddr: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut new_sock: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut addrlen: socklen_t = 0;
    if flags & !(UV_PIPE_NO_TRUNCATE as ::core::ffi::c_int) as ::core::ffi::c_uint != 0 {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if name.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if namelen == 0 as size_t {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if includes_nul(name, namelen) != 0 {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if flags & UV_PIPE_NO_TRUNCATE as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        if namelen > ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize {
            return UV_EINVAL as ::core::ffi::c_int;
        }
    }
    if namelen > ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize {
        namelen = ::core::mem::size_of::<[::core::ffi::c_char; 108]>() as usize as size_t;
    }
    new_sock = ((*handle).io_watcher.fd == -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
    if new_sock != 0 {
        err = uv__socket(
            AF_UNIX,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if err < 0 as ::core::ffi::c_int {
            current_block = 16426492275326419079;
        } else {
            (*handle).io_watcher.fd = err;
            current_block = 17407779659766490442;
        }
    } else {
        current_block = 17407779659766490442;
    }
    match current_block {
        17407779659766490442 => {
            memset(
                &raw mut saddr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_un>() as size_t,
            );
            memcpy(
                &raw mut saddr.sun_path as *mut ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                namelen,
            );
            saddr.sun_family = AF_UNIX as sa_family_t;
            if *name as ::core::ffi::c_int == '\0' as i32 {
                addrlen = (2 as size_t).wrapping_add(namelen) as socklen_t;
            } else {
                addrlen = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
            }
            loop {
                r = connect(
                    (*handle).io_watcher.fd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: &raw mut saddr as *mut sockaddr,
                    },
                    addrlen,
                );
                if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if r == -(1 as ::core::ffi::c_int) && *__errno_location() != EINPROGRESS {
                err = -*__errno_location();
            } else {
                err = 0 as ::core::ffi::c_int;
                if new_sock != 0 {
                    err = uv__stream_open(
                        handle as *mut uv_stream_t,
                        (*handle).io_watcher.fd,
                        UV_HANDLE_READABLE as ::core::ffi::c_int
                            | UV_HANDLE_WRITABLE as ::core::ffi::c_int,
                    );
                }
                if err == 0 as ::core::ffi::c_int {
                    uv__io_start(
                        (*handle).loop_0,
                        &raw mut (*handle).io_watcher,
                        POLLOUT as ::core::ffi::c_uint,
                    );
                }
            }
        }
        _ => {}
    }
    (*handle).delayed_error = err;
    (*handle).connect_req = req;
    (*req).type_0 = UV_CONNECT;
    (*(*handle).loop_0).active_reqs.count = (*(*handle).loop_0).active_reqs.count.wrapping_add(1);
    (*req).handle = handle as *mut uv_stream_t;
    (*req).cb = cb;
    uv__queue_init(&raw mut (*req).queue);
    if err != 0 {
        uv__io_feed((*handle).loop_0, &raw mut (*handle).io_watcher);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__pipe_getsockpeername(
    mut handle: *const uv_pipe_t,
    mut func: uv__peersockfunc,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut addrlen: socklen_t = 0;
    let mut err: ::core::ffi::c_int = 0;
    addrlen = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
    memset(
        &raw mut sa as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        addrlen as size_t,
    );
    err = uv__getsockpeername(
        handle as *const uv_handle_t,
        func,
        &raw mut sa as *mut sockaddr,
        &raw mut addrlen as *mut ::core::ffi::c_int,
    );
    if err < 0 as ::core::ffi::c_int {
        *size = 0 as size_t;
        return err;
    }
    if sa.sun_path[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        addrlen = (addrlen as ::core::ffi::c_ulong).wrapping_sub(2 as ::core::ffi::c_ulong)
            as socklen_t as socklen_t;
    } else {
        addrlen = strlen(&raw mut sa.sun_path as *mut ::core::ffi::c_char) as socklen_t;
    }
    if addrlen as size_t >= *size {
        *size = addrlen.wrapping_add(1 as socklen_t) as size_t;
        return UV_ENOBUFS as ::core::ffi::c_int;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        &raw mut sa.sun_path as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        addrlen as size_t,
    );
    *size = addrlen as size_t;
    if *buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32 {
        *buffer.offset(addrlen as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_pipe_getsockname(
    mut handle: *const uv_pipe_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    return uv__pipe_getsockpeername(
        handle,
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
        buffer,
        size,
    );
}
pub(crate) unsafe fn uv_pipe_getpeername(
    mut handle: *const uv_pipe_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    return uv__pipe_getsockpeername(
        handle,
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
        buffer,
        size,
    );
}
pub(crate) unsafe fn uv_pipe_pending_instances(
    mut handle: *mut uv_pipe_t,
    mut count: ::core::ffi::c_int,
) {
}
pub(crate) unsafe fn uv_pipe_pending_count(mut handle: *mut uv_pipe_t) -> ::core::ffi::c_int {
    let mut queued_fds: *mut uv__stream_queued_fds_t =
        ::core::ptr::null_mut::<uv__stream_queued_fds_t>();
    if (*handle).ipc == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*handle).accepted_fd == -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    if (*handle).queued_fds.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    queued_fds = (*handle).queued_fds as *mut uv__stream_queued_fds_t;
    return (*queued_fds).offset.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_pipe_pending_type(mut handle: *mut uv_pipe_t) -> uv_handle_type {
    if (*handle).ipc == 0 {
        return UV_UNKNOWN_HANDLE;
    }
    if (*handle).accepted_fd == -(1 as ::core::ffi::c_int) {
        return UV_UNKNOWN_HANDLE;
    } else {
        return uv_guess_handle((*handle).accepted_fd as uv_file);
    };
}
pub(crate) unsafe fn uv_pipe_chmod(
    mut handle: *mut uv_pipe_t,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut desired_mode: ::core::ffi::c_uint = 0;
    let mut pipe_stat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut name_buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut name_len: size_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    if handle.is_null() || (*handle).io_watcher.fd == -(1 as ::core::ffi::c_int) {
        return UV_EBADF as ::core::ffi::c_int;
    }
    if mode != UV_READABLE as ::core::ffi::c_int
        && mode != UV_WRITABLE as ::core::ffi::c_int
        && mode != UV_WRITABLE as ::core::ffi::c_int | UV_READABLE as ::core::ffi::c_int
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    name_len = 0 as size_t;
    r = uv_pipe_getsockname(
        handle,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        &raw mut name_len,
    );
    if r != UV_ENOBUFS as ::core::ffi::c_int {
        return r;
    }
    name_buffer = uv__malloc(name_len) as *mut ::core::ffi::c_char;
    if name_buffer.is_null() {
        return UV_ENOMEM as ::core::ffi::c_int;
    }
    r = uv_pipe_getsockname(handle, name_buffer, &raw mut name_len);
    if r != 0 as ::core::ffi::c_int {
        uv__free(name_buffer as *mut ::core::ffi::c_void);
        return r;
    }
    if uv__stat(name_buffer, &raw mut pipe_stat) == -(1 as ::core::ffi::c_int) {
        uv__free(name_buffer as *mut ::core::ffi::c_void);
        return -*__errno_location();
    }
    desired_mode = 0 as ::core::ffi::c_uint;
    if mode & UV_READABLE as ::core::ffi::c_int != 0 {
        desired_mode |= (S_IRUSR | S_IRGRP | S_IROTH) as ::core::ffi::c_uint;
    }
    if mode & UV_WRITABLE as ::core::ffi::c_int != 0 {
        desired_mode |= (S_IWUSR | S_IWGRP | S_IWOTH) as ::core::ffi::c_uint;
    }
    if pipe_stat.st_mode as ::core::ffi::c_uint & desired_mode == desired_mode {
        uv__free(name_buffer as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    pipe_stat.st_mode |= desired_mode;
    r = chmod(name_buffer, pipe_stat.st_mode);
    uv__free(name_buffer as *mut ::core::ffi::c_void);
    return if r != -(1 as ::core::ffi::c_int) {
        0 as ::core::ffi::c_int
    } else {
        -*__errno_location()
    };
}
pub(crate) unsafe fn uv_pipe(
    mut fds: *mut uv_os_fd_t,
    mut read_flags: ::core::ffi::c_int,
    mut write_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut temp: [uv_os_fd_t; 2] = [0; 2];
    let mut err: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = O_CLOEXEC;
    if read_flags & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0
        && write_flags & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0
    {
        flags |= UV_FS_O_NONBLOCK;
    }
    if pipe2(&raw mut temp as *mut ::core::ffi::c_int, flags) != 0 {
        return -*__errno_location();
    }
    if flags & UV_FS_O_NONBLOCK != 0 {
        *fds.offset(0 as ::core::ffi::c_int as isize) = temp[0 as ::core::ffi::c_int as usize];
        *fds.offset(1 as ::core::ffi::c_int as isize) = temp[1 as ::core::ffi::c_int as usize];
        return 0 as ::core::ffi::c_int;
    }
    if read_flags & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0 {
        err = uv__nonblock_ioctl(
            temp[0 as ::core::ffi::c_int as usize],
            1 as ::core::ffi::c_int,
        );
        if err != 0 {
            current_block = 6504592942649250576;
        } else {
            current_block = 5720623009719927633;
        }
    } else {
        current_block = 5720623009719927633;
    }
    match current_block {
        5720623009719927633 => {
            if write_flags & UV_NONBLOCK_PIPE as ::core::ffi::c_int != 0 {
                err = uv__nonblock_ioctl(
                    temp[1 as ::core::ffi::c_int as usize],
                    1 as ::core::ffi::c_int,
                );
                if err != 0 {
                    current_block = 6504592942649250576;
                } else {
                    current_block = 11650488183268122163;
                }
            } else {
                current_block = 11650488183268122163;
            }
            match current_block {
                6504592942649250576 => {}
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
#[no_mangle]
pub unsafe extern "C" fn uv__make_pipe(
    mut fds: *mut ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return uv_pipe(
        fds as *mut uv_os_fd_t,
        flags & UV_NONBLOCK_PIPE as ::core::ffi::c_int,
        flags & UV_NONBLOCK_PIPE as ::core::ffi::c_int,
    );
}

pub(crate) unsafe fn bind_pipe(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_bind(handle.cast(), name) }
}

pub(crate) unsafe fn bind2(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    namelen: usize,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_bind2(handle.cast(), name, namelen, flags) }
}

pub(crate) unsafe fn chmod_pipe(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_chmod(handle.cast(), flags) }
}

pub(crate) unsafe fn connect_pipe(
    req: *mut crate::abi::linux_x86_64::uv_connect_t,
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    cb: crate::abi::linux_x86_64::uv_connect_cb,
) {
    unsafe {
        uv_pipe_connect(
            req.cast(),
            handle.cast(),
            name,
            std::mem::transmute::<_, uv_connect_cb>(cb),
        );
    }
}

pub(crate) unsafe fn connect2(
    req: *mut crate::abi::linux_x86_64::uv_connect_t,
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    namelen: usize,
    flags: ::std::os::raw::c_uint,
    cb: crate::abi::linux_x86_64::uv_connect_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        uv_pipe_connect2(
            req.cast(),
            handle.cast(),
            name,
            namelen,
            flags,
            std::mem::transmute::<_, uv_connect_cb>(cb),
        )
    }
}

pub(crate) unsafe fn getpeername_pipe(
    handle: *const crate::abi::linux_x86_64::uv_pipe_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_getpeername(handle.cast(), buffer, size) }
}

pub(crate) unsafe fn getsockname_pipe(
    handle: *const crate::abi::linux_x86_64::uv_pipe_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_getsockname(handle.cast(), buffer, size) }
}

pub(crate) unsafe fn init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    ipc: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_init(loop_.cast(), handle.cast(), ipc) }
}

pub(crate) unsafe fn open(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    file: crate::abi::linux_x86_64::uv_file,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_open(handle.cast(), file) }
}

pub(crate) unsafe fn pending_instances(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
    count: ::std::os::raw::c_int,
) {
    unsafe { uv_pipe_pending_instances(handle.cast(), count) }
}

pub(crate) unsafe fn pending_count(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
) -> ::std::os::raw::c_int {
    unsafe { uv_pipe_pending_count(handle.cast()) }
}

pub(crate) unsafe fn pending_type(
    handle: *mut crate::abi::linux_x86_64::uv_pipe_t,
) -> crate::abi::linux_x86_64::uv_handle_type {
    unsafe {
        std::mem::transmute::<uv_handle_type, crate::abi::linux_x86_64::uv_handle_type>(
            uv_pipe_pending_type(handle.cast()),
        )
    }
}
