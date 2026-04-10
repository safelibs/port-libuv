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
#[repr(C)]
pub struct __dirstream {
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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn uv_loop_init(loop_0: *mut uv_loop_t) -> ::core::ffi::c_int;
    fn uv_udp_getpeername(
        handle: *const uv_udp_t,
        name: *mut sockaddr,
        namelen: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv_inet_ntop(
        af: ::core::ffi::c_int,
        src: *const ::core::ffi::c_void,
        dst: *mut ::core::ffi::c_char,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn uv_inet_pton(
        af: ::core::ffi::c_int,
        src: *const ::core::ffi::c_char,
        dst: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn uv_hrtime() -> uint64_t;
    fn uv_mutex_lock(handle: *mut uv_mutex_t);
    fn uv_mutex_unlock(handle: *mut uv_mutex_t);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn uv__strscpy(
        d: *mut ::core::ffi::c_char,
        s: *const ::core::ffi::c_char,
        n: size_t,
    ) -> ssize_t;
    fn uv__loop_configure(
        loop_0: *mut uv_loop_t,
        option: uv_loop_option,
        arg: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv__loop_close(loop_0: *mut uv_loop_t);
    fn uv__read_start(
        stream: *mut uv_stream_t,
        alloc_cb: uv_alloc_cb,
        read_cb: uv_read_cb,
    ) -> ::core::ffi::c_int;
    fn uv__tcp_bind(
        tcp: *mut uv_tcp_t,
        addr: *const sockaddr,
        addrlen: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn uv__tcp_connect(
        req: *mut uv_connect_t,
        handle: *mut uv_tcp_t,
        addr: *const sockaddr,
        addrlen: ::core::ffi::c_uint,
        cb: uv_connect_cb,
    ) -> ::core::ffi::c_int;
    fn uv__udp_init_ex(
        loop_0: *mut uv_loop_t,
        handle: *mut uv_udp_t,
        flags: ::core::ffi::c_uint,
        domain: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv__udp_bind(
        handle: *mut uv_udp_t,
        addr: *const sockaddr,
        addrlen: ::core::ffi::c_uint,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn uv__udp_connect(
        handle: *mut uv_udp_t,
        addr: *const sockaddr,
        addrlen: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn uv__udp_disconnect(handle: *mut uv_udp_t) -> ::core::ffi::c_int;
    fn uv__udp_send(
        req: *mut uv_udp_send_t,
        handle: *mut uv_udp_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        addr: *const sockaddr,
        addrlen: ::core::ffi::c_uint,
        send_cb: uv_udp_send_cb,
    ) -> ::core::ffi::c_int;
    fn uv__udp_try_send(
        handle: *mut uv_udp_t,
        bufs: *const uv_buf_t,
        nbufs: ::core::ffi::c_uint,
        addr: *const sockaddr,
        addrlen: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn uv__udp_recv_start(
        handle: *mut uv_udp_t,
        alloccb: uv_alloc_cb,
        recv_cb: uv_udp_recv_cb,
    ) -> ::core::ffi::c_int;
    fn uv__udp_recv_stop(handle: *mut uv_udp_t) -> ::core::ffi::c_int;
    fn uv__socket_sockopt(
        handle: *mut uv_handle_t,
        optname: ::core::ffi::c_int,
        value: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn uv__process_title_cleanup();
    fn uv__signal_cleanup();
    fn uv__threadpool_cleanup();
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn if_nametoindex(__ifname: *const ::core::ffi::c_char) -> ::core::ffi::c_uint;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino64_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
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
pub type va_list = __gnuc_va_list;
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
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
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const DT_WHT: C2RustUnnamed = 14;
pub const DT_SOCK: C2RustUnnamed = 12;
pub const DT_LNK: C2RustUnnamed = 10;
pub const DT_REG: C2RustUnnamed = 8;
pub const DT_BLK: C2RustUnnamed = 6;
pub const DT_DIR: C2RustUnnamed = 4;
pub const DT_CHR: C2RustUnnamed = 2;
pub const DT_FIFO: C2RustUnnamed = 1;
pub const DT_UNKNOWN: C2RustUnnamed = 0;
pub type DIR = __dirstream;
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
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
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
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
pub type cc_t = ::core::ffi::c_uchar;
pub type speed_t = ::core::ffi::c_uint;
pub type tcflag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__work {
    pub work: Option<unsafe extern "C" fn(*mut uv__work) -> ()>,
    pub done: Option<unsafe extern "C" fn(*mut uv__work, ::core::ffi::c_int) -> ()>,
    pub loop_0: *mut uv_loop_s,
    pub wq: uv__queue,
}
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
pub type uv_file = ::core::ffi::c_int;
pub type uv_gid_t = gid_t;
pub type uv_uid_t = uid_t;
pub type uv__dirent_t = dirent;
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
pub struct uv_dir_s {
    pub dirents: *mut uv_dirent_t,
    pub nentries: size_t,
    pub reserved: [*mut ::core::ffi::c_void; 4],
    pub dir: *mut DIR,
}
pub type uv_dirent_t = uv_dirent_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_dirent_s {
    pub name: *const ::core::ffi::c_char,
    pub type_0: uv_dirent_type_t,
}
pub type uv_dirent_type_t = ::core::ffi::c_uint;
pub const UV_DIRENT_BLOCK: uv_dirent_type_t = 7;
pub const UV_DIRENT_CHAR: uv_dirent_type_t = 6;
pub const UV_DIRENT_SOCKET: uv_dirent_type_t = 5;
pub const UV_DIRENT_FIFO: uv_dirent_type_t = 4;
pub const UV_DIRENT_LINK: uv_dirent_type_t = 3;
pub const UV_DIRENT_DIR: uv_dirent_type_t = 2;
pub const UV_DIRENT_FILE: uv_dirent_type_t = 1;
pub const UV_DIRENT_UNKNOWN: uv_dirent_type_t = 0;
pub type uv_dir_t = uv_dir_s;
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
pub struct uv_tty_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_12,
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
    pub orig_termios: termios,
    pub mode: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_tty_t = uv_tty_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_poll_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_13,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub poll_cb: uv_poll_cb,
    pub io_watcher: uv__io_t,
}
pub type uv_poll_cb =
    Option<unsafe extern "C" fn(*mut uv_poll_t, ::core::ffi::c_int, ::core::ffi::c_int) -> ()>;
pub type uv_poll_t = uv_poll_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timer_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_15,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub timer_cb: uv_timer_cb,
    pub node: C2RustUnnamed_14,
    pub timeout: uint64_t,
    pub repeat: uint64_t,
    pub start_id: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub heap: [*mut ::core::ffi::c_void; 3],
    pub queue: uv__queue,
}
pub type uv_timer_cb = Option<unsafe extern "C" fn(*mut uv_timer_t) -> ()>;
pub type uv_timer_t = uv_timer_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_prepare_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_16,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub prepare_cb: uv_prepare_cb,
    pub queue: uv__queue,
}
pub type uv_prepare_cb = Option<unsafe extern "C" fn(*mut uv_prepare_t) -> ()>;
pub type uv_prepare_t = uv_prepare_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_check_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_17,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub check_cb: uv_check_cb,
    pub queue: uv__queue,
}
pub type uv_check_cb = Option<unsafe extern "C" fn(*mut uv_check_t) -> ()>;
pub type uv_check_t = uv_check_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_idle_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_18,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub idle_cb: uv_idle_cb,
    pub queue: uv__queue,
}
pub type uv_idle_cb = Option<unsafe extern "C" fn(*mut uv_idle_t) -> ()>;
pub type uv_idle_t = uv_idle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_process_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_19,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub exit_cb: uv_exit_cb,
    pub pid: ::core::ffi::c_int,
    pub queue: uv__queue,
    pub status: ::core::ffi::c_int,
}
pub type uv_exit_cb =
    Option<unsafe extern "C" fn(*mut uv_process_t, int64_t, ::core::ffi::c_int) -> ()>;
pub type uv_process_t = uv_process_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_fs_event_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_20,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub path: *mut ::core::ffi::c_char,
    pub cb: uv_fs_event_cb,
    pub watchers: uv__queue,
    pub wd: ::core::ffi::c_int,
}
pub type uv_fs_event_cb = Option<
    unsafe extern "C" fn(
        *mut uv_fs_event_t,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type uv_fs_event_t = uv_fs_event_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_fs_poll_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_21,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub poll_ctx: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_21 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_fs_poll_t = uv_fs_poll_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_req_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
}
pub type uv_req_t = uv_req_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_getaddrinfo_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub loop_0: *mut uv_loop_t,
    pub work_req: uv__work,
    pub cb: uv_getaddrinfo_cb,
    pub hints: *mut addrinfo,
    pub hostname: *mut ::core::ffi::c_char,
    pub service: *mut ::core::ffi::c_char,
    pub addrinfo: *mut addrinfo,
    pub retcode: ::core::ffi::c_int,
}
pub type uv_getaddrinfo_cb =
    Option<unsafe extern "C" fn(*mut uv_getaddrinfo_t, ::core::ffi::c_int, *mut addrinfo) -> ()>;
pub type uv_getaddrinfo_t = uv_getaddrinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_getnameinfo_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub loop_0: *mut uv_loop_t,
    pub work_req: uv__work,
    pub getnameinfo_cb: uv_getnameinfo_cb,
    pub storage: sockaddr_storage,
    pub flags: ::core::ffi::c_int,
    pub host: [::core::ffi::c_char; 1025],
    pub service: [::core::ffi::c_char; 32],
    pub retcode: ::core::ffi::c_int,
}
pub type uv_getnameinfo_cb = Option<
    unsafe extern "C" fn(
        *mut uv_getnameinfo_t,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
    ) -> (),
>;
pub type uv_getnameinfo_t = uv_getnameinfo_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_udp_send_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub handle: *mut uv_udp_t,
    pub cb: uv_udp_send_cb,
    pub queue: uv__queue,
    pub addr: sockaddr_storage,
    pub nbufs: ::core::ffi::c_uint,
    pub bufs: *mut uv_buf_t,
    pub status: ssize_t,
    pub send_cb: uv_udp_send_cb,
    pub bufsml: [uv_buf_t; 4],
}
pub type uv_udp_send_cb =
    Option<unsafe extern "C" fn(*mut uv_udp_send_t, ::core::ffi::c_int) -> ()>;
pub type uv_udp_send_t = uv_udp_send_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_fs_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub fs_type: uv_fs_type,
    pub loop_0: *mut uv_loop_t,
    pub cb: uv_fs_cb,
    pub result: ssize_t,
    pub ptr: *mut ::core::ffi::c_void,
    pub path: *const ::core::ffi::c_char,
    pub statbuf: uv_stat_t,
    pub new_path: *const ::core::ffi::c_char,
    pub file: uv_file,
    pub flags: ::core::ffi::c_int,
    pub mode: mode_t,
    pub nbufs: ::core::ffi::c_uint,
    pub bufs: *mut uv_buf_t,
    pub off: off_t,
    pub uid: uv_uid_t,
    pub gid: uv_gid_t,
    pub atime: ::core::ffi::c_double,
    pub mtime: ::core::ffi::c_double,
    pub work_req: uv__work,
    pub bufsml: [uv_buf_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stat_t {
    pub st_dev: uint64_t,
    pub st_mode: uint64_t,
    pub st_nlink: uint64_t,
    pub st_uid: uint64_t,
    pub st_gid: uint64_t,
    pub st_rdev: uint64_t,
    pub st_ino: uint64_t,
    pub st_size: uint64_t,
    pub st_blksize: uint64_t,
    pub st_blocks: uint64_t,
    pub st_flags: uint64_t,
    pub st_gen: uint64_t,
    pub st_atim: uv_timespec_t,
    pub st_mtim: uv_timespec_t,
    pub st_ctim: uv_timespec_t,
    pub st_birthtim: uv_timespec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timespec_t {
    pub tv_sec: ::core::ffi::c_long,
    pub tv_nsec: ::core::ffi::c_long,
}
pub type uv_fs_cb = Option<unsafe extern "C" fn(*mut uv_fs_t) -> ()>;
pub type uv_fs_t = uv_fs_s;
pub type uv_fs_type = ::core::ffi::c_int;
pub const UV_FS_LUTIME: uv_fs_type = 36;
pub const UV_FS_MKSTEMP: uv_fs_type = 35;
pub const UV_FS_STATFS: uv_fs_type = 34;
pub const UV_FS_CLOSEDIR: uv_fs_type = 33;
pub const UV_FS_READDIR: uv_fs_type = 32;
pub const UV_FS_OPENDIR: uv_fs_type = 31;
pub const UV_FS_LCHOWN: uv_fs_type = 30;
pub const UV_FS_COPYFILE: uv_fs_type = 29;
pub const UV_FS_REALPATH: uv_fs_type = 28;
pub const UV_FS_FCHOWN: uv_fs_type = 27;
pub const UV_FS_CHOWN: uv_fs_type = 26;
pub const UV_FS_READLINK: uv_fs_type = 25;
pub const UV_FS_SYMLINK: uv_fs_type = 24;
pub const UV_FS_LINK: uv_fs_type = 23;
pub const UV_FS_SCANDIR: uv_fs_type = 22;
pub const UV_FS_RENAME: uv_fs_type = 21;
pub const UV_FS_MKDTEMP: uv_fs_type = 20;
pub const UV_FS_MKDIR: uv_fs_type = 19;
pub const UV_FS_RMDIR: uv_fs_type = 18;
pub const UV_FS_UNLINK: uv_fs_type = 17;
pub const UV_FS_FDATASYNC: uv_fs_type = 16;
pub const UV_FS_FSYNC: uv_fs_type = 15;
pub const UV_FS_FCHMOD: uv_fs_type = 14;
pub const UV_FS_CHMOD: uv_fs_type = 13;
pub const UV_FS_ACCESS: uv_fs_type = 12;
pub const UV_FS_FUTIME: uv_fs_type = 11;
pub const UV_FS_UTIME: uv_fs_type = 10;
pub const UV_FS_FTRUNCATE: uv_fs_type = 9;
pub const UV_FS_FSTAT: uv_fs_type = 8;
pub const UV_FS_LSTAT: uv_fs_type = 7;
pub const UV_FS_STAT: uv_fs_type = 6;
pub const UV_FS_SENDFILE: uv_fs_type = 5;
pub const UV_FS_WRITE: uv_fs_type = 4;
pub const UV_FS_READ: uv_fs_type = 3;
pub const UV_FS_CLOSE: uv_fs_type = 2;
pub const UV_FS_OPEN: uv_fs_type = 1;
pub const UV_FS_CUSTOM: uv_fs_type = 0;
pub const UV_FS_UNKNOWN: uv_fs_type = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_work_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub loop_0: *mut uv_loop_t,
    pub work_cb: uv_work_cb,
    pub after_work_cb: uv_after_work_cb,
    pub work_req: uv__work,
}
pub type uv_after_work_cb = Option<unsafe extern "C" fn(*mut uv_work_t, ::core::ffi::c_int) -> ()>;
pub type uv_work_t = uv_work_s;
pub type uv_work_cb = Option<unsafe extern "C" fn(*mut uv_work_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_random_s {
    pub data: *mut ::core::ffi::c_void,
    pub type_0: uv_req_type,
    pub reserved: [*mut ::core::ffi::c_void; 6],
    pub loop_0: *mut uv_loop_t,
    pub status: ::core::ffi::c_int,
    pub buf: *mut ::core::ffi::c_void,
    pub buflen: size_t,
    pub cb: uv_random_cb,
    pub work_req: uv__work,
}
pub type uv_random_cb = Option<
    unsafe extern "C" fn(
        *mut uv_random_t,
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
        size_t,
    ) -> (),
>;
pub type uv_random_t = uv_random_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_env_item_s {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
}
pub type uv_env_item_t = uv_env_item_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_cpu_info_s {
    pub model: *mut ::core::ffi::c_char,
    pub speed: ::core::ffi::c_int,
    pub cpu_times: uv_cpu_times_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_cpu_times_s {
    pub user: uint64_t,
    pub nice: uint64_t,
    pub sys: uint64_t,
    pub idle: uint64_t,
    pub irq: uint64_t,
}
pub type uv_cpu_info_t = uv_cpu_info_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_passwd_s {
    pub username: *mut ::core::ffi::c_char,
    pub uid: ::core::ffi::c_ulong,
    pub gid: ::core::ffi::c_ulong,
    pub shell: *mut ::core::ffi::c_char,
    pub homedir: *mut ::core::ffi::c_char,
}
pub type uv_passwd_t = uv_passwd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_group_s {
    pub groupname: *mut ::core::ffi::c_char,
    pub gid: ::core::ffi::c_ulong,
    pub members: *mut *mut ::core::ffi::c_char,
}
pub type uv_group_t = uv_group_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_metrics_s {
    pub loop_count: uint64_t,
    pub events: uint64_t,
    pub events_waiting: uint64_t,
    pub reserved: [*mut uint64_t; 13],
}
pub type uv_metrics_t = uv_metrics_s;
pub type uv_loop_option = ::core::ffi::c_uint;
pub const UV_METRICS_IDLE_TIME: uv_loop_option = 1;
pub const UV_LOOP_BLOCK_SIGNAL: uv_loop_option = 0;
pub type uv_malloc_func = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type uv_realloc_func =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>;
pub type uv_calloc_func = Option<unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void>;
pub type uv_free_func = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const memory_order_relaxed: memory_order = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__allocator_t {
    pub local_malloc: uv_malloc_func,
    pub local_realloc: uv_realloc_func,
    pub local_calloc: uv_calloc_func,
    pub local_free: uv_free_func,
}
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_22 = 16;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_22 = 4;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_22 = 1;
pub const UV_HANDLE_REF: C2RustUnnamed_22 = 8;
pub type uv_walk_cb =
    Option<unsafe extern "C" fn(*mut uv_handle_t, *mut ::core::ffi::c_void) -> ()>;
pub const UV_HANDLE_READABLE: C2RustUnnamed_22 = 16384;
pub const UV_HANDLE_READING: C2RustUnnamed_22 = 4096;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_22 = 2;
pub type uv_udp_flags = ::core::ffi::c_uint;
pub const UV_UDP_RECVMMSG: uv_udp_flags = 256;
pub const UV_UDP_LINUX_RECVERR: uv_udp_flags = 32;
pub const UV_UDP_MMSG_FREE: uv_udp_flags = 16;
pub const UV_UDP_MMSG_CHUNK: uv_udp_flags = 8;
pub const UV_UDP_REUSEADDR: uv_udp_flags = 4;
pub const UV_UDP_PARTIAL: uv_udp_flags = 2;
pub const UV_UDP_IPV6ONLY: uv_udp_flags = 1;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_22 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_22 = 33554432;
pub type uv__loop_metrics_t = uv__loop_metrics_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__loop_metrics_s {
    pub metrics: uv_metrics_t,
    pub provider_entry_time: uint64_t,
    pub provider_idle_time: uint64_t,
    pub lock: uv_mutex_t,
}
pub type uv__loop_internal_fields_t = uv__loop_internal_fields_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__loop_internal_fields_s {
    pub flags: ::core::ffi::c_uint,
    pub loop_metrics: uv__loop_metrics_t,
    pub current_timeout: ::core::ffi::c_int,
    pub ctl: uv__iou,
    pub iou: uv__iou,
    pub inv: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__iou {
    pub sqhead: *mut uint32_t,
    pub sqtail: *mut uint32_t,
    pub sqarray: *mut uint32_t,
    pub sqmask: uint32_t,
    pub sqflags: *mut uint32_t,
    pub cqhead: *mut uint32_t,
    pub cqtail: *mut uint32_t,
    pub cqmask: uint32_t,
    pub sq: *mut ::core::ffi::c_void,
    pub cqe: *mut ::core::ffi::c_void,
    pub sqe: *mut ::core::ffi::c_void,
    pub sqlen: size_t,
    pub cqlen: size_t,
    pub maxlen: size_t,
    pub sqelen: size_t,
    pub ringfd: ::core::ffi::c_int,
    pub in_flight: uint32_t,
    pub flags: uint32_t,
}
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_22 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_22 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_22 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_22 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_22 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_22 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_22 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_22 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_22 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_22 = 16777216;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_22 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_22 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_22 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_22 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_22 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_22 = 16777216;
pub const UV_HANDLE_IPV6: C2RustUnnamed_22 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_22 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_22 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_22 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_22 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_22 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_22 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_22 = 32768;
pub const UV_HANDLE_BOUND: C2RustUnnamed_22 = 8192;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_22 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_22 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_22 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_22 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_22 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_22 = 32;
pub const SO_SNDBUF: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SO_RCVBUF: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const DT_FIFO_0: ::core::ffi::c_int = 1;
pub const DT_CHR_0: ::core::ffi::c_int = 2;
pub const DT_DIR_0: ::core::ffi::c_int = 4;
pub const DT_BLK_0: ::core::ffi::c_int = 6;
pub const DT_REG_0: ::core::ffi::c_int = 8;
pub const DT_LNK_0: ::core::ffi::c_int = 10;
pub const DT_SOCK_0: ::core::ffi::c_int = 12;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const UV__DT_FILE: ::core::ffi::c_int = DT_REG_0;
pub const UV__DT_DIR: ::core::ffi::c_int = DT_DIR_0;
pub const UV__DT_LINK: ::core::ffi::c_int = DT_LNK_0;
pub const UV__DT_FIFO: ::core::ffi::c_int = DT_FIFO_0;
pub const UV__DT_SOCKET: ::core::ffi::c_int = DT_SOCK_0;
pub const UV__DT_CHAR: ::core::ffi::c_int = DT_CHR_0;
pub const UV__DT_BLOCK: ::core::ffi::c_int = DT_BLK_0;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
// SAFETY(syscall_ffi): the process-global allocator table stores raw libc function pointers for the C ABI.
static mut uv__allocator: uv__allocator_t = unsafe {
    uv__allocator_t {
        local_malloc: Some(malloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        local_realloc: Some(
            realloc
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    size_t,
                ) -> *mut ::core::ffi::c_void,
        ),
        local_calloc: Some(
            calloc as unsafe extern "C" fn(size_t, size_t) -> *mut ::core::ffi::c_void,
        ),
        local_free: Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    }
};
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__strdup(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut len: size_t = strlen(s).wrapping_add(1 as size_t);
        let mut m: *mut ::core::ffi::c_char = uv__malloc(len) as *mut ::core::ffi::c_char;
        if m.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        return memcpy(
            m as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__strndup(
    mut s: *const ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut m: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut len: size_t = strlen(s);
        if n < len {
            len = n;
        }
        m = uv__malloc(len.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if m.is_null() {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        *m.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
        return memcpy(
            m as *mut ::core::ffi::c_void,
            s as *const ::core::ffi::c_void,
            len,
        ) as *mut ::core::ffi::c_char;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__malloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    unsafe {
        if size > 0 as size_t {
            return uv__allocator
                .local_malloc
                .expect("non-null function pointer")(size);
        }
        return NULL;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__free(mut ptr: *mut ::core::ffi::c_void) {
    unsafe {
        let mut saved_errno: ::core::ffi::c_int = 0;
        saved_errno = *__errno_location();
        uv__allocator.local_free.expect("non-null function pointer")(ptr);
        *__errno_location() = saved_errno;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__calloc(mut count: size_t, mut size: size_t) -> *mut ::core::ffi::c_void {
    unsafe {
        return uv__allocator
            .local_calloc
            .expect("non-null function pointer")(count, size);
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        if size > 0 as size_t {
            return uv__allocator
                .local_realloc
                .expect("non-null function pointer")(ptr, size);
        }
        uv__free(ptr);
        return NULL;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__reallocf(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    unsafe {
        let mut newptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
        newptr = uv__realloc(ptr, size);
        if newptr.is_null() {
            if size > 0 as size_t {
                uv__free(ptr);
            }
        }
        return newptr;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_replace_allocator(
    mut malloc_func: uv_malloc_func,
    mut realloc_func: uv_realloc_func,
    mut calloc_func: uv_calloc_func,
    mut free_func: uv_free_func,
) -> ::core::ffi::c_int {
    unsafe {
        if malloc_func.is_none()
            || realloc_func.is_none()
            || calloc_func.is_none()
            || free_func.is_none()
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        uv__allocator.local_malloc = malloc_func;
        uv__allocator.local_realloc = realloc_func;
        uv__allocator.local_calloc = calloc_func;
        uv__allocator.local_free = free_func;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_os_free_passwd(mut pwd: *mut uv_passwd_t) {
    unsafe {
        if pwd.is_null() {
            return;
        }
        uv__free((*pwd).username as *mut ::core::ffi::c_void);
        (*pwd).username = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pwd).shell = ::core::ptr::null_mut::<::core::ffi::c_char>();
        (*pwd).homedir = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_os_free_group(mut grp: *mut uv_group_t) {
    unsafe {
        if grp.is_null() {
            return;
        }
        uv__free((*grp).members as *mut ::core::ffi::c_void);
        (*grp).members = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*grp).groupname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_handle_size(mut type_0: uv_handle_type) -> size_t {
    unsafe {
        match type_0 as ::core::ffi::c_uint {
            1 => return ::core::mem::size_of::<uv_async_t>() as size_t,
            2 => return ::core::mem::size_of::<uv_check_t>() as size_t,
            3 => return ::core::mem::size_of::<uv_fs_event_t>() as size_t,
            4 => return ::core::mem::size_of::<uv_fs_poll_t>() as size_t,
            5 => return ::core::mem::size_of::<uv_handle_t>() as size_t,
            6 => return ::core::mem::size_of::<uv_idle_t>() as size_t,
            7 => return ::core::mem::size_of::<uv_pipe_t>() as size_t,
            8 => return ::core::mem::size_of::<uv_poll_t>() as size_t,
            9 => return ::core::mem::size_of::<uv_prepare_t>() as size_t,
            10 => return ::core::mem::size_of::<uv_process_t>() as size_t,
            11 => return ::core::mem::size_of::<uv_stream_t>() as size_t,
            12 => return ::core::mem::size_of::<uv_tcp_t>() as size_t,
            13 => return ::core::mem::size_of::<uv_timer_t>() as size_t,
            14 => return ::core::mem::size_of::<uv_tty_t>() as size_t,
            15 => return ::core::mem::size_of::<uv_udp_t>() as size_t,
            16 => return ::core::mem::size_of::<uv_signal_t>() as size_t,
            _ => return -(1 as ::core::ffi::c_int) as size_t,
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_req_size(mut type_0: uv_req_type) -> size_t {
    unsafe {
        match type_0 as ::core::ffi::c_uint {
            1 => return ::core::mem::size_of::<uv_req_t>() as size_t,
            2 => return ::core::mem::size_of::<uv_connect_t>() as size_t,
            3 => return ::core::mem::size_of::<uv_write_t>() as size_t,
            4 => return ::core::mem::size_of::<uv_shutdown_t>() as size_t,
            5 => return ::core::mem::size_of::<uv_udp_send_t>() as size_t,
            6 => return ::core::mem::size_of::<uv_fs_t>() as size_t,
            7 => return ::core::mem::size_of::<uv_work_t>() as size_t,
            8 => return ::core::mem::size_of::<uv_getaddrinfo_t>() as size_t,
            9 => return ::core::mem::size_of::<uv_getnameinfo_t>() as size_t,
            10 => return ::core::mem::size_of::<uv_random_t>() as size_t,
            _ => return -(1 as ::core::ffi::c_int) as size_t,
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_size() -> size_t {
    unsafe {
        return ::core::mem::size_of::<uv_loop_t>() as size_t;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_buf_init(
    mut base: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_uint,
) -> uv_buf_t {
    unsafe {
        let mut buf: uv_buf_t = uv_buf_t {
            base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            len: 0,
        };
        buf.base = base;
        buf.len = len as size_t;
        return buf;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__unknown_err_code(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    unsafe {
        let mut buf: [::core::ffi::c_char; 32] = [0; 32];
        let mut copy: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
            b"Unknown system error %d\0" as *const u8 as *const ::core::ffi::c_char,
            err,
        );
        copy = uv__strdup(&raw mut buf as *mut ::core::ffi::c_char);
        return if !copy.is_null() {
            copy as *const ::core::ffi::c_char
        } else {
            b"Unknown system error\0" as *const u8 as *const ::core::ffi::c_char
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_err_name_r(
    mut err: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: size_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        match err {
            -7 => {
                uv__strscpy(
                    buf,
                    b"E2BIG\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -13 => {
                uv__strscpy(
                    buf,
                    b"EACCES\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -98 => {
                uv__strscpy(
                    buf,
                    b"EADDRINUSE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -99 => {
                uv__strscpy(
                    buf,
                    b"EADDRNOTAVAIL\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -97 => {
                uv__strscpy(
                    buf,
                    b"EAFNOSUPPORT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -11 => {
                uv__strscpy(
                    buf,
                    b"EAGAIN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3000 => {
                uv__strscpy(
                    buf,
                    b"EAI_ADDRFAMILY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3001 => {
                uv__strscpy(
                    buf,
                    b"EAI_AGAIN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3002 => {
                uv__strscpy(
                    buf,
                    b"EAI_BADFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3013 => {
                uv__strscpy(
                    buf,
                    b"EAI_BADHINTS\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3003 => {
                uv__strscpy(
                    buf,
                    b"EAI_CANCELED\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3004 => {
                uv__strscpy(
                    buf,
                    b"EAI_FAIL\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3005 => {
                uv__strscpy(
                    buf,
                    b"EAI_FAMILY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3006 => {
                uv__strscpy(
                    buf,
                    b"EAI_MEMORY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3007 => {
                uv__strscpy(
                    buf,
                    b"EAI_NODATA\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3008 => {
                uv__strscpy(
                    buf,
                    b"EAI_NONAME\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3009 => {
                uv__strscpy(
                    buf,
                    b"EAI_OVERFLOW\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3014 => {
                uv__strscpy(
                    buf,
                    b"EAI_PROTOCOL\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3010 => {
                uv__strscpy(
                    buf,
                    b"EAI_SERVICE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3011 => {
                uv__strscpy(
                    buf,
                    b"EAI_SOCKTYPE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -114 => {
                uv__strscpy(
                    buf,
                    b"EALREADY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -9 => {
                uv__strscpy(
                    buf,
                    b"EBADF\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -16 => {
                uv__strscpy(
                    buf,
                    b"EBUSY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -125 => {
                uv__strscpy(
                    buf,
                    b"ECANCELED\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -4080 => {
                uv__strscpy(
                    buf,
                    b"ECHARSET\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -103 => {
                uv__strscpy(
                    buf,
                    b"ECONNABORTED\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -111 => {
                uv__strscpy(
                    buf,
                    b"ECONNREFUSED\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -104 => {
                uv__strscpy(
                    buf,
                    b"ECONNRESET\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -89 => {
                uv__strscpy(
                    buf,
                    b"EDESTADDRREQ\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -17 => {
                uv__strscpy(
                    buf,
                    b"EEXIST\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -14 => {
                uv__strscpy(
                    buf,
                    b"EFAULT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -27 => {
                uv__strscpy(
                    buf,
                    b"EFBIG\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -113 => {
                uv__strscpy(
                    buf,
                    b"EHOSTUNREACH\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -4 => {
                uv__strscpy(
                    buf,
                    b"EINTR\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -22 => {
                uv__strscpy(
                    buf,
                    b"EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -5 => {
                uv__strscpy(
                    buf,
                    b"EIO\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -106 => {
                uv__strscpy(
                    buf,
                    b"EISCONN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -21 => {
                uv__strscpy(
                    buf,
                    b"EISDIR\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -40 => {
                uv__strscpy(
                    buf,
                    b"ELOOP\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -24 => {
                uv__strscpy(
                    buf,
                    b"EMFILE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -90 => {
                uv__strscpy(
                    buf,
                    b"EMSGSIZE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -36 => {
                uv__strscpy(
                    buf,
                    b"ENAMETOOLONG\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -100 => {
                uv__strscpy(
                    buf,
                    b"ENETDOWN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -101 => {
                uv__strscpy(
                    buf,
                    b"ENETUNREACH\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -23 => {
                uv__strscpy(
                    buf,
                    b"ENFILE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -105 => {
                uv__strscpy(
                    buf,
                    b"ENOBUFS\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -19 => {
                uv__strscpy(
                    buf,
                    b"ENODEV\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -2 => {
                uv__strscpy(
                    buf,
                    b"ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -12 => {
                uv__strscpy(
                    buf,
                    b"ENOMEM\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -64 => {
                uv__strscpy(
                    buf,
                    b"ENONET\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -92 => {
                uv__strscpy(
                    buf,
                    b"ENOPROTOOPT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -28 => {
                uv__strscpy(
                    buf,
                    b"ENOSPC\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -38 => {
                uv__strscpy(
                    buf,
                    b"ENOSYS\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -107 => {
                uv__strscpy(
                    buf,
                    b"ENOTCONN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -20 => {
                uv__strscpy(
                    buf,
                    b"ENOTDIR\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -39 => {
                uv__strscpy(
                    buf,
                    b"ENOTEMPTY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -88 => {
                uv__strscpy(
                    buf,
                    b"ENOTSOCK\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -95 => {
                uv__strscpy(
                    buf,
                    b"ENOTSUP\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -75 => {
                uv__strscpy(
                    buf,
                    b"EOVERFLOW\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -1 => {
                uv__strscpy(
                    buf,
                    b"EPERM\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -32 => {
                uv__strscpy(
                    buf,
                    b"EPIPE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -71 => {
                uv__strscpy(
                    buf,
                    b"EPROTO\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -93 => {
                uv__strscpy(
                    buf,
                    b"EPROTONOSUPPORT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -91 => {
                uv__strscpy(
                    buf,
                    b"EPROTOTYPE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -34 => {
                uv__strscpy(
                    buf,
                    b"ERANGE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -30 => {
                uv__strscpy(
                    buf,
                    b"EROFS\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -108 => {
                uv__strscpy(
                    buf,
                    b"ESHUTDOWN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -29 => {
                uv__strscpy(
                    buf,
                    b"ESPIPE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -3 => {
                uv__strscpy(
                    buf,
                    b"ESRCH\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -110 => {
                uv__strscpy(
                    buf,
                    b"ETIMEDOUT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -26 => {
                uv__strscpy(
                    buf,
                    b"ETXTBSY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -18 => {
                uv__strscpy(
                    buf,
                    b"EXDEV\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -4094 => {
                uv__strscpy(
                    buf,
                    b"UNKNOWN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -4095 => {
                uv__strscpy(
                    buf,
                    b"EOF\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -6 => {
                uv__strscpy(
                    buf,
                    b"ENXIO\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -31 => {
                uv__strscpy(
                    buf,
                    b"EMLINK\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -112 => {
                uv__strscpy(
                    buf,
                    b"EHOSTDOWN\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -121 => {
                uv__strscpy(
                    buf,
                    b"EREMOTEIO\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -25 => {
                uv__strscpy(
                    buf,
                    b"ENOTTY\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -4028 => {
                uv__strscpy(
                    buf,
                    b"EFTYPE\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -84 => {
                uv__strscpy(
                    buf,
                    b"EILSEQ\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -94 => {
                uv__strscpy(
                    buf,
                    b"ESOCKTNOSUPPORT\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -61 => {
                uv__strscpy(
                    buf,
                    b"ENODATA\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            -49 => {
                uv__strscpy(
                    buf,
                    b"EUNATCH\0" as *const u8 as *const ::core::ffi::c_char,
                    buflen,
                );
            }
            _ => {
                snprintf(
                    buf,
                    buflen,
                    b"Unknown system error %d\0" as *const u8 as *const ::core::ffi::c_char,
                    err,
                );
            }
        }
        return buf;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_err_name(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    unsafe {
        match err {
            -7 => return b"E2BIG\0" as *const u8 as *const ::core::ffi::c_char,
            -13 => return b"EACCES\0" as *const u8 as *const ::core::ffi::c_char,
            -98 => return b"EADDRINUSE\0" as *const u8 as *const ::core::ffi::c_char,
            -99 => return b"EADDRNOTAVAIL\0" as *const u8 as *const ::core::ffi::c_char,
            -97 => return b"EAFNOSUPPORT\0" as *const u8 as *const ::core::ffi::c_char,
            -11 => return b"EAGAIN\0" as *const u8 as *const ::core::ffi::c_char,
            -3000 => return b"EAI_ADDRFAMILY\0" as *const u8 as *const ::core::ffi::c_char,
            -3001 => return b"EAI_AGAIN\0" as *const u8 as *const ::core::ffi::c_char,
            -3002 => return b"EAI_BADFLAGS\0" as *const u8 as *const ::core::ffi::c_char,
            -3013 => return b"EAI_BADHINTS\0" as *const u8 as *const ::core::ffi::c_char,
            -3003 => return b"EAI_CANCELED\0" as *const u8 as *const ::core::ffi::c_char,
            -3004 => return b"EAI_FAIL\0" as *const u8 as *const ::core::ffi::c_char,
            -3005 => return b"EAI_FAMILY\0" as *const u8 as *const ::core::ffi::c_char,
            -3006 => return b"EAI_MEMORY\0" as *const u8 as *const ::core::ffi::c_char,
            -3007 => return b"EAI_NODATA\0" as *const u8 as *const ::core::ffi::c_char,
            -3008 => return b"EAI_NONAME\0" as *const u8 as *const ::core::ffi::c_char,
            -3009 => return b"EAI_OVERFLOW\0" as *const u8 as *const ::core::ffi::c_char,
            -3014 => return b"EAI_PROTOCOL\0" as *const u8 as *const ::core::ffi::c_char,
            -3010 => return b"EAI_SERVICE\0" as *const u8 as *const ::core::ffi::c_char,
            -3011 => return b"EAI_SOCKTYPE\0" as *const u8 as *const ::core::ffi::c_char,
            -114 => return b"EALREADY\0" as *const u8 as *const ::core::ffi::c_char,
            -9 => return b"EBADF\0" as *const u8 as *const ::core::ffi::c_char,
            -16 => return b"EBUSY\0" as *const u8 as *const ::core::ffi::c_char,
            -125 => return b"ECANCELED\0" as *const u8 as *const ::core::ffi::c_char,
            -4080 => return b"ECHARSET\0" as *const u8 as *const ::core::ffi::c_char,
            -103 => return b"ECONNABORTED\0" as *const u8 as *const ::core::ffi::c_char,
            -111 => return b"ECONNREFUSED\0" as *const u8 as *const ::core::ffi::c_char,
            -104 => return b"ECONNRESET\0" as *const u8 as *const ::core::ffi::c_char,
            -89 => return b"EDESTADDRREQ\0" as *const u8 as *const ::core::ffi::c_char,
            -17 => return b"EEXIST\0" as *const u8 as *const ::core::ffi::c_char,
            -14 => return b"EFAULT\0" as *const u8 as *const ::core::ffi::c_char,
            -27 => return b"EFBIG\0" as *const u8 as *const ::core::ffi::c_char,
            -113 => return b"EHOSTUNREACH\0" as *const u8 as *const ::core::ffi::c_char,
            -4 => return b"EINTR\0" as *const u8 as *const ::core::ffi::c_char,
            -22 => return b"EINVAL\0" as *const u8 as *const ::core::ffi::c_char,
            -5 => return b"EIO\0" as *const u8 as *const ::core::ffi::c_char,
            -106 => return b"EISCONN\0" as *const u8 as *const ::core::ffi::c_char,
            -21 => return b"EISDIR\0" as *const u8 as *const ::core::ffi::c_char,
            -40 => return b"ELOOP\0" as *const u8 as *const ::core::ffi::c_char,
            -24 => return b"EMFILE\0" as *const u8 as *const ::core::ffi::c_char,
            -90 => return b"EMSGSIZE\0" as *const u8 as *const ::core::ffi::c_char,
            -36 => return b"ENAMETOOLONG\0" as *const u8 as *const ::core::ffi::c_char,
            -100 => return b"ENETDOWN\0" as *const u8 as *const ::core::ffi::c_char,
            -101 => return b"ENETUNREACH\0" as *const u8 as *const ::core::ffi::c_char,
            -23 => return b"ENFILE\0" as *const u8 as *const ::core::ffi::c_char,
            -105 => return b"ENOBUFS\0" as *const u8 as *const ::core::ffi::c_char,
            -19 => return b"ENODEV\0" as *const u8 as *const ::core::ffi::c_char,
            -2 => return b"ENOENT\0" as *const u8 as *const ::core::ffi::c_char,
            -12 => return b"ENOMEM\0" as *const u8 as *const ::core::ffi::c_char,
            -64 => return b"ENONET\0" as *const u8 as *const ::core::ffi::c_char,
            -92 => return b"ENOPROTOOPT\0" as *const u8 as *const ::core::ffi::c_char,
            -28 => return b"ENOSPC\0" as *const u8 as *const ::core::ffi::c_char,
            -38 => return b"ENOSYS\0" as *const u8 as *const ::core::ffi::c_char,
            -107 => return b"ENOTCONN\0" as *const u8 as *const ::core::ffi::c_char,
            -20 => return b"ENOTDIR\0" as *const u8 as *const ::core::ffi::c_char,
            -39 => return b"ENOTEMPTY\0" as *const u8 as *const ::core::ffi::c_char,
            -88 => return b"ENOTSOCK\0" as *const u8 as *const ::core::ffi::c_char,
            -95 => return b"ENOTSUP\0" as *const u8 as *const ::core::ffi::c_char,
            -75 => return b"EOVERFLOW\0" as *const u8 as *const ::core::ffi::c_char,
            -1 => return b"EPERM\0" as *const u8 as *const ::core::ffi::c_char,
            -32 => return b"EPIPE\0" as *const u8 as *const ::core::ffi::c_char,
            -71 => return b"EPROTO\0" as *const u8 as *const ::core::ffi::c_char,
            -93 => return b"EPROTONOSUPPORT\0" as *const u8 as *const ::core::ffi::c_char,
            -91 => return b"EPROTOTYPE\0" as *const u8 as *const ::core::ffi::c_char,
            -34 => return b"ERANGE\0" as *const u8 as *const ::core::ffi::c_char,
            -30 => return b"EROFS\0" as *const u8 as *const ::core::ffi::c_char,
            -108 => return b"ESHUTDOWN\0" as *const u8 as *const ::core::ffi::c_char,
            -29 => return b"ESPIPE\0" as *const u8 as *const ::core::ffi::c_char,
            -3 => return b"ESRCH\0" as *const u8 as *const ::core::ffi::c_char,
            -110 => return b"ETIMEDOUT\0" as *const u8 as *const ::core::ffi::c_char,
            -26 => return b"ETXTBSY\0" as *const u8 as *const ::core::ffi::c_char,
            -18 => return b"EXDEV\0" as *const u8 as *const ::core::ffi::c_char,
            -4094 => return b"UNKNOWN\0" as *const u8 as *const ::core::ffi::c_char,
            -4095 => return b"EOF\0" as *const u8 as *const ::core::ffi::c_char,
            -6 => return b"ENXIO\0" as *const u8 as *const ::core::ffi::c_char,
            -31 => return b"EMLINK\0" as *const u8 as *const ::core::ffi::c_char,
            -112 => return b"EHOSTDOWN\0" as *const u8 as *const ::core::ffi::c_char,
            -121 => return b"EREMOTEIO\0" as *const u8 as *const ::core::ffi::c_char,
            -25 => return b"ENOTTY\0" as *const u8 as *const ::core::ffi::c_char,
            -4028 => return b"EFTYPE\0" as *const u8 as *const ::core::ffi::c_char,
            -84 => return b"EILSEQ\0" as *const u8 as *const ::core::ffi::c_char,
            -94 => return b"ESOCKTNOSUPPORT\0" as *const u8 as *const ::core::ffi::c_char,
            -61 => return b"ENODATA\0" as *const u8 as *const ::core::ffi::c_char,
            -49 => return b"EUNATCH\0" as *const u8 as *const ::core::ffi::c_char,
            _ => {}
        }
        return uv__unknown_err_code(err);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_strerror_r(
    mut err: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: size_t,
) -> *mut ::core::ffi::c_char {
    unsafe {
        match err {
            -7 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"argument list too long\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -13 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"permission denied\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -98 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"address already in use\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -99 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"address not available\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -97 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"address family not supported\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -11 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"resource temporarily unavailable\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -3000 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"address family not supported\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3001 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"temporary failure\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3002 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"bad ai_flags value\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3013 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"invalid value for hints\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3003 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"request canceled\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3004 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"permanent failure\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3005 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"ai_family not supported\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3006 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3007 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no address\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3008 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"unknown node or service\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3009 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"argument buffer overflow\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3014 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"resolved protocol is unknown\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3010 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"service not available for socket type\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -3011 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"socket type not supported\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -114 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"connection already in progress\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -9 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"bad file descriptor\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -16 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"resource busy or locked\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -125 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"operation canceled\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -4080 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"invalid Unicode character\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -103 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"software caused connection abort\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -111 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"connection refused\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -104 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"connection reset by peer\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -89 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"destination address required\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -17 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"file already exists\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -14 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"bad address in system call argument\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -27 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"file too large\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -113 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"host is unreachable\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -4 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"interrupted system call\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -22 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -5 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"i/o error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -106 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"socket is already connected\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -21 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"illegal operation on a directory\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -40 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"too many symbolic links encountered\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -24 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"too many open files\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -90 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"message too long\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -36 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"name too long\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -100 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"network is down\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -101 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"network is unreachable\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -23 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"file table overflow\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -105 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no buffer space available\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -19 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no such device\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -2 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no such file or directory\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -12 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"not enough memory\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -64 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"machine is not on the network\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -92 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"protocol not available\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -28 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no space left on device\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -38 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"function not implemented\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -107 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"socket is not connected\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -20 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"not a directory\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -39 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"directory not empty\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -88 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"socket operation on non-socket\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -95 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"operation not supported on socket\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -75 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"value too large for defined data type\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -1 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"operation not permitted\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -32 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"broken pipe\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -71 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"protocol error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -93 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"protocol not supported\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -91 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"protocol wrong type for socket\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -34 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"result too large\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -30 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"read-only file system\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -108 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cannot send after transport endpoint shutdown\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -29 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"invalid seek\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -3 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no such process\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -110 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"connection timed out\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -26 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"text file is busy\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -18 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"cross-device link not permitted\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -4094 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"unknown error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -4095 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"end of file\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -6 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no such device or address\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -31 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"too many links\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -112 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"host is down\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -121 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"remote I/O error\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -25 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"inappropriate ioctl for device\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -4028 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"inappropriate file type or format\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            -84 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"illegal byte sequence\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -94 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"socket type not supported\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -61 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"no data available\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            -49 => {
                snprintf(
                    buf,
                    buflen,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    b"protocol driver not attached\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            _ => {
                snprintf(
                    buf,
                    buflen,
                    b"Unknown system error %d\0" as *const u8 as *const ::core::ffi::c_char,
                    err,
                );
            }
        }
        return buf;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_strerror(mut err: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    unsafe {
        match err {
            -7 => {
                return b"argument list too long\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -13 => return b"permission denied\0" as *const u8 as *const ::core::ffi::c_char,
            -98 => {
                return b"address already in use\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -99 => {
                return b"address not available\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -97 => {
                return b"address family not supported\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -11 => {
                return b"resource temporarily unavailable\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -3000 => {
                return b"address family not supported\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -3001 => return b"temporary failure\0" as *const u8 as *const ::core::ffi::c_char,
            -3002 => {
                return b"bad ai_flags value\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -3013 => {
                return b"invalid value for hints\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -3003 => return b"request canceled\0" as *const u8 as *const ::core::ffi::c_char,
            -3004 => return b"permanent failure\0" as *const u8 as *const ::core::ffi::c_char,
            -3005 => {
                return b"ai_family not supported\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -3006 => return b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            -3007 => return b"no address\0" as *const u8 as *const ::core::ffi::c_char,
            -3008 => {
                return b"unknown node or service\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -3009 => {
                return b"argument buffer overflow\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -3014 => {
                return b"resolved protocol is unknown\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -3010 => {
                return b"service not available for socket type\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -3011 => {
                return b"socket type not supported\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -114 => {
                return b"connection already in progress\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -9 => return b"bad file descriptor\0" as *const u8 as *const ::core::ffi::c_char,
            -16 => {
                return b"resource busy or locked\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -125 => return b"operation canceled\0" as *const u8 as *const ::core::ffi::c_char,
            -4080 => {
                return b"invalid Unicode character\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -103 => {
                return b"software caused connection abort\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -111 => return b"connection refused\0" as *const u8 as *const ::core::ffi::c_char,
            -104 => {
                return b"connection reset by peer\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -89 => {
                return b"destination address required\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -17 => return b"file already exists\0" as *const u8 as *const ::core::ffi::c_char,
            -14 => {
                return b"bad address in system call argument\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -27 => return b"file too large\0" as *const u8 as *const ::core::ffi::c_char,
            -113 => {
                return b"host is unreachable\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -4 => {
                return b"interrupted system call\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -22 => return b"invalid argument\0" as *const u8 as *const ::core::ffi::c_char,
            -5 => return b"i/o error\0" as *const u8 as *const ::core::ffi::c_char,
            -106 => {
                return b"socket is already connected\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -21 => {
                return b"illegal operation on a directory\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -40 => {
                return b"too many symbolic links encountered\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -24 => return b"too many open files\0" as *const u8 as *const ::core::ffi::c_char,
            -90 => return b"message too long\0" as *const u8 as *const ::core::ffi::c_char,
            -36 => return b"name too long\0" as *const u8 as *const ::core::ffi::c_char,
            -100 => return b"network is down\0" as *const u8 as *const ::core::ffi::c_char,
            -101 => {
                return b"network is unreachable\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -23 => return b"file table overflow\0" as *const u8 as *const ::core::ffi::c_char,
            -105 => {
                return b"no buffer space available\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -19 => return b"no such device\0" as *const u8 as *const ::core::ffi::c_char,
            -2 => {
                return b"no such file or directory\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -12 => return b"not enough memory\0" as *const u8 as *const ::core::ffi::c_char,
            -64 => {
                return b"machine is not on the network\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -92 => {
                return b"protocol not available\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -28 => {
                return b"no space left on device\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -38 => {
                return b"function not implemented\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -107 => {
                return b"socket is not connected\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -20 => return b"not a directory\0" as *const u8 as *const ::core::ffi::c_char,
            -39 => return b"directory not empty\0" as *const u8 as *const ::core::ffi::c_char,
            -88 => {
                return b"socket operation on non-socket\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -95 => {
                return b"operation not supported on socket\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -75 => {
                return b"value too large for defined data type\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -1 => {
                return b"operation not permitted\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -32 => return b"broken pipe\0" as *const u8 as *const ::core::ffi::c_char,
            -71 => return b"protocol error\0" as *const u8 as *const ::core::ffi::c_char,
            -93 => {
                return b"protocol not supported\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -91 => {
                return b"protocol wrong type for socket\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -34 => return b"result too large\0" as *const u8 as *const ::core::ffi::c_char,
            -30 => {
                return b"read-only file system\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -108 => {
                return b"cannot send after transport endpoint shutdown\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -29 => return b"invalid seek\0" as *const u8 as *const ::core::ffi::c_char,
            -3 => return b"no such process\0" as *const u8 as *const ::core::ffi::c_char,
            -110 => {
                return b"connection timed out\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -26 => return b"text file is busy\0" as *const u8 as *const ::core::ffi::c_char,
            -18 => {
                return b"cross-device link not permitted\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -4094 => return b"unknown error\0" as *const u8 as *const ::core::ffi::c_char,
            -4095 => return b"end of file\0" as *const u8 as *const ::core::ffi::c_char,
            -6 => {
                return b"no such device or address\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -31 => return b"too many links\0" as *const u8 as *const ::core::ffi::c_char,
            -112 => return b"host is down\0" as *const u8 as *const ::core::ffi::c_char,
            -121 => return b"remote I/O error\0" as *const u8 as *const ::core::ffi::c_char,
            -25 => {
                return b"inappropriate ioctl for device\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -4028 => {
                return b"inappropriate file type or format\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            -84 => {
                return b"illegal byte sequence\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -94 => {
                return b"socket type not supported\0" as *const u8 as *const ::core::ffi::c_char;
            }
            -61 => return b"no data available\0" as *const u8 as *const ::core::ffi::c_char,
            -49 => {
                return b"protocol driver not attached\0" as *const u8
                    as *const ::core::ffi::c_char;
            }
            _ => {}
        }
        return uv__unknown_err_code(err);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_ip4_addr(
    mut ip: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut addr: *mut sockaddr_in,
) -> ::core::ffi::c_int {
    unsafe {
        memset(
            addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in>() as size_t,
        );
        (*addr).sin_family = AF_INET as sa_family_t;
        (*addr).sin_port = htons(port as uint16_t) as in_port_t;
        return uv_inet_pton(
            AF_INET,
            ip,
            &raw mut (*addr).sin_addr.s_addr as *mut ::core::ffi::c_void,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_ip6_addr(
    mut ip: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut addr: *mut sockaddr_in6,
) -> ::core::ffi::c_int {
    unsafe {
        let mut address_part: [::core::ffi::c_char; 40] = [0; 40];
        let mut address_part_size: size_t = 0;
        let mut zone_index: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        memset(
            addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_in6>() as size_t,
        );
        (*addr).sin6_family = AF_INET6 as sa_family_t;
        (*addr).sin6_port = htons(port as uint16_t) as in_port_t;
        zone_index = strchr(ip, '%' as i32);
        if !zone_index.is_null() {
            address_part_size = zone_index.offset_from(ip) as ::core::ffi::c_long as size_t;
            if address_part_size >= ::core::mem::size_of::<[::core::ffi::c_char; 40]>() as usize {
                address_part_size = (::core::mem::size_of::<[::core::ffi::c_char; 40]>() as usize)
                    .wrapping_sub(1 as usize) as size_t;
            }
            memcpy(
                &raw mut address_part as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ip as *const ::core::ffi::c_void,
                address_part_size,
            );
            address_part[address_part_size as usize] = '\0' as i32 as ::core::ffi::c_char;
            ip = &raw mut address_part as *mut ::core::ffi::c_char;
            zone_index = zone_index.offset(1);
            (*addr).sin6_scope_id = if_nametoindex(zone_index) as uint32_t;
        }
        return uv_inet_pton(
            AF_INET6,
            ip,
            &raw mut (*addr).sin6_addr as *mut ::core::ffi::c_void,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_ip4_name(
    mut src: *const sockaddr_in,
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    unsafe {
        return uv_inet_ntop(
            AF_INET,
            &raw const (*src).sin_addr as *const ::core::ffi::c_void,
            dst,
            size,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_ip6_name(
    mut src: *const sockaddr_in6,
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    unsafe {
        return uv_inet_ntop(
            AF_INET6,
            &raw const (*src).sin6_addr as *const ::core::ffi::c_void,
            dst,
            size,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_ip_name(
    mut src: *const sockaddr,
    mut dst: *mut ::core::ffi::c_char,
    mut size: size_t,
) -> ::core::ffi::c_int {
    unsafe {
        match (*src).sa_family as ::core::ffi::c_int {
            AF_INET => {
                return uv_inet_ntop(
                    AF_INET,
                    &raw mut (*(src as *mut sockaddr_in)).sin_addr as *const ::core::ffi::c_void,
                    dst,
                    size,
                );
            }
            AF_INET6 => {
                return uv_inet_ntop(
                    AF_INET6,
                    &raw mut (*(src as *mut sockaddr_in6)).sin6_addr as *const ::core::ffi::c_void,
                    dst,
                    size,
                );
            }
            _ => return UV_EAFNOSUPPORT as ::core::ffi::c_int,
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_tcp_bind(
    mut handle: *mut uv_tcp_t,
    mut addr: *const sockaddr,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_uint = 0;
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*handle).flags
            & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
            addrlen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_uint;
        } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
            addrlen = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_uint;
        } else {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        return uv__tcp_bind(handle, addr, addrlen, flags);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_init_ex(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_udp_t,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut extra_flags: ::core::ffi::c_uint = 0;
        let mut domain: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        domain = (flags & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int;
        if domain != AF_INET && domain != AF_INET6 && domain != AF_UNSPEC {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        extra_flags = flags & !(0xff as ::core::ffi::c_int) as ::core::ffi::c_uint;
        if extra_flags & !(UV_UDP_RECVMMSG as ::core::ffi::c_int) as ::core::ffi::c_uint != 0 {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        rc = uv__udp_init_ex(loop_0, handle, flags, domain);
        if rc == 0 as ::core::ffi::c_int {
            if extra_flags & UV_UDP_RECVMMSG as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
                (*handle).flags |=
                    UV_HANDLE_UDP_RECVMMSG as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
        }
        return rc;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_udp_t,
) -> ::core::ffi::c_int {
    unsafe {
        return uv_udp_init_ex(loop_0, handle, AF_UNSPEC as ::core::ffi::c_uint);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_bind(
    mut handle: *mut uv_udp_t,
    mut addr: *const sockaddr,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_uint = 0;
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
            addrlen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_uint;
        } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
            addrlen = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_uint;
        } else {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        return uv__udp_bind(handle, addr, addrlen, flags);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_tcp_connect(
    mut req: *mut uv_connect_t,
    mut handle: *mut uv_tcp_t,
    mut addr: *const sockaddr,
    mut cb: uv_connect_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_uint = 0;
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
            addrlen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_uint;
        } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
            addrlen = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_uint;
        } else {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        return uv__tcp_connect(req, handle, addr, addrlen, cb);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_connect(
    mut handle: *mut uv_udp_t,
    mut addr: *const sockaddr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_uint = 0;
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if addr.is_null() {
            if (*handle).flags
                & UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
            {
                return UV_ENOTCONN as ::core::ffi::c_int;
            }
            return uv__udp_disconnect(handle);
        }
        if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
            addrlen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_uint;
        } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
            addrlen = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_uint;
        } else {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*handle).flags & UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            return UV_EISCONN as ::core::ffi::c_int;
        }
        return uv__udp_connect(handle, addr, addrlen);
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__udp_is_connected(mut handle: *mut uv_udp_t) -> ::core::ffi::c_int {
    unsafe {
        let mut addr: sockaddr_storage = sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        };
        let mut addrlen: ::core::ffi::c_int = 0;
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        addrlen = ::core::mem::size_of::<sockaddr_storage>() as ::core::ffi::c_int;
        if uv_udp_getpeername(handle, &raw mut addr as *mut sockaddr, &raw mut addrlen)
            != 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        return (addrlen > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__udp_check_before_send(
    mut handle: *mut uv_udp_t,
    mut addr: *const sockaddr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_uint = 0;
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if !addr.is_null()
            && (*handle).flags
                & UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            return UV_EISCONN as ::core::ffi::c_int;
        }
        if addr.is_null()
            && (*handle).flags
                & UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
                == 0
        {
            return UV_EDESTADDRREQ as ::core::ffi::c_int;
        }
        if !addr.is_null() {
            if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
                addrlen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_uint;
            } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
                addrlen = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_uint;
            } else if (*addr).sa_family as ::core::ffi::c_int == AF_UNIX {
                addrlen = ::core::mem::size_of::<sockaddr_un>() as ::core::ffi::c_uint;
            } else {
                return UV_EINVAL as ::core::ffi::c_int;
            }
        } else {
            addrlen = 0 as ::core::ffi::c_uint;
        }
        return addrlen as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_send(
    mut req: *mut uv_udp_send_t,
    mut handle: *mut uv_udp_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut addr: *const sockaddr,
    mut send_cb: uv_udp_send_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_int = 0;
        addrlen = uv__udp_check_before_send(handle, addr);
        if addrlen < 0 as ::core::ffi::c_int {
            return addrlen;
        }
        return uv__udp_send(
            req,
            handle,
            bufs,
            nbufs,
            addr,
            addrlen as ::core::ffi::c_uint,
            send_cb,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_try_send(
    mut handle: *mut uv_udp_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut addr: *const sockaddr,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrlen: ::core::ffi::c_int = 0;
        addrlen = uv__udp_check_before_send(handle, addr);
        if addrlen < 0 as ::core::ffi::c_int {
            return addrlen;
        }
        return uv__udp_try_send(handle, bufs, nbufs, addr, addrlen as ::core::ffi::c_uint);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_recv_start(
    mut handle: *mut uv_udp_t,
    mut alloc_cb: uv_alloc_cb,
    mut recv_cb: uv_udp_recv_cb,
) -> ::core::ffi::c_int {
    unsafe {
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
            || alloc_cb.is_none()
            || recv_cb.is_none()
        {
            return UV_EINVAL as ::core::ffi::c_int;
        } else {
            return uv__udp_recv_start(handle, alloc_cb, recv_cb);
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_udp_recv_stop(mut handle: *mut uv_udp_t) -> ::core::ffi::c_int {
    unsafe {
        if (*handle).type_0 as ::core::ffi::c_uint
            != UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        } else {
            return uv__udp_recv_stop(handle);
        };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_walk(
    mut loop_0: *mut uv_loop_t,
    mut walk_cb: uv_walk_cb,
    mut arg: *mut ::core::ffi::c_void,
) {
    unsafe {
        let mut queue: uv__queue = uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        };
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut h: *mut uv_handle_t = ::core::ptr::null_mut::<uv_handle_t>();
        uv__queue_move(&raw mut (*loop_0).handle_queue, &raw mut queue);
        while uv__queue_empty(&raw mut queue) == 0 {
            q = uv__queue_head(&raw mut queue);
            h = (q as *mut ::core::ffi::c_char).offset(-(32 as ::core::ffi::c_ulong as isize))
                as *mut uv_handle_t;
            uv__queue_remove(q);
            uv__queue_insert_tail(&raw mut (*loop_0).handle_queue, q);
            if (*h).flags & UV_HANDLE_INTERNAL as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
                continue;
            }
            walk_cb.expect("non-null function pointer")(h, arg);
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__print_handles(
    mut loop_0: *mut uv_loop_t,
    mut only_active: ::core::ffi::c_int,
    mut stream: *mut FILE,
) {
    unsafe {
        let mut type_0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut h: *mut uv_handle_t = ::core::ptr::null_mut::<uv_handle_t>();
        if loop_0.is_null() {
            loop_0 = uv_default_loop();
        }
        if stream.is_null() {
            stream = stderr;
        }
        q = (*loop_0).handle_queue.next;
        while q != &raw mut (*loop_0).handle_queue {
            h = (q as *mut ::core::ffi::c_char).offset(-(32 as ::core::ffi::c_ulong as isize))
                as *mut uv_handle_t;
            if !(only_active != 0
                && !((*h).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint))
            {
                match (*h).type_0 as ::core::ffi::c_uint {
                    1 => {
                        type_0 = b"async\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    2 => {
                        type_0 = b"check\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    3 => {
                        type_0 = b"fs_event\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    4 => {
                        type_0 = b"fs_poll\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    5 => {
                        type_0 = b"handle\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    6 => {
                        type_0 = b"idle\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    7 => {
                        type_0 = b"pipe\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    8 => {
                        type_0 = b"poll\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    9 => {
                        type_0 = b"prepare\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    10 => {
                        type_0 = b"process\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    11 => {
                        type_0 = b"stream\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    12 => {
                        type_0 = b"tcp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    13 => {
                        type_0 = b"timer\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    14 => {
                        type_0 = b"tty\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    15 => {
                        type_0 = b"udp\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    16 => {
                        type_0 = b"signal\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                    _ => {
                        type_0 = b"<unknown>\0" as *const u8 as *const ::core::ffi::c_char;
                    }
                }
                fprintf(
                    stream,
                    b"[%c%c%c] %-8s %p\n\0" as *const u8 as *const ::core::ffi::c_char,
                    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"R-\0")[((*h)
                        .flags
                        & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0)
                        as ::core::ffi::c_int
                        as usize] as ::core::ffi::c_int,
                    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"A-\0")[((*h)
                        .flags
                        & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0)
                        as ::core::ffi::c_int
                        as usize] as ::core::ffi::c_int,
                    ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"I-\0")[((*h)
                        .flags
                        & UV_HANDLE_INTERNAL as ::core::ffi::c_int as ::core::ffi::c_uint
                        == 0)
                        as ::core::ffi::c_int
                        as usize] as ::core::ffi::c_int,
                    type_0,
                    h as *mut ::core::ffi::c_void,
                );
            }
            q = (*q).next;
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_print_all_handles(mut loop_0: *mut uv_loop_t, mut stream: *mut FILE) {
    unsafe {
        uv__print_handles(loop_0, 0 as ::core::ffi::c_int, stream);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_print_active_handles(mut loop_0: *mut uv_loop_t, mut stream: *mut FILE) {
    unsafe {
        uv__print_handles(loop_0, 1 as ::core::ffi::c_int, stream);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_ref(mut handle: *mut uv_handle_t) {
    unsafe {
        if !((*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            (*handle).flags |= UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
            if !((*handle).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint)
            {
                if (*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    (*(*handle).loop_0).active_handles =
                        (*(*handle).loop_0).active_handles.wrapping_add(1);
                }
            }
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_unref(mut handle: *mut uv_handle_t) {
    unsafe {
        if !((*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0 as ::core::ffi::c_uint)
        {
            (*handle).flags &= !(UV_HANDLE_REF as ::core::ffi::c_int) as ::core::ffi::c_uint;
            if !((*handle).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint)
            {
                if (*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0 as ::core::ffi::c_uint
                {
                    (*(*handle).loop_0).active_handles =
                        (*(*handle).loop_0).active_handles.wrapping_sub(1);
                }
            }
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_has_ref(mut handle: *const uv_handle_t) -> ::core::ffi::c_int {
    unsafe {
        return ((*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_stop(mut loop_0: *mut uv_loop_t) {
    unsafe {
        (*loop_0).stop_flag = 1 as ::core::ffi::c_uint;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_now(mut loop_0: *const uv_loop_t) -> uint64_t {
    unsafe {
        return (*loop_0).time;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__count_bufs(
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
) -> size_t {
    unsafe {
        let mut i: ::core::ffi::c_uint = 0;
        let mut bytes: size_t = 0;
        bytes = 0 as size_t;
        i = 0 as ::core::ffi::c_uint;
        while i < nbufs {
            bytes = bytes.wrapping_add((*bufs.offset(i as isize)).len);
            i = i.wrapping_add(1);
        }
        return bytes;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_recv_buffer_size(
    mut handle: *mut uv_handle_t,
    mut value: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return uv__socket_sockopt(handle, SO_RCVBUF, value);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_send_buffer_size(
    mut handle: *mut uv_handle_t,
    mut value: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        return uv__socket_sockopt(handle, SO_SNDBUF, value);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_fs_event_getpath(
    mut handle: *mut uv_fs_event_t,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut required_len: size_t = 0;
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            *size = 0 as size_t;
            return UV_EINVAL as ::core::ffi::c_int;
        }
        required_len = strlen((*handle).path);
        if required_len >= *size {
            *size = required_len.wrapping_add(1 as size_t);
            return UV_ENOBUFS as ::core::ffi::c_int;
        }
        memcpy(
            buffer as *mut ::core::ffi::c_void,
            (*handle).path as *const ::core::ffi::c_void,
            required_len,
        );
        *size = required_len;
        *buffer.offset(required_len as isize) = '\0' as i32 as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__get_nbufs(mut req: *mut uv_fs_t) -> *mut ::core::ffi::c_uint {
    unsafe {
        return &raw mut (*req).nbufs;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__fs_scandir_cleanup(mut req: *mut uv_fs_t) {
    unsafe {
        let mut dents: *mut *mut uv__dirent_t = ::core::ptr::null_mut::<*mut uv__dirent_t>();
        let mut nbufs: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
        let mut i: ::core::ffi::c_uint = 0;
        let mut n: ::core::ffi::c_uint = 0;
        if (*req).result >= 0 as ssize_t {
            dents = (*req).ptr as *mut *mut uv__dirent_t;
            nbufs = uv__get_nbufs(req);
            i = 0 as ::core::ffi::c_uint;
            if *nbufs > 0 as ::core::ffi::c_uint {
                i = (*nbufs).wrapping_sub(1 as ::core::ffi::c_uint);
            }
            n = (*req).result as ::core::ffi::c_uint;
            while i < n {
                free(*dents.offset(i as isize) as *mut ::core::ffi::c_void);
                i = i.wrapping_add(1);
            }
        }
        free((*req).ptr);
        (*req).ptr = NULL;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_fs_scandir_next(
    mut req: *mut uv_fs_t,
    mut ent: *mut uv_dirent_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut dents: *mut *mut uv__dirent_t = ::core::ptr::null_mut::<*mut uv__dirent_t>();
        let mut dent: *mut uv__dirent_t = ::core::ptr::null_mut::<uv__dirent_t>();
        let mut nbufs: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<::core::ffi::c_uint>();
        if (*req).result < 0 as ssize_t {
            return (*req).result as ::core::ffi::c_int;
        }
        if (*req).ptr.is_null() {
            return UV_EOF as ::core::ffi::c_int;
        }
        nbufs = uv__get_nbufs(req);
        '_c2rust_label: {
            if !nbufs.is_null() {
            } else {
                __assert_fail(
                    b"nbufs\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/uv-common.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    725 as ::core::ffi::c_uint,
                    b"int uv_fs_scandir_next(uv_fs_t *, uv_dirent_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        dents = (*req).ptr as *mut *mut uv__dirent_t;
        if *nbufs > 0 as ::core::ffi::c_uint {
            free(
                *dents.offset((*nbufs).wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                    as *mut ::core::ffi::c_void,
            );
        }
        if *nbufs == (*req).result as ::core::ffi::c_uint {
            free(dents as *mut ::core::ffi::c_void);
            (*req).ptr = NULL;
            return UV_EOF as ::core::ffi::c_int;
        }
        let fresh0 = *nbufs;
        *nbufs = (*nbufs).wrapping_add(1);
        dent = *dents.offset(fresh0 as isize);
        (*ent).name = &raw mut (*dent).d_name as *mut ::core::ffi::c_char;
        (*ent).type_0 = uv__fs_get_dirent_type(dent);
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__fs_get_dirent_type(mut dent: *mut uv__dirent_t) -> uv_dirent_type_t {
    unsafe {
        let mut type_0: uv_dirent_type_t = UV_DIRENT_UNKNOWN;
        match (*dent).d_type as ::core::ffi::c_int {
            UV__DT_DIR => {
                type_0 = UV_DIRENT_DIR;
            }
            UV__DT_FILE => {
                type_0 = UV_DIRENT_FILE;
            }
            UV__DT_LINK => {
                type_0 = UV_DIRENT_LINK;
            }
            UV__DT_FIFO => {
                type_0 = UV_DIRENT_FIFO;
            }
            UV__DT_SOCKET => {
                type_0 = UV_DIRENT_SOCKET;
            }
            UV__DT_CHAR => {
                type_0 = UV_DIRENT_CHAR;
            }
            UV__DT_BLOCK => {
                type_0 = UV_DIRENT_BLOCK;
            }
            _ => {
                type_0 = UV_DIRENT_UNKNOWN;
            }
        }
        return type_0;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__fs_readdir_cleanup(mut req: *mut uv_fs_t) {
    unsafe {
        let mut dir: *mut uv_dir_t = ::core::ptr::null_mut::<uv_dir_t>();
        let mut dirents: *mut uv_dirent_t = ::core::ptr::null_mut::<uv_dirent_t>();
        let mut i: ::core::ffi::c_int = 0;
        if (*req).ptr.is_null() {
            return;
        }
        dir = (*req).ptr as *mut uv_dir_t;
        dirents = (*dir).dirents;
        (*req).ptr = NULL;
        if dirents.is_null() {
            return;
        }
        i = 0 as ::core::ffi::c_int;
        while (i as ssize_t) < (*req).result {
            uv__free(
                (*dirents.offset(i as isize)).name as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void,
            );
            let ref mut fresh1 = (*dirents.offset(i as isize)).name;
            *fresh1 = ::core::ptr::null::<::core::ffi::c_char>();
            i += 1;
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_configure(
    mut loop_0: *mut uv_loop_t,
    mut option: uv_loop_option,
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
        if option as ::core::ffi::c_uint
            == UV_METRICS_IDLE_TIME as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*lfields).flags |= UV_METRICS_IDLE_TIME as ::core::ffi::c_int as ::core::ffi::c_uint;
            return 0 as ::core::ffi::c_int;
        }
        if option as ::core::ffi::c_uint
            != UV_LOOP_BLOCK_SIGNAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return UV_ENOSYS as ::core::ffi::c_int;
        }
        if arg != 27 as ::core::ffi::c_int {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        (*loop_0).flags |= 1 as ::core::ffi::c_ulong;
        0
    }
}
static mut default_loop_struct: uv_loop_t = uv_loop_s {
    data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    active_handles: 0,
    handle_queue: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    active_reqs: C2RustUnnamed_6 {
        unused: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    },
    internal_fields: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    stop_flag: 0,
    flags: 0,
    backend_fd: 0,
    pending_queue: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    watcher_queue: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    watchers: ::core::ptr::null::<*mut uv__io_t>() as *mut *mut uv__io_t,
    nwatchers: 0,
    nfds: 0,
    wq: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    wq_mutex: pthread_mutex_t {
        __data: __pthread_mutex_s {
            __lock: 0,
            __count: 0,
            __owner: 0,
            __nusers: 0,
            __kind: 0,
            __spins: 0,
            __elision: 0,
            __list: __pthread_internal_list {
                __prev: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
                __next: ::core::ptr::null::<__pthread_internal_list>()
                    as *mut __pthread_internal_list,
            },
        },
    },
    wq_async: uv_async_s {
        data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        loop_0: ::core::ptr::null::<uv_loop_t>() as *mut uv_loop_t,
        type_0: UV_UNKNOWN_HANDLE,
        close_cb: None,
        handle_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        u: C2RustUnnamed_5 { fd: 0 },
        next_closing: ::core::ptr::null::<uv_handle_t>() as *mut uv_handle_t,
        flags: 0,
        async_cb: None,
        queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        pending: 0,
    },
    cloexec_lock: pthread_rwlock_t {
        __data: __pthread_rwlock_arch_t {
            __readers: 0,
            __writers: 0,
            __wrphase_futex: 0,
            __writers_futex: 0,
            __pad3: 0,
            __pad4: 0,
            __cur_writer: 0,
            __shared: 0,
            __rwelision: 0,
            __pad1: [0; 7],
            __pad2: 0,
            __flags: 0,
        },
    },
    closing_handles: ::core::ptr::null::<uv_handle_t>() as *mut uv_handle_t,
    process_handles: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    prepare_handles: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    check_handles: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    idle_handles: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    async_handles: uv__queue {
        next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
    },
    async_unused: None,
    async_io_watcher: uv__io_s {
        cb: None,
        pending_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        watcher_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        pevents: 0,
        events: 0,
        fd: 0,
    },
    async_wfd: 0,
    timer_heap: C2RustUnnamed_4 {
        min: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        nelts: 0,
    },
    timer_counter: 0,
    time: 0,
    signal_pipefd: [0; 2],
    signal_io_watcher: uv__io_s {
        cb: None,
        pending_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        watcher_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        pevents: 0,
        events: 0,
        fd: 0,
    },
    child_watcher: uv_signal_s {
        data: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
        loop_0: ::core::ptr::null::<uv_loop_t>() as *mut uv_loop_t,
        type_0: UV_UNKNOWN_HANDLE,
        close_cb: None,
        handle_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        u: C2RustUnnamed_3 { fd: 0 },
        next_closing: ::core::ptr::null::<uv_handle_t>() as *mut uv_handle_t,
        flags: 0,
        signal_cb: None,
        signum: 0,
        tree_entry: C2RustUnnamed_1 {
            rbe_left: ::core::ptr::null::<uv_signal_s>() as *mut uv_signal_s,
            rbe_right: ::core::ptr::null::<uv_signal_s>() as *mut uv_signal_s,
            rbe_parent: ::core::ptr::null::<uv_signal_s>() as *mut uv_signal_s,
            rbe_color: 0,
        },
        caught_signals: 0,
        dispatched_signals: 0,
    },
    emfile_fd: 0,
    inotify_read_watcher: uv__io_s {
        cb: None,
        pending_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        watcher_queue: uv__queue {
            next: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
            prev: ::core::ptr::null::<uv__queue>() as *mut uv__queue,
        },
        pevents: 0,
        events: 0,
        fd: 0,
    },
    inotify_watchers: ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void,
    inotify_fd: 0,
};
static mut default_loop_ptr: *mut uv_loop_t = ::core::ptr::null::<uv_loop_t>() as *mut uv_loop_t;
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_default_loop() -> *mut uv_loop_t {
    unsafe {
        if !default_loop_ptr.is_null() {
            return default_loop_ptr;
        }
        if uv_loop_init(&raw mut default_loop_struct) != 0 {
            return ::core::ptr::null_mut::<uv_loop_t>();
        }
        default_loop_ptr = &raw mut default_loop_struct;
        return default_loop_ptr;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_new() -> *mut uv_loop_t {
    unsafe {
        let mut loop_0: *mut uv_loop_t = ::core::ptr::null_mut::<uv_loop_t>();
        loop_0 = uv__malloc(::core::mem::size_of::<uv_loop_t>() as size_t) as *mut uv_loop_t;
        if loop_0.is_null() {
            return ::core::ptr::null_mut::<uv_loop_t>();
        }
        if uv_loop_init(loop_0) != 0 {
            uv__free(loop_0 as *mut ::core::ffi::c_void);
            return ::core::ptr::null_mut::<uv_loop_t>();
        }
        return loop_0;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_close(mut loop_0: *mut uv_loop_t) -> ::core::ffi::c_int {
    unsafe {
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut h: *mut uv_handle_t = ::core::ptr::null_mut::<uv_handle_t>();
        let mut saved_data: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        if (*loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
            return UV_EBUSY as ::core::ffi::c_int;
        }
        q = (*loop_0).handle_queue.next;
        while q != &raw mut (*loop_0).handle_queue {
            h = (q as *mut ::core::ffi::c_char).offset(-(32 as ::core::ffi::c_ulong as isize))
                as *mut uv_handle_t;
            if (*h).flags & UV_HANDLE_INTERNAL as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
                return UV_EBUSY as ::core::ffi::c_int;
            }
            q = (*q).next;
        }
        uv__loop_close(loop_0);
        saved_data = (*loop_0).data;
        memset(
            loop_0 as *mut ::core::ffi::c_void,
            -(1 as ::core::ffi::c_int),
            ::core::mem::size_of::<uv_loop_t>() as size_t,
        );
        (*loop_0).data = saved_data;
        if loop_0 == default_loop_ptr {
            default_loop_ptr = ::core::ptr::null_mut::<uv_loop_t>();
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_delete(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut default_loop: *mut uv_loop_t = ::core::ptr::null_mut::<uv_loop_t>();
        let mut err: ::core::ffi::c_int = 0;
        default_loop = default_loop_ptr;
        err = uv_loop_close(loop_0);
        '_c2rust_label: {
            if err == 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"err == 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/uv-common.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    889 as ::core::ffi::c_uint,
                    b"void uv_loop_delete(uv_loop_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if loop_0 != default_loop {
            uv__free(loop_0 as *mut ::core::ffi::c_void);
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_read_start(
    mut stream: *mut uv_stream_t,
    mut alloc_cb: uv_alloc_cb,
    mut read_cb: uv_read_cb,
) -> ::core::ffi::c_int {
    unsafe {
        if stream.is_null() || alloc_cb.is_none() || read_cb.is_none() {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*stream).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        if (*stream).flags & UV_HANDLE_READING as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            return UV_EALREADY as ::core::ffi::c_int;
        }
        if (*stream).flags & UV_HANDLE_READABLE as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
            return UV_ENOTCONN as ::core::ffi::c_int;
        }
        return uv__read_start(stream, alloc_cb, read_cb);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_os_free_environ(mut envitems: *mut uv_env_item_t, mut count: ::core::ffi::c_int) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < count {
            uv__free((*envitems.offset(i as isize)).name as *mut ::core::ffi::c_void);
            i += 1;
        }
        uv__free(envitems as *mut ::core::ffi::c_void);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_free_cpu_info(mut cpu_infos: *mut uv_cpu_info_t, mut count: ::core::ffi::c_int) {
    unsafe {
        &raw mut count;
        uv__free(cpu_infos as *mut ::core::ffi::c_void);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_library_shutdown() {
    unsafe {
        static mut was_shutdown: ::core::ffi::c_int = 0;
        if crate::upstream_support::atomics::atomic_xchg_relaxed_i32(
            &raw mut was_shutdown as *mut ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            return;
        }
        uv__process_title_cleanup();
        uv__signal_cleanup();
        uv__threadpool_cleanup();
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__metrics_update_idle_time(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut loop_metrics: *mut uv__loop_metrics_t =
            ::core::ptr::null_mut::<uv__loop_metrics_t>();
        let mut entry_time: uint64_t = 0;
        let mut exit_time: uint64_t = 0;
        if (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).flags
            & UV_METRICS_IDLE_TIME as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            return;
        }
        loop_metrics =
            &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).loop_metrics;
        if (*loop_metrics).provider_entry_time == 0 as uint64_t {
            return;
        }
        exit_time = uv_hrtime();
        uv_mutex_lock(&raw mut (*loop_metrics).lock);
        entry_time = (*loop_metrics).provider_entry_time;
        (*loop_metrics).provider_entry_time = 0 as uint64_t;
        (*loop_metrics).provider_idle_time = (*loop_metrics)
            .provider_idle_time
            .wrapping_add(exit_time.wrapping_sub(entry_time));
        uv_mutex_unlock(&raw mut (*loop_metrics).lock);
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__metrics_set_provider_entry_time(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut loop_metrics: *mut uv__loop_metrics_t =
            ::core::ptr::null_mut::<uv__loop_metrics_t>();
        let mut now: uint64_t = 0;
        if (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).flags
            & UV_METRICS_IDLE_TIME as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
            return;
        }
        now = uv_hrtime();
        loop_metrics =
            &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).loop_metrics;
        uv_mutex_lock(&raw mut (*loop_metrics).lock);
        (*loop_metrics).provider_entry_time = now;
        uv_mutex_unlock(&raw mut (*loop_metrics).lock);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_metrics_info(
    mut loop_0: *mut uv_loop_t,
    mut metrics: *mut uv_metrics_t,
) -> ::core::ffi::c_int {
    unsafe {
        memcpy(
            metrics as *mut ::core::ffi::c_void,
            &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t))
                .loop_metrics
                .metrics as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uv_metrics_t>() as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_metrics_idle_time(mut loop_0: *mut uv_loop_t) -> uint64_t {
    unsafe {
        let mut loop_metrics: *mut uv__loop_metrics_t =
            ::core::ptr::null_mut::<uv__loop_metrics_t>();
        let mut entry_time: uint64_t = 0;
        let mut idle_time: uint64_t = 0;
        loop_metrics =
            &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).loop_metrics;
        uv_mutex_lock(&raw mut (*loop_metrics).lock);
        idle_time = (*loop_metrics).provider_idle_time;
        entry_time = (*loop_metrics).provider_entry_time;
        uv_mutex_unlock(&raw mut (*loop_metrics).lock);
        if entry_time > 0 as uint64_t {
            idle_time = idle_time.wrapping_add(uv_hrtime().wrapping_sub(entry_time));
        }
        return idle_time;
    }
}
