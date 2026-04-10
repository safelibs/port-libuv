#[repr(C)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
    fn sendmsg(
        __fd: ::core::ffi::c_int,
        __message: *const msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn shutdown(__fd: ::core::ffi::c_int, __how: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn uv_buf_init(base: *mut ::core::ffi::c_char, len: ::core::ffi::c_uint) -> uv_buf_t;
    fn uv_udp_open(handle: *mut uv_udp_t, sock: uv_os_sock_t) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn uv__count_bufs(bufs: *const uv_buf_t, nbufs: ::core::ffi::c_uint) -> size_t;
    fn uv__malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__free(ptr: *mut ::core::ffi::c_void);
    fn uv__realloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
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
    fn uv__recvmsg(fd: ::core::ffi::c_int, msg: *mut msghdr, flags: ::core::ffi::c_int) -> ssize_t;
    fn uv__getiovmax() -> ::core::ffi::c_int;
    fn uv__io_init(w: *mut uv__io_t, cb: uv__io_cb, fd: ::core::ffi::c_int);
    fn uv__io_start(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_stop(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_close(loop_0: *mut uv_loop_t, w: *mut uv__io_t);
    fn uv__io_feed(loop_0: *mut uv_loop_t, w: *mut uv__io_t);
    fn uv__io_active(w: *const uv__io_t, events: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn uv__accept(sockfd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__open_cloexec(
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv__tcp_listen(
        tcp: *mut uv_tcp_t,
        backlog: ::core::ffi::c_int,
        cb: uv_connection_cb,
    ) -> ::core::ffi::c_int;
    fn uv__tcp_nodelay(fd: ::core::ffi::c_int, on: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__tcp_keepalive(
        fd: ::core::ffi::c_int,
        on: ::core::ffi::c_int,
        delay: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn uv__pipe_listen(
        handle: *mut uv_pipe_t,
        backlog: ::core::ffi::c_int,
        cb: uv_connection_cb,
    ) -> ::core::ffi::c_int;
    fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut ::core::ffi::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::core::ffi::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::core::ffi::c_int,
    pub cmsg_type: ::core::ffi::c_int,
    pub __cmsg_data: [::core::ffi::c_uchar; 0],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const SCM_PIDFD: C2RustUnnamed = 4;
pub const SCM_SECURITY: C2RustUnnamed = 3;
pub const SCM_CREDENTIALS: C2RustUnnamed = 2;
pub const SCM_RIGHTS: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const SHUT_RDWR: C2RustUnnamed_0 = 2;
pub const SHUT_WR: C2RustUnnamed_0 = 1;
pub const SHUT_RD: C2RustUnnamed_0 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_udp_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_10,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub send_queue_size: size_t,
    pub send_queue_count: size_t,
    pub alloc_cb: uv_alloc_cb,
    pub recv_cb: uv_udp_recv_cb,
    pub io_watcher: uv__io_t,
    pub write_queue: uv__queue,
    pub write_completed_queue: uv__queue,
}
pub type uv_udp_recv_cb = Option<
    unsafe extern "C" fn(
        *mut uv_udp_t,
        ssize_t,
        *const uv_buf_t,
        *const sockaddr,
        ::core::ffi::c_uint,
    ) -> (),
>;
pub type uv_udp_t = uv_udp_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
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
    pub u: C2RustUnnamed_11,
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
pub union C2RustUnnamed_11 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_pipe_t = uv_pipe_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_write_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub cb: uv_write_cb,
    pub send_handle: *mut uv_stream_t,
    pub handle: *mut uv_stream_t,
    pub queue: uv__queue,
    pub write_index: ::core::ffi::c_uint,
    pub bufs: *mut uv_buf_t,
    pub nbufs: ::core::ffi::c_uint,
    pub error: ::core::ffi::c_int,
    pub bufsml: [uv_buf_t; 4],
}
pub type uv_write_cb = Option<unsafe extern "C" fn(*mut uv_write_t, ::core::ffi::c_int) -> ()>;
pub type uv_write_t = uv_write_s;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_12 = 32768;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_12 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_12 = 1;
pub const UV_HANDLE_SHUT: C2RustUnnamed_12 = 512;
pub const UV_HANDLE_REF: C2RustUnnamed_12 = 8;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_12 = 4;
pub type uv__stream_queued_fds_t = uv__stream_queued_fds_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__stream_queued_fds_s {
    pub size: ::core::ffi::c_uint,
    pub offset: ::core::ffi::c_uint,
    pub fds: [::core::ffi::c_int; 1],
}
pub const UV_HANDLE_BOUND: C2RustUnnamed_12 = 8192;
pub const UV_HANDLE_READABLE: C2RustUnnamed_12 = 16384;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_12 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_12 = 16777216;
pub const UV_HANDLE_READING: C2RustUnnamed_12 = 4096;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_12 = 1048576;
#[derive(Copy, Clone)]
#[repr(C)]
pub union uv__cmsg {
    pub hdr: cmsghdr,
    pub pad: [::core::ffi::c_char; 256],
}
pub type C2RustUnnamed_12 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_12 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_12 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_12 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_12 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_12 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_12 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_12 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_12 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_12 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_12 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_12 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_12 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_12 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_12 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_12 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_12 = 67108864;
pub const UV_HANDLE_IPV6: C2RustUnnamed_12 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_12 = 2097152;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_12 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_12 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_12 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_12 = 65536;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_12 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_12 = 1024;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_12 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_12 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_12 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_12 = 16;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const ENOBUFS: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_ERROR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_init(mut q: *mut uv__queue) {
    unsafe {
        (*q).next = q;
        (*q).prev = q;
    }
}
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_empty(mut q: *const uv__queue) -> ::core::ffi::c_int {
    unsafe {
        return (q == (*q).next as *const uv__queue) as ::core::ffi::c_int;
    }
}
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_head(mut q: *const uv__queue) -> *mut uv__queue {
    unsafe {
        return (*q).next;
    }
}
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_split(mut h: *mut uv__queue, mut q: *mut uv__queue, mut n: *mut uv__queue) {
    unsafe {
        (*n).prev = (*h).prev;
        (*(*n).prev).next = n;
        (*n).next = q;
        (*h).prev = (*q).prev;
        (*(*h).prev).next = h;
        (*q).prev = n;
    }
}
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_move(mut h: *mut uv__queue, mut n: *mut uv__queue) {
    unsafe {
        if uv__queue_empty(h) != 0 {
            uv__queue_init(n);
        } else {
            uv__queue_split(h, (*h).next, n);
        };
    }
}
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_insert_tail(mut h: *mut uv__queue, mut q: *mut uv__queue) {
    unsafe {
        (*q).next = h;
        (*q).prev = (*h).prev;
        (*(*q).prev).next = q;
        (*h).prev = q;
    }
}
#[inline]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__queue_remove(mut q: *mut uv__queue) {
    unsafe {
        (*(*q).prev).next = (*q).next;
        (*(*q).next).prev = (*q).prev;
    }
}
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__stream_init(
    mut loop_0: *mut uv_loop_t,
    mut stream: *mut uv_stream_t,
    mut type_0: uv_handle_type,
) {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        let ref mut fresh1 = (*(stream as *mut uv_handle_t)).loop_0;
        *fresh1 = loop_0;
        (*(stream as *mut uv_handle_t)).type_0 = type_0;
        (*(stream as *mut uv_handle_t)).flags =
            UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
        uv__queue_insert_tail(
            &raw mut (*loop_0).handle_queue,
            &raw mut (*(stream as *mut uv_handle_t)).handle_queue,
        );
        let ref mut fresh2 = (*(stream as *mut uv_handle_t)).next_closing;
        *fresh2 = ::core::ptr::null_mut::<uv_handle_t>();
        (*stream).read_cb = None;
        (*stream).alloc_cb = None;
        (*stream).close_cb = None;
        (*stream).connection_cb = None;
        (*stream).connect_req = ::core::ptr::null_mut::<uv_connect_t>();
        (*stream).shutdown_req = ::core::ptr::null_mut::<uv_shutdown_t>();
        (*stream).accepted_fd = -(1 as ::core::ffi::c_int);
        (*stream).queued_fds = NULL;
        (*stream).delayed_error = 0 as ::core::ffi::c_int;
        uv__queue_init(&raw mut (*stream).write_queue);
        uv__queue_init(&raw mut (*stream).write_completed_queue);
        (*stream).write_queue_size = 0 as size_t;
        if (*loop_0).emfile_fd == -(1 as ::core::ffi::c_int) {
            err = uv__open_cloexec(
                b"/dev/null\0" as *const u8 as *const ::core::ffi::c_char,
                O_RDONLY,
            );
            if err < 0 as ::core::ffi::c_int {
                err = uv__open_cloexec(b"/\0" as *const u8 as *const ::core::ffi::c_char, O_RDONLY);
            }
            if err >= 0 as ::core::ffi::c_int {
                (*loop_0).emfile_fd = err;
            }
        }
        uv__io_init(
            &raw mut (*stream).io_watcher,
            Some(
                uv__stream_io
                    as unsafe extern "C" fn(
                        *mut uv_loop_t,
                        *mut uv__io_t,
                        ::core::ffi::c_uint,
                    ) -> (),
            ),
            -(1 as ::core::ffi::c_int),
        );
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__stream_osx_interrupt_select(mut stream: *mut uv_stream_t) {
    unsafe {}
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__stream_open(
    mut stream: *mut uv_stream_t,
    mut fd: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if !((*stream).io_watcher.fd == -(1 as ::core::ffi::c_int) || (*stream).io_watcher.fd == fd)
        {
            return UV_EBUSY as ::core::ffi::c_int;
        }
        '_c2rust_label: {
            if fd >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    411 as ::core::ffi::c_uint,
                    b"int uv__stream_open(uv_stream_t *, int, int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*stream).flags |= flags as ::core::ffi::c_uint;
        if (*stream).type_0 as ::core::ffi::c_uint
            == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*stream).flags & UV_HANDLE_TCP_NODELAY as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && uv__tcp_nodelay(fd, 1 as ::core::ffi::c_int) != 0
            {
                return -*__errno_location();
            }
            if (*stream).flags
                & UV_HANDLE_TCP_KEEPALIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
                && uv__tcp_keepalive(fd, 1 as ::core::ffi::c_int, 60 as ::core::ffi::c_uint) != 0
            {
                return -*__errno_location();
            }
        }
        (*stream).io_watcher.fd = fd;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__stream_flush_write_queue(
    mut stream: *mut uv_stream_t,
    mut error: ::core::ffi::c_int,
) {
    unsafe {
        let mut req: *mut uv_write_t = ::core::ptr::null_mut::<uv_write_t>();
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        while uv__queue_empty(&raw mut (*stream).write_queue) == 0 {
            q = uv__queue_head(&raw mut (*stream).write_queue);
            uv__queue_remove(q);
            req = (q as *mut ::core::ffi::c_char).offset(-(88 as ::core::ffi::c_ulong as isize))
                as *mut uv_write_t;
            (*req).error = error;
            uv__queue_insert_tail(
                &raw mut (*stream).write_completed_queue,
                &raw mut (*req).queue,
            );
        }
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__stream_destroy(mut stream: *mut uv_stream_t) {
    unsafe {
        '_c2rust_label: {
            if uv__io_active(
                &raw mut (*stream).io_watcher,
                (0x1 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) == 0
            {
            } else {
                __assert_fail(
                    b"!uv__io_active(&stream->io_watcher, POLLIN | POLLOUT)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    456 as ::core::ffi::c_uint,
                    b"void uv__stream_destroy(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if (*stream).flags & UV_HANDLE_CLOSED as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            {
            } else {
                __assert_fail(
                    b"stream->flags & UV_HANDLE_CLOSED\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    457 as ::core::ffi::c_uint,
                    b"void uv__stream_destroy(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if !(*stream).connect_req.is_null() {
            '_c2rust_label_1: {
                if (*(*stream).loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
                } else {
                    __assert_fail(
                        b"uv__has_active_reqs(stream->loop)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        460 as ::core::ffi::c_uint,
                        b"void uv__stream_destroy(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*(*stream).loop_0).active_reqs.count =
                (*(*stream).loop_0).active_reqs.count.wrapping_sub(1);
            (*(*stream).connect_req)
                .cb
                .expect("non-null function pointer")(
                (*stream).connect_req,
                UV_ECANCELED as ::core::ffi::c_int,
            );
            (*stream).connect_req = ::core::ptr::null_mut::<uv_connect_t>();
        }
        uv__stream_flush_write_queue(stream, UV_ECANCELED as ::core::ffi::c_int);
        uv__write_callbacks(stream);
        uv__drain(stream);
        '_c2rust_label_2: {
            if (*stream).write_queue_size == 0 as size_t {
            } else {
                __assert_fail(
                    b"stream->write_queue_size == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    469 as ::core::ffi::c_uint,
                    b"void uv__stream_destroy(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__emfile_trick(
    mut loop_0: *mut uv_loop_t,
    mut accept_fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        let mut emfile_fd: ::core::ffi::c_int = 0;
        if (*loop_0).emfile_fd == -(1 as ::core::ffi::c_int) {
            return UV_EMFILE as ::core::ffi::c_int;
        }
        uv__close((*loop_0).emfile_fd);
        (*loop_0).emfile_fd = -(1 as ::core::ffi::c_int);
        loop {
            err = uv__accept(accept_fd);
            if err >= 0 as ::core::ffi::c_int {
                uv__close(err);
            }
            if !(err >= 0 as ::core::ffi::c_int || err == UV_EINTR as ::core::ffi::c_int) {
                break;
            }
        }
        emfile_fd = uv__open_cloexec(b"/\0" as *const u8 as *const ::core::ffi::c_char, O_RDONLY);
        if emfile_fd >= 0 as ::core::ffi::c_int {
            (*loop_0).emfile_fd = emfile_fd;
        }
        return err;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__server_io(
    mut loop_0: *mut uv_loop_t,
    mut w: *mut uv__io_t,
    mut events: ::core::ffi::c_uint,
) {
    unsafe {
        let mut stream: *mut uv_stream_t = ::core::ptr::null_mut::<uv_stream_t>();
        let mut err: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = 0;
        stream = (w as *mut ::core::ffi::c_char).offset(-(136 as ::core::ffi::c_ulong as isize))
            as *mut uv_stream_t;
        '_c2rust_label: {
            if events & 0x1 as ::core::ffi::c_uint != 0 {
            } else {
                __assert_fail(
                    b"events & POLLIN\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    514 as ::core::ffi::c_uint,
                    b"void uv__server_io(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if (*stream).accepted_fd == -(1 as ::core::ffi::c_int) {
            } else {
                __assert_fail(
                    b"stream->accepted_fd == -1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    515 as ::core::ffi::c_uint,
                    b"void uv__server_io(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_1: {
            if (*stream).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint == 0
            {
            } else {
                __assert_fail(
                    b"!(stream->flags & UV_HANDLE_CLOSING)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    516 as ::core::ffi::c_uint,
                    b"void uv__server_io(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        fd = (*stream).io_watcher.fd;
        err = uv__accept(fd);
        if err == UV_EMFILE as ::core::ffi::c_int || err == UV_ENFILE as ::core::ffi::c_int {
            err = uv__emfile_trick(loop_0, fd);
        }
        if err < 0 as ::core::ffi::c_int {
            return;
        }
        (*stream).accepted_fd = err;
        (*stream).connection_cb.expect("non-null function pointer")(
            stream,
            0 as ::core::ffi::c_int,
        );
        if (*stream).accepted_fd != -(1 as ::core::ffi::c_int) {
            uv__io_stop(
                loop_0,
                &raw mut (*stream).io_watcher,
                POLLIN as ::core::ffi::c_uint,
            );
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_accept(
    mut server: *mut uv_stream_t,
    mut client: *mut uv_stream_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut err: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if (*server).loop_0 == (*client).loop_0 {
            } else {
                __assert_fail(
                    b"server->loop == client->loop\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    539 as ::core::ffi::c_uint,
                    b"int uv_accept(uv_stream_t *, uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if (*server).accepted_fd == -(1 as ::core::ffi::c_int) {
            return UV_EAGAIN as ::core::ffi::c_int;
        }
        match (*client).type_0 as ::core::ffi::c_uint {
            7 | 12 => {
                err = uv__stream_open(
                    client,
                    (*server).accepted_fd,
                    UV_HANDLE_READABLE as ::core::ffi::c_int
                        | UV_HANDLE_WRITABLE as ::core::ffi::c_int,
                );
                if err != 0 {
                    uv__close((*server).accepted_fd);
                    current_block = 9336919489047625507;
                } else {
                    current_block = 17965632435239708295;
                }
            }
            15 => {
                err = uv_udp_open(
                    client as *mut uv_udp_t,
                    (*server).accepted_fd as uv_os_sock_t,
                );
                if err != 0 {
                    uv__close((*server).accepted_fd);
                    current_block = 9336919489047625507;
                } else {
                    current_block = 17965632435239708295;
                }
            }
            _ => return UV_EINVAL as ::core::ffi::c_int,
        }
        match current_block {
            17965632435239708295 => {
                (*client).flags |= UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
            _ => {}
        }
        if !(*server).queued_fds.is_null() {
            let mut queued_fds: *mut uv__stream_queued_fds_t =
                ::core::ptr::null_mut::<uv__stream_queued_fds_t>();
            queued_fds = (*server).queued_fds as *mut uv__stream_queued_fds_t;
            (*server).accepted_fd = *(&raw mut (*queued_fds).fds as *mut ::core::ffi::c_int)
                .offset(0 as ::core::ffi::c_int as isize);
            '_c2rust_label_0: {
                if (*queued_fds).offset > 0 as ::core::ffi::c_uint {
                } else {
                    __assert_fail(
                        b"queued_fds->offset > 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        582 as ::core::ffi::c_uint,
                        b"int uv_accept(uv_stream_t *, uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*queued_fds).offset = (*queued_fds).offset.wrapping_sub(1);
            if (*queued_fds).offset == 0 as ::core::ffi::c_uint {
                uv__free(queued_fds as *mut ::core::ffi::c_void);
                (*server).queued_fds = NULL;
            } else {
                memmove(
                    &raw mut (*queued_fds).fds as *mut ::core::ffi::c_int
                        as *mut ::core::ffi::c_void,
                    (&raw mut (*queued_fds).fds as *mut ::core::ffi::c_int)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    ((*queued_fds).offset as size_t)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
                );
            }
        } else {
            (*server).accepted_fd = -(1 as ::core::ffi::c_int);
            if err == 0 as ::core::ffi::c_int {
                uv__io_start(
                    (*server).loop_0,
                    &raw mut (*server).io_watcher,
                    POLLIN as ::core::ffi::c_uint,
                );
            }
        }
        return err;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_listen(
    mut stream: *mut uv_stream_t,
    mut backlog: ::core::ffi::c_int,
    mut cb: uv_connection_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        if (*stream).flags
            & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        match (*stream).type_0 as ::core::ffi::c_uint {
            12 => {
                err = uv__tcp_listen(stream as *mut uv_tcp_t, backlog, cb);
            }
            7 => {
                err = uv__pipe_listen(stream as *mut uv_pipe_t, backlog, cb);
            }
            _ => {
                err = UV_EINVAL as ::core::ffi::c_int;
            }
        }
        if err == 0 as ::core::ffi::c_int {
            if !((*stream).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint)
            {
                (*stream).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
                if (*stream).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    (*(*stream).loop_0).active_handles =
                        (*(*stream).loop_0).active_handles.wrapping_add(1);
                }
            }
        }
        return err;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__drain(mut stream: *mut uv_stream_t) {
    unsafe {
        let mut req: *mut uv_shutdown_t = ::core::ptr::null_mut::<uv_shutdown_t>();
        let mut err: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if uv__queue_empty(&raw mut (*stream).write_queue) != 0 {
            } else {
                __assert_fail(
                    b"uv__queue_empty(&stream->write_queue)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    630 as ::core::ffi::c_uint,
                    b"void uv__drain(uv_stream_t *)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if (*stream).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
            uv__io_stop(
                (*stream).loop_0,
                &raw mut (*stream).io_watcher,
                POLLOUT as ::core::ffi::c_uint,
            );
            uv__stream_osx_interrupt_select(stream);
        }
        if (*stream).shutdown_req.is_null() {
            return;
        }
        req = (*stream).shutdown_req;
        '_c2rust_label_0: {
            if !req.is_null() {
            } else {
                __assert_fail(
                    b"req\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    640 as ::core::ffi::c_uint,
                    b"void uv__drain(uv_stream_t *)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if (*stream).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            || (*stream).flags & UV_HANDLE_SHUT as ::core::ffi::c_int as ::core::ffi::c_uint == 0
        {
            (*stream).shutdown_req = ::core::ptr::null_mut::<uv_shutdown_t>();
            '_c2rust_label_1: {
                if (*(*stream).loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
                } else {
                    __assert_fail(
                        b"uv__has_active_reqs(stream->loop)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        645 as ::core::ffi::c_uint,
                        b"void uv__drain(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*(*stream).loop_0).active_reqs.count =
                (*(*stream).loop_0).active_reqs.count.wrapping_sub(1);
            err = 0 as ::core::ffi::c_int;
            if (*stream).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            {
                err = UV_ECANCELED as ::core::ffi::c_int;
            } else if shutdown((*stream).io_watcher.fd, SHUT_WR as ::core::ffi::c_int) != 0 {
                err = -*__errno_location();
            } else {
                (*stream).flags |= UV_HANDLE_SHUT as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
            if (*req).cb.is_some() {
                (*req).cb.expect("non-null function pointer")(req, err);
            }
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__writev(
    mut fd: ::core::ffi::c_int,
    mut vec: *mut iovec,
    mut n: size_t,
) -> ssize_t {
    unsafe {
        if n == 1 as size_t {
            return write(fd, (*vec).iov_base, (*vec).iov_len);
        } else {
            return writev(fd, vec, n as ::core::ffi::c_int);
        };
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__write_req_size(mut req: *mut uv_write_t) -> size_t {
    unsafe {
        let mut size: size_t = 0;
        '_c2rust_label: {
            if !(*req).bufs.is_null() {
            } else {
                __assert_fail(
                    b"req->bufs != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    673 as ::core::ffi::c_uint,
                    b"size_t uv__write_req_size(uv_write_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        size = uv__count_bufs(
            (*req).bufs.offset((*req).write_index as isize) as *const uv_buf_t,
            (*req).nbufs.wrapping_sub((*req).write_index),
        );
        '_c2rust_label_0: {
            if (*(*req).handle).write_queue_size >= size {
            } else {
                __assert_fail(
                    b"req->handle->write_queue_size >= size\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    676 as ::core::ffi::c_uint,
                    b"size_t uv__write_req_size(uv_write_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        return size;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__write_req_update(
    mut stream: *mut uv_stream_t,
    mut req: *mut uv_write_t,
    mut n: size_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut buf: *mut uv_buf_t = ::core::ptr::null_mut::<uv_buf_t>();
        let mut len: size_t = 0;
        '_c2rust_label: {
            if n <= (*stream).write_queue_size {
            } else {
                __assert_fail(
                    b"n <= stream->write_queue_size\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    694 as ::core::ffi::c_uint,
                    b"int uv__write_req_update(uv_stream_t *, uv_write_t *, size_t)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*stream).write_queue_size = (*stream).write_queue_size.wrapping_sub(n);
        buf = (*req).bufs.offset((*req).write_index as isize);
        loop {
            len = if n < (*buf).len { n } else { (*buf).len };
            (*buf).base = (*buf).base.offset(len as isize);
            (*buf).len = (*buf).len.wrapping_sub(len);
            buf = buf.offset(((*buf).len == 0 as size_t) as ::core::ffi::c_int as isize);
            n = n.wrapping_sub(len);
            if !(n > 0 as size_t) {
                break;
            }
        }
        (*req).write_index =
            buf.offset_from((*req).bufs) as ::core::ffi::c_long as ::core::ffi::c_uint;
        return ((*req).write_index == (*req).nbufs) as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__write_req_finish(mut req: *mut uv_write_t) {
    unsafe {
        let mut stream: *mut uv_stream_t = (*req).handle;
        uv__queue_remove(&raw mut (*req).queue);
        if (*req).error == 0 as ::core::ffi::c_int {
            if (*req).bufs != &raw mut (*req).bufsml as *mut uv_buf_t {
                uv__free((*req).bufs as *mut ::core::ffi::c_void);
            }
            (*req).bufs = ::core::ptr::null_mut::<uv_buf_t>();
        }
        uv__queue_insert_tail(
            &raw mut (*stream).write_completed_queue,
            &raw mut (*req).queue,
        );
        uv__io_feed((*stream).loop_0, &raw mut (*stream).io_watcher);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__handle_fd(mut handle: *mut uv_handle_t) -> ::core::ffi::c_int {
    unsafe {
        match (*handle).type_0 as ::core::ffi::c_uint {
            7 | 12 => return (*(handle as *mut uv_stream_t)).io_watcher.fd,
            15 => return (*(handle as *mut uv_udp_t)).io_watcher.fd,
            _ => return -(1 as ::core::ffi::c_int),
        };
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__try_write(
    mut stream: *mut uv_stream_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut send_handle: *mut uv_stream_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut iov: *mut iovec = ::core::ptr::null_mut::<iovec>();
        let mut iovmax: ::core::ffi::c_int = 0;
        let mut iovcnt: ::core::ffi::c_int = 0;
        let mut n: ssize_t = 0;
        iov = bufs as *mut iovec;
        iovcnt = nbufs as ::core::ffi::c_int;
        iovmax = uv__getiovmax();
        if iovcnt > iovmax {
            iovcnt = iovmax;
        }
        if !send_handle.is_null() {
            let mut fd_to_send: ::core::ffi::c_int = 0;
            let mut msg: msghdr = msghdr {
                msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                msg_namelen: 0,
                msg_iov: ::core::ptr::null_mut::<iovec>(),
                msg_iovlen: 0,
                msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                msg_controllen: 0,
                msg_flags: 0,
            };
            let mut cmsg: uv__cmsg = uv__cmsg {
                hdr: cmsghdr {
                    cmsg_len: 0,
                    cmsg_level: 0,
                    cmsg_type: 0,
                    __cmsg_data: [],
                },
            };
            if (*send_handle).flags
                & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                return UV_EBADF as ::core::ffi::c_int;
            }
            fd_to_send = uv__handle_fd(send_handle as *mut uv_handle_t);
            memset(
                &raw mut cmsg as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<uv__cmsg>() as size_t,
            );
            '_c2rust_label: {
                if fd_to_send >= 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"fd_to_send >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        791 as ::core::ffi::c_uint,
                        b"int uv__try_write(uv_stream_t *, const uv_buf_t *, unsigned int, uv_stream_t *)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            msg.msg_name = NULL;
            msg.msg_namelen = 0 as socklen_t;
            msg.msg_iov = iov;
            msg.msg_iovlen = iovcnt as size_t;
            msg.msg_flags = 0 as ::core::ffi::c_int;
            msg.msg_control = &raw mut cmsg.hdr as *mut ::core::ffi::c_void;
            msg.msg_controllen = ((::core::mem::size_of::<::core::ffi::c_int>() as usize)
                .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                .wrapping_sub(1 as usize)
                & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
            .wrapping_add(
                (::core::mem::size_of::<cmsghdr>() as usize)
                    .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                    .wrapping_sub(1 as usize)
                    & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize),
            ) as size_t;
            cmsg.hdr.cmsg_level = SOL_SOCKET;
            cmsg.hdr.cmsg_type = SCM_RIGHTS as ::core::ffi::c_int;
            cmsg.hdr.cmsg_len = ((::core::mem::size_of::<cmsghdr>() as usize)
                .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                .wrapping_sub(1 as usize)
                & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
            .wrapping_add(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                as size_t;
            memcpy(
                &raw mut cmsg.hdr.__cmsg_data as *mut ::core::ffi::c_uchar
                    as *mut ::core::ffi::c_void,
                &raw mut fd_to_send as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
            );
            loop {
                n = sendmsg(
                    (*stream).io_watcher.fd,
                    &raw mut msg,
                    0 as ::core::ffi::c_int,
                );
                if !(n == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                    break;
                }
            }
        } else {
            loop {
                n = uv__writev((*stream).io_watcher.fd, iov, iovcnt as size_t);
                if !(n == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                    break;
                }
            }
        }
        if n >= 0 as ssize_t {
            return n as ::core::ffi::c_int;
        }
        if *__errno_location() == EAGAIN
            || *__errno_location() == EWOULDBLOCK
            || *__errno_location() == ENOBUFS
        {
            return UV_EAGAIN as ::core::ffi::c_int;
        }
        return -*__errno_location();
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__write(mut stream: *mut uv_stream_t) {
    unsafe {
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut req: *mut uv_write_t = ::core::ptr::null_mut::<uv_write_t>();
        let mut n: ssize_t = 0;
        let mut count: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if (*stream).io_watcher.fd >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"uv__stream_fd(stream) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    845 as ::core::ffi::c_uint,
                    b"void uv__write(uv_stream_t *)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        count = 32 as ::core::ffi::c_int;
        loop {
            if uv__queue_empty(&raw mut (*stream).write_queue) != 0 {
                return;
            }
            q = uv__queue_head(&raw mut (*stream).write_queue);
            req = (q as *mut ::core::ffi::c_char).offset(-(88 as ::core::ffi::c_ulong as isize))
                as *mut uv_write_t;
            '_c2rust_label_0: {
                if (*req).handle == stream {
                } else {
                    __assert_fail(
                        b"req->handle == stream\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        859 as ::core::ffi::c_uint,
                        b"void uv__write(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            n = uv__try_write(
                stream,
                (*req).bufs.offset((*req).write_index as isize) as *mut uv_buf_t as *const uv_buf_t,
                (*req).nbufs.wrapping_sub((*req).write_index),
                (*req).send_handle,
            ) as ssize_t;
            if n >= 0 as ssize_t {
                (*req).send_handle = ::core::ptr::null_mut::<uv_stream_t>();
                if uv__write_req_update(stream, req, n as size_t) != 0 {
                    uv__write_req_finish(req);
                    let fresh0 = count;
                    count = count - 1;
                    if fresh0 > 0 as ::core::ffi::c_int {
                        continue;
                    }
                    return;
                }
            } else if n != UV_EAGAIN as ::core::ffi::c_int as ssize_t {
                break;
            }
            if (*stream).flags
                & UV_HANDLE_BLOCKING_WRITES as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                continue;
            }
            uv__io_start(
                (*stream).loop_0,
                &raw mut (*stream).io_watcher,
                POLLOUT as ::core::ffi::c_uint,
            );
            uv__stream_osx_interrupt_select(stream);
            return;
        }
        (*req).error = n as ::core::ffi::c_int;
        uv__write_req_finish(req);
        uv__io_stop(
            (*stream).loop_0,
            &raw mut (*stream).io_watcher,
            POLLOUT as ::core::ffi::c_uint,
        );
        uv__stream_osx_interrupt_select(stream);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__write_callbacks(mut stream: *mut uv_stream_t) {
    unsafe {
        let mut req: *mut uv_write_t = ::core::ptr::null_mut::<uv_write_t>();
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut pq: uv__queue = uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        };
        if uv__queue_empty(&raw mut (*stream).write_completed_queue) != 0 {
            return;
        }
        uv__queue_move(&raw mut (*stream).write_completed_queue, &raw mut pq);
        while uv__queue_empty(&raw mut pq) == 0 {
            q = uv__queue_head(&raw mut pq);
            req = (q as *mut ::core::ffi::c_char).offset(-(88 as ::core::ffi::c_ulong as isize))
                as *mut uv_write_t;
            uv__queue_remove(q);
            '_c2rust_label: {
                if (*(*stream).loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
                } else {
                    __assert_fail(
                        b"uv__has_active_reqs(stream->loop)\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        915 as ::core::ffi::c_uint,
                        b"void uv__write_callbacks(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*(*stream).loop_0).active_reqs.count =
                (*(*stream).loop_0).active_reqs.count.wrapping_sub(1);
            if !(*req).bufs.is_null() {
                (*stream).write_queue_size = (*stream)
                    .write_queue_size
                    .wrapping_sub(uv__write_req_size(req));
                if (*req).bufs != &raw mut (*req).bufsml as *mut uv_buf_t {
                    uv__free((*req).bufs as *mut ::core::ffi::c_void);
                }
                (*req).bufs = ::core::ptr::null_mut::<uv_buf_t>();
            }
            if (*req).cb.is_some() {
                (*req).cb.expect("non-null function pointer")(req, (*req).error);
            }
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__stream_eof(mut stream: *mut uv_stream_t, mut buf: *const uv_buf_t) {
    unsafe {
        (*stream).flags |= UV_HANDLE_READ_EOF as ::core::ffi::c_int as ::core::ffi::c_uint;
        (*stream).flags &= !(UV_HANDLE_READING as ::core::ffi::c_int) as ::core::ffi::c_uint;
        uv__io_stop(
            (*stream).loop_0,
            &raw mut (*stream).io_watcher,
            POLLIN as ::core::ffi::c_uint,
        );
        if !((*stream).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint)
        {
            (*stream).flags &= !(UV_HANDLE_ACTIVE as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if (*stream).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*stream).loop_0).active_handles =
                    (*(*stream).loop_0).active_handles.wrapping_sub(1);
            }
        }
        uv__stream_osx_interrupt_select(stream);
        (*stream).read_cb.expect("non-null function pointer")(
            stream,
            UV_EOF as ::core::ffi::c_int as ssize_t,
            buf,
        );
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__stream_queue_fd(
    mut stream: *mut uv_stream_t,
    mut fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut queued_fds: *mut uv__stream_queued_fds_t =
            ::core::ptr::null_mut::<uv__stream_queued_fds_t>();
        let mut queue_size: ::core::ffi::c_uint = 0;
        queued_fds = (*stream).queued_fds as *mut uv__stream_queued_fds_t;
        if queued_fds.is_null() {
            queue_size = 8 as ::core::ffi::c_uint;
            queued_fds = uv__malloc(
                (queue_size.wrapping_sub(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                    .wrapping_add(::core::mem::size_of::<uv__stream_queued_fds_t>() as size_t),
            ) as *mut uv__stream_queued_fds_t;
            if queued_fds.is_null() {
                return UV_ENOMEM as ::core::ffi::c_int;
            }
            (*queued_fds).size = queue_size;
            (*queued_fds).offset = 0 as ::core::ffi::c_uint;
            (*stream).queued_fds = queued_fds as *mut ::core::ffi::c_void;
        } else if (*queued_fds).size == (*queued_fds).offset {
            queue_size = (*queued_fds).size.wrapping_add(8 as ::core::ffi::c_uint);
            queued_fds = uv__realloc(
                queued_fds as *mut ::core::ffi::c_void,
                (queue_size.wrapping_sub(1 as ::core::ffi::c_uint) as size_t)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                    .wrapping_add(::core::mem::size_of::<uv__stream_queued_fds_t>() as size_t),
            ) as *mut uv__stream_queued_fds_t;
            if queued_fds.is_null() {
                return UV_ENOMEM as ::core::ffi::c_int;
            }
            (*queued_fds).size = queue_size;
            (*stream).queued_fds = queued_fds as *mut ::core::ffi::c_void;
        }
        let fresh4 = (*queued_fds).offset;
        (*queued_fds).offset = (*queued_fds).offset.wrapping_add(1);
        *(&raw mut (*queued_fds).fds as *mut ::core::ffi::c_int).offset(fresh4 as isize) = fd;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__stream_recv_cmsg(
    mut stream: *mut uv_stream_t,
    mut msg: *mut msghdr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
        let mut fd: ::core::ffi::c_int = 0;
        let mut err: ::core::ffi::c_int = 0;
        let mut i: size_t = 0;
        let mut count: size_t = 0;
        cmsg = if (*msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
            (*msg).msg_control as *mut cmsghdr
        } else {
            ::core::ptr::null_mut::<cmsghdr>()
        };
        while !cmsg.is_null() {
            if (*cmsg).cmsg_type != SCM_RIGHTS as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"ignoring non-SCM_RIGHTS ancillary data: %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*cmsg).cmsg_type,
                );
            } else {
                '_c2rust_label: {
                    if (*cmsg).cmsg_len
                        >= ((::core::mem::size_of::<cmsghdr>() as usize)
                            .wrapping_add(::core::mem::size_of::<size_t>() as usize)
                            .wrapping_sub(1 as usize)
                            & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
                        .wrapping_add(0 as usize)
                    {
                    } else {
                        __assert_fail(
                            b"cmsg->cmsg_len >= CMSG_LEN(0)\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            994 as ::core::ffi::c_uint,
                            b"int uv__stream_recv_cmsg(uv_stream_t *, struct msghdr *)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                count = (*cmsg).cmsg_len.wrapping_sub(
                    ((::core::mem::size_of::<cmsghdr>() as size_t)
                        .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
                        .wrapping_sub(1 as size_t)
                        & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
                    .wrapping_add(0 as size_t),
                );
                '_c2rust_label_0: {
                    if count.wrapping_rem(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
                        == 0 as size_t
                    {
                    } else {
                        __assert_fail(
                            b"count % sizeof(fd) == 0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            996 as ::core::ffi::c_uint,
                            b"int uv__stream_recv_cmsg(uv_stream_t *, struct msghdr *)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                count = (count as ::core::ffi::c_ulong)
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize
                        as ::core::ffi::c_ulong) as size_t as size_t;
                i = 0 as size_t;
                while i < count {
                    memcpy(
                        &raw mut fd as *mut ::core::ffi::c_void,
                        (&raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar
                            as *mut ::core::ffi::c_char)
                            .offset(
                                i.wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t
                                ) as isize,
                            ) as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                    );
                    if (*stream).accepted_fd != -(1 as ::core::ffi::c_int) {
                        err = uv__stream_queue_fd(stream, fd);
                        if err != 0 as ::core::ffi::c_int {
                            while i < count {
                                uv__close(fd);
                                i = i.wrapping_add(1);
                            }
                            return err;
                        }
                    } else {
                        (*stream).accepted_fd = fd;
                    }
                    i = i.wrapping_add(1);
                }
            }
            cmsg = __cmsg_nxthdr(msg, cmsg);
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__read(mut stream: *mut uv_stream_t) {
    unsafe {
        let mut buf: uv_buf_t = uv_buf_t {
            base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            len: 0,
        };
        let mut nread: ssize_t = 0;
        let mut msg: msghdr = msghdr {
            msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            msg_namelen: 0,
            msg_iov: ::core::ptr::null_mut::<iovec>(),
            msg_iovlen: 0,
            msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            msg_controllen: 0,
            msg_flags: 0,
        };
        let mut cmsg: uv__cmsg = uv__cmsg {
            hdr: cmsghdr {
                cmsg_len: 0,
                cmsg_level: 0,
                cmsg_type: 0,
                __cmsg_data: [],
            },
        };
        let mut count: ::core::ffi::c_int = 0;
        let mut err: ::core::ffi::c_int = 0;
        let mut is_ipc: ::core::ffi::c_int = 0;
        (*stream).flags &= !(UV_HANDLE_READ_PARTIAL as ::core::ffi::c_int) as ::core::ffi::c_uint;
        count = 32 as ::core::ffi::c_int;
        is_ipc = ((*stream).type_0 as ::core::ffi::c_uint
            == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*(stream as *mut uv_pipe_t)).ipc != 0) as ::core::ffi::c_int;
        while (*stream).read_cb.is_some()
            && (*stream).flags & UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            && {
                let fresh3 = count;
                count = count - 1;
                fresh3 > 0 as ::core::ffi::c_int
            }
        {
            '_c2rust_label: {
                if (*stream).alloc_cb.is_some() {
                } else {
                    __assert_fail(
                        b"stream->alloc_cb != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1044 as ::core::ffi::c_uint,
                        b"void uv__read(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            buf = uv_buf_init(
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as ::core::ffi::c_uint,
            );
            (*stream).alloc_cb.expect("non-null function pointer")(
                stream as *mut uv_handle_t,
                (64 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t,
                &raw mut buf,
            );
            if buf.base.is_null() || buf.len == 0 as size_t {
                (*stream).read_cb.expect("non-null function pointer")(
                    stream,
                    UV_ENOBUFS as ::core::ffi::c_int as ssize_t,
                    &raw mut buf,
                );
                return;
            }
            '_c2rust_label_0: {
                if !buf.base.is_null() {
                } else {
                    __assert_fail(
                        b"buf.base != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1054 as ::core::ffi::c_uint,
                        b"void uv__read(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_1: {
                if (*stream).io_watcher.fd >= 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"uv__stream_fd(stream) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1055 as ::core::ffi::c_uint,
                        b"void uv__read(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            if is_ipc == 0 {
                loop {
                    nread = read(
                        (*stream).io_watcher.fd,
                        buf.base as *mut ::core::ffi::c_void,
                        buf.len,
                    );
                    if !(nread < 0 as ssize_t && *__errno_location() == EINTR) {
                        break;
                    }
                }
            } else {
                msg.msg_flags = 0 as ::core::ffi::c_int;
                msg.msg_iov = &raw mut buf as *mut iovec;
                msg.msg_iovlen = 1 as size_t;
                msg.msg_name = NULL;
                msg.msg_namelen = 0 as socklen_t;
                msg.msg_controllen = ::core::mem::size_of::<uv__cmsg>() as usize as size_t;
                msg.msg_control = &raw mut cmsg.hdr as *mut ::core::ffi::c_void;
                loop {
                    nread = uv__recvmsg(
                        (*stream).io_watcher.fd,
                        &raw mut msg,
                        0 as ::core::ffi::c_int,
                    );
                    if !(nread < 0 as ssize_t && *__errno_location() == EINTR) {
                        break;
                    }
                }
            }
            if nread < 0 as ssize_t {
                if *__errno_location() == EAGAIN || *__errno_location() == EWOULDBLOCK {
                    if (*stream).flags
                        & UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                    {
                        uv__io_start(
                            (*stream).loop_0,
                            &raw mut (*stream).io_watcher,
                            POLLIN as ::core::ffi::c_uint,
                        );
                        uv__stream_osx_interrupt_select(stream);
                    }
                    (*stream).read_cb.expect("non-null function pointer")(
                        stream,
                        0 as ssize_t,
                        &raw mut buf,
                    );
                } else {
                    (*stream).flags &= !(UV_HANDLE_READABLE as ::core::ffi::c_int
                        | UV_HANDLE_WRITABLE as ::core::ffi::c_int)
                        as ::core::ffi::c_uint;
                    (*stream).read_cb.expect("non-null function pointer")(
                        stream,
                        -*__errno_location() as ssize_t,
                        &raw mut buf,
                    );
                    if (*stream).flags
                        & UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                    {
                        (*stream).flags &=
                            !(UV_HANDLE_READING as ::core::ffi::c_int) as ::core::ffi::c_uint;
                        uv__io_stop(
                            (*stream).loop_0,
                            &raw mut (*stream).io_watcher,
                            POLLIN as ::core::ffi::c_uint,
                        );
                        if !((*stream).flags
                            & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                            == 0 as ::core::ffi::c_uint)
                        {
                            (*stream).flags &=
                                !(UV_HANDLE_ACTIVE as ::core::ffi::c_int) as ::core::ffi::c_uint;
                            if (*stream).flags
                                & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                                != 0 as ::core::ffi::c_uint
                            {
                                (*(*stream).loop_0).active_handles =
                                    (*(*stream).loop_0).active_handles.wrapping_sub(1);
                            }
                        }
                        uv__stream_osx_interrupt_select(stream);
                    }
                }
                return;
            } else if nread == 0 as ssize_t {
                uv__stream_eof(stream, &raw mut buf);
                return;
            } else {
                let mut buflen: ssize_t = buf.len as ssize_t;
                if is_ipc != 0 {
                    err = uv__stream_recv_cmsg(stream, &raw mut msg);
                    if err != 0 as ::core::ffi::c_int {
                        (*stream).read_cb.expect("non-null function pointer")(
                            stream,
                            err as ssize_t,
                            &raw mut buf,
                        );
                        return;
                    }
                }
                (*stream).read_cb.expect("non-null function pointer")(stream, nread, &raw mut buf);
                if nread < buflen {
                    (*stream).flags |=
                        UV_HANDLE_READ_PARTIAL as ::core::ffi::c_int as ::core::ffi::c_uint;
                    return;
                }
            }
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_shutdown(
    mut req: *mut uv_shutdown_t,
    mut stream: *mut uv_stream_t,
    mut cb: uv_shutdown_cb,
) -> ::core::ffi::c_int {
    unsafe {
        '_c2rust_label: {
            if (*stream).type_0 as ::core::ffi::c_uint
                == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_TTY as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"stream->type == UV_TCP || stream->type == UV_TTY || stream->type == UV_NAMED_PIPE\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1158 as ::core::ffi::c_uint,
                    b"int uv_shutdown(uv_shutdown_t *, uv_stream_t *, uv_shutdown_cb)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if (*stream).flags & UV_HANDLE_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint == 0
            || (*stream).flags & UV_HANDLE_SHUT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            || !(*stream).shutdown_req.is_null()
            || (*stream).flags
                & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
        {
            return UV_ENOTCONN as ::core::ffi::c_int;
        }
        '_c2rust_label_0: {
            if (*stream).io_watcher.fd >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"uv__stream_fd(stream) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1167 as ::core::ffi::c_uint,
                    b"int uv_shutdown(uv_shutdown_t *, uv_stream_t *, uv_shutdown_cb)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        (*req).type_0 = UV_SHUTDOWN;
        (*(*stream).loop_0).active_reqs.count =
            (*(*stream).loop_0).active_reqs.count.wrapping_add(1);
        (*req).handle = stream;
        (*req).cb = cb;
        (*stream).shutdown_req = req;
        (*stream).flags &= !(UV_HANDLE_WRITABLE as ::core::ffi::c_int) as ::core::ffi::c_uint;
        if uv__queue_empty(&raw mut (*stream).write_queue) != 0 {
            uv__io_feed((*stream).loop_0, &raw mut (*stream).io_watcher);
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__stream_io(
    mut loop_0: *mut uv_loop_t,
    mut w: *mut uv__io_t,
    mut events: ::core::ffi::c_uint,
) {
    unsafe {
        let mut stream: *mut uv_stream_t = ::core::ptr::null_mut::<uv_stream_t>();
        stream = (w as *mut ::core::ffi::c_char).offset(-(136 as ::core::ffi::c_ulong as isize))
            as *mut uv_stream_t;
        '_c2rust_label: {
            if (*stream).type_0 as ::core::ffi::c_uint
                == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_TTY as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"stream->type == UV_TCP || stream->type == UV_NAMED_PIPE || stream->type == UV_TTY\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1191 as ::core::ffi::c_uint,
                    b"void uv__stream_io(uv_loop_t *, uv__io_t *, unsigned int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if (*stream).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint == 0
            {
            } else {
                __assert_fail(
                    b"!(stream->flags & UV_HANDLE_CLOSING)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1192 as ::core::ffi::c_uint,
                    b"void uv__stream_io(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if !(*stream).connect_req.is_null() {
            uv__stream_connect(stream);
            return;
        }
        '_c2rust_label_1: {
            if (*stream).io_watcher.fd >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"uv__stream_fd(stream) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1199 as ::core::ffi::c_uint,
                    b"void uv__stream_io(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if events & (POLLIN | POLLERR | POLLHUP) as ::core::ffi::c_uint != 0 {
            uv__read(stream);
        }
        if (*stream).io_watcher.fd == -(1 as ::core::ffi::c_int) {
            return;
        }
        if events & POLLHUP as ::core::ffi::c_uint != 0
            && (*stream).flags & UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            && (*stream).flags & UV_HANDLE_READ_PARTIAL as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            && (*stream).flags & UV_HANDLE_READ_EOF as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            let mut buf: uv_buf_t = uv_buf_t {
                base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                len: 0 as size_t,
            };
            uv__stream_eof(stream, &raw mut buf);
        }
        if (*stream).io_watcher.fd == -(1 as ::core::ffi::c_int) {
            return;
        }
        if events & (POLLOUT | POLLERR | POLLHUP) as ::core::ffi::c_uint != 0 {
            uv__write(stream);
            uv__write_callbacks(stream);
            if uv__queue_empty(&raw mut (*stream).write_queue) != 0 {
                uv__drain(stream);
            }
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__stream_connect(mut stream: *mut uv_stream_t) {
    unsafe {
        let mut error: ::core::ffi::c_int = 0;
        let mut req: *mut uv_connect_t = (*stream).connect_req;
        let mut errorsize: socklen_t = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
        '_c2rust_label: {
            if (*stream).type_0 as ::core::ffi::c_uint
                == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"stream->type == UV_TCP || stream->type == UV_NAMED_PIPE\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1246 as ::core::ffi::c_uint,
                    b"void uv__stream_connect(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if !req.is_null() {
            } else {
                __assert_fail(
                    b"req\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1247 as ::core::ffi::c_uint,
                    b"void uv__stream_connect(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if (*stream).delayed_error != 0 {
            error = (*stream).delayed_error;
            (*stream).delayed_error = 0 as ::core::ffi::c_int;
        } else {
            '_c2rust_label_1: {
                if (*stream).io_watcher.fd >= 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"uv__stream_fd(stream) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1258 as ::core::ffi::c_uint,
                        b"void uv__stream_connect(uv_stream_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            getsockopt(
                (*stream).io_watcher.fd,
                SOL_SOCKET,
                SO_ERROR,
                &raw mut error as *mut ::core::ffi::c_void,
                &raw mut errorsize,
            );
            error = -error;
        }
        if error == -(115 as ::core::ffi::c_int) {
            return;
        }
        (*stream).connect_req = ::core::ptr::null_mut::<uv_connect_t>();
        '_c2rust_label_2: {
            if (*(*stream).loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"uv__has_active_reqs(stream->loop)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1271 as ::core::ffi::c_uint,
                    b"void uv__stream_connect(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*(*stream).loop_0).active_reqs.count =
            (*(*stream).loop_0).active_reqs.count.wrapping_sub(1);
        if error < 0 as ::core::ffi::c_int || uv__queue_empty(&raw mut (*stream).write_queue) != 0 {
            uv__io_stop(
                (*stream).loop_0,
                &raw mut (*stream).io_watcher,
                POLLOUT as ::core::ffi::c_uint,
            );
        }
        if (*req).cb.is_some() {
            (*req).cb.expect("non-null function pointer")(req, error);
        }
        if (*stream).io_watcher.fd == -(1 as ::core::ffi::c_int) {
            return;
        }
        if error < 0 as ::core::ffi::c_int {
            uv__stream_flush_write_queue(stream, UV_ECANCELED as ::core::ffi::c_int);
            uv__write_callbacks(stream);
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__check_before_write(
    mut stream: *mut uv_stream_t,
    mut nbufs: ::core::ffi::c_uint,
    mut send_handle: *mut uv_stream_t,
) -> ::core::ffi::c_int {
    unsafe {
        '_c2rust_label: {
            if nbufs > 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"nbufs > 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1293 as ::core::ffi::c_uint,
                    b"int uv__check_before_write(uv_stream_t *, unsigned int, uv_stream_t *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if ((*stream).type_0 as ::core::ffi::c_uint
                == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_TTY as ::core::ffi::c_int as ::core::ffi::c_uint)
                && !(b"uv_write (unix) does not yet support other types of streams\0" as *const u8
                    as *const ::core::ffi::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"(stream->type == UV_TCP || stream->type == UV_NAMED_PIPE || stream->type == UV_TTY) && \"uv_write (unix) does not yet support other types of streams\"\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1297 as ::core::ffi::c_uint,
                    b"int uv__check_before_write(uv_stream_t *, unsigned int, uv_stream_t *)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        if (*stream).io_watcher.fd < 0 as ::core::ffi::c_int {
            return UV_EBADF as ::core::ffi::c_int;
        }
        if (*stream).flags & UV_HANDLE_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
            return UV_EPIPE as ::core::ffi::c_int;
        }
        if !send_handle.is_null() {
            if (*stream).type_0 as ::core::ffi::c_uint
                != UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*(stream as *mut uv_pipe_t)).ipc == 0
            {
                return UV_EINVAL as ::core::ffi::c_int;
            }
            if uv__handle_fd(send_handle as *mut uv_handle_t) < 0 as ::core::ffi::c_int {
                return UV_EBADF as ::core::ffi::c_int;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_write2(
    mut req: *mut uv_write_t,
    mut stream: *mut uv_stream_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut send_handle: *mut uv_stream_t,
    mut cb: uv_write_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut empty_queue: ::core::ffi::c_int = 0;
        let mut err: ::core::ffi::c_int = 0;
        err = uv__check_before_write(stream, nbufs, send_handle);
        if err < 0 as ::core::ffi::c_int {
            return err;
        }
        empty_queue = ((*stream).write_queue_size == 0 as size_t) as ::core::ffi::c_int;
        (*req).type_0 = UV_WRITE;
        (*(*stream).loop_0).active_reqs.count =
            (*(*stream).loop_0).active_reqs.count.wrapping_add(1);
        (*req).cb = cb;
        (*req).handle = stream;
        (*req).error = 0 as ::core::ffi::c_int;
        (*req).send_handle = send_handle;
        uv__queue_init(&raw mut (*req).queue);
        (*req).bufs = &raw mut (*req).bufsml as *mut uv_buf_t;
        if nbufs as usize
            > (::core::mem::size_of::<[uv_buf_t; 4]>() as usize)
                .wrapping_div(::core::mem::size_of::<uv_buf_t>() as usize)
        {
            (*req).bufs = uv__malloc(
                (nbufs as size_t).wrapping_mul(::core::mem::size_of::<uv_buf_t>() as size_t),
            ) as *mut uv_buf_t;
        }
        if (*req).bufs.is_null() {
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        memcpy(
            (*req).bufs as *mut ::core::ffi::c_void,
            bufs as *const ::core::ffi::c_void,
            (nbufs as size_t).wrapping_mul(::core::mem::size_of::<uv_buf_t>() as size_t),
        );
        (*req).nbufs = nbufs;
        (*req).write_index = 0 as ::core::ffi::c_uint;
        (*stream).write_queue_size = (*stream)
            .write_queue_size
            .wrapping_add(uv__count_bufs(bufs, nbufs));
        uv__queue_insert_tail(&raw mut (*stream).write_queue, &raw mut (*req).queue);
        if (*stream).connect_req.is_null() {
            if empty_queue != 0 {
                uv__write(stream);
            } else {
                '_c2rust_label: {
                    if (*stream).flags
                        & UV_HANDLE_BLOCKING_WRITES as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0
                    {
                    } else {
                        __assert_fail(
                            b"!(stream->flags & UV_HANDLE_BLOCKING_WRITES)\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                                as *const u8 as *const ::core::ffi::c_char,
                            1388 as ::core::ffi::c_uint,
                            b"int uv_write2(uv_write_t *, uv_stream_t *, const uv_buf_t *, unsigned int, uv_stream_t *, uv_write_cb)\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                };
                uv__io_start(
                    (*stream).loop_0,
                    &raw mut (*stream).io_watcher,
                    POLLOUT as ::core::ffi::c_uint,
                );
                uv__stream_osx_interrupt_select(stream);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_write(
    mut req: *mut uv_write_t,
    mut handle: *mut uv_stream_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut cb: uv_write_cb,
) -> ::core::ffi::c_int {
    unsafe {
        return uv_write2(
            req,
            handle,
            bufs,
            nbufs,
            ::core::ptr::null_mut::<uv_stream_t>(),
            cb,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_try_write(
    mut stream: *mut uv_stream_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        return uv_try_write2(stream, bufs, nbufs, ::core::ptr::null_mut::<uv_stream_t>());
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_try_write2(
    mut stream: *mut uv_stream_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut send_handle: *mut uv_stream_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        if !(*stream).connect_req.is_null() || (*stream).write_queue_size != 0 as size_t {
            return UV_EAGAIN as ::core::ffi::c_int;
        }
        err = uv__check_before_write(stream, nbufs, ::core::ptr::null_mut::<uv_stream_t>());
        if err < 0 as ::core::ffi::c_int {
            return err;
        }
        return uv__try_write(stream, bufs, nbufs, send_handle);
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__read_start(
    mut stream: *mut uv_stream_t,
    mut alloc_cb: uv_alloc_cb,
    mut read_cb: uv_read_cb,
) -> ::core::ffi::c_int {
    unsafe {
        '_c2rust_label: {
            if (*stream).type_0 as ::core::ffi::c_uint
                == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*stream).type_0 as ::core::ffi::c_uint
                    == UV_TTY as ::core::ffi::c_int as ::core::ffi::c_uint
            {
            } else {
                __assert_fail(
                    b"stream->type == UV_TCP || stream->type == UV_NAMED_PIPE || stream->type == UV_TTY\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    1438 as ::core::ffi::c_uint,
                    b"int uv__read_start(uv_stream_t *, uv_alloc_cb, uv_read_cb)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        (*stream).flags |= UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint;
        (*stream).flags &= !(UV_HANDLE_READ_EOF as ::core::ffi::c_int) as ::core::ffi::c_uint;
        '_c2rust_label_0: {
            if (*stream).io_watcher.fd >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"uv__stream_fd(stream) >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1446 as ::core::ffi::c_uint,
                    b"int uv__read_start(uv_stream_t *, uv_alloc_cb, uv_read_cb)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_1: {
            if alloc_cb.is_some() {
            } else {
                __assert_fail(
                    b"alloc_cb\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1447 as ::core::ffi::c_uint,
                    b"int uv__read_start(uv_stream_t *, uv_alloc_cb, uv_read_cb)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*stream).read_cb = read_cb;
        (*stream).alloc_cb = alloc_cb;
        uv__io_start(
            (*stream).loop_0,
            &raw mut (*stream).io_watcher,
            POLLIN as ::core::ffi::c_uint,
        );
        if !((*stream).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            (*stream).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
            if (*stream).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*stream).loop_0).active_handles =
                    (*(*stream).loop_0).active_handles.wrapping_add(1);
            }
        }
        uv__stream_osx_interrupt_select(stream);
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_read_stop(mut stream: *mut uv_stream_t) -> ::core::ffi::c_int {
    unsafe {
        if (*stream).flags & UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*stream).flags &= !(UV_HANDLE_READING as ::core::ffi::c_int) as ::core::ffi::c_uint;
        uv__io_stop(
            (*stream).loop_0,
            &raw mut (*stream).io_watcher,
            POLLIN as ::core::ffi::c_uint,
        );
        if !((*stream).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint)
        {
            (*stream).flags &= !(UV_HANDLE_ACTIVE as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if (*stream).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*stream).loop_0).active_handles =
                    (*(*stream).loop_0).active_handles.wrapping_sub(1);
            }
        }
        uv__stream_osx_interrupt_select(stream);
        (*stream).read_cb = None;
        (*stream).alloc_cb = None;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_is_readable(mut stream: *const uv_stream_t) -> ::core::ffi::c_int {
    unsafe {
        return ((*stream).flags & UV_HANDLE_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0) as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_is_writable(mut stream: *const uv_stream_t) -> ::core::ffi::c_int {
    unsafe {
        return ((*stream).flags & UV_HANDLE_WRITABLE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0) as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__stream_close(mut handle: *mut uv_stream_t) {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut queued_fds: *mut uv__stream_queued_fds_t =
            ::core::ptr::null_mut::<uv__stream_queued_fds_t>();
        uv__io_close((*handle).loop_0, &raw mut (*handle).io_watcher);
        uv_read_stop(handle);
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint)
        {
            (*handle).flags &= !(UV_HANDLE_ACTIVE as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*handle).loop_0).active_handles =
                    (*(*handle).loop_0).active_handles.wrapping_sub(1);
            }
        }
        (*handle).flags &= !(UV_HANDLE_READABLE as ::core::ffi::c_int
            | UV_HANDLE_WRITABLE as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        if (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int) {
            if (*handle).io_watcher.fd > STDERR_FILENO {
                uv__close((*handle).io_watcher.fd);
            }
            (*handle).io_watcher.fd = -(1 as ::core::ffi::c_int);
        }
        if (*handle).accepted_fd != -(1 as ::core::ffi::c_int) {
            uv__close((*handle).accepted_fd);
            (*handle).accepted_fd = -(1 as ::core::ffi::c_int);
        }
        if !(*handle).queued_fds.is_null() {
            queued_fds = (*handle).queued_fds as *mut uv__stream_queued_fds_t;
            i = 0 as ::core::ffi::c_uint;
            while i < (*queued_fds).offset {
                uv__close(
                    *(&raw mut (*queued_fds).fds as *mut ::core::ffi::c_int).offset(i as isize),
                );
                i = i.wrapping_add(1);
            }
            uv__free((*handle).queued_fds);
            (*handle).queued_fds = NULL;
        }
        '_c2rust_label: {
            if uv__io_active(
                &raw mut (*handle).io_watcher,
                (0x1 as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) == 0
            {
            } else {
                __assert_fail(
                    b"!uv__io_active(&handle->io_watcher, POLLIN | POLLOUT)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/stream.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1553 as ::core::ffi::c_uint,
                    b"void uv__stream_close(uv_stream_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_stream_set_blocking(
    mut handle: *mut uv_stream_t,
    mut blocking: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return uv__nonblock_ioctl(
            (*handle).io_watcher.fd,
            (blocking == 0) as ::core::ffi::c_int,
        );
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn accept(
    server: *mut crate::abi::linux_x86_64::uv_stream_t,
    client: *mut crate::abi::linux_x86_64::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_accept(server.cast(), client.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn is_readable(
    handle: *const crate::abi::linux_x86_64::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_is_readable(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn is_writable(
    handle: *const crate::abi::linux_x86_64::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_is_writable(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn listen(
    stream: *mut crate::abi::linux_x86_64::uv_stream_t,
    backlog: ::std::os::raw::c_int,
    cb: crate::abi::linux_x86_64::uv_connection_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            uv_listen(
                stream.cast(),
                backlog,
                std::mem::transmute::<_, uv_connection_cb>(cb),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn read_start(
    stream: *mut crate::abi::linux_x86_64::uv_stream_t,
    alloc_cb: crate::abi::linux_x86_64::uv_alloc_cb,
    read_cb: crate::abi::linux_x86_64::uv_read_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            crate::upstream_support::uv_common::uv_read_start(
                stream.cast(),
                std::mem::transmute::<_, crate::upstream_support::uv_common::uv_alloc_cb>(alloc_cb),
                std::mem::transmute::<_, crate::upstream_support::uv_common::uv_read_cb>(read_cb),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn read_stop(
    stream: *mut crate::abi::linux_x86_64::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_read_stop(stream.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn shutdown_stream(
    req: *mut crate::abi::linux_x86_64::uv_shutdown_t,
    handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    cb: crate::abi::linux_x86_64::uv_shutdown_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            uv_shutdown(
                req.cast(),
                handle.cast(),
                std::mem::transmute::<_, uv_shutdown_cb>(cb),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn set_blocking(
    handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    blocking: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_stream_set_blocking(handle.cast(), blocking) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn try_write(
    handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    bufs: *const crate::abi::linux_x86_64::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_try_write(handle.cast(), bufs.cast(), nbufs) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn try_write2(
    handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    bufs: *const crate::abi::linux_x86_64::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    send_handle: *mut crate::abi::linux_x86_64::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_try_write2(handle.cast(), bufs.cast(), nbufs, send_handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn write_stream(
    req: *mut crate::abi::linux_x86_64::uv_write_t,
    handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    bufs: *const crate::abi::linux_x86_64::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    cb: crate::abi::linux_x86_64::uv_write_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            uv_write(
                req.cast(),
                handle.cast(),
                bufs.cast(),
                nbufs,
                std::mem::transmute::<_, uv_write_cb>(cb),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn write2(
    req: *mut crate::abi::linux_x86_64::uv_write_t,
    handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    bufs: *const crate::abi::linux_x86_64::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    send_handle: *mut crate::abi::linux_x86_64::uv_stream_t,
    cb: crate::abi::linux_x86_64::uv_write_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            uv_write2(
                req.cast(),
                handle.cast(),
                bufs.cast(),
                nbufs,
                send_handle.cast(),
                std::mem::transmute::<_, uv_write_cb>(cb),
            )
        }
    }
}
