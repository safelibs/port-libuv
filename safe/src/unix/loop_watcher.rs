pub type __uint64_t = u64;
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
pub struct uv_prepare_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_6,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub prepare_cb: uv_prepare_cb,
    pub queue: uv__queue,
}
pub type uv_prepare_cb = Option<unsafe extern "C" fn(*mut uv_prepare_t) -> ()>;
pub type uv_prepare_t = uv_prepare_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
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
    pub u: C2RustUnnamed_7,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub check_cb: uv_check_cb,
    pub queue: uv__queue,
}
pub type uv_check_cb = Option<unsafe extern "C" fn(*mut uv_check_t) -> ()>;
pub type uv_check_t = uv_check_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
    pub u: C2RustUnnamed_8,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub idle_cb: uv_idle_cb,
    pub queue: uv__queue,
}
pub type uv_idle_cb = Option<unsafe extern "C" fn(*mut uv_idle_t) -> ()>;
pub type uv_idle_t = uv_idle_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub const UV_HANDLE_REF: C2RustUnnamed_9 = 8;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_9 = 4;
pub type C2RustUnnamed_9 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_9 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_9 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_9 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_9 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_9 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_9 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_9 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_9 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_9 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_9 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_9 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_9 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_9 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_9 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_9 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_9 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_9 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_9 = 16777216;
pub const UV_HANDLE_IPV6: C2RustUnnamed_9 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_9 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_9 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_9 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_9 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_9 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_9 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_9 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_9 = 16384;
pub const UV_HANDLE_BOUND: C2RustUnnamed_9 = 8192;
pub const UV_HANDLE_READING: C2RustUnnamed_9 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_9 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_9 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_9 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_9 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_9 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_9 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_9 = 16;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_9 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_9 = 1;
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
extern "C" fn uv__queue_insert_head(mut h: *mut uv__queue, mut q: *mut uv__queue) {
    unsafe {
        (*q).next = (*h).next;
        (*q).prev = h;
        (*(*q).next).prev = q;
        (*h).next = q;
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
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__prepare_close(mut handle: *mut uv_prepare_t) {
    unsafe {
        uv_prepare_stop(handle);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_prepare_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_prepare_t,
) -> ::core::ffi::c_int {
    unsafe {
        let ref mut fresh0 = (*(handle as *mut uv_handle_t)).loop_0;
        *fresh0 = loop_0;
        (*(handle as *mut uv_handle_t)).type_0 = UV_PREPARE;
        (*(handle as *mut uv_handle_t)).flags =
            UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
        uv__queue_insert_tail(
            &raw mut (*loop_0).handle_queue,
            &raw mut (*(handle as *mut uv_handle_t)).handle_queue,
        );
        let ref mut fresh1 = (*(handle as *mut uv_handle_t)).next_closing;
        *fresh1 = ::core::ptr::null_mut::<uv_handle_t>();
        (*handle).prepare_cb = None;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_prepare_stop(mut handle: *mut uv_prepare_t) -> ::core::ffi::c_int {
    unsafe {
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            return 0 as ::core::ffi::c_int;
        }
        uv__queue_remove(&raw mut (*handle).queue);
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
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_prepare_start(
    mut handle: *mut uv_prepare_t,
    mut cb: uv_prepare_cb,
) -> ::core::ffi::c_int {
    unsafe {
        if (*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        if cb.is_none() {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        uv__queue_insert_head(
            &raw mut (*(*handle).loop_0).prepare_handles,
            &raw mut (*handle).queue,
        );
        (*handle).prepare_cb = cb;
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            (*handle).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
            if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*handle).loop_0).active_handles =
                    (*(*handle).loop_0).active_handles.wrapping_add(1);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__run_prepare(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut h: *mut uv_prepare_t = ::core::ptr::null_mut::<uv_prepare_t>();
        let mut queue: uv__queue = uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        };
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        uv__queue_move(&raw mut (*loop_0).prepare_handles, &raw mut queue);
        while uv__queue_empty(&raw mut queue) == 0 {
            q = uv__queue_head(&raw mut queue);
            h = (q as *mut ::core::ffi::c_char).offset(-(104 as ::core::ffi::c_ulong as isize))
                as *mut uv_prepare_t;
            uv__queue_remove(q);
            uv__queue_insert_tail(&raw mut (*loop_0).prepare_handles, q);
            (*h).prepare_cb.expect("non-null function pointer")(h);
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_check_stop(mut handle: *mut uv_check_t) -> ::core::ffi::c_int {
    unsafe {
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            return 0 as ::core::ffi::c_int;
        }
        uv__queue_remove(&raw mut (*handle).queue);
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
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__check_close(mut handle: *mut uv_check_t) {
    unsafe {
        uv_check_stop(handle);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_check_start(
    mut handle: *mut uv_check_t,
    mut cb: uv_check_cb,
) -> ::core::ffi::c_int {
    unsafe {
        if (*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        if cb.is_none() {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        uv__queue_insert_head(
            &raw mut (*(*handle).loop_0).check_handles,
            &raw mut (*handle).queue,
        );
        (*handle).check_cb = cb;
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            (*handle).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
            if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*handle).loop_0).active_handles =
                    (*(*handle).loop_0).active_handles.wrapping_add(1);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_check_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_check_t,
) -> ::core::ffi::c_int {
    unsafe {
        let ref mut fresh2 = (*(handle as *mut uv_handle_t)).loop_0;
        *fresh2 = loop_0;
        (*(handle as *mut uv_handle_t)).type_0 = UV_CHECK;
        (*(handle as *mut uv_handle_t)).flags =
            UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
        uv__queue_insert_tail(
            &raw mut (*loop_0).handle_queue,
            &raw mut (*(handle as *mut uv_handle_t)).handle_queue,
        );
        let ref mut fresh3 = (*(handle as *mut uv_handle_t)).next_closing;
        *fresh3 = ::core::ptr::null_mut::<uv_handle_t>();
        (*handle).check_cb = None;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__run_check(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut h: *mut uv_check_t = ::core::ptr::null_mut::<uv_check_t>();
        let mut queue: uv__queue = uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        };
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        uv__queue_move(&raw mut (*loop_0).check_handles, &raw mut queue);
        while uv__queue_empty(&raw mut queue) == 0 {
            q = uv__queue_head(&raw mut queue);
            h = (q as *mut ::core::ffi::c_char).offset(-(104 as ::core::ffi::c_ulong as isize))
                as *mut uv_check_t;
            uv__queue_remove(q);
            uv__queue_insert_tail(&raw mut (*loop_0).check_handles, q);
            (*h).check_cb.expect("non-null function pointer")(h);
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_idle_start(mut handle: *mut uv_idle_t, mut cb: uv_idle_cb) -> ::core::ffi::c_int {
    unsafe {
        if (*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return 0 as ::core::ffi::c_int;
        }
        if cb.is_none() {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        uv__queue_insert_head(
            &raw mut (*(*handle).loop_0).idle_handles,
            &raw mut (*handle).queue,
        );
        (*handle).idle_cb = cb;
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            (*handle).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
            if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint
            {
                (*(*handle).loop_0).active_handles =
                    (*(*handle).loop_0).active_handles.wrapping_add(1);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__run_idle(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut h: *mut uv_idle_t = ::core::ptr::null_mut::<uv_idle_t>();
        let mut queue: uv__queue = uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        };
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        uv__queue_move(&raw mut (*loop_0).idle_handles, &raw mut queue);
        while uv__queue_empty(&raw mut queue) == 0 {
            q = uv__queue_head(&raw mut queue);
            h = (q as *mut ::core::ffi::c_char).offset(-(104 as ::core::ffi::c_ulong as isize))
                as *mut uv_idle_t;
            uv__queue_remove(q);
            uv__queue_insert_tail(&raw mut (*loop_0).idle_handles, q);
            (*h).idle_cb.expect("non-null function pointer")(h);
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_idle_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_idle_t,
) -> ::core::ffi::c_int {
    unsafe {
        let ref mut fresh4 = (*(handle as *mut uv_handle_t)).loop_0;
        *fresh4 = loop_0;
        (*(handle as *mut uv_handle_t)).type_0 = UV_IDLE;
        (*(handle as *mut uv_handle_t)).flags =
            UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
        uv__queue_insert_tail(
            &raw mut (*loop_0).handle_queue,
            &raw mut (*(handle as *mut uv_handle_t)).handle_queue,
        );
        let ref mut fresh5 = (*(handle as *mut uv_handle_t)).next_closing;
        *fresh5 = ::core::ptr::null_mut::<uv_handle_t>();
        (*handle).idle_cb = None;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__idle_close(mut handle: *mut uv_idle_t) {
    unsafe {
        uv_idle_stop(handle);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_idle_stop(mut handle: *mut uv_idle_t) -> ::core::ffi::c_int {
    unsafe {
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            return 0 as ::core::ffi::c_int;
        }
        uv__queue_remove(&raw mut (*handle).queue);
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
        return 0 as ::core::ffi::c_int;
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn prepare_init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    prepare: *mut crate::abi::linux_x86_64::uv_prepare_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_prepare_init(loop_.cast(), prepare.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn prepare_start(
    prepare: *mut crate::abi::linux_x86_64::uv_prepare_t,
    cb: crate::abi::linux_x86_64::uv_prepare_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe { uv_prepare_start(prepare.cast(), std::mem::transmute::<_, uv_prepare_cb>(cb)) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn prepare_stop(
    prepare: *mut crate::abi::linux_x86_64::uv_prepare_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_prepare_stop(prepare.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn check_init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    check: *mut crate::abi::linux_x86_64::uv_check_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_check_init(loop_.cast(), check.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn check_start(
    check: *mut crate::abi::linux_x86_64::uv_check_t,
    cb: crate::abi::linux_x86_64::uv_check_cb,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_check_start(check.cast(), std::mem::transmute::<_, uv_check_cb>(cb)) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn check_stop(
    check: *mut crate::abi::linux_x86_64::uv_check_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_check_stop(check.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn idle_init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    idle: *mut crate::abi::linux_x86_64::uv_idle_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_idle_init(loop_.cast(), idle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn idle_start(
    idle: *mut crate::abi::linux_x86_64::uv_idle_t,
    cb: crate::abi::linux_x86_64::uv_idle_cb,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_idle_start(idle.cast(), std::mem::transmute::<_, uv_idle_cb>(cb)) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn idle_stop(idle: *mut crate::abi::linux_x86_64::uv_idle_t) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_idle_stop(idle.cast()) } }
}
