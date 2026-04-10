extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn uv__nonblock_ioctl(fd: ::core::ffi::c_int, set: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__nonblock_fcntl(fd: ::core::ffi::c_int, set: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__io_init(w: *mut uv__io_t, cb: uv__io_cb, fd: ::core::ffi::c_int);
    fn uv__io_start(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_stop(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_check_fd(loop_0: *mut uv_loop_t, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__fd_exists(loop_0: *mut uv_loop_t, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__platform_invalidate_fd(loop_0: *mut uv_loop_t, fd: ::core::ffi::c_int);
}
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
pub type uv_os_sock_t = ::core::ffi::c_int;
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
pub struct uv_poll_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_6,
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
pub union C2RustUnnamed_6 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_poll_event = ::core::ffi::c_uint;
pub const UV_PRIORITIZED: uv_poll_event = 8;
pub const UV_DISCONNECT: uv_poll_event = 4;
pub const UV_WRITABLE: uv_poll_event = 2;
pub const UV_READABLE: uv_poll_event = 1;
pub const UV_HANDLE_REF: C2RustUnnamed_7 = 8;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_7 = 4;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_7 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_7 = 1;
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_7 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_7 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_7 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_7 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_7 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_7 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_7 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_7 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_7 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_7 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_7 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_7 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_7 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_7 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_7 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_7 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_7 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_7 = 16777216;
pub const UV_HANDLE_IPV6: C2RustUnnamed_7 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_7 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_7 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_7 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_7 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_7 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_7 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_7 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_7 = 16384;
pub const UV_HANDLE_BOUND: C2RustUnnamed_7 = 8192;
pub const UV_HANDLE_READING: C2RustUnnamed_7 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_7 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_7 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_7 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_7 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_7 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_7 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_7 = 16;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
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
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLPRI: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLRDHUP: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const UV__POLLRDHUP: ::core::ffi::c_int = POLLRDHUP;
pub const UV__POLLPRI: ::core::ffi::c_int = POLLPRI;
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__poll_io(
    mut loop_0: *mut uv_loop_t,
    mut w: *mut uv__io_t,
    mut events: ::core::ffi::c_uint,
) {
    unsafe {
        let mut handle: *mut uv_poll_t = ::core::ptr::null_mut::<uv_poll_t>();
        let mut pevents: ::core::ffi::c_int = 0;
        handle = (w as *mut ::core::ffi::c_char).offset(-(104 as ::core::ffi::c_ulong as isize))
            as *mut uv_poll_t;
        if events & POLLERR as ::core::ffi::c_uint != 0
            && events & UV__POLLPRI as ::core::ffi::c_uint == 0
        {
            uv__io_stop(
                loop_0,
                w,
                (POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI) as ::core::ffi::c_uint,
            );
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
            (*handle).poll_cb.expect("non-null function pointer")(
                handle,
                UV_EBADF as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            return;
        }
        pevents = 0 as ::core::ffi::c_int;
        if events & POLLIN as ::core::ffi::c_uint != 0 {
            pevents |= UV_READABLE as ::core::ffi::c_int;
        }
        if events & UV__POLLPRI as ::core::ffi::c_uint != 0 {
            pevents |= UV_PRIORITIZED as ::core::ffi::c_int;
        }
        if events & POLLOUT as ::core::ffi::c_uint != 0 {
            pevents |= UV_WRITABLE as ::core::ffi::c_int;
        }
        if events & UV__POLLRDHUP as ::core::ffi::c_uint != 0 {
            pevents |= UV_DISCONNECT as ::core::ffi::c_int;
        }
        (*handle).poll_cb.expect("non-null function pointer")(
            handle,
            0 as ::core::ffi::c_int,
            pevents,
        );
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_poll_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_poll_t,
    mut fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        if uv__fd_exists(loop_0, fd) != 0 {
            return UV_EEXIST as ::core::ffi::c_int;
        }
        err = uv__io_check_fd(loop_0, fd);
        if err != 0 {
            return err;
        }
        err = uv__nonblock_ioctl(fd, 1 as ::core::ffi::c_int);
        if err == UV_ENOTTY as ::core::ffi::c_int {
            err = uv__nonblock_fcntl(fd, 1 as ::core::ffi::c_int);
        }
        if err != 0 {
            return err;
        }
        let ref mut fresh0 = (*(handle as *mut uv_handle_t)).loop_0;
        *fresh0 = loop_0;
        (*(handle as *mut uv_handle_t)).type_0 = UV_POLL;
        (*(handle as *mut uv_handle_t)).flags =
            UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
        uv__queue_insert_tail(
            &raw mut (*loop_0).handle_queue,
            &raw mut (*(handle as *mut uv_handle_t)).handle_queue,
        );
        let ref mut fresh1 = (*(handle as *mut uv_handle_t)).next_closing;
        *fresh1 = ::core::ptr::null_mut::<uv_handle_t>();
        uv__io_init(
            &raw mut (*handle).io_watcher,
            Some(
                uv__poll_io
                    as unsafe extern "C" fn(
                        *mut uv_loop_t,
                        *mut uv__io_t,
                        ::core::ffi::c_uint,
                    ) -> (),
            ),
            fd,
        );
        (*handle).poll_cb = None;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_poll_init_socket(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_poll_t,
    mut socket: uv_os_sock_t,
) -> ::core::ffi::c_int {
    unsafe {
        return uv_poll_init(loop_0, handle, socket as ::core::ffi::c_int);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__poll_stop(mut handle: *mut uv_poll_t) {
    unsafe {
        uv__io_stop(
            (*handle).loop_0,
            &raw mut (*handle).io_watcher,
            (POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI) as ::core::ffi::c_uint,
        );
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
        uv__platform_invalidate_fd((*handle).loop_0, (*handle).io_watcher.fd);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_poll_stop(mut handle: *mut uv_poll_t) -> ::core::ffi::c_int {
    unsafe {
        '_c2rust_label: {
            if !((*handle).flags
                & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint)
            {
            } else {
                __assert_fail(
                    b"!uv__is_closing(handle)\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/poll.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    113 as ::core::ffi::c_uint,
                    b"int uv_poll_stop(uv_poll_t *)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        uv__poll_stop(handle);
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_poll_start(
    mut handle: *mut uv_poll_t,
    mut pevents: ::core::ffi::c_int,
    mut poll_cb: uv_poll_cb,
) -> ::core::ffi::c_int {
    unsafe {
        let mut watchers: *mut *mut uv__io_t = ::core::ptr::null_mut::<*mut uv__io_t>();
        let mut w: *mut uv__io_t = ::core::ptr::null_mut::<uv__io_t>();
        let mut events: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if pevents
                & !(UV_READABLE as ::core::ffi::c_int
                    | UV_WRITABLE as ::core::ffi::c_int
                    | UV_DISCONNECT as ::core::ffi::c_int
                    | UV_PRIORITIZED as ::core::ffi::c_int)
                == 0 as ::core::ffi::c_int
            {
            } else {
                __assert_fail(
                    b"(pevents & ~(UV_READABLE | UV_WRITABLE | UV_DISCONNECT | UV_PRIORITIZED)) == 0\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/poll.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    125 as ::core::ffi::c_uint,
                    b"int uv_poll_start(uv_poll_t *, int, uv_poll_cb)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if !((*handle).flags
                & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                != 0 as ::core::ffi::c_uint)
            {
            } else {
                __assert_fail(
                    b"!uv__is_closing(handle)\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/poll.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    126 as ::core::ffi::c_uint,
                    b"int uv_poll_start(uv_poll_t *, int, uv_poll_cb)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        watchers = (*(*handle).loop_0).watchers;
        w = &raw mut (*handle).io_watcher;
        if uv__fd_exists((*handle).loop_0, (*w).fd) != 0 {
            if *watchers.offset((*w).fd as isize) != w {
                return UV_EEXIST as ::core::ffi::c_int;
            }
        }
        uv__poll_stop(handle);
        if pevents == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        events = 0 as ::core::ffi::c_int;
        if pevents & UV_READABLE as ::core::ffi::c_int != 0 {
            events |= POLLIN;
        }
        if pevents & UV_PRIORITIZED as ::core::ffi::c_int != 0 {
            events |= UV__POLLPRI;
        }
        if pevents & UV_WRITABLE as ::core::ffi::c_int != 0 {
            events |= POLLOUT;
        }
        if pevents & UV_DISCONNECT as ::core::ffi::c_int != 0 {
            events |= UV__POLLRDHUP;
        }
        uv__io_start(
            (*handle).loop_0,
            &raw mut (*handle).io_watcher,
            events as ::core::ffi::c_uint,
        );
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
        (*handle).poll_cb = poll_cb;
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__poll_close(mut handle: *mut uv_poll_t) {
    unsafe {
        uv__poll_stop(handle);
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_poll_t,
    fd: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_poll_init(loop_.cast(), handle.cast(), fd) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init_socket(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_poll_t,
    socket: crate::abi::linux_x86_64::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_poll_init_socket(loop_.cast(), handle.cast(), socket) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn start(
    handle: *mut crate::abi::linux_x86_64::uv_poll_t,
    events: ::std::os::raw::c_int,
    cb: crate::abi::linux_x86_64::uv_poll_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            uv_poll_start(
                handle.cast(),
                events,
                std::mem::transmute::<_, uv_poll_cb>(cb),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn stop(handle: *mut crate::abi::linux_x86_64::uv_poll_t) -> ::std::os::raw::c_int {
    unsafe { unsafe { uv_poll_stop(handle.cast()) } }
}
