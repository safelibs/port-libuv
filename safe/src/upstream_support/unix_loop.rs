extern "C" {
    fn uv_async_init(
        _: *mut uv_loop_t,
        async_0: *mut uv_async_t,
        async_cb: uv_async_cb,
    ) -> ::core::ffi::c_int;
    fn uv_mutex_init(handle: *mut uv_mutex_t) -> ::core::ffi::c_int;
    fn uv_mutex_destroy(handle: *mut uv_mutex_t);
    fn uv_mutex_lock(handle: *mut uv_mutex_t);
    fn uv_mutex_unlock(handle: *mut uv_mutex_t);
    fn uv_rwlock_init(rwlock: *mut uv_rwlock_t) -> ::core::ffi::c_int;
    fn uv_rwlock_destroy(rwlock: *mut uv_rwlock_t);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn uv__work_done(handle: *mut uv_async_t);
    fn uv__calloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__free(ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn uv__close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__io_fork(loop_0: *mut uv_loop_t) -> ::core::ffi::c_int;
    fn uv__async_stop(loop_0: *mut uv_loop_t);
    fn uv__async_fork(loop_0: *mut uv_loop_t) -> ::core::ffi::c_int;
    fn uv__signal_global_once_init();
    fn uv__signal_loop_cleanup(loop_0: *mut uv_loop_t);
    fn uv__signal_loop_fork(loop_0: *mut uv_loop_t) -> ::core::ffi::c_int;
    fn uv__hrtime(type_0: uv_clocktype_t) -> uint64_t;
    fn uv__platform_loop_init(loop_0: *mut uv_loop_t) -> ::core::ffi::c_int;
    fn uv__platform_loop_delete(loop_0: *mut uv_loop_t);
    fn uv__process_init(loop_0: *mut uv_loop_t) -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2RustUnnamed_4,
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
    pub timer_heap: C2RustUnnamed_2,
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
    pub u: C2RustUnnamed_1,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2RustUnnamed,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
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
    pub u: C2RustUnnamed_0,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub union C2RustUnnamed_1 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub u: C2RustUnnamed_3,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub unused: *mut ::core::ffi::c_void,
    pub count: ::core::ffi::c_uint,
}
pub type C2RustUnnamed_5 = ::core::ffi::c_int;
pub const UV_ERRNO_MAX: C2RustUnnamed_5 = -4096;
pub const UV_EUNATCH: C2RustUnnamed_5 = -49;
pub const UV_ENODATA: C2RustUnnamed_5 = -61;
pub const UV_ESOCKTNOSUPPORT: C2RustUnnamed_5 = -94;
pub const UV_EILSEQ: C2RustUnnamed_5 = -84;
pub const UV_EFTYPE: C2RustUnnamed_5 = -4028;
pub const UV_ENOTTY: C2RustUnnamed_5 = -25;
pub const UV_EREMOTEIO: C2RustUnnamed_5 = -121;
pub const UV_EHOSTDOWN: C2RustUnnamed_5 = -112;
pub const UV_EMLINK: C2RustUnnamed_5 = -31;
pub const UV_ENXIO: C2RustUnnamed_5 = -6;
pub const UV_EOF: C2RustUnnamed_5 = -4095;
pub const UV_UNKNOWN: C2RustUnnamed_5 = -4094;
pub const UV_EXDEV: C2RustUnnamed_5 = -18;
pub const UV_ETXTBSY: C2RustUnnamed_5 = -26;
pub const UV_ETIMEDOUT: C2RustUnnamed_5 = -110;
pub const UV_ESRCH: C2RustUnnamed_5 = -3;
pub const UV_ESPIPE: C2RustUnnamed_5 = -29;
pub const UV_ESHUTDOWN: C2RustUnnamed_5 = -108;
pub const UV_EROFS: C2RustUnnamed_5 = -30;
pub const UV_ERANGE: C2RustUnnamed_5 = -34;
pub const UV_EPROTOTYPE: C2RustUnnamed_5 = -91;
pub const UV_EPROTONOSUPPORT: C2RustUnnamed_5 = -93;
pub const UV_EPROTO: C2RustUnnamed_5 = -71;
pub const UV_EPIPE: C2RustUnnamed_5 = -32;
pub const UV_EPERM: C2RustUnnamed_5 = -1;
pub const UV_EOVERFLOW: C2RustUnnamed_5 = -75;
pub const UV_ENOTSUP: C2RustUnnamed_5 = -95;
pub const UV_ENOTSOCK: C2RustUnnamed_5 = -88;
pub const UV_ENOTEMPTY: C2RustUnnamed_5 = -39;
pub const UV_ENOTDIR: C2RustUnnamed_5 = -20;
pub const UV_ENOTCONN: C2RustUnnamed_5 = -107;
pub const UV_ENOSYS: C2RustUnnamed_5 = -38;
pub const UV_ENOSPC: C2RustUnnamed_5 = -28;
pub const UV_ENOPROTOOPT: C2RustUnnamed_5 = -92;
pub const UV_ENONET: C2RustUnnamed_5 = -64;
pub const UV_ENOMEM: C2RustUnnamed_5 = -12;
pub const UV_ENOENT: C2RustUnnamed_5 = -2;
pub const UV_ENODEV: C2RustUnnamed_5 = -19;
pub const UV_ENOBUFS: C2RustUnnamed_5 = -105;
pub const UV_ENFILE: C2RustUnnamed_5 = -23;
pub const UV_ENETUNREACH: C2RustUnnamed_5 = -101;
pub const UV_ENETDOWN: C2RustUnnamed_5 = -100;
pub const UV_ENAMETOOLONG: C2RustUnnamed_5 = -36;
pub const UV_EMSGSIZE: C2RustUnnamed_5 = -90;
pub const UV_EMFILE: C2RustUnnamed_5 = -24;
pub const UV_ELOOP: C2RustUnnamed_5 = -40;
pub const UV_EISDIR: C2RustUnnamed_5 = -21;
pub const UV_EISCONN: C2RustUnnamed_5 = -106;
pub const UV_EIO: C2RustUnnamed_5 = -5;
pub const UV_EINVAL: C2RustUnnamed_5 = -22;
pub const UV_EINTR: C2RustUnnamed_5 = -4;
pub const UV_EHOSTUNREACH: C2RustUnnamed_5 = -113;
pub const UV_EFBIG: C2RustUnnamed_5 = -27;
pub const UV_EFAULT: C2RustUnnamed_5 = -14;
pub const UV_EEXIST: C2RustUnnamed_5 = -17;
pub const UV_EDESTADDRREQ: C2RustUnnamed_5 = -89;
pub const UV_ECONNRESET: C2RustUnnamed_5 = -104;
pub const UV_ECONNREFUSED: C2RustUnnamed_5 = -111;
pub const UV_ECONNABORTED: C2RustUnnamed_5 = -103;
pub const UV_ECHARSET: C2RustUnnamed_5 = -4080;
pub const UV_ECANCELED: C2RustUnnamed_5 = -125;
pub const UV_EBUSY: C2RustUnnamed_5 = -16;
pub const UV_EBADF: C2RustUnnamed_5 = -9;
pub const UV_EALREADY: C2RustUnnamed_5 = -114;
pub const UV_EAI_SOCKTYPE: C2RustUnnamed_5 = -3011;
pub const UV_EAI_SERVICE: C2RustUnnamed_5 = -3010;
pub const UV_EAI_PROTOCOL: C2RustUnnamed_5 = -3014;
pub const UV_EAI_OVERFLOW: C2RustUnnamed_5 = -3009;
pub const UV_EAI_NONAME: C2RustUnnamed_5 = -3008;
pub const UV_EAI_NODATA: C2RustUnnamed_5 = -3007;
pub const UV_EAI_MEMORY: C2RustUnnamed_5 = -3006;
pub const UV_EAI_FAMILY: C2RustUnnamed_5 = -3005;
pub const UV_EAI_FAIL: C2RustUnnamed_5 = -3004;
pub const UV_EAI_CANCELED: C2RustUnnamed_5 = -3003;
pub const UV_EAI_BADHINTS: C2RustUnnamed_5 = -3013;
pub const UV_EAI_BADFLAGS: C2RustUnnamed_5 = -3002;
pub const UV_EAI_AGAIN: C2RustUnnamed_5 = -3001;
pub const UV_EAI_ADDRFAMILY: C2RustUnnamed_5 = -3000;
pub const UV_EAGAIN: C2RustUnnamed_5 = -11;
pub const UV_EAFNOSUPPORT: C2RustUnnamed_5 = -97;
pub const UV_EADDRNOTAVAIL: C2RustUnnamed_5 = -99;
pub const UV_EADDRINUSE: C2RustUnnamed_5 = -98;
pub const UV_EACCES: C2RustUnnamed_5 = -13;
pub const UV_E2BIG: C2RustUnnamed_5 = -7;
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
pub type uv__loop_metrics_t = uv__loop_metrics_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__loop_metrics_s {
    pub metrics: uv_metrics_t,
    pub provider_entry_time: uint64_t,
    pub provider_idle_time: uint64_t,
    pub lock: uv_mutex_t,
}
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_6 = 16;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_6 = 4;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_6 = 1;
pub const UV_HANDLE_REF: C2RustUnnamed_6 = 8;
pub type uv_clocktype_t = ::core::ffi::c_uint;
pub const UV_CLOCK_FAST: uv_clocktype_t = 1;
pub const UV_CLOCK_PRECISE: uv_clocktype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap {
    pub min: *mut heap_node,
    pub nelts: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct heap_node {
    pub left: *mut heap_node,
    pub right: *mut heap_node,
    pub parent: *mut heap_node,
}
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_6 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_6 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_6 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_6 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_6 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_6 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_6 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_6 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_6 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_6 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_6 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_6 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_6 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_6 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_6 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_6 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_6 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_6 = 16777216;
pub const UV_HANDLE_IPV6: C2RustUnnamed_6 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_6 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_6 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_6 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_6 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_6 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_6 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_6 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_6 = 16384;
pub const UV_HANDLE_BOUND: C2RustUnnamed_6 = 8192;
pub const UV_HANDLE_READING: C2RustUnnamed_6 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_6 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_6 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_6 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_6 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_6 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_6 = 32;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_6 = 2;
pub const UV_LOOP_BLOCK_SIGPROF: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const UV_LOOP_REAP_CHILDREN: C2RustUnnamed_7 = 2;
pub const SIGPROF: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
// SAFETY(ffi_callback): this translated constant materializes a C string literal with the expected ABI type.
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<[u8; 33], [::core::ffi::c_char; 33]>(
        *b"void uv__loop_close(uv_loop_t *)\0",
    )
};
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
extern "C" fn uv__queue_insert_tail(mut h: *mut uv__queue, mut q: *mut uv__queue) {
    unsafe {
        (*q).next = h;
        (*q).prev = (*h).prev;
        (*(*q).prev).next = q;
        (*h).prev = q;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__update_time(mut loop_0: *mut uv_loop_t) {
    unsafe {
        (*loop_0).time = uv__hrtime(UV_CLOCK_FAST).wrapping_div(1000000 as uint64_t);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn heap_init(mut heap: *mut heap) {
    unsafe {
        (*heap).min = ::core::ptr::null_mut::<heap_node>();
        (*heap).nelts = 0 as ::core::ffi::c_uint;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_init(mut loop_0: *mut uv_loop_t) -> ::core::ffi::c_int {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        let mut saved_data: *mut ::core::ffi::c_void =
            ::core::ptr::null_mut::<::core::ffi::c_void>();
        let mut err: ::core::ffi::c_int = 0;
        saved_data = (*loop_0).data;
        memset(
            loop_0 as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uv_loop_t>() as size_t,
        );
        (*loop_0).data = saved_data;
        lfields = uv__calloc(
            1 as size_t,
            ::core::mem::size_of::<uv__loop_internal_fields_t>() as size_t,
        ) as *mut uv__loop_internal_fields_t;
        if lfields.is_null() {
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        (*loop_0).internal_fields = lfields as *mut ::core::ffi::c_void;
        err = uv_mutex_init(&raw mut (*lfields).loop_metrics.lock);
        if !(err != 0) {
            memset(
                &raw mut (*lfields).loop_metrics.metrics as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<uv_metrics_t>() as size_t,
            );
            heap_init(&raw mut (*loop_0).timer_heap as *mut heap);
            uv__queue_init(&raw mut (*loop_0).wq);
            uv__queue_init(&raw mut (*loop_0).idle_handles);
            uv__queue_init(&raw mut (*loop_0).async_handles);
            uv__queue_init(&raw mut (*loop_0).check_handles);
            uv__queue_init(&raw mut (*loop_0).prepare_handles);
            uv__queue_init(&raw mut (*loop_0).handle_queue);
            (*loop_0).active_handles = 0 as ::core::ffi::c_uint;
            (*loop_0).active_reqs.count = 0 as ::core::ffi::c_uint;
            (*loop_0).nfds = 0 as ::core::ffi::c_uint;
            (*loop_0).watchers = ::core::ptr::null_mut::<*mut uv__io_t>();
            (*loop_0).nwatchers = 0 as ::core::ffi::c_uint;
            uv__queue_init(&raw mut (*loop_0).pending_queue);
            uv__queue_init(&raw mut (*loop_0).watcher_queue);
            (*loop_0).closing_handles = ::core::ptr::null_mut::<uv_handle_t>();
            uv__update_time(loop_0);
            (*loop_0).async_io_watcher.fd = -(1 as ::core::ffi::c_int);
            (*loop_0).async_wfd = -(1 as ::core::ffi::c_int);
            (*loop_0).signal_pipefd[0 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
            (*loop_0).signal_pipefd[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
            (*loop_0).backend_fd = -(1 as ::core::ffi::c_int);
            (*loop_0).emfile_fd = -(1 as ::core::ffi::c_int);
            (*loop_0).timer_counter = 0 as uint64_t;
            (*loop_0).stop_flag = 0 as ::core::ffi::c_uint;
            err = uv__platform_loop_init(loop_0);
            if !(err != 0) {
                uv__signal_global_once_init();
                err = uv__process_init(loop_0);
                if !(err != 0) {
                    uv__queue_init(&raw mut (*loop_0).process_handles);
                    err = uv_rwlock_init(&raw mut (*loop_0).cloexec_lock);
                    if !(err != 0) {
                        err = uv_mutex_init(&raw mut (*loop_0).wq_mutex);
                        if !(err != 0) {
                            err = uv_async_init(
                                loop_0,
                                &raw mut (*loop_0).wq_async,
                                Some(uv__work_done as unsafe extern "C" fn(*mut uv_async_t) -> ()),
                            );
                            if err != 0 {
                                uv_mutex_destroy(&raw mut (*loop_0).wq_mutex);
                            } else {
                                if !((*loop_0).wq_async.flags
                                    & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                                    == 0 as ::core::ffi::c_uint)
                                {
                                    (*loop_0).wq_async.flags &= !(UV_HANDLE_REF
                                        as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint;
                                    if !((*loop_0).wq_async.flags
                                        & UV_HANDLE_CLOSING as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                        != 0 as ::core::ffi::c_uint)
                                    {
                                        if (*loop_0).wq_async.flags
                                            & UV_HANDLE_ACTIVE as ::core::ffi::c_int
                                                as ::core::ffi::c_uint
                                            != 0 as ::core::ffi::c_uint
                                        {
                                            (*(*loop_0).wq_async.loop_0).active_handles =
                                                (*(*loop_0).wq_async.loop_0)
                                                    .active_handles
                                                    .wrapping_sub(1);
                                        }
                                    }
                                }
                                (*loop_0).wq_async.flags |=
                                    UV_HANDLE_INTERNAL as ::core::ffi::c_int as ::core::ffi::c_uint;
                                return 0 as ::core::ffi::c_int;
                            }
                        }
                        uv_rwlock_destroy(&raw mut (*loop_0).cloexec_lock);
                    }
                    uv__signal_loop_cleanup(loop_0);
                }
                uv__platform_loop_delete(loop_0);
            }
            uv_mutex_destroy(&raw mut (*lfields).loop_metrics.lock);
        }
        uv__free(lfields as *mut ::core::ffi::c_void);
        (*loop_0).internal_fields = NULL;
        uv__free((*loop_0).watchers as *mut ::core::ffi::c_void);
        (*loop_0).nwatchers = 0 as ::core::ffi::c_uint;
        return err;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loop_fork(mut loop_0: *mut uv_loop_t) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_uint = 0;
        let mut w: *mut uv__io_t = ::core::ptr::null_mut::<uv__io_t>();
        err = uv__io_fork(loop_0);
        if err != 0 {
            return err;
        }
        err = uv__async_fork(loop_0);
        if err != 0 {
            return err;
        }
        err = uv__signal_loop_fork(loop_0);
        if err != 0 {
            return err;
        }
        i = 0 as ::core::ffi::c_uint;
        while i < (*loop_0).nwatchers {
            w = *(*loop_0).watchers.offset(i as isize);
            if !w.is_null() {
                if (*w).pevents != 0 as ::core::ffi::c_uint
                    && uv__queue_empty(&raw mut (*w).watcher_queue) != 0
                {
                    (*w).events = 0 as ::core::ffi::c_uint;
                    uv__queue_insert_tail(
                        &raw mut (*loop_0).watcher_queue,
                        &raw mut (*w).watcher_queue,
                    );
                }
            }
            i = i.wrapping_add(1);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__loop_close(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        uv__signal_loop_cleanup(loop_0);
        uv__platform_loop_delete(loop_0);
        uv__async_stop(loop_0);
        if (*loop_0).emfile_fd != -(1 as ::core::ffi::c_int) {
            uv__close((*loop_0).emfile_fd);
            (*loop_0).emfile_fd = -(1 as ::core::ffi::c_int);
        }
        if (*loop_0).backend_fd != -(1 as ::core::ffi::c_int) {
            uv__close((*loop_0).backend_fd);
            (*loop_0).backend_fd = -(1 as ::core::ffi::c_int);
        }
        uv_mutex_lock(&raw mut (*loop_0).wq_mutex);
        '_c2rust_label: {
            if uv__queue_empty(&raw mut (*loop_0).wq) != 0
                && !(b"thread pool work queue not empty!\0" as *const u8
                    as *const ::core::ffi::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"uv__queue_empty(&loop->wq) && \"thread pool work queue not empty!\"\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/loop.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    183 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        '_c2rust_label_0: {
            if !((*loop_0).active_reqs.count > 0 as ::core::ffi::c_uint) {
            } else {
                __assert_fail(
                    b"!uv__has_active_reqs(loop)\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/loop.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    184 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
        };
        uv_mutex_unlock(&raw mut (*loop_0).wq_mutex);
        uv_mutex_destroy(&raw mut (*loop_0).wq_mutex);
        uv_rwlock_destroy(&raw mut (*loop_0).cloexec_lock);
        uv__free((*loop_0).watchers as *mut ::core::ffi::c_void);
        (*loop_0).watchers = ::core::ptr::null_mut::<*mut uv__io_t>();
        (*loop_0).nwatchers = 0 as ::core::ffi::c_uint;
        lfields = (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
        uv_mutex_destroy(&raw mut (*lfields).loop_metrics.lock);
        uv__free(lfields as *mut ::core::ffi::c_void);
        (*loop_0).internal_fields = NULL;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__loop_configure(
    mut loop_0: *mut uv_loop_t,
    mut option: uv_loop_option,
    mut arg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        lfields = (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
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
        if arg != SIGPROF {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        (*loop_0).flags |= UV_LOOP_BLOCK_SIGPROF as ::core::ffi::c_int as ::core::ffi::c_ulong;
        return 0 as ::core::ffi::c_int;
    }
}
