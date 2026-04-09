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
    fn sendmsg(
        __fd: ::core::ffi::c_int,
        __message: *const msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn sendmmsg(
        __fd: ::core::ffi::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: ::core::ffi::c_uint,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn recvmsg(
        __fd: ::core::ffi::c_int,
        __message: *mut msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn recvmmsg(
        __fd: ::core::ffi::c_int,
        __vmessages: *mut mmsghdr,
        __vlen: ::core::ffi::c_uint,
        __flags: ::core::ffi::c_int,
        __tmo: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    static in6addr_any: in6_addr;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
    fn uv_buf_init(base: *mut ::core::ffi::c_char, len: ::core::ffi::c_uint) -> uv_buf_t;
    fn uv_ip4_addr(
        ip: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        addr: *mut sockaddr_in,
    ) -> ::core::ffi::c_int;
    fn uv_ip6_addr(
        ip: *const ::core::ffi::c_char,
        port: ::core::ffi::c_int,
        addr: *mut sockaddr_in6,
    ) -> ::core::ffi::c_int;
    fn uv_inet_pton(
        af: ::core::ffi::c_int,
        src: *const ::core::ffi::c_char,
        dst: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn uv__udp_is_connected(handle: *mut uv_udp_t) -> ::core::ffi::c_int;
    fn uv__count_bufs(bufs: *const uv_buf_t, nbufs: ::core::ffi::c_uint) -> size_t;
    fn uv__malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__free(ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
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
    fn uv__io_init(w: *mut uv__io_t, cb: uv__io_cb, fd: ::core::ffi::c_int);
    fn uv__io_start(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_stop(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_close(loop_0: *mut uv_loop_t, w: *mut uv__io_t);
    fn uv__io_feed(loop_0: *mut uv_loop_t, w: *mut uv__io_t);
    fn uv__io_active(w: *const uv__io_t, events: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn uv__fd_exists(loop_0: *mut uv_loop_t, fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type __time_t = ::core::ffi::c_long;
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
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
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
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const MSG_CMSG_CLOEXEC: C2RustUnnamed = 1073741824;
pub const MSG_FASTOPEN: C2RustUnnamed = 536870912;
pub const MSG_ZEROCOPY: C2RustUnnamed = 67108864;
pub const MSG_BATCH: C2RustUnnamed = 262144;
pub const MSG_WAITFORONE: C2RustUnnamed = 65536;
pub const MSG_MORE: C2RustUnnamed = 32768;
pub const MSG_NOSIGNAL: C2RustUnnamed = 16384;
pub const MSG_ERRQUEUE: C2RustUnnamed = 8192;
pub const MSG_RST: C2RustUnnamed = 4096;
pub const MSG_CONFIRM: C2RustUnnamed = 2048;
pub const MSG_SYN: C2RustUnnamed = 1024;
pub const MSG_FIN: C2RustUnnamed = 512;
pub const MSG_WAITALL: C2RustUnnamed = 256;
pub const MSG_EOR: C2RustUnnamed = 128;
pub const MSG_DONTWAIT: C2RustUnnamed = 64;
pub const MSG_TRUNC: C2RustUnnamed = 32;
pub const MSG_PROXY: C2RustUnnamed = 16;
pub const MSG_CTRUNC: C2RustUnnamed = 8;
pub const MSG_TRYHARD: C2RustUnnamed = 4;
pub const MSG_DONTROUTE: C2RustUnnamed = 4;
pub const MSG_PEEK: C2RustUnnamed = 2;
pub const MSG_OOB: C2RustUnnamed = 1;
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
pub struct mmsghdr {
    pub msg_hdr: msghdr,
    pub msg_len: ::core::ffi::c_uint,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_1 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_1 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_1 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreq {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreq_source {
    pub imr_multiaddr: in_addr,
    pub imr_interface: in_addr,
    pub imr_sourceaddr: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group_source_req {
    pub gsr_interface: uint32_t,
    pub gsr_group: sockaddr_storage,
    pub gsr_source: sockaddr_storage,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_loop_s {
    pub data: *mut ::core::ffi::c_void,
    pub active_handles: ::core::ffi::c_uint,
    pub handle_queue: uv__queue,
    pub active_reqs: C2RustUnnamed_7,
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
    pub timer_heap: C2RustUnnamed_5,
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
    pub u: C2RustUnnamed_4,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub signal_cb: uv_signal_cb,
    pub signum: ::core::ffi::c_int,
    pub tree_entry: C2RustUnnamed_2,
    pub caught_signals: ::core::ffi::c_uint,
    pub dispatched_signals: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
    pub u: C2RustUnnamed_3,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub union C2RustUnnamed_4 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
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
    pub u: C2RustUnnamed_6,
    pub next_closing: *mut uv_handle_t,
    pub flags: ::core::ffi::c_uint,
    pub async_cb: uv_async_cb,
    pub queue: uv__queue,
    pub pending: ::core::ffi::c_int,
}
pub type uv_async_cb = Option<unsafe extern "C" fn(*mut uv_async_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_mutex_t = pthread_mutex_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
pub type C2RustUnnamed_8 = ::core::ffi::c_int;
pub const UV_ERRNO_MAX: C2RustUnnamed_8 = -4096;
pub const UV_EUNATCH: C2RustUnnamed_8 = -49;
pub const UV_ENODATA: C2RustUnnamed_8 = -61;
pub const UV_ESOCKTNOSUPPORT: C2RustUnnamed_8 = -94;
pub const UV_EILSEQ: C2RustUnnamed_8 = -84;
pub const UV_EFTYPE: C2RustUnnamed_8 = -4028;
pub const UV_ENOTTY: C2RustUnnamed_8 = -25;
pub const UV_EREMOTEIO: C2RustUnnamed_8 = -121;
pub const UV_EHOSTDOWN: C2RustUnnamed_8 = -112;
pub const UV_EMLINK: C2RustUnnamed_8 = -31;
pub const UV_ENXIO: C2RustUnnamed_8 = -6;
pub const UV_EOF: C2RustUnnamed_8 = -4095;
pub const UV_UNKNOWN: C2RustUnnamed_8 = -4094;
pub const UV_EXDEV: C2RustUnnamed_8 = -18;
pub const UV_ETXTBSY: C2RustUnnamed_8 = -26;
pub const UV_ETIMEDOUT: C2RustUnnamed_8 = -110;
pub const UV_ESRCH: C2RustUnnamed_8 = -3;
pub const UV_ESPIPE: C2RustUnnamed_8 = -29;
pub const UV_ESHUTDOWN: C2RustUnnamed_8 = -108;
pub const UV_EROFS: C2RustUnnamed_8 = -30;
pub const UV_ERANGE: C2RustUnnamed_8 = -34;
pub const UV_EPROTOTYPE: C2RustUnnamed_8 = -91;
pub const UV_EPROTONOSUPPORT: C2RustUnnamed_8 = -93;
pub const UV_EPROTO: C2RustUnnamed_8 = -71;
pub const UV_EPIPE: C2RustUnnamed_8 = -32;
pub const UV_EPERM: C2RustUnnamed_8 = -1;
pub const UV_EOVERFLOW: C2RustUnnamed_8 = -75;
pub const UV_ENOTSUP: C2RustUnnamed_8 = -95;
pub const UV_ENOTSOCK: C2RustUnnamed_8 = -88;
pub const UV_ENOTEMPTY: C2RustUnnamed_8 = -39;
pub const UV_ENOTDIR: C2RustUnnamed_8 = -20;
pub const UV_ENOTCONN: C2RustUnnamed_8 = -107;
pub const UV_ENOSYS: C2RustUnnamed_8 = -38;
pub const UV_ENOSPC: C2RustUnnamed_8 = -28;
pub const UV_ENOPROTOOPT: C2RustUnnamed_8 = -92;
pub const UV_ENONET: C2RustUnnamed_8 = -64;
pub const UV_ENOMEM: C2RustUnnamed_8 = -12;
pub const UV_ENOENT: C2RustUnnamed_8 = -2;
pub const UV_ENODEV: C2RustUnnamed_8 = -19;
pub const UV_ENOBUFS: C2RustUnnamed_8 = -105;
pub const UV_ENFILE: C2RustUnnamed_8 = -23;
pub const UV_ENETUNREACH: C2RustUnnamed_8 = -101;
pub const UV_ENETDOWN: C2RustUnnamed_8 = -100;
pub const UV_ENAMETOOLONG: C2RustUnnamed_8 = -36;
pub const UV_EMSGSIZE: C2RustUnnamed_8 = -90;
pub const UV_EMFILE: C2RustUnnamed_8 = -24;
pub const UV_ELOOP: C2RustUnnamed_8 = -40;
pub const UV_EISDIR: C2RustUnnamed_8 = -21;
pub const UV_EISCONN: C2RustUnnamed_8 = -106;
pub const UV_EIO: C2RustUnnamed_8 = -5;
pub const UV_EINVAL: C2RustUnnamed_8 = -22;
pub const UV_EINTR: C2RustUnnamed_8 = -4;
pub const UV_EHOSTUNREACH: C2RustUnnamed_8 = -113;
pub const UV_EFBIG: C2RustUnnamed_8 = -27;
pub const UV_EFAULT: C2RustUnnamed_8 = -14;
pub const UV_EEXIST: C2RustUnnamed_8 = -17;
pub const UV_EDESTADDRREQ: C2RustUnnamed_8 = -89;
pub const UV_ECONNRESET: C2RustUnnamed_8 = -104;
pub const UV_ECONNREFUSED: C2RustUnnamed_8 = -111;
pub const UV_ECONNABORTED: C2RustUnnamed_8 = -103;
pub const UV_ECHARSET: C2RustUnnamed_8 = -4080;
pub const UV_ECANCELED: C2RustUnnamed_8 = -125;
pub const UV_EBUSY: C2RustUnnamed_8 = -16;
pub const UV_EBADF: C2RustUnnamed_8 = -9;
pub const UV_EALREADY: C2RustUnnamed_8 = -114;
pub const UV_EAI_SOCKTYPE: C2RustUnnamed_8 = -3011;
pub const UV_EAI_SERVICE: C2RustUnnamed_8 = -3010;
pub const UV_EAI_PROTOCOL: C2RustUnnamed_8 = -3014;
pub const UV_EAI_OVERFLOW: C2RustUnnamed_8 = -3009;
pub const UV_EAI_NONAME: C2RustUnnamed_8 = -3008;
pub const UV_EAI_NODATA: C2RustUnnamed_8 = -3007;
pub const UV_EAI_MEMORY: C2RustUnnamed_8 = -3006;
pub const UV_EAI_FAMILY: C2RustUnnamed_8 = -3005;
pub const UV_EAI_FAIL: C2RustUnnamed_8 = -3004;
pub const UV_EAI_CANCELED: C2RustUnnamed_8 = -3003;
pub const UV_EAI_BADHINTS: C2RustUnnamed_8 = -3013;
pub const UV_EAI_BADFLAGS: C2RustUnnamed_8 = -3002;
pub const UV_EAI_AGAIN: C2RustUnnamed_8 = -3001;
pub const UV_EAI_ADDRFAMILY: C2RustUnnamed_8 = -3000;
pub const UV_EAGAIN: C2RustUnnamed_8 = -11;
pub const UV_EAFNOSUPPORT: C2RustUnnamed_8 = -97;
pub const UV_EADDRNOTAVAIL: C2RustUnnamed_8 = -99;
pub const UV_EADDRINUSE: C2RustUnnamed_8 = -98;
pub const UV_EACCES: C2RustUnnamed_8 = -13;
pub const UV_E2BIG: C2RustUnnamed_8 = -7;
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
pub type uv_alloc_cb = Option<unsafe extern "C" fn(*mut uv_handle_t, size_t, *mut uv_buf_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_udp_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_9,
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
pub union C2RustUnnamed_9 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
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
pub type uv_membership = ::core::ffi::c_uint;
pub const UV_JOIN_GROUP: uv_membership = 1;
pub const UV_LEAVE_GROUP: uv_membership = 0;
pub type uv_udp_flags = ::core::ffi::c_uint;
pub const UV_UDP_RECVMMSG: uv_udp_flags = 256;
pub const UV_UDP_LINUX_RECVERR: uv_udp_flags = 32;
pub const UV_UDP_MMSG_FREE: uv_udp_flags = 16;
pub const UV_UDP_MMSG_CHUNK: uv_udp_flags = 8;
pub const UV_UDP_REUSEADDR: uv_udp_flags = 4;
pub const UV_UDP_PARTIAL: uv_udp_flags = 2;
pub const UV_UDP_IPV6ONLY: uv_udp_flags = 1;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_10 = 33554432;
pub type uv__peersockfunc = Option<
    unsafe extern "C" fn(::core::ffi::c_int, *mut sockaddr, *mut socklen_t) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union uv__sockaddr {
    pub in6: sockaddr_in6,
    pub in_0: sockaddr_in,
    pub addr: sockaddr,
}
pub const UV_HANDLE_BOUND: C2RustUnnamed_10 = 8192;
pub const UV_HANDLE_IPV6: C2RustUnnamed_10 = 4194304;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_10 = 67108864;
pub type C2RustUnnamed_10 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_10 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_10 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_10 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_10 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_10 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_10 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_10 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_10 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_10 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_10 = 16777216;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_10 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_10 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_10 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_10 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_10 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_10 = 16777216;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_10 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_10 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_10 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_10 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_10 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_10 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_10 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_10 = 16384;
pub const UV_HANDLE_READING: C2RustUnnamed_10 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_10 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_10 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_10 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_10 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_10 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_10 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_10 = 16;
pub const UV_HANDLE_REF: C2RustUnnamed_10 = 8;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_10 = 4;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_10 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_10 = 1;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const ENOBUFS: ::core::ffi::c_int = 105 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
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
pub const IP_TTL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MCAST_JOIN_SOURCE_GROUP: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const MCAST_LEAVE_SOURCE_GROUP: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const IP_RECVERR: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const IP_MULTICAST_IF: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const IP_MULTICAST_TTL: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const IP_MULTICAST_LOOP: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const IP_ADD_MEMBERSHIP: ::core::ffi::c_int = 35 as ::core::ffi::c_int;
pub const IP_DROP_MEMBERSHIP: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::core::ffi::c_int = 39 as ::core::ffi::c_int;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const IPV6_UNICAST_HOPS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const IPV6_MULTICAST_IF: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const IPV6_MULTICAST_HOPS: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const IPV6_MULTICAST_LOOP: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const IPV6_JOIN_GROUP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const IPV6_LEAVE_GROUP: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const IPV6_RECVERR: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const IPV6_V6ONLY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const IPV6_ADD_MEMBERSHIP: ::core::ffi::c_int = IPV6_JOIN_GROUP;
pub const IPV6_DROP_MEMBERSHIP: ::core::ffi::c_int = IPV6_LEAVE_GROUP;
pub const INADDR_ANY: in_addr_t = 0 as ::core::ffi::c_int as in_addr_t;
#[inline]
unsafe extern "C" fn uv__queue_init(mut q: *mut uv__queue) {
    (*q).next = q;
    (*q).prev = q;
}
#[inline]
unsafe extern "C" fn uv__queue_empty(mut q: *const uv__queue) -> ::core::ffi::c_int {
    return (q == (*q).next as *const uv__queue) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn uv__queue_head(mut q: *const uv__queue) -> *mut uv__queue {
    return (*q).next;
}
#[inline]
unsafe extern "C" fn uv__queue_insert_tail(mut h: *mut uv__queue, mut q: *mut uv__queue) {
    (*q).next = h;
    (*q).prev = (*h).prev;
    (*(*q).prev).next = q;
    (*h).prev = q;
}
#[inline]
unsafe extern "C" fn uv__queue_remove(mut q: *mut uv__queue) {
    (*(*q).prev).next = (*q).next;
    (*(*q).next).prev = (*q).prev;
}
pub const UV__UDP_DGRAM_MAXSIZE: ::core::ffi::c_int =
    64 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn uv__udp_close(mut handle: *mut uv_udp_t) {
    uv__io_close((*handle).loop_0, &raw mut (*handle).io_watcher);
    if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint)
    {
        (*handle).flags &= !(UV_HANDLE_ACTIVE as ::core::ffi::c_int) as ::core::ffi::c_uint;
        if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            (*(*handle).loop_0).active_handles = (*(*handle).loop_0).active_handles.wrapping_sub(1);
        }
    }
    if (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int) {
        uv__close((*handle).io_watcher.fd);
        (*handle).io_watcher.fd = -(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_finish_close(mut handle: *mut uv_udp_t) {
    let mut req: *mut uv_udp_send_t = ::core::ptr::null_mut::<uv_udp_send_t>();
    let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
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
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"void uv__udp_finish_close(uv_udp_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*handle).io_watcher.fd == -(1 as ::core::ffi::c_int) {
        } else {
            __assert_fail(
                b"handle->io_watcher.fd == -1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                b"void uv__udp_finish_close(uv_udp_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while uv__queue_empty(&raw mut (*handle).write_queue) == 0 {
        q = uv__queue_head(&raw mut (*handle).write_queue);
        uv__queue_remove(q);
        req = (q as *mut ::core::ffi::c_char).offset(-(80 as ::core::ffi::c_ulong as isize))
            as *mut uv_udp_send_t;
        (*req).status = UV_ECANCELED as ::core::ffi::c_int as ssize_t;
        uv__queue_insert_tail(
            &raw mut (*handle).write_completed_queue,
            &raw mut (*req).queue,
        );
    }
    uv__udp_run_completed(handle);
    '_c2rust_label_1: {
        if (*handle).send_queue_size == 0 as size_t {
        } else {
            __assert_fail(
                b"handle->send_queue_size == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                81 as ::core::ffi::c_uint,
                b"void uv__udp_finish_close(uv_udp_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*handle).send_queue_count == 0 as size_t {
        } else {
            __assert_fail(
                b"handle->send_queue_count == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                82 as ::core::ffi::c_uint,
                b"void uv__udp_finish_close(uv_udp_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*handle).recv_cb = None;
    (*handle).alloc_cb = None;
}
unsafe extern "C" fn uv__udp_run_completed(mut handle: *mut uv_udp_t) {
    let mut req: *mut uv_udp_send_t = ::core::ptr::null_mut::<uv_udp_send_t>();
    let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
    '_c2rust_label: {
        if (*handle).flags & UV_HANDLE_UDP_PROCESSING as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
        {
        } else {
            __assert_fail(
                b"!(handle->flags & UV_HANDLE_UDP_PROCESSING)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                95 as ::core::ffi::c_uint,
                b"void uv__udp_run_completed(uv_udp_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*handle).flags |= UV_HANDLE_UDP_PROCESSING as ::core::ffi::c_int as ::core::ffi::c_uint;
    while uv__queue_empty(&raw mut (*handle).write_completed_queue) == 0 {
        q = uv__queue_head(&raw mut (*handle).write_completed_queue);
        uv__queue_remove(q);
        req = (q as *mut ::core::ffi::c_char).offset(-(80 as ::core::ffi::c_ulong as isize))
            as *mut uv_udp_send_t;
        '_c2rust_label_0: {
            if (*(*handle).loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"uv__has_active_reqs(handle->loop)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    103 as ::core::ffi::c_uint,
                    b"void uv__udp_run_completed(uv_udp_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*(*handle).loop_0).active_reqs.count =
            (*(*handle).loop_0).active_reqs.count.wrapping_sub(1);
        (*handle).send_queue_size = (*handle)
            .send_queue_size
            .wrapping_sub(uv__count_bufs((*req).bufs as *const uv_buf_t, (*req).nbufs));
        (*handle).send_queue_count = (*handle).send_queue_count.wrapping_sub(1);
        if (*req).bufs != &raw mut (*req).bufsml as *mut uv_buf_t {
            uv__free((*req).bufs as *mut ::core::ffi::c_void);
        }
        (*req).bufs = ::core::ptr::null_mut::<uv_buf_t>();
        if (*req).send_cb.is_none() {
            continue;
        }
        if (*req).status >= 0 as ssize_t {
            (*req).send_cb.expect("non-null function pointer")(req, 0 as ::core::ffi::c_int);
        } else {
            (*req).send_cb.expect("non-null function pointer")(
                req,
                (*req).status as ::core::ffi::c_int,
            );
        }
    }
    if uv__queue_empty(&raw mut (*handle).write_queue) != 0 {
        uv__io_stop(
            (*handle).loop_0,
            &raw mut (*handle).io_watcher,
            POLLOUT as ::core::ffi::c_uint,
        );
        if uv__io_active(&raw mut (*handle).io_watcher, POLLIN as ::core::ffi::c_uint) == 0 {
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
        }
    }
    (*handle).flags &= !(UV_HANDLE_UDP_PROCESSING as ::core::ffi::c_int) as ::core::ffi::c_uint;
}
unsafe extern "C" fn uv__udp_io(
    mut loop_0: *mut uv_loop_t,
    mut w: *mut uv__io_t,
    mut revents: ::core::ffi::c_uint,
) {
    let mut handle: *mut uv_udp_t = ::core::ptr::null_mut::<uv_udp_t>();
    handle = (w as *mut ::core::ffi::c_char).offset(-(128 as ::core::ffi::c_ulong as isize))
        as *mut uv_udp_t;
    '_c2rust_label: {
        if (*handle).type_0 as ::core::ffi::c_uint
            == UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"handle->type == UV_UDP\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                139 as ::core::ffi::c_uint,
                b"void uv__udp_io(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if revents & POLLIN as ::core::ffi::c_uint != 0 {
        uv__udp_recvmsg(handle);
    }
    if revents & POLLOUT as ::core::ffi::c_uint != 0 {
        uv__udp_sendmsg(handle);
        uv__udp_run_completed(handle);
    }
}
unsafe extern "C" fn uv__udp_recvmmsg(
    mut handle: *mut uv_udp_t,
    mut buf: *mut uv_buf_t,
) -> ::core::ffi::c_int {
    let mut peers: [sockaddr_in6; 20] = [sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed_0 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    }; 20];
    let mut iov: [iovec; 20] = [iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    }; 20];
    let mut msgs: [mmsghdr; 20] = [mmsghdr {
        msg_hdr: msghdr {
            msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            msg_namelen: 0,
            msg_iov: ::core::ptr::null_mut::<iovec>(),
            msg_iovlen: 0,
            msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            msg_controllen: 0,
            msg_flags: 0,
        },
        msg_len: 0,
    }; 20];
    let mut nread: ssize_t = 0;
    let mut chunk_buf: uv_buf_t = uv_buf_t {
        base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
    };
    let mut chunks: size_t = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut k: size_t = 0;
    chunks = (*buf).len.wrapping_div(UV__UDP_DGRAM_MAXSIZE as size_t);
    if chunks
        > (::core::mem::size_of::<[iovec; 20]>() as usize)
            .wrapping_div(::core::mem::size_of::<iovec>() as usize)
    {
        chunks = (::core::mem::size_of::<[iovec; 20]>() as usize)
            .wrapping_div(::core::mem::size_of::<iovec>() as usize) as size_t;
    }
    k = 0 as size_t;
    while k < chunks {
        iov[k as usize].iov_base = (*buf)
            .base
            .offset(k.wrapping_mul(UV__UDP_DGRAM_MAXSIZE as size_t) as isize)
            as *mut ::core::ffi::c_void;
        iov[k as usize].iov_len = UV__UDP_DGRAM_MAXSIZE as size_t;
        memset(
            &raw mut (*(&raw mut msgs as *mut mmsghdr).offset(k as isize)).msg_hdr
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<msghdr>() as size_t,
        );
        msgs[k as usize].msg_hdr.msg_iov = (&raw mut iov as *mut iovec).offset(k as isize);
        msgs[k as usize].msg_hdr.msg_iovlen = 1 as size_t;
        msgs[k as usize].msg_hdr.msg_name =
            (&raw mut peers as *mut sockaddr_in6).offset(k as isize) as *mut ::core::ffi::c_void;
        msgs[k as usize].msg_hdr.msg_namelen = ::core::mem::size_of::<sockaddr_in6>() as socklen_t;
        msgs[k as usize].msg_hdr.msg_control = NULL;
        msgs[k as usize].msg_hdr.msg_controllen = 0 as size_t;
        msgs[k as usize].msg_hdr.msg_flags = 0 as ::core::ffi::c_int;
        k = k.wrapping_add(1);
    }
    loop {
        nread = recvmmsg(
            (*handle).io_watcher.fd,
            &raw mut msgs as *mut mmsghdr,
            chunks as ::core::ffi::c_uint,
            0 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<timespec>(),
        ) as ssize_t;
        if !(nread == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if nread < 1 as ssize_t {
        if nread == 0 as ssize_t
            || *__errno_location() == EAGAIN
            || *__errno_location() == EWOULDBLOCK
        {
            (*handle).recv_cb.expect("non-null function pointer")(
                handle,
                0 as ssize_t,
                buf,
                ::core::ptr::null::<sockaddr>(),
                0 as ::core::ffi::c_uint,
            );
        } else {
            (*handle).recv_cb.expect("non-null function pointer")(
                handle,
                -*__errno_location() as ssize_t,
                buf,
                ::core::ptr::null::<sockaddr>(),
                0 as ::core::ffi::c_uint,
            );
        }
    } else {
        k = 0 as size_t;
        while k < nread as size_t && (*handle).recv_cb.is_some() {
            flags = UV_UDP_MMSG_CHUNK as ::core::ffi::c_int;
            if msgs[k as usize].msg_hdr.msg_flags & MSG_TRUNC as ::core::ffi::c_int != 0 {
                flags |= UV_UDP_PARTIAL as ::core::ffi::c_int;
            }
            chunk_buf = uv_buf_init(
                iov[k as usize].iov_base as *mut ::core::ffi::c_char,
                iov[k as usize].iov_len as ::core::ffi::c_uint,
            );
            (*handle).recv_cb.expect("non-null function pointer")(
                handle,
                msgs[k as usize].msg_len as ssize_t,
                &raw mut chunk_buf,
                msgs[k as usize].msg_hdr.msg_name as *const sockaddr,
                flags as ::core::ffi::c_uint,
            );
            k = k.wrapping_add(1);
        }
        if (*handle).recv_cb.is_some() {
            (*handle).recv_cb.expect("non-null function pointer")(
                handle,
                0 as ssize_t,
                buf,
                ::core::ptr::null::<sockaddr>(),
                UV_UDP_MMSG_FREE as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
    }
    return nread as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__udp_recvmsg(mut handle: *mut uv_udp_t) {
    let mut peer: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut h: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut nread: ssize_t = 0;
    let mut buf: uv_buf_t = uv_buf_t {
        base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
    };
    let mut flags: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*handle).recv_cb.is_some() {
        } else {
            __assert_fail(
                b"handle->recv_cb != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                220 as ::core::ffi::c_uint,
                b"void uv__udp_recvmsg(uv_udp_t *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*handle).alloc_cb.is_some() {
        } else {
            __assert_fail(
                b"handle->alloc_cb != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                221 as ::core::ffi::c_uint,
                b"void uv__udp_recvmsg(uv_udp_t *)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    count = 32 as ::core::ffi::c_int;
    loop {
        buf = uv_buf_init(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_uint,
        );
        (*handle).alloc_cb.expect("non-null function pointer")(
            handle as *mut uv_handle_t,
            UV__UDP_DGRAM_MAXSIZE as size_t,
            &raw mut buf,
        );
        if buf.base.is_null() || buf.len == 0 as size_t {
            (*handle).recv_cb.expect("non-null function pointer")(
                handle,
                UV_ENOBUFS as ::core::ffi::c_int as ssize_t,
                &raw mut buf,
                ::core::ptr::null::<sockaddr>(),
                0 as ::core::ffi::c_uint,
            );
            return;
        }
        '_c2rust_label_1: {
            if !buf.base.is_null() {
            } else {
                __assert_fail(
                    b"buf.base != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    235 as ::core::ffi::c_uint,
                    b"void uv__udp_recvmsg(uv_udp_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        if uv_udp_using_recvmmsg(handle) != 0 {
            nread = uv__udp_recvmmsg(handle, &raw mut buf) as ssize_t;
            if nread > 0 as ssize_t {
                count = (count as ssize_t - nread) as ::core::ffi::c_int;
            }
        } else {
            memset(
                &raw mut h as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<msghdr>() as size_t,
            );
            memset(
                &raw mut peer as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_storage>() as size_t,
            );
            h.msg_name = &raw mut peer as *mut ::core::ffi::c_void;
            h.msg_namelen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
            h.msg_iov = &raw mut buf as *mut ::core::ffi::c_void as *mut iovec;
            h.msg_iovlen = 1 as size_t;
            loop {
                nread = recvmsg((*handle).io_watcher.fd, &raw mut h, 0 as ::core::ffi::c_int);
                if !(nread == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR)
                {
                    break;
                }
            }
            if nread == -(1 as ::core::ffi::c_int) as ssize_t {
                if *__errno_location() == EAGAIN || *__errno_location() == EWOULDBLOCK {
                    (*handle).recv_cb.expect("non-null function pointer")(
                        handle,
                        0 as ssize_t,
                        &raw mut buf,
                        ::core::ptr::null::<sockaddr>(),
                        0 as ::core::ffi::c_uint,
                    );
                } else {
                    (*handle).recv_cb.expect("non-null function pointer")(
                        handle,
                        -*__errno_location() as ssize_t,
                        &raw mut buf,
                        ::core::ptr::null::<sockaddr>(),
                        0 as ::core::ffi::c_uint,
                    );
                }
            } else {
                flags = 0 as ::core::ffi::c_int;
                if h.msg_flags & MSG_TRUNC as ::core::ffi::c_int != 0 {
                    flags |= UV_UDP_PARTIAL as ::core::ffi::c_int;
                }
                (*handle).recv_cb.expect("non-null function pointer")(
                    handle,
                    nread,
                    &raw mut buf,
                    &raw mut peer as *const sockaddr,
                    flags as ::core::ffi::c_uint,
                );
            }
            count -= 1;
        }
        if !(nread != -(1 as ::core::ffi::c_int) as ssize_t
            && count > 0 as ::core::ffi::c_int
            && (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int)
            && (*handle).recv_cb.is_some())
        {
            break;
        }
    }
}
unsafe extern "C" fn uv__udp_sendmsg(mut handle: *mut uv_udp_t) {
    let mut req: *mut uv_udp_send_t = ::core::ptr::null_mut::<uv_udp_send_t>();
    let mut h: [mmsghdr; 20] = [mmsghdr {
        msg_hdr: msghdr {
            msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            msg_namelen: 0,
            msg_iov: ::core::ptr::null_mut::<iovec>(),
            msg_iovlen: 0,
            msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            msg_controllen: 0,
            msg_flags: 0,
        },
        msg_len: 0,
    }; 20];
    let mut p: *mut mmsghdr = ::core::ptr::null_mut::<mmsghdr>();
    let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
    let mut npkts: ssize_t = 0;
    let mut pkts: size_t = 0;
    let mut i: size_t = 0;
    if uv__queue_empty(&raw mut (*handle).write_queue) != 0 {
        return;
    }
    loop {
        pkts = 0 as size_t;
        q = uv__queue_head(&raw mut (*handle).write_queue);
        while pkts
            < (::core::mem::size_of::<[mmsghdr; 20]>() as usize)
                .wrapping_div(::core::mem::size_of::<mmsghdr>() as usize)
            && q != &raw mut (*handle).write_queue
        {
            '_c2rust_label: {
                if !q.is_null() {
                } else {
                    __assert_fail(
                        b"q != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        295 as ::core::ffi::c_uint,
                        b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            req = (q as *mut ::core::ffi::c_char).offset(-(80 as ::core::ffi::c_ulong as isize))
                as *mut uv_udp_send_t;
            '_c2rust_label_0: {
                if !req.is_null() {
                } else {
                    __assert_fail(
                        b"req != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        297 as ::core::ffi::c_uint,
                        b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            p = (&raw mut h as *mut mmsghdr).offset(pkts as isize) as *mut mmsghdr;
            memset(
                p as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<mmsghdr>() as size_t,
            );
            if (*req).addr.ss_family as ::core::ffi::c_int == AF_UNSPEC {
                (*p).msg_hdr.msg_name = NULL;
                (*p).msg_hdr.msg_namelen = 0 as socklen_t;
            } else {
                (*p).msg_hdr.msg_name = &raw mut (*req).addr as *mut ::core::ffi::c_void;
                if (*req).addr.ss_family as ::core::ffi::c_int == AF_INET6 {
                    (*p).msg_hdr.msg_namelen = ::core::mem::size_of::<sockaddr_in6>() as socklen_t;
                } else if (*req).addr.ss_family as ::core::ffi::c_int == AF_INET {
                    (*p).msg_hdr.msg_namelen = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
                } else if (*req).addr.ss_family as ::core::ffi::c_int == AF_UNIX {
                    (*p).msg_hdr.msg_namelen = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
                } else {
                    '_c2rust_label_1: {
                        if 0 as ::core::ffi::c_int != 0
                            && !(b"unsupported address family\0" as *const u8
                                as *const ::core::ffi::c_char)
                                .is_null()
                        {
                        } else {
                            __assert_fail(
                                b"0 && \"unsupported address family\"\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0"
                                    as *const u8
                                    as *const ::core::ffi::c_char,
                                313 as ::core::ffi::c_uint,
                                b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    };
                    abort();
                }
            }
            h[pkts as usize].msg_hdr.msg_iov = (*req).bufs as *mut iovec;
            h[pkts as usize].msg_hdr.msg_iovlen = (*req).nbufs as size_t;
            pkts = pkts.wrapping_add(1);
            q = uv__queue_head(q);
        }
        loop {
            npkts = sendmmsg(
                (*handle).io_watcher.fd,
                &raw mut h as *mut mmsghdr,
                pkts as ::core::ffi::c_uint,
                0 as ::core::ffi::c_int,
            ) as ssize_t;
            if !(npkts == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
        if npkts < 1 as ssize_t {
            if *__errno_location() == EAGAIN
                || *__errno_location() == EWOULDBLOCK
                || *__errno_location() == ENOBUFS
            {
                return;
            }
            i = 0 as size_t;
            q = uv__queue_head(&raw mut (*handle).write_queue);
            while i < pkts && q != &raw mut (*handle).write_queue {
                '_c2rust_label_2: {
                    if !q.is_null() {
                    } else {
                        __assert_fail(
                            b"q != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            331 as ::core::ffi::c_uint,
                            b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                req = (q as *mut ::core::ffi::c_char).offset(-(80 as ::core::ffi::c_ulong as isize))
                    as *mut uv_udp_send_t;
                '_c2rust_label_3: {
                    if !req.is_null() {
                    } else {
                        __assert_fail(
                            b"req != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            333 as ::core::ffi::c_uint,
                            b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                (*req).status = -*__errno_location() as ssize_t;
                uv__queue_remove(&raw mut (*req).queue);
                uv__queue_insert_tail(
                    &raw mut (*handle).write_completed_queue,
                    &raw mut (*req).queue,
                );
                i = i.wrapping_add(1);
                q = uv__queue_head(&raw mut (*handle).write_queue);
            }
            uv__io_feed((*handle).loop_0, &raw mut (*handle).io_watcher);
            return;
        }
        i = 0 as size_t;
        q = uv__queue_head(&raw mut (*handle).write_queue);
        while i < npkts as size_t && q != &raw mut (*handle).write_queue {
            '_c2rust_label_4: {
                if !q.is_null() {
                } else {
                    __assert_fail(
                        b"q != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        349 as ::core::ffi::c_uint,
                        b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            req = (q as *mut ::core::ffi::c_char).offset(-(80 as ::core::ffi::c_ulong as isize))
                as *mut uv_udp_send_t;
            '_c2rust_label_5: {
                if !req.is_null() {
                } else {
                    __assert_fail(
                        b"req != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        351 as ::core::ffi::c_uint,
                        b"void uv__udp_sendmsg(uv_udp_t *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*req).status = (*(*req).bufs.offset(0 as ::core::ffi::c_int as isize)).len as ssize_t;
            uv__queue_remove(&raw mut (*req).queue);
            uv__queue_insert_tail(
                &raw mut (*handle).write_completed_queue,
                &raw mut (*req).queue,
            );
            i = i.wrapping_add(1);
            q = uv__queue_head(&raw mut (*handle).write_queue);
        }
        if !(uv__queue_empty(&raw mut (*handle).write_queue) == 0) {
            break;
        }
    }
    uv__io_feed((*handle).loop_0, &raw mut (*handle).io_watcher);
}
unsafe extern "C" fn uv__set_reuse(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut yes: ::core::ffi::c_int = 0;
    yes = 1 as ::core::ffi::c_int;
    if setsockopt(
        fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &raw mut yes as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__set_recverr(
    mut fd: ::core::ffi::c_int,
    mut ss_family: sa_family_t,
) -> ::core::ffi::c_int {
    let mut yes: ::core::ffi::c_int = 0;
    yes = 1 as ::core::ffi::c_int;
    if ss_family as ::core::ffi::c_int == AF_INET {
        if setsockopt(
            fd,
            IPPROTO_IP as ::core::ffi::c_int,
            IP_RECVERR,
            &raw mut yes as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) != 0
        {
            return -*__errno_location();
        }
    } else if ss_family as ::core::ffi::c_int == AF_INET6 {
        if setsockopt(
            fd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_RECVERR,
            &raw mut yes as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) != 0
        {
            return -*__errno_location();
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_bind(
    mut handle: *mut uv_udp_t,
    mut addr: *const sockaddr,
    mut addrlen: ::core::ffi::c_uint,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut yes: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    if flags
        & !(UV_UDP_IPV6ONLY as ::core::ffi::c_int
            | UV_UDP_REUSEADDR as ::core::ffi::c_int
            | UV_UDP_LINUX_RECVERR as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if flags & UV_UDP_IPV6ONLY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && (*addr).sa_family as ::core::ffi::c_int != AF_INET6
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    fd = (*handle).io_watcher.fd;
    if fd == -(1 as ::core::ffi::c_int) {
        err = uv__socket(
            (*addr).sa_family as ::core::ffi::c_int,
            SOCK_DGRAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if err < 0 as ::core::ffi::c_int {
            return err;
        }
        fd = err;
        (*handle).io_watcher.fd = fd;
    }
    if flags & UV_UDP_LINUX_RECVERR as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        err = uv__set_recverr(fd, (*addr).sa_family);
        if err != 0 {
            return err;
        }
    }
    if flags & UV_UDP_REUSEADDR as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        err = uv__set_reuse(fd);
        if err != 0 {
            return err;
        }
    }
    if flags & UV_UDP_IPV6ONLY as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        yes = 1 as ::core::ffi::c_int;
        if setsockopt(
            fd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_V6ONLY,
            &raw mut yes as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            err = -*__errno_location();
            return err;
        }
    }
    if bind(
        fd,
        __CONST_SOCKADDR_ARG { __sockaddr__: addr },
        addrlen as socklen_t,
    ) != 0
    {
        err = -*__errno_location();
        if *__errno_location() == EAFNOSUPPORT {
            err = UV_EINVAL as ::core::ffi::c_int;
        }
        return err;
    }
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        (*handle).flags |= UV_HANDLE_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    (*handle).flags |= UV_HANDLE_BOUND as ::core::ffi::c_int as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__udp_maybe_deferred_bind(
    mut handle: *mut uv_udp_t,
    mut domain: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut taddr: uv__sockaddr = uv__sockaddr {
        in6: sockaddr_in6 {
            sin6_family: 0,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: in6_addr {
                __in6_u: C2RustUnnamed_0 {
                    __u6_addr8: [0; 16],
                },
            },
            sin6_scope_id: 0,
        },
    };
    let mut addrlen: socklen_t = 0;
    if (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int) {
        return 0 as ::core::ffi::c_int;
    }
    match domain {
        AF_INET => {
            let mut addr: *mut sockaddr_in = &raw mut taddr.in_0;
            memset(
                addr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_in>() as size_t,
            );
            (*addr).sin_family = AF_INET as sa_family_t;
            (*addr).sin_addr.s_addr = INADDR_ANY;
            addrlen = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
        }
        AF_INET6 => {
            let mut addr_0: *mut sockaddr_in6 = &raw mut taddr.in6;
            memset(
                addr_0 as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_in6>() as size_t,
            );
            (*addr_0).sin6_family = AF_INET6 as sa_family_t;
            (*addr_0).sin6_addr = in6addr_any;
            addrlen = ::core::mem::size_of::<sockaddr_in6>() as socklen_t;
        }
        _ => {
            '_c2rust_label: {
                if 0 as ::core::ffi::c_int != 0
                    && !(b"unsupported address family\0" as *const u8 as *const ::core::ffi::c_char)
                        .is_null()
                {
                } else {
                    __assert_fail(
                        b"0 && \"unsupported address family\"\0" as *const u8
                            as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        582 as ::core::ffi::c_uint,
                        b"int uv__udp_maybe_deferred_bind(uv_udp_t *, int, unsigned int)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            };
            abort();
        }
    }
    return uv__udp_bind(
        handle,
        &raw mut taddr.addr,
        addrlen as ::core::ffi::c_uint,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_connect(
    mut handle: *mut uv_udp_t,
    mut addr: *const sockaddr,
    mut addrlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    err = uv__udp_maybe_deferred_bind(
        handle,
        (*addr).sa_family as ::core::ffi::c_int,
        0 as ::core::ffi::c_uint,
    );
    if err != 0 {
        return err;
    }
    loop {
        *__errno_location() = 0 as ::core::ffi::c_int;
        err = connect(
            (*handle).io_watcher.fd,
            __CONST_SOCKADDR_ARG { __sockaddr__: addr },
            addrlen as socklen_t,
        );
        if !(err == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if err != 0 {
        return -*__errno_location();
    }
    (*handle).flags |= UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_disconnect(mut handle: *mut uv_udp_t) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut addr: sockaddr = sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    };
    memset(
        &raw mut addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr>() as size_t,
    );
    addr.sa_family = AF_UNSPEC as sa_family_t;
    loop {
        *__errno_location() = 0 as ::core::ffi::c_int;
        r = connect(
            (*handle).io_watcher.fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut addr,
            },
            ::core::mem::size_of::<sockaddr>() as socklen_t,
        );
        if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if r == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    (*handle).flags &= !(UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int) as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_send(
    mut req: *mut uv_udp_send_t,
    mut handle: *mut uv_udp_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut addr: *const sockaddr,
    mut addrlen: ::core::ffi::c_uint,
    mut send_cb: uv_udp_send_cb,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut empty_queue: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if nbufs > 0 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"nbufs > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                696 as ::core::ffi::c_uint,
                b"int uv__udp_send(uv_udp_send_t *, uv_udp_t *, const uv_buf_t *, unsigned int, const struct sockaddr *, unsigned int, uv_udp_send_cb)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if !addr.is_null() {
        err = uv__udp_maybe_deferred_bind(
            handle,
            (*addr).sa_family as ::core::ffi::c_int,
            0 as ::core::ffi::c_uint,
        );
        if err != 0 {
            return err;
        }
    }
    empty_queue = ((*handle).send_queue_count == 0 as size_t) as ::core::ffi::c_int;
    (*req).type_0 = UV_UDP_SEND;
    (*(*handle).loop_0).active_reqs.count = (*(*handle).loop_0).active_reqs.count.wrapping_add(1);
    '_c2rust_label_0: {
        if addrlen as usize <= ::core::mem::size_of::<sockaddr_storage>() as usize {
        } else {
            __assert_fail(
                b"addrlen <= sizeof(req->addr)\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                711 as ::core::ffi::c_uint,
                b"int uv__udp_send(uv_udp_send_t *, uv_udp_t *, const uv_buf_t *, unsigned int, const struct sockaddr *, unsigned int, uv_udp_send_cb)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if addr.is_null() {
        (*req).addr.ss_family = AF_UNSPEC as sa_family_t;
    } else {
        memcpy(
            &raw mut (*req).addr as *mut ::core::ffi::c_void,
            addr as *const ::core::ffi::c_void,
            addrlen as size_t,
        );
    }
    (*req).send_cb = send_cb;
    (*req).handle = handle;
    (*req).nbufs = nbufs;
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
        '_c2rust_label_1: {
            if (*(*handle).loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
            } else {
                __assert_fail(
                    b"uv__has_active_reqs(handle->loop)\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    725 as ::core::ffi::c_uint,
                    b"int uv__udp_send(uv_udp_send_t *, uv_udp_t *, const uv_buf_t *, unsigned int, const struct sockaddr *, unsigned int, uv_udp_send_cb)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        (*(*handle).loop_0).active_reqs.count =
            (*(*handle).loop_0).active_reqs.count.wrapping_sub(1);
        return UV_ENOMEM as ::core::ffi::c_int;
    }
    memcpy(
        (*req).bufs as *mut ::core::ffi::c_void,
        bufs as *const ::core::ffi::c_void,
        (nbufs as size_t).wrapping_mul(::core::mem::size_of::<uv_buf_t>() as size_t),
    );
    (*handle).send_queue_size = (*handle)
        .send_queue_size
        .wrapping_add(uv__count_bufs((*req).bufs as *const uv_buf_t, (*req).nbufs));
    (*handle).send_queue_count = (*handle).send_queue_count.wrapping_add(1);
    uv__queue_insert_tail(&raw mut (*handle).write_queue, &raw mut (*req).queue);
    if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint)
    {
        (*handle).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
        if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            (*(*handle).loop_0).active_handles = (*(*handle).loop_0).active_handles.wrapping_add(1);
        }
    }
    if empty_queue != 0
        && (*handle).flags & UV_HANDLE_UDP_PROCESSING as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
    {
        uv__udp_sendmsg(handle);
        if uv__queue_empty(&raw mut (*handle).write_queue) == 0 {
            uv__io_start(
                (*handle).loop_0,
                &raw mut (*handle).io_watcher,
                POLLOUT as ::core::ffi::c_uint,
            );
        }
    } else {
        uv__io_start(
            (*handle).loop_0,
            &raw mut (*handle).io_watcher,
            POLLOUT as ::core::ffi::c_uint,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_try_send(
    mut handle: *mut uv_udp_t,
    mut bufs: *const uv_buf_t,
    mut nbufs: ::core::ffi::c_uint,
    mut addr: *const sockaddr,
    mut addrlen: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut h: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut size: ssize_t = 0;
    '_c2rust_label: {
        if nbufs > 0 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"nbufs > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                761 as ::core::ffi::c_uint,
                b"int uv__udp_try_send(uv_udp_t *, const uv_buf_t *, unsigned int, const struct sockaddr *, unsigned int)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    if (*handle).send_queue_count != 0 as size_t {
        return UV_EAGAIN as ::core::ffi::c_int;
    }
    if !addr.is_null() {
        err = uv__udp_maybe_deferred_bind(
            handle,
            (*addr).sa_family as ::core::ffi::c_int,
            0 as ::core::ffi::c_uint,
        );
        if err != 0 {
            return err;
        }
    } else {
        '_c2rust_label_0: {
            if (*handle).flags
                & UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
            } else {
                __assert_fail(
                    b"handle->flags & UV_HANDLE_UDP_CONNECTED\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    772 as ::core::ffi::c_uint,
                    b"int uv__udp_try_send(uv_udp_t *, const uv_buf_t *, unsigned int, const struct sockaddr *, unsigned int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
    }
    memset(
        &raw mut h as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<msghdr>() as size_t,
    );
    h.msg_name = addr as *mut sockaddr as *mut ::core::ffi::c_void;
    h.msg_namelen = addrlen as socklen_t;
    h.msg_iov = bufs as *mut iovec;
    h.msg_iovlen = nbufs as size_t;
    loop {
        size = sendmsg((*handle).io_watcher.fd, &raw mut h, 0 as ::core::ffi::c_int);
        if !(size == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if size == -(1 as ::core::ffi::c_int) as ssize_t {
        if *__errno_location() == EAGAIN
            || *__errno_location() == EWOULDBLOCK
            || *__errno_location() == ENOBUFS
        {
            return UV_EAGAIN as ::core::ffi::c_int;
        } else {
            return -*__errno_location();
        }
    }
    return size as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__udp_set_membership4(
    mut handle: *mut uv_udp_t,
    mut multicast_addr: *const sockaddr_in,
    mut interface_addr: *const ::core::ffi::c_char,
    mut membership: uv_membership,
) -> ::core::ffi::c_int {
    let mut mreq: ip_mreq = ip_mreq {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_interface: in_addr { s_addr: 0 },
    };
    let mut optname: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    memset(
        &raw mut mreq as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ip_mreq>() as size_t,
    );
    if !interface_addr.is_null() {
        err = uv_inet_pton(
            AF_INET,
            interface_addr,
            &raw mut mreq.imr_interface.s_addr as *mut ::core::ffi::c_void,
        );
        if err != 0 {
            return err;
        }
    } else {
        mreq.imr_interface.s_addr = htonl(INADDR_ANY) as in_addr_t;
    }
    mreq.imr_multiaddr.s_addr = (*multicast_addr).sin_addr.s_addr;
    match membership as ::core::ffi::c_uint {
        1 => {
            optname = IP_ADD_MEMBERSHIP;
        }
        0 => {
            optname = IP_DROP_MEMBERSHIP;
        }
        _ => return UV_EINVAL as ::core::ffi::c_int,
    }
    if setsockopt(
        (*handle).io_watcher.fd,
        IPPROTO_IP as ::core::ffi::c_int,
        optname,
        &raw mut mreq as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ip_mreq>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__udp_set_membership6(
    mut handle: *mut uv_udp_t,
    mut multicast_addr: *const sockaddr_in6,
    mut interface_addr: *const ::core::ffi::c_char,
    mut membership: uv_membership,
) -> ::core::ffi::c_int {
    let mut optname: ::core::ffi::c_int = 0;
    let mut mreq: ipv6_mreq = ipv6_mreq {
        ipv6mr_multiaddr: in6_addr {
            __in6_u: C2RustUnnamed_0 {
                __u6_addr8: [0; 16],
            },
        },
        ipv6mr_interface: 0,
    };
    let mut addr6: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed_0 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    memset(
        &raw mut mreq as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ipv6_mreq>() as size_t,
    );
    if !interface_addr.is_null() {
        if uv_ip6_addr(interface_addr, 0 as ::core::ffi::c_int, &raw mut addr6) != 0 {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        mreq.ipv6mr_interface = addr6.sin6_scope_id as ::core::ffi::c_uint;
    } else {
        mreq.ipv6mr_interface = 0 as ::core::ffi::c_uint;
    }
    mreq.ipv6mr_multiaddr = (*multicast_addr).sin6_addr;
    match membership as ::core::ffi::c_uint {
        1 => {
            optname = IPV6_ADD_MEMBERSHIP;
        }
        0 => {
            optname = IPV6_DROP_MEMBERSHIP;
        }
        _ => return UV_EINVAL as ::core::ffi::c_int,
    }
    if setsockopt(
        (*handle).io_watcher.fd,
        IPPROTO_IPV6 as ::core::ffi::c_int,
        optname,
        &raw mut mreq as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ipv6_mreq>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__udp_set_source_membership4(
    mut handle: *mut uv_udp_t,
    mut multicast_addr: *const sockaddr_in,
    mut interface_addr: *const ::core::ffi::c_char,
    mut source_addr: *const sockaddr_in,
    mut membership: uv_membership,
) -> ::core::ffi::c_int {
    let mut mreq: ip_mreq_source = ip_mreq_source {
        imr_multiaddr: in_addr { s_addr: 0 },
        imr_interface: in_addr { s_addr: 0 },
        imr_sourceaddr: in_addr { s_addr: 0 },
    };
    let mut optname: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    err = uv__udp_maybe_deferred_bind(
        handle,
        AF_INET,
        UV_UDP_REUSEADDR as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if err != 0 {
        return err;
    }
    memset(
        &raw mut mreq as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ip_mreq_source>() as size_t,
    );
    if !interface_addr.is_null() {
        err = uv_inet_pton(
            AF_INET,
            interface_addr,
            &raw mut mreq.imr_interface.s_addr as *mut ::core::ffi::c_void,
        );
        if err != 0 {
            return err;
        }
    } else {
        mreq.imr_interface.s_addr = htonl(INADDR_ANY) as in_addr_t;
    }
    mreq.imr_multiaddr.s_addr = (*multicast_addr).sin_addr.s_addr;
    mreq.imr_sourceaddr.s_addr = (*source_addr).sin_addr.s_addr;
    if membership as ::core::ffi::c_uint
        == UV_JOIN_GROUP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        optname = IP_ADD_SOURCE_MEMBERSHIP;
    } else if membership as ::core::ffi::c_uint
        == UV_LEAVE_GROUP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        optname = IP_DROP_SOURCE_MEMBERSHIP;
    } else {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if setsockopt(
        (*handle).io_watcher.fd,
        IPPROTO_IP as ::core::ffi::c_int,
        optname,
        &raw mut mreq as *const ::core::ffi::c_void,
        ::core::mem::size_of::<ip_mreq_source>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__udp_set_source_membership6(
    mut handle: *mut uv_udp_t,
    mut multicast_addr: *const sockaddr_in6,
    mut interface_addr: *const ::core::ffi::c_char,
    mut source_addr: *const sockaddr_in6,
    mut membership: uv_membership,
) -> ::core::ffi::c_int {
    let mut mreq: group_source_req = group_source_req {
        gsr_interface: 0,
        gsr_group: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
        gsr_source: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
    };
    let mut addr6: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed_0 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    let mut optname: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    err = uv__udp_maybe_deferred_bind(
        handle,
        AF_INET6,
        UV_UDP_REUSEADDR as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    if err != 0 {
        return err;
    }
    memset(
        &raw mut mreq as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<group_source_req>() as size_t,
    );
    if !interface_addr.is_null() {
        err = uv_ip6_addr(interface_addr, 0 as ::core::ffi::c_int, &raw mut addr6);
        if err != 0 {
            return err;
        }
        mreq.gsr_interface = addr6.sin6_scope_id;
    } else {
        mreq.gsr_interface = 0 as uint32_t;
    }
    memcpy(
        &raw mut mreq.gsr_group as *mut ::core::ffi::c_void,
        multicast_addr as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sockaddr_in6>() as size_t,
    );
    memcpy(
        &raw mut mreq.gsr_source as *mut ::core::ffi::c_void,
        source_addr as *const ::core::ffi::c_void,
        ::core::mem::size_of::<sockaddr_in6>() as size_t,
    );
    if membership as ::core::ffi::c_uint
        == UV_JOIN_GROUP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        optname = MCAST_JOIN_SOURCE_GROUP;
    } else if membership as ::core::ffi::c_uint
        == UV_LEAVE_GROUP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        optname = MCAST_LEAVE_SOURCE_GROUP;
    } else {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if setsockopt(
        (*handle).io_watcher.fd,
        IPPROTO_IPV6 as ::core::ffi::c_int,
        optname,
        &raw mut mreq as *const ::core::ffi::c_void,
        ::core::mem::size_of::<group_source_req>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_init_ex(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_udp_t,
    mut flags: ::core::ffi::c_uint,
    mut domain: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    fd = -(1 as ::core::ffi::c_int);
    if domain != AF_UNSPEC {
        fd = uv__socket(
            domain,
            SOCK_DGRAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if fd < 0 as ::core::ffi::c_int {
            return fd;
        }
    }
    let ref mut fresh0 = (*(handle as *mut uv_handle_t)).loop_0;
    *fresh0 = loop_0;
    (*(handle as *mut uv_handle_t)).type_0 = UV_UDP;
    (*(handle as *mut uv_handle_t)).flags =
        UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
    uv__queue_insert_tail(
        &raw mut (*loop_0).handle_queue,
        &raw mut (*(handle as *mut uv_handle_t)).handle_queue,
    );
    let ref mut fresh1 = (*(handle as *mut uv_handle_t)).next_closing;
    *fresh1 = ::core::ptr::null_mut::<uv_handle_t>();
    (*handle).alloc_cb = None;
    (*handle).recv_cb = None;
    (*handle).send_queue_size = 0 as size_t;
    (*handle).send_queue_count = 0 as size_t;
    uv__io_init(
        &raw mut (*handle).io_watcher,
        Some(
            uv__udp_io
                as unsafe extern "C" fn(*mut uv_loop_t, *mut uv__io_t, ::core::ffi::c_uint) -> (),
        ),
        fd,
    );
    uv__queue_init(&raw mut (*handle).write_queue);
    uv__queue_init(&raw mut (*handle).write_completed_queue);
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_udp_using_recvmmsg(mut handle: *const uv_udp_t) -> ::core::ffi::c_int {
    if (*handle).flags & UV_HANDLE_UDP_RECVMMSG as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_udp_open(
    mut handle: *mut uv_udp_t,
    mut sock: uv_os_sock_t,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    if (*handle).io_watcher.fd != -(1 as ::core::ffi::c_int) {
        return UV_EBUSY as ::core::ffi::c_int;
    }
    if uv__fd_exists((*handle).loop_0, sock as ::core::ffi::c_int) != 0 {
        return UV_EEXIST as ::core::ffi::c_int;
    }
    err = uv__nonblock_ioctl(sock as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    if err != 0 {
        return err;
    }
    err = uv__set_reuse(sock as ::core::ffi::c_int);
    if err != 0 {
        return err;
    }
    (*handle).io_watcher.fd = sock as ::core::ffi::c_int;
    if uv__udp_is_connected(handle) != 0 {
        (*handle).flags |= UV_HANDLE_UDP_CONNECTED as ::core::ffi::c_int as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_udp_set_membership(
    mut handle: *mut uv_udp_t,
    mut multicast_addr: *const ::core::ffi::c_char,
    mut interface_addr: *const ::core::ffi::c_char,
    mut membership: uv_membership,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut addr4: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut addr6: sockaddr_in6 = sockaddr_in6 {
        sin6_family: 0,
        sin6_port: 0,
        sin6_flowinfo: 0,
        sin6_addr: in6_addr {
            __in6_u: C2RustUnnamed_0 {
                __u6_addr8: [0; 16],
            },
        },
        sin6_scope_id: 0,
    };
    if uv_ip4_addr(multicast_addr, 0 as ::core::ffi::c_int, &raw mut addr4)
        == 0 as ::core::ffi::c_int
    {
        err = uv__udp_maybe_deferred_bind(
            handle,
            AF_INET,
            UV_UDP_REUSEADDR as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if err != 0 {
            return err;
        }
        return uv__udp_set_membership4(handle, &raw mut addr4, interface_addr, membership);
    } else if uv_ip6_addr(multicast_addr, 0 as ::core::ffi::c_int, &raw mut addr6)
        == 0 as ::core::ffi::c_int
    {
        err = uv__udp_maybe_deferred_bind(
            handle,
            AF_INET6,
            UV_UDP_REUSEADDR as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        if err != 0 {
            return err;
        }
        return uv__udp_set_membership6(handle, &raw mut addr6, interface_addr, membership);
    } else {
        return UV_EINVAL as ::core::ffi::c_int;
    };
}
pub(crate) unsafe fn uv_udp_set_source_membership(
    mut handle: *mut uv_udp_t,
    mut multicast_addr: *const ::core::ffi::c_char,
    mut interface_addr: *const ::core::ffi::c_char,
    mut source_addr: *const ::core::ffi::c_char,
    mut membership: uv_membership,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut mcast_addr: uv__sockaddr = uv__sockaddr {
        in6: sockaddr_in6 {
            sin6_family: 0,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: in6_addr {
                __in6_u: C2RustUnnamed_0 {
                    __u6_addr8: [0; 16],
                },
            },
            sin6_scope_id: 0,
        },
    };
    let mut src_addr: uv__sockaddr = uv__sockaddr {
        in6: sockaddr_in6 {
            sin6_family: 0,
            sin6_port: 0,
            sin6_flowinfo: 0,
            sin6_addr: in6_addr {
                __in6_u: C2RustUnnamed_0 {
                    __u6_addr8: [0; 16],
                },
            },
            sin6_scope_id: 0,
        },
    };
    err = uv_ip4_addr(
        multicast_addr,
        0 as ::core::ffi::c_int,
        &raw mut mcast_addr.in_0,
    );
    if err != 0 {
        err = uv_ip6_addr(
            multicast_addr,
            0 as ::core::ffi::c_int,
            &raw mut mcast_addr.in6,
        );
        if err != 0 {
            return err;
        }
        err = uv_ip6_addr(source_addr, 0 as ::core::ffi::c_int, &raw mut src_addr.in6);
        if err != 0 {
            return err;
        }
        return uv__udp_set_source_membership6(
            handle,
            &raw mut mcast_addr.in6,
            interface_addr,
            &raw mut src_addr.in6,
            membership,
        );
    }
    err = uv_ip4_addr(source_addr, 0 as ::core::ffi::c_int, &raw mut src_addr.in_0);
    if err != 0 {
        return err;
    }
    return uv__udp_set_source_membership4(
        handle,
        &raw mut mcast_addr.in_0,
        interface_addr,
        &raw mut src_addr.in_0,
        membership,
    );
}
unsafe extern "C" fn uv__setsockopt(
    mut handle: *mut uv_udp_t,
    mut option4: ::core::ffi::c_int,
    mut option6: ::core::ffi::c_int,
    mut val: *const ::core::ffi::c_void,
    mut size: socklen_t,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    if (*handle).flags & UV_HANDLE_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        r = setsockopt(
            (*handle).io_watcher.fd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            option6,
            val,
            size,
        );
    } else {
        r = setsockopt(
            (*handle).io_watcher.fd,
            IPPROTO_IP as ::core::ffi::c_int,
            option4,
            val,
            size,
        );
    }
    if r != 0 {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__setsockopt_maybe_char(
    mut handle: *mut uv_udp_t,
    mut option4: ::core::ffi::c_int,
    mut option6: ::core::ffi::c_int,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut arg: ::core::ffi::c_int = val;
    if val < 0 as ::core::ffi::c_int || val > 255 as ::core::ffi::c_int {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    return uv__setsockopt(
        handle,
        option4,
        option6,
        &raw mut arg as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
}
pub(crate) unsafe fn uv_udp_set_broadcast(
    mut handle: *mut uv_udp_t,
    mut on: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if setsockopt(
        (*handle).io_watcher.fd,
        SOL_SOCKET,
        SO_BROADCAST,
        &raw mut on as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0
    {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_udp_set_ttl(
    mut handle: *mut uv_udp_t,
    mut ttl: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if ttl < 1 as ::core::ffi::c_int || ttl > 255 as ::core::ffi::c_int {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    return uv__setsockopt_maybe_char(handle, IP_TTL, IPV6_UNICAST_HOPS, ttl);
}
pub(crate) unsafe fn uv_udp_set_multicast_ttl(
    mut handle: *mut uv_udp_t,
    mut ttl: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return uv__setsockopt_maybe_char(handle, IP_MULTICAST_TTL, IPV6_MULTICAST_HOPS, ttl);
}
pub(crate) unsafe fn uv_udp_set_multicast_loop(
    mut handle: *mut uv_udp_t,
    mut on: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return uv__setsockopt_maybe_char(handle, IP_MULTICAST_LOOP, IPV6_MULTICAST_LOOP, on);
}
pub(crate) unsafe fn uv_udp_set_multicast_interface(
    mut handle: *mut uv_udp_t,
    mut interface_addr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut addr_st: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut addr4: *mut sockaddr_in = ::core::ptr::null_mut::<sockaddr_in>();
    let mut addr6: *mut sockaddr_in6 = ::core::ptr::null_mut::<sockaddr_in6>();
    addr4 = &raw mut addr_st as *mut sockaddr_in;
    addr6 = &raw mut addr_st as *mut sockaddr_in6;
    if interface_addr.is_null() {
        memset(
            &raw mut addr_st as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_storage>() as size_t,
        );
        if (*handle).flags & UV_HANDLE_IPV6 as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            addr_st.ss_family = AF_INET6 as sa_family_t;
            (*addr6).sin6_scope_id = 0 as uint32_t;
        } else {
            addr_st.ss_family = AF_INET as sa_family_t;
            (*addr4).sin_addr.s_addr = htonl(INADDR_ANY) as in_addr_t;
        }
    } else if !(uv_ip4_addr(interface_addr, 0 as ::core::ffi::c_int, addr4)
        == 0 as ::core::ffi::c_int)
    {
        if uv_ip6_addr(interface_addr, 0 as ::core::ffi::c_int, addr6) == 0 as ::core::ffi::c_int {
        } else {
            return UV_EINVAL as ::core::ffi::c_int;
        }
    }
    if addr_st.ss_family as ::core::ffi::c_int == AF_INET {
        if setsockopt(
            (*handle).io_watcher.fd,
            IPPROTO_IP as ::core::ffi::c_int,
            IP_MULTICAST_IF,
            &raw mut (*addr4).sin_addr as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<in_addr>() as socklen_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            return -*__errno_location();
        }
    } else if addr_st.ss_family as ::core::ffi::c_int == AF_INET6 {
        if setsockopt(
            (*handle).io_watcher.fd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_MULTICAST_IF,
            &raw mut (*addr6).sin6_scope_id as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint32_t>() as socklen_t,
        ) == -(1 as ::core::ffi::c_int)
        {
            return -*__errno_location();
        }
    } else {
        '_c2rust_label: {
            if 0 as ::core::ffi::c_int != 0
                && !(b"unexpected address family\0" as *const u8 as *const ::core::ffi::c_char)
                    .is_null()
            {
            } else {
                __assert_fail(
                    b"0 && \"unexpected address family\"\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/udp.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1305 as ::core::ffi::c_uint,
                    b"int uv_udp_set_multicast_interface(uv_udp_t *, const char *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        abort();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_udp_getpeername(
    mut handle: *const uv_udp_t,
    mut name: *mut sockaddr,
    mut namelen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
pub(crate) unsafe fn uv_udp_getsockname(
    mut handle: *const uv_udp_t,
    mut name: *mut sockaddr,
    mut namelen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
#[no_mangle]
pub unsafe extern "C" fn uv__udp_recv_start(
    mut handle: *mut uv_udp_t,
    mut alloc_cb: uv_alloc_cb,
    mut recv_cb: uv_udp_recv_cb,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    if alloc_cb.is_none() || recv_cb.is_none() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if uv__io_active(&raw mut (*handle).io_watcher, POLLIN as ::core::ffi::c_uint) != 0 {
        return UV_EALREADY as ::core::ffi::c_int;
    }
    err = uv__udp_maybe_deferred_bind(handle, AF_INET, 0 as ::core::ffi::c_uint);
    if err != 0 {
        return err;
    }
    (*handle).alloc_cb = alloc_cb;
    (*handle).recv_cb = recv_cb;
    uv__io_start(
        (*handle).loop_0,
        &raw mut (*handle).io_watcher,
        POLLIN as ::core::ffi::c_uint,
    );
    if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint)
    {
        (*handle).flags |= UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint;
        if (*handle).flags & UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            (*(*handle).loop_0).active_handles = (*(*handle).loop_0).active_handles.wrapping_add(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__udp_recv_stop(mut handle: *mut uv_udp_t) -> ::core::ffi::c_int {
    uv__io_stop(
        (*handle).loop_0,
        &raw mut (*handle).io_watcher,
        POLLIN as ::core::ffi::c_uint,
    );
    if uv__io_active(
        &raw mut (*handle).io_watcher,
        POLLOUT as ::core::ffi::c_uint,
    ) == 0
    {
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
    }
    (*handle).alloc_cb = None;
    (*handle).recv_cb = None;
    return 0 as ::core::ffi::c_int;
}

pub(crate) unsafe fn using_recvmmsg(
    handle: *const crate::abi::linux_x86_64::uv_udp_t,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_using_recvmmsg(handle.cast()) }
}

pub(crate) unsafe fn bind_udp(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    addr: *const crate::abi::linux_x86_64::sockaddr,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { crate::upstream_support::uv_common::uv_udp_bind(handle.cast(), addr.cast(), flags) }
}

pub(crate) unsafe fn connect_udp(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    addr: *const crate::abi::linux_x86_64::sockaddr,
) -> ::std::os::raw::c_int {
    unsafe { crate::upstream_support::uv_common::uv_udp_connect(handle.cast(), addr.cast()) }
}

pub(crate) unsafe fn getpeername_udp(
    handle: *const crate::abi::linux_x86_64::uv_udp_t,
    name: *mut crate::abi::linux_x86_64::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_getpeername(handle.cast(), name.cast(), namelen) }
}

pub(crate) unsafe fn getsockname_udp(
    handle: *const crate::abi::linux_x86_64::uv_udp_t,
    name: *mut crate::abi::linux_x86_64::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_getsockname(handle.cast(), name.cast(), namelen) }
}

pub(crate) unsafe fn init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::upstream_support::uv_common::uv_udp_init(loop_.cast(), handle.cast()) }
}

pub(crate) unsafe fn init_ex(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe {
        crate::upstream_support::uv_common::uv_udp_init_ex(loop_.cast(), handle.cast(), flags)
    }
}

pub(crate) unsafe fn open(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    sock: crate::abi::linux_x86_64::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_open(handle.cast(), sock) }
}

pub(crate) unsafe fn recv_start(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    alloc_cb: crate::abi::linux_x86_64::uv_alloc_cb,
    recv_cb: crate::abi::linux_x86_64::uv_udp_recv_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        crate::upstream_support::uv_common::uv_udp_recv_start(
            handle.cast(),
            std::mem::transmute::<_, crate::upstream_support::uv_common::uv_alloc_cb>(alloc_cb),
            std::mem::transmute::<_, crate::upstream_support::uv_common::uv_udp_recv_cb>(recv_cb),
        )
    }
}

pub(crate) unsafe fn recv_stop(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::upstream_support::uv_common::uv_udp_recv_stop(handle.cast()) }
}

pub(crate) unsafe fn send(
    req: *mut crate::abi::linux_x86_64::uv_udp_send_t,
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    bufs: *const crate::abi::linux_x86_64::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    addr: *const crate::abi::linux_x86_64::sockaddr,
    send_cb: crate::abi::linux_x86_64::uv_udp_send_cb,
) -> ::std::os::raw::c_int {
    unsafe {
        crate::upstream_support::uv_common::uv_udp_send(
            req.cast(),
            handle.cast(),
            bufs.cast(),
            nbufs,
            addr.cast(),
            std::mem::transmute::<_, crate::upstream_support::uv_common::uv_udp_send_cb>(send_cb),
        )
    }
}

pub(crate) unsafe fn set_broadcast(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    on: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_set_broadcast(handle.cast(), on) }
}

pub(crate) unsafe fn set_membership(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    multicast_addr: *const ::std::os::raw::c_char,
    interface_addr: *const ::std::os::raw::c_char,
    membership: crate::abi::linux_x86_64::uv_membership,
) -> ::std::os::raw::c_int {
    unsafe {
        uv_udp_set_membership(
            handle.cast(),
            multicast_addr,
            interface_addr,
            membership as uv_membership,
        )
    }
}

pub(crate) unsafe fn set_multicast_interface(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    interface_addr: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_set_multicast_interface(handle.cast(), interface_addr) }
}

pub(crate) unsafe fn set_multicast_loop(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    on: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_set_multicast_loop(handle.cast(), on) }
}

pub(crate) unsafe fn set_multicast_ttl(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    ttl: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_set_multicast_ttl(handle.cast(), ttl) }
}

pub(crate) unsafe fn set_source_membership(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    multicast_addr: *const ::std::os::raw::c_char,
    interface_addr: *const ::std::os::raw::c_char,
    source_addr: *const ::std::os::raw::c_char,
    membership: crate::abi::linux_x86_64::uv_membership,
) -> ::std::os::raw::c_int {
    unsafe {
        uv_udp_set_source_membership(
            handle.cast(),
            multicast_addr,
            interface_addr,
            source_addr,
            membership as uv_membership,
        )
    }
}

pub(crate) unsafe fn set_ttl(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    ttl: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { uv_udp_set_ttl(handle.cast(), ttl) }
}

pub(crate) unsafe fn try_send(
    handle: *mut crate::abi::linux_x86_64::uv_udp_t,
    bufs: *const crate::abi::linux_x86_64::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    addr: *const crate::abi::linux_x86_64::sockaddr,
) -> ::std::os::raw::c_int {
    unsafe {
        crate::upstream_support::uv_common::uv_udp_try_send(
            handle.cast(),
            bufs.cast(),
            nbufs,
            addr.cast(),
        )
    }
}
