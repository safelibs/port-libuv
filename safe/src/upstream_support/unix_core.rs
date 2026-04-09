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
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn recvmsg(
        __fd: ::core::ffi::c_int,
        __message: *mut msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn getsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *mut ::core::ffi::c_void,
        __optlen: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn accept4(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getpwuid_r(
        __uid: __uid_t,
        __resultbuf: *mut passwd,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut passwd,
    ) -> ::core::ffi::c_int;
    fn access(__name: *const ::core::ffi::c_char, __type: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    fn dup3(
        __fd: ::core::ffi::c_int,
        __fd2: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn geteuid() -> __uid_t;
    fn gethostname(__name: *mut ::core::ffi::c_char, __len: size_t) -> ::core::ffi::c_int;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    fn __sched_cpucount(__setsize: size_t, __setp: *const cpu_set_t) -> ::core::ffi::c_int;
    fn sched_get_priority_max(__algorithm: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sched_get_priority_min(__algorithm: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> ::core::ffi::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> ::core::ffi::c_int;
    fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: ::core::ffi::c_int,
        __param: *const sched_param,
    ) -> ::core::ffi::c_int;
    fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut ::core::ffi::c_int,
        __param: *mut sched_param,
    ) -> ::core::ffi::c_int;
    fn uv_os_free_passwd(pwd: *mut uv_passwd_t);
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
    fn uv__fs_poll_close(handle: *mut uv_fs_poll_t);
    fn uv__next_timeout(loop_0: *const uv_loop_t) -> ::core::ffi::c_int;
    fn uv__run_timers(loop_0: *mut uv_loop_t);
    fn uv__timer_close(handle: *mut uv_timer_t);
    fn uv__calloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__strdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn uv__malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__free(ptr: *mut ::core::ffi::c_void);
    fn uv__reallocf(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__metrics_update_idle_time(loop_0: *mut uv_loop_t);
    fn abort() -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn realpath(
        __name: *const ::core::ffi::c_char,
        __resolved: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn uv__io_poll(loop_0: *mut uv_loop_t, timeout: ::core::ffi::c_int);
    fn uv__run_idle(loop_0: *mut uv_loop_t);
    fn uv__run_check(loop_0: *mut uv_loop_t);
    fn uv__run_prepare(loop_0: *mut uv_loop_t);
    fn uv__stream_destroy(stream: *mut uv_stream_t);
    fn uv__signal_close(handle: *mut uv_signal_t);
    fn uv__hrtime(type_0: uv_clocktype_t) -> uint64_t;
    fn uv__platform_invalidate_fd(loop_0: *mut uv_loop_t, fd: ::core::ffi::c_int);
    fn uv__async_close(handle: *mut uv_async_t);
    fn uv__check_close(handle: *mut uv_check_t);
    fn uv__fs_event_close(handle: *mut uv_fs_event_t);
    fn uv__idle_close(handle: *mut uv_idle_t);
    fn uv__pipe_close(handle: *mut uv_pipe_t);
    fn uv__poll_close(handle: *mut uv_poll_t);
    fn uv__prepare_close(handle: *mut uv_prepare_t);
    fn uv__process_close(handle: *mut uv_process_t);
    fn uv__stream_close(handle: *mut uv_stream_t);
    fn uv__tcp_close(handle: *mut uv_tcp_t);
    fn uv__udp_close(handle: *mut uv_udp_t);
    fn uv__udp_finish_close(handle: *mut uv_udp_t);
    fn uv__strtok(
        str: *mut ::core::ffi::c_char,
        sep: *const ::core::ffi::c_char,
        itr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::core::ffi::c_int;
    fn getpriority(__which: __priority_which_t, __who: id_t) -> ::core::ffi::c_int;
    fn setpriority(
        __which: __priority_which_t,
        __who: id_t,
        __prio: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getgrgid_r(
        __gid: __gid_t,
        __resultbuf: *mut group,
        __buffer: *mut ::core::ffi::c_char,
        __buflen: size_t,
        __result: *mut *mut group,
    ) -> ::core::ffi::c_int;
    fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __id_t = ::core::ffi::c_uint;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type int32_t = __int32_t;
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
pub type uid_t = __uid_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
pub type pthread_t = ::core::ffi::c_ulong;
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
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_1 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_1 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_1 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_1 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_1 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_1 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_1 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_1 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_1 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_1 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_1 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_1 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_1 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_1 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_1 = 236;
pub const _SC_IPV6: C2RustUnnamed_1 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_1 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_1 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_1 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_1 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_1 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_1 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_1 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_1 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_1 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_1 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_1 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_1 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_1 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_1 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_1 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_1 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_1 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_1 = 182;
pub const _SC_TRACE: C2RustUnnamed_1 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_1 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_1 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_1 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_1 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_1 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_1 = 175;
pub const _SC_STREAMS: C2RustUnnamed_1 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_1 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_1 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_1 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_1 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_1 = 169;
pub const _SC_2_PBS: C2RustUnnamed_1 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_1 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_1 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_1 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_1 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_1 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_1 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_1 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_1 = 160;
pub const _SC_SPAWN: C2RustUnnamed_1 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_1 = 158;
pub const _SC_SHELL: C2RustUnnamed_1 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_1 = 156;
pub const _SC_REGEXP: C2RustUnnamed_1 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_1 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_1 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_1 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_1 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_1 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_1 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_1 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_1 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_1 = 146;
pub const _SC_PIPE: C2RustUnnamed_1 = 145;
pub const _SC_FIFO: C2RustUnnamed_1 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_1 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_1 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_1 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_1 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_1 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_1 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_1 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_1 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_1 = 135;
pub const _SC_BASE: C2RustUnnamed_1 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_1 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_1 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_1 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_1 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_1 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_1 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_1 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_1 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_1 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_1 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_1 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_1 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_1 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_1 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_1 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_1 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_1 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_1 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_1 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_1 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_1 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_1 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_1 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_1 = 110;
pub const _SC_NZERO: C2RustUnnamed_1 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_1 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_1 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_1 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_1 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_1 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_1 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_1 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_1 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_1 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_1 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_1 = 98;
pub const _SC_2_UPE: C2RustUnnamed_1 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_1 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_1 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_1 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_1 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_1 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_1 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_1 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_1 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_1 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_1 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_1 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_1 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_1 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_1 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_1 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_1 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_1 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_1 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_1 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_1 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_1 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_1 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_1 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_1 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_1 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_1 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_1 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_1 = 68;
pub const _SC_THREADS: C2RustUnnamed_1 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_1 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_1 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_1 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_1 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_1 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_1 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_1 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_1 = 60;
pub const _SC_SELECT: C2RustUnnamed_1 = 59;
pub const _SC_POLL: C2RustUnnamed_1 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_1 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_1 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_1 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_1 = 54;
pub const _SC_PII: C2RustUnnamed_1 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_1 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_1 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_1 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_1 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_1 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_1 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_1 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_1 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_1 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_1 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_1 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_1 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_1 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_1 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_1 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_1 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_1 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_1 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_1 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_1 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_1 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_1 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_1 = 30;
pub const _SC_VERSION: C2RustUnnamed_1 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_1 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_1 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_1 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_1 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_1 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_1 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_1 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_1 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_1 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_1 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_1 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_1 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_1 = 16;
pub const _SC_FSYNC: C2RustUnnamed_1 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_1 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_1 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_1 = 12;
pub const _SC_TIMERS: C2RustUnnamed_1 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_1 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_1 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_1 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_1 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_1 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_1 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_1 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_1 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_1 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: ::core::ffi::c_int,
}
pub type __cpu_mask = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
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
pub type uv_os_fd_t = ::core::ffi::c_int;
pub type uv_pid_t = pid_t;
pub type uv_thread_t = pthread_t;
pub type uv_uid_t = uid_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_stream_s {
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
pub union C2RustUnnamed_9 {
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
    pub u: C2RustUnnamed_10,
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
pub union C2RustUnnamed_10 {
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
    pub u: C2RustUnnamed_11,
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
pub union C2RustUnnamed_11 {
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
    pub ipc: ::core::ffi::c_int,
    pub pipe_fname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
pub type uv_pipe_t = uv_pipe_s;
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
pub struct uv_env_item_s {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
}
pub type uv_env_item_t = uv_env_item_s;
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
pub struct uv_utsname_s {
    pub sysname: [::core::ffi::c_char; 256],
    pub release: [::core::ffi::c_char; 256],
    pub version: [::core::ffi::c_char; 256],
    pub machine: [::core::ffi::c_char; 256],
}
pub type uv_utsname_t = uv_utsname_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_metrics_s {
    pub loop_count: uint64_t,
    pub events: uint64_t,
    pub events_waiting: uint64_t,
    pub reserved: [*mut uint64_t; 13],
}
pub type uv_metrics_t = uv_metrics_s;
pub type uv_run_mode = ::core::ffi::c_uint;
pub const UV_RUN_NOWAIT: uv_run_mode = 2;
pub const UV_RUN_ONCE: uv_run_mode = 1;
pub const UV_RUN_DEFAULT: uv_run_mode = 0;
pub type uv_clocktype_t = ::core::ffi::c_uint;
pub const UV_CLOCK_FAST: uv_clocktype_t = 1;
pub const UV_CLOCK_PRECISE: uv_clocktype_t = 0;
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_37 = 4;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_37 = 1;
pub const UV_HANDLE_REF: C2RustUnnamed_37 = 8;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_37 = 2;
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
pub const UV_LOOP_REAP_CHILDREN: C2RustUnnamed_38 = 2;
pub type uv_clock_id = ::core::ffi::c_uint;
pub const UV_CLOCK_REALTIME: uv_clock_id = 1;
pub const UV_CLOCK_MONOTONIC: uv_clock_id = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timespec64_t {
    pub tv_sec: int64_t,
    pub tv_nsec: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timeval_t {
    pub tv_sec: ::core::ffi::c_long,
    pub tv_usec: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_timeval64_t {
    pub tv_sec: int64_t,
    pub tv_usec: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_rusage_t {
    pub ru_utime: uv_timeval_t,
    pub ru_stime: uv_timeval_t,
    pub ru_maxrss: uint64_t,
    pub ru_ixrss: uint64_t,
    pub ru_idrss: uint64_t,
    pub ru_isrss: uint64_t,
    pub ru_minflt: uint64_t,
    pub ru_majflt: uint64_t,
    pub ru_nswap: uint64_t,
    pub ru_inblock: uint64_t,
    pub ru_oublock: uint64_t,
    pub ru_msgsnd: uint64_t,
    pub ru_msgrcv: uint64_t,
    pub ru_nsignals: uint64_t,
    pub ru_nvcsw: uint64_t,
    pub ru_nivcsw: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_22 {
    pub ru_nivcsw: ::core::ffi::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_35,
    pub c2rust_unnamed_0: C2RustUnnamed_34,
    pub c2rust_unnamed_1: C2RustUnnamed_33,
    pub c2rust_unnamed_2: C2RustUnnamed_32,
    pub c2rust_unnamed_3: C2RustUnnamed_31,
    pub c2rust_unnamed_4: C2RustUnnamed_30,
    pub c2rust_unnamed_5: C2RustUnnamed_29,
    pub c2rust_unnamed_6: C2RustUnnamed_28,
    pub c2rust_unnamed_7: C2RustUnnamed_27,
    pub c2rust_unnamed_8: C2RustUnnamed_26,
    pub c2rust_unnamed_9: C2RustUnnamed_25,
    pub c2rust_unnamed_10: C2RustUnnamed_24,
    pub c2rust_unnamed_11: C2RustUnnamed_23,
    pub c2rust_unnamed_12: C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_23 {
    pub ru_nvcsw: ::core::ffi::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub ru_nsignals: ::core::ffi::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub ru_msgrcv: ::core::ffi::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
    pub ru_msgsnd: ::core::ffi::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub ru_oublock: ::core::ffi::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_28 {
    pub ru_inblock: ::core::ffi::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_29 {
    pub ru_nswap: ::core::ffi::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub ru_majflt: ::core::ffi::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub ru_minflt: ::core::ffi::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub ru_isrss: ::core::ffi::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub ru_idrss: ::core::ffi::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_34 {
    pub ru_ixrss: ::core::ffi::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
    pub ru_maxrss: ::core::ffi::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
pub type __rusage_who = ::core::ffi::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
}
pub type __priority_which_t = __priority_which;
pub type __priority_which = ::core::ffi::c_uint;
pub const PRIO_USER: __priority_which = 2;
pub const PRIO_PGRP: __priority_which = 1;
pub const PRIO_PROCESS: __priority_which = 0;
pub type C2RustUnnamed_36 = ::core::ffi::c_int;
pub const UV_THREAD_PRIORITY_LOWEST: C2RustUnnamed_36 = -2;
pub const UV_THREAD_PRIORITY_BELOW_NORMAL: C2RustUnnamed_36 = -1;
pub const UV_THREAD_PRIORITY_NORMAL: C2RustUnnamed_36 = 0;
pub const UV_THREAD_PRIORITY_ABOVE_NORMAL: C2RustUnnamed_36 = 1;
pub const UV_THREAD_PRIORITY_HIGHEST: C2RustUnnamed_36 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
}
pub type C2RustUnnamed_37 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_37 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_37 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_37 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_37 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_37 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_37 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_37 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_37 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_37 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_37 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_37 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_37 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_37 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_37 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_37 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_37 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_37 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_37 = 16777216;
pub const UV_HANDLE_IPV6: C2RustUnnamed_37 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_37 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_37 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_37 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_37 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_37 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_37 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_37 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_37 = 16384;
pub const UV_HANDLE_BOUND: C2RustUnnamed_37 = 8192;
pub const UV_HANDLE_READING: C2RustUnnamed_37 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_37 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_37 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_37 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_37 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_37 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_37 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_37 = 16;
pub type C2RustUnnamed_38 = ::core::ffi::c_uint;
pub const UV_LOOP_BLOCK_SIGPROF: C2RustUnnamed_38 = 1;
pub type uv__peersockfunc = Option<
    unsafe extern "C" fn(::core::ffi::c_int, *mut sockaddr, *mut socklen_t) -> ::core::ffi::c_int,
>;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IOV_MAX: ::core::ffi::c_int = __IOV_MAX;
pub const __IOV_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const X_OK: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SCHED_OTHER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __CPU_SETSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const CPU_SETSIZE: ::core::ffi::c_int = __CPU_SETSIZE;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const UV_PRIORITY_LOW: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const UV_PRIORITY_HIGHEST: ::core::ffi::c_int = -(20 as ::core::ffi::c_int);
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
unsafe extern "C" fn uv__queue_split(
    mut h: *mut uv__queue,
    mut q: *mut uv__queue,
    mut n: *mut uv__queue,
) {
    (*n).prev = (*h).prev;
    (*(*n).prev).next = n;
    (*n).next = q;
    (*h).prev = (*q).prev;
    (*(*h).prev).next = h;
    (*q).prev = n;
}
#[inline]
unsafe extern "C" fn uv__queue_move(mut h: *mut uv__queue, mut n: *mut uv__queue) {
    if uv__queue_empty(h) != 0 {
        uv__queue_init(n);
    } else {
        uv__queue_split(h, (*h).next, n);
    };
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
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLPRI: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLRDHUP: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const FIONBIO: ::core::ffi::c_int = 0x5421 as ::core::ffi::c_int;
pub const UV__POLLRDHUP: ::core::ffi::c_int = POLLRDHUP;
pub const UV__POLLPRI: ::core::ffi::c_int = POLLPRI;
unsafe extern "C" fn uv__update_time(mut loop_0: *mut uv_loop_t) {
    (*loop_0).time = uv__hrtime(UV_CLOCK_FAST).wrapping_div(1000000 as uint64_t);
}
pub const __NR_close: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __NR_gettid: ::core::ffi::c_int = 186 as ::core::ffi::c_int;
pub const SYS_close: ::core::ffi::c_int = __NR_close;
pub const SYS_gettid: ::core::ffi::c_int = __NR_gettid;
pub(crate) unsafe fn uv_clock_gettime(
    mut clock_id: uv_clock_id,
    mut ts: *mut uv_timespec64_t,
) -> ::core::ffi::c_int {
    let mut t: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut r: ::core::ffi::c_int = 0;
    if ts.is_null() {
        return UV_EFAULT as ::core::ffi::c_int;
    }
    match clock_id as ::core::ffi::c_uint {
        0 => {
            r = clock_gettime(CLOCK_MONOTONIC, &raw mut t);
        }
        1 => {
            r = clock_gettime(CLOCK_REALTIME, &raw mut t);
        }
        _ => return UV_EINVAL as ::core::ffi::c_int,
    }
    if r != 0 {
        return -*__errno_location();
    }
    (*ts).tv_sec = t.tv_sec as int64_t;
    (*ts).tv_nsec = t.tv_nsec as int32_t;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_hrtime() -> uint64_t {
    return uv__hrtime(UV_CLOCK_PRECISE);
}
pub(crate) unsafe fn uv_close(mut handle: *mut uv_handle_t, mut close_cb: uv_close_cb) {
    '_c2rust_label: {
        if !((*handle).flags
            & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
        } else {
            __assert_fail(
                b"!uv__is_closing(handle)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                148 as ::core::ffi::c_uint,
                b"void uv_close(uv_handle_t *, uv_close_cb)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*handle).flags |= UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint;
    (*handle).close_cb = close_cb;
    match (*handle).type_0 as ::core::ffi::c_uint {
        7 => {
            uv__pipe_close(handle as *mut uv_pipe_t);
        }
        14 => {
            uv__stream_close(handle as *mut uv_stream_t);
        }
        12 => {
            uv__tcp_close(handle as *mut uv_tcp_t);
        }
        15 => {
            uv__udp_close(handle as *mut uv_udp_t);
        }
        9 => {
            uv__prepare_close(handle as *mut uv_prepare_t);
        }
        2 => {
            uv__check_close(handle as *mut uv_check_t);
        }
        6 => {
            uv__idle_close(handle as *mut uv_idle_t);
        }
        1 => {
            uv__async_close(handle as *mut uv_async_t);
        }
        13 => {
            uv__timer_close(handle as *mut uv_timer_t);
        }
        10 => {
            uv__process_close(handle as *mut uv_process_t);
        }
        3 => {
            uv__fs_event_close(handle as *mut uv_fs_event_t);
        }
        8 => {
            uv__poll_close(handle as *mut uv_poll_t);
        }
        4 => {
            uv__fs_poll_close(handle as *mut uv_fs_poll_t);
            return;
        }
        16 => {
            uv__signal_close(handle as *mut uv_signal_t);
        }
        _ => {
            '_c2rust_label_0: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    222 as ::core::ffi::c_uint,
                    b"void uv_close(uv_handle_t *, uv_close_cb)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
    uv__make_close_pending(handle);
}
#[no_mangle]
pub unsafe extern "C" fn uv__socket_sockopt(
    mut handle: *mut uv_handle_t,
    mut optname: ::core::ffi::c_int,
    mut value: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = 0;
    let mut len: socklen_t = 0;
    if handle.is_null() || value.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if (*handle).type_0 as ::core::ffi::c_uint
        == UV_TCP as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*handle).type_0 as ::core::ffi::c_uint
            == UV_NAMED_PIPE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fd = (*(handle as *mut uv_stream_t)).io_watcher.fd;
    } else if (*handle).type_0 as ::core::ffi::c_uint
        == UV_UDP as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fd = (*(handle as *mut uv_udp_t)).io_watcher.fd;
    } else {
        return UV_ENOTSUP as ::core::ffi::c_int;
    }
    len = ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t;
    if *value == 0 as ::core::ffi::c_int {
        r = getsockopt(
            fd,
            SOL_SOCKET,
            optname,
            value as *mut ::core::ffi::c_void,
            &raw mut len,
        );
    } else {
        r = setsockopt(
            fd,
            SOL_SOCKET,
            optname,
            value as *const ::core::ffi::c_void,
            len,
        );
    }
    if r < 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__make_close_pending(mut handle: *mut uv_handle_t) {
    '_c2rust_label: {
        if (*handle).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        } else {
            __assert_fail(
                b"handle->flags & UV_HANDLE_CLOSING\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                257 as ::core::ffi::c_uint,
                b"void uv__make_close_pending(uv_handle_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*handle).flags & UV_HANDLE_CLOSED as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
        } else {
            __assert_fail(
                b"!(handle->flags & UV_HANDLE_CLOSED)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                258 as ::core::ffi::c_uint,
                b"void uv__make_close_pending(uv_handle_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*handle).next_closing = (*(*handle).loop_0).closing_handles;
    (*(*handle).loop_0).closing_handles = handle;
}
#[no_mangle]
pub unsafe extern "C" fn uv__getiovmax() -> ::core::ffi::c_int {
    return IOV_MAX;
}
unsafe extern "C" fn uv__finish_close(mut handle: *mut uv_handle_t) {
    let mut sh: *mut uv_signal_t = ::core::ptr::null_mut::<uv_signal_t>();
    '_c2rust_label: {
        if (*handle).flags & UV_HANDLE_CLOSING as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        } else {
            __assert_fail(
                b"handle->flags & UV_HANDLE_CLOSING\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                302 as ::core::ffi::c_uint,
                b"void uv__finish_close(uv_handle_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*handle).flags & UV_HANDLE_CLOSED as ::core::ffi::c_int as ::core::ffi::c_uint == 0 {
        } else {
            __assert_fail(
                b"!(handle->flags & UV_HANDLE_CLOSED)\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                303 as ::core::ffi::c_uint,
                b"void uv__finish_close(uv_handle_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*handle).flags |= UV_HANDLE_CLOSED as ::core::ffi::c_int as ::core::ffi::c_uint;
    match (*handle).type_0 as ::core::ffi::c_uint {
        9 | 2 | 6 | 1 | 13 | 10 | 3 | 4 | 8 => {}
        16 => {
            sh = handle as *mut uv_signal_t;
            if (*sh).caught_signals > (*sh).dispatched_signals {
                (*handle).flags ^= UV_HANDLE_CLOSED as ::core::ffi::c_int as ::core::ffi::c_uint;
                uv__make_close_pending(handle);
                return;
            }
        }
        7 | 12 | 14 => {
            uv__stream_destroy(handle as *mut uv_stream_t);
        }
        15 => {
            uv__udp_finish_close(handle as *mut uv_udp_t);
        }
        _ => {
            '_c2rust_label_1: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    343 as ::core::ffi::c_uint,
                    b"void uv__finish_close(uv_handle_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
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
    uv__queue_remove(&raw mut (*handle).handle_queue);
    if (*handle).close_cb.is_some() {
        (*handle).close_cb.expect("non-null function pointer")(handle);
    }
}
unsafe extern "C" fn uv__run_closing_handles(mut loop_0: *mut uv_loop_t) {
    let mut p: *mut uv_handle_t = ::core::ptr::null_mut::<uv_handle_t>();
    let mut q: *mut uv_handle_t = ::core::ptr::null_mut::<uv_handle_t>();
    p = (*loop_0).closing_handles;
    (*loop_0).closing_handles = ::core::ptr::null_mut::<uv_handle_t>();
    while !p.is_null() {
        q = (*p).next_closing;
        uv__finish_close(p);
        p = q;
    }
}
pub(crate) unsafe fn uv_is_closing(mut handle: *const uv_handle_t) -> ::core::ffi::c_int {
    return ((*handle).flags
        & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
            as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_backend_fd(mut loop_0: *const uv_loop_t) -> ::core::ffi::c_int {
    return (*loop_0).backend_fd;
}
unsafe extern "C" fn uv__loop_alive(mut loop_0: *const uv_loop_t) -> ::core::ffi::c_int {
    return ((*loop_0).active_handles > 0 as ::core::ffi::c_uint
        || (*loop_0).active_reqs.count > 0 as ::core::ffi::c_uint
        || uv__queue_empty(&raw const (*loop_0).pending_queue) == 0
        || !(*loop_0).closing_handles.is_null()) as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__backend_timeout(mut loop_0: *const uv_loop_t) -> ::core::ffi::c_int {
    if (*loop_0).stop_flag == 0 as ::core::ffi::c_uint
        && ((*loop_0).active_handles > 0 as ::core::ffi::c_uint
            || (*loop_0).active_reqs.count > 0 as ::core::ffi::c_uint)
        && uv__queue_empty(&raw const (*loop_0).pending_queue) != 0
        && uv__queue_empty(&raw const (*loop_0).idle_handles) != 0
        && (*loop_0).flags & UV_LOOP_REAP_CHILDREN as ::core::ffi::c_int as ::core::ffi::c_ulong
            == 0 as ::core::ffi::c_ulong
        && (*loop_0).closing_handles.is_null()
    {
        return uv__next_timeout(loop_0);
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_backend_timeout(mut loop_0: *const uv_loop_t) -> ::core::ffi::c_int {
    if uv__queue_empty(&raw const (*loop_0).watcher_queue) != 0 {
        return uv__backend_timeout(loop_0);
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_loop_alive(mut loop_0: *const uv_loop_t) -> ::core::ffi::c_int {
    return uv__loop_alive(loop_0);
}
pub(crate) unsafe fn uv_run(
    mut loop_0: *mut uv_loop_t,
    mut mode: uv_run_mode,
) -> ::core::ffi::c_int {
    let mut timeout: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    let mut can_sleep: ::core::ffi::c_int = 0;
    r = uv__loop_alive(loop_0);
    if r == 0 {
        uv__update_time(loop_0);
    }
    if mode as ::core::ffi::c_uint == UV_RUN_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        && r != 0 as ::core::ffi::c_int
        && (*loop_0).stop_flag == 0 as ::core::ffi::c_uint
    {
        uv__update_time(loop_0);
        uv__run_timers(loop_0);
    }
    while r != 0 as ::core::ffi::c_int && (*loop_0).stop_flag == 0 as ::core::ffi::c_uint {
        can_sleep = (uv__queue_empty(&raw mut (*loop_0).pending_queue) != 0
            && uv__queue_empty(&raw mut (*loop_0).idle_handles) != 0)
            as ::core::ffi::c_int;
        uv__run_pending(loop_0);
        uv__run_idle(loop_0);
        uv__run_prepare(loop_0);
        timeout = 0 as ::core::ffi::c_int;
        if mode as ::core::ffi::c_uint == UV_RUN_ONCE as ::core::ffi::c_int as ::core::ffi::c_uint
            && can_sleep != 0
            || mode as ::core::ffi::c_uint
                == UV_RUN_DEFAULT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            timeout = uv__backend_timeout(loop_0);
        }
        let ref mut fresh0 = (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t))
            .loop_metrics
            .metrics
            .loop_count;
        *fresh0 = (*fresh0).wrapping_add(1);
        uv__io_poll(loop_0, timeout);
        crate::unix::fd::process_reap(loop_0.cast());
        r = 0 as ::core::ffi::c_int;
        while r < 8 as ::core::ffi::c_int && uv__queue_empty(&raw mut (*loop_0).pending_queue) == 0
        {
            uv__run_pending(loop_0);
            r += 1;
        }
        uv__metrics_update_idle_time(loop_0);
        uv__run_check(loop_0);
        uv__run_closing_handles(loop_0);
        uv__update_time(loop_0);
        uv__run_timers(loop_0);
        r = uv__loop_alive(loop_0);
        if mode as ::core::ffi::c_uint == UV_RUN_ONCE as ::core::ffi::c_int as ::core::ffi::c_uint
            || mode as ::core::ffi::c_uint
                == UV_RUN_NOWAIT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            break;
        }
    }
    if (*loop_0).stop_flag != 0 as ::core::ffi::c_uint {
        (*loop_0).stop_flag = 0 as ::core::ffi::c_uint;
    }
    return r;
}
pub(crate) unsafe fn uv_update_time(mut loop_0: *mut uv_loop_t) {
    uv__update_time(loop_0);
}
pub(crate) unsafe fn uv_is_active(mut handle: *const uv_handle_t) -> ::core::ffi::c_int {
    return ((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__socket(
    mut domain: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut protocol: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sockfd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    sockfd = socket(
        domain,
        type_0 | SOCK_NONBLOCK as ::core::ffi::c_int | SOCK_CLOEXEC as ::core::ffi::c_int,
        protocol,
    );
    if sockfd != -(1 as ::core::ffi::c_int) {
        return sockfd;
    }
    if *__errno_location() != EINVAL {
        return -*__errno_location();
    }
    sockfd = socket(domain, type_0, protocol);
    if sockfd == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    err = uv__nonblock_ioctl(sockfd, 1 as ::core::ffi::c_int);
    if err == 0 as ::core::ffi::c_int {
        err = uv__cloexec(sockfd, 1 as ::core::ffi::c_int);
    }
    if err != 0 {
        uv__close(sockfd);
        return err;
    }
    return sockfd;
}
#[no_mangle]
pub unsafe extern "C" fn uv__open_file(mut path: *const ::core::ffi::c_char) -> *mut FILE {
    let mut fd: ::core::ffi::c_int = 0;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fd = uv__open_cloexec(path, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<FILE>();
    }
    fp = fdopen(fd, b"r\0" as *const u8 as *const ::core::ffi::c_char);
    if fp.is_null() {
        uv__close(fd);
    }
    return fp;
}
#[no_mangle]
pub unsafe extern "C" fn uv__accept(mut sockfd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut peerfd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    &raw mut err;
    '_c2rust_label: {
        if sockfd >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"sockfd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                552 as ::core::ffi::c_uint,
                b"int uv__accept(int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    loop {
        peerfd = accept4(
            sockfd,
            __SOCKADDR_ARG {
                __sockaddr__: ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut sockaddr,
            },
            ::core::ptr::null_mut::<socklen_t>(),
            SOCK_NONBLOCK as ::core::ffi::c_int | SOCK_CLOEXEC as ::core::ffi::c_int,
        );
        if !(peerfd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if peerfd == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    return peerfd;
}
#[no_mangle]
pub unsafe extern "C" fn uv__close_nocancel(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return syscall(SYS_close as ::core::ffi::c_long, fd) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__close_nocheckstdio(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut saved_errno: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if fd > -(1 as ::core::ffi::c_int) {
        } else {
            __assert_fail(
                b"fd > -1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                618 as ::core::ffi::c_uint,
                b"int uv__close_nocheckstdio(int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    saved_errno = *__errno_location();
    rc = uv__close_nocancel(fd);
    if rc == -(1 as ::core::ffi::c_int) {
        rc = -*__errno_location();
        if rc == UV_EINTR as ::core::ffi::c_int || rc == -(115 as ::core::ffi::c_int) {
            rc = 0 as ::core::ffi::c_int;
        }
        *__errno_location() = saved_errno;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn uv__close(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if fd > 2 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"fd > STDERR_FILENO\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                634 as ::core::ffi::c_uint,
                b"int uv__close(int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    return uv__close_nocheckstdio(fd);
}
#[no_mangle]
pub unsafe extern "C" fn uv__nonblock_ioctl(
    mut fd: ::core::ffi::c_int,
    mut set: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    loop {
        r = ioctl(fd, FIONBIO as ::core::ffi::c_ulong, &raw mut set);
        if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if r != 0 {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__nonblock_fcntl(
    mut fd: ::core::ffi::c_int,
    mut set: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    loop {
        r = fcntl(fd, F_GETFL);
        if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if r == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    if (r & O_NONBLOCK != 0) as ::core::ffi::c_int == (set != 0) as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    if set != 0 {
        flags = r | O_NONBLOCK;
    } else {
        flags = r & !O_NONBLOCK;
    }
    loop {
        r = fcntl(fd, F_SETFL, flags);
        if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if r != 0 {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__cloexec(
    mut fd: ::core::ffi::c_int,
    mut set: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut flags: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_int = 0;
    flags = 0 as ::core::ffi::c_int;
    if set != 0 {
        flags = FD_CLOEXEC;
    }
    loop {
        r = fcntl(fd, F_SETFD, flags);
        if !(r == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    if r != 0 {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__recvmsg(
    mut fd: ::core::ffi::c_int,
    mut msg: *mut msghdr,
    mut flags: ::core::ffi::c_int,
) -> ssize_t {
    let mut rc: ssize_t = 0;
    rc = recvmsg(fd, msg, flags | MSG_CMSG_CLOEXEC as ::core::ffi::c_int);
    if rc == -(1 as ::core::ffi::c_int) as ssize_t {
        return -*__errno_location() as ssize_t;
    }
    return rc;
}
pub(crate) unsafe fn uv_cwd(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut scratch: [::core::ffi::c_char; 4097] = [0; 4097];
    if buffer.is_null() || size.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if getcwd(buffer, *size).is_null() {
        if *__errno_location() != ERANGE {
            return -*__errno_location();
        }
        if getcwd(
            &raw mut scratch as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
        )
        .is_null()
        {
            return -*__errno_location();
        }
        buffer = &raw mut scratch as *mut ::core::ffi::c_char;
    }
    *size = strlen(buffer);
    if *size > 1 as size_t
        && *buffer.offset((*size).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            == '/' as i32
    {
        *size = (*size).wrapping_sub(1 as size_t);
        *buffer.offset(*size as isize) = '\0' as i32 as ::core::ffi::c_char;
    }
    if buffer == &raw mut scratch as *mut ::core::ffi::c_char {
        *size = (*size).wrapping_add(1 as size_t);
        return UV_ENOBUFS as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_chdir(mut dir: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if chdir(dir) != 0 {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_disable_stdio_inheritance() {
    let mut fd: ::core::ffi::c_int = 0;
    fd = 0 as ::core::ffi::c_int;
    while !(uv__cloexec(fd, 1 as ::core::ffi::c_int) != 0 && fd > 15 as ::core::ffi::c_int) {
        fd += 1;
    }
}
pub(crate) unsafe fn uv_fileno(
    mut handle: *const uv_handle_t,
    mut fd: *mut uv_os_fd_t,
) -> ::core::ffi::c_int {
    let mut fd_out: ::core::ffi::c_int = 0;
    match (*handle).type_0 as ::core::ffi::c_uint {
        12 | 7 | 14 => {
            fd_out = (*(handle as *mut uv_stream_t)).io_watcher.fd;
        }
        15 => {
            fd_out = (*(handle as *mut uv_udp_t)).io_watcher.fd;
        }
        8 => {
            fd_out = (*(handle as *mut uv_poll_t)).io_watcher.fd;
        }
        _ => return UV_EINVAL as ::core::ffi::c_int,
    }
    if (*handle).flags
        & (UV_HANDLE_CLOSING as ::core::ffi::c_int | UV_HANDLE_CLOSED as ::core::ffi::c_int)
            as ::core::ffi::c_uint
        != 0 as ::core::ffi::c_uint
        || fd_out == -(1 as ::core::ffi::c_int)
    {
        return UV_EBADF as ::core::ffi::c_int;
    }
    *fd = fd_out as uv_os_fd_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__run_pending(mut loop_0: *mut uv_loop_t) {
    let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
    let mut pq: uv__queue = uv__queue {
        next: ::core::ptr::null_mut::<uv__queue>(),
        prev: ::core::ptr::null_mut::<uv__queue>(),
    };
    let mut w: *mut uv__io_t = ::core::ptr::null_mut::<uv__io_t>();
    uv__queue_move(&raw mut (*loop_0).pending_queue, &raw mut pq);
    while uv__queue_empty(&raw mut pq) == 0 {
        q = uv__queue_head(&raw mut pq);
        uv__queue_remove(q);
        uv__queue_init(q);
        w = (q as *mut ::core::ffi::c_char).offset(-(8 as ::core::ffi::c_ulong as isize))
            as *mut uv__io_t;
        (*w).cb.expect("non-null function pointer")(
            loop_0 as *mut uv_loop_s,
            w as *mut uv__io_s,
            POLLOUT as ::core::ffi::c_uint,
        );
    }
}
unsafe extern "C" fn next_power_of_two(mut val: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    val = val.wrapping_sub(1 as ::core::ffi::c_uint);
    val |= val >> 1 as ::core::ffi::c_int;
    val |= val >> 2 as ::core::ffi::c_int;
    val |= val >> 4 as ::core::ffi::c_int;
    val |= val >> 8 as ::core::ffi::c_int;
    val |= val >> 16 as ::core::ffi::c_int;
    val = val.wrapping_add(1 as ::core::ffi::c_uint);
    return val;
}
unsafe extern "C" fn maybe_resize(mut loop_0: *mut uv_loop_t, mut len: ::core::ffi::c_uint) {
    let mut watchers: *mut *mut uv__io_t = ::core::ptr::null_mut::<*mut uv__io_t>();
    let mut fake_watcher_list: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut fake_watcher_count: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut nwatchers: ::core::ffi::c_uint = 0;
    let mut i: ::core::ffi::c_uint = 0;
    if len <= (*loop_0).nwatchers {
        return;
    }
    if !(*loop_0).watchers.is_null() {
        fake_watcher_list =
            *(*loop_0).watchers.offset((*loop_0).nwatchers as isize) as *mut ::core::ffi::c_void;
        fake_watcher_count = *(*loop_0)
            .watchers
            .offset((*loop_0).nwatchers.wrapping_add(1 as ::core::ffi::c_uint) as isize)
            as *mut ::core::ffi::c_void;
    } else {
        fake_watcher_list = ::core::ptr::null_mut::<::core::ffi::c_void>();
        fake_watcher_count = ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    nwatchers = next_power_of_two(len.wrapping_add(2 as ::core::ffi::c_uint))
        .wrapping_sub(2 as ::core::ffi::c_uint);
    watchers = uv__reallocf(
        (*loop_0).watchers as *mut ::core::ffi::c_void,
        (nwatchers.wrapping_add(2 as ::core::ffi::c_uint) as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut uv__io_t>() as size_t),
    ) as *mut *mut uv__io_t;
    if watchers.is_null() {
        abort();
    }
    i = (*loop_0).nwatchers;
    while i < nwatchers {
        let ref mut fresh4 = *watchers.offset(i as isize);
        *fresh4 = ::core::ptr::null_mut::<uv__io_t>();
        i = i.wrapping_add(1);
    }
    let ref mut fresh5 = *watchers.offset(nwatchers as isize);
    *fresh5 = fake_watcher_list as *mut uv__io_t;
    let ref mut fresh6 =
        *watchers.offset(nwatchers.wrapping_add(1 as ::core::ffi::c_uint) as isize);
    *fresh6 = fake_watcher_count as *mut uv__io_t;
    (*loop_0).watchers = watchers;
    (*loop_0).nwatchers = nwatchers;
}
#[no_mangle]
pub unsafe extern "C" fn uv__io_init(
    mut w: *mut uv__io_t,
    mut cb: uv__io_cb,
    mut fd: ::core::ffi::c_int,
) {
    '_c2rust_label: {
        if cb.is_some() {
        } else {
            __assert_fail(
                b"cb != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                894 as ::core::ffi::c_uint,
                b"void uv__io_init(uv__io_t *, uv__io_cb, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if fd >= -(1 as ::core::ffi::c_int) {
        } else {
            __assert_fail(
                b"fd >= -1\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                895 as ::core::ffi::c_uint,
                b"void uv__io_init(uv__io_t *, uv__io_cb, int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    uv__queue_init(&raw mut (*w).pending_queue);
    uv__queue_init(&raw mut (*w).watcher_queue);
    (*w).cb = cb;
    (*w).fd = fd;
    (*w).events = 0 as ::core::ffi::c_uint;
    (*w).pevents = 0 as ::core::ffi::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn uv__io_start(
    mut loop_0: *mut uv_loop_t,
    mut w: *mut uv__io_t,
    mut events: ::core::ffi::c_uint,
) {
    '_c2rust_label: {
        if 0 as ::core::ffi::c_uint
            == events
                & !(0x1 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int
                    | 0x2000 as ::core::ffi::c_int
                    | 0x2 as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"0 == (events & ~(POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                906 as ::core::ffi::c_uint,
                b"void uv__io_start(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_uint != events {
        } else {
            __assert_fail(
                b"0 != events\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                907 as ::core::ffi::c_uint,
                b"void uv__io_start(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*w).fd >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"w->fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                908 as ::core::ffi::c_uint,
                b"void uv__io_start(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*w).fd < 2147483647 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"w->fd < INT_MAX\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                909 as ::core::ffi::c_uint,
                b"void uv__io_start(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*w).pevents |= events;
    maybe_resize(
        loop_0,
        ((*w).fd + 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    if (*w).events == (*w).pevents {
        return;
    }
    if uv__queue_empty(&raw mut (*w).watcher_queue) != 0 {
        uv__queue_insert_tail(
            &raw mut (*loop_0).watcher_queue,
            &raw mut (*w).watcher_queue,
        );
    }
    if (*(*loop_0).watchers.offset((*w).fd as isize)).is_null() {
        let ref mut fresh3 = *(*loop_0).watchers.offset((*w).fd as isize);
        *fresh3 = w;
        (*loop_0).nfds = (*loop_0).nfds.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn uv__io_stop(
    mut loop_0: *mut uv_loop_t,
    mut w: *mut uv__io_t,
    mut events: ::core::ffi::c_uint,
) {
    '_c2rust_label: {
        if 0 as ::core::ffi::c_uint
            == events
                & !(0x1 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int
                    | 0x2000 as ::core::ffi::c_int
                    | 0x2 as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"0 == (events & ~(POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                934 as ::core::ffi::c_uint,
                b"void uv__io_stop(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_uint != events {
        } else {
            __assert_fail(
                b"0 != events\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                935 as ::core::ffi::c_uint,
                b"void uv__io_stop(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*w).fd == -(1 as ::core::ffi::c_int) {
        return;
    }
    '_c2rust_label_1: {
        if (*w).fd >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"w->fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                940 as ::core::ffi::c_uint,
                b"void uv__io_stop(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*w).fd as ::core::ffi::c_uint >= (*loop_0).nwatchers {
        return;
    }
    (*w).pevents &= !events;
    if (*w).pevents == 0 as ::core::ffi::c_uint {
        uv__queue_remove(&raw mut (*w).watcher_queue);
        uv__queue_init(&raw mut (*w).watcher_queue);
        (*w).events = 0 as ::core::ffi::c_uint;
        if w == *(*loop_0).watchers.offset((*w).fd as isize) {
            '_c2rust_label_2: {
                if (*loop_0).nfds > 0 as ::core::ffi::c_uint {
                } else {
                    __assert_fail(
                        b"loop->nfds > 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        954 as ::core::ffi::c_uint,
                        b"void uv__io_stop(uv_loop_t *, uv__io_t *, unsigned int)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            let ref mut fresh7 = *(*loop_0).watchers.offset((*w).fd as isize);
            *fresh7 = ::core::ptr::null_mut::<uv__io_t>();
            (*loop_0).nfds = (*loop_0).nfds.wrapping_sub(1);
        }
    } else if uv__queue_empty(&raw mut (*w).watcher_queue) != 0 {
        uv__queue_insert_tail(
            &raw mut (*loop_0).watcher_queue,
            &raw mut (*w).watcher_queue,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn uv__io_close(mut loop_0: *mut uv_loop_t, mut w: *mut uv__io_t) {
    uv__io_stop(
        loop_0,
        w,
        (POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI) as ::core::ffi::c_uint,
    );
    uv__queue_remove(&raw mut (*w).pending_queue);
    if (*w).fd != -(1 as ::core::ffi::c_int) {
        uv__platform_invalidate_fd(loop_0, (*w).fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn uv__io_feed(mut loop_0: *mut uv_loop_t, mut w: *mut uv__io_t) {
    if uv__queue_empty(&raw mut (*w).pending_queue) != 0 {
        uv__queue_insert_tail(
            &raw mut (*loop_0).pending_queue,
            &raw mut (*w).pending_queue,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn uv__io_active(
    mut w: *const uv__io_t,
    mut events: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if 0 as ::core::ffi::c_uint
            == events
                & !(0x1 as ::core::ffi::c_int
                    | 0x4 as ::core::ffi::c_int
                    | 0x2000 as ::core::ffi::c_int
                    | 0x2 as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"0 == (events & ~(POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI))\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                981 as ::core::ffi::c_uint,
                b"int uv__io_active(const uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if 0 as ::core::ffi::c_uint != events {
        } else {
            __assert_fail(
                b"0 != events\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                982 as ::core::ffi::c_uint,
                b"int uv__io_active(const uv__io_t *, unsigned int)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (0 as ::core::ffi::c_uint != (*w).pevents & events) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__fd_exists(
    mut loop_0: *mut uv_loop_t,
    mut fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return ((fd as ::core::ffi::c_uint) < (*loop_0).nwatchers
        && !(*(*loop_0).watchers.offset(fd as isize)).is_null()) as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_getrusage(mut rusage: *mut uv_rusage_t) -> ::core::ffi::c_int {
    let mut usage: rusage = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        c2rust_unnamed: C2RustUnnamed_35 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_34 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_33 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_32 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_31 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_30 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_29 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_28 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_27 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_26 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_25 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_24 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_23 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed_22 { ru_nivcsw: 0 },
    };
    if getrusage(RUSAGE_SELF, &raw mut usage) != 0 {
        return -*__errno_location();
    }
    (*rusage).ru_utime.tv_sec = usage.ru_utime.tv_sec as ::core::ffi::c_long;
    (*rusage).ru_utime.tv_usec = usage.ru_utime.tv_usec as ::core::ffi::c_long;
    (*rusage).ru_stime.tv_sec = usage.ru_stime.tv_sec as ::core::ffi::c_long;
    (*rusage).ru_stime.tv_usec = usage.ru_stime.tv_usec as ::core::ffi::c_long;
    (*rusage).ru_maxrss = usage.c2rust_unnamed.ru_maxrss as uint64_t;
    (*rusage).ru_ixrss = usage.c2rust_unnamed_0.ru_ixrss as uint64_t;
    (*rusage).ru_idrss = usage.c2rust_unnamed_1.ru_idrss as uint64_t;
    (*rusage).ru_isrss = usage.c2rust_unnamed_2.ru_isrss as uint64_t;
    (*rusage).ru_minflt = usage.c2rust_unnamed_3.ru_minflt as uint64_t;
    (*rusage).ru_majflt = usage.c2rust_unnamed_4.ru_majflt as uint64_t;
    (*rusage).ru_nswap = usage.c2rust_unnamed_5.ru_nswap as uint64_t;
    (*rusage).ru_inblock = usage.c2rust_unnamed_6.ru_inblock as uint64_t;
    (*rusage).ru_oublock = usage.c2rust_unnamed_7.ru_oublock as uint64_t;
    (*rusage).ru_msgsnd = usage.c2rust_unnamed_8.ru_msgsnd as uint64_t;
    (*rusage).ru_msgrcv = usage.c2rust_unnamed_9.ru_msgrcv as uint64_t;
    (*rusage).ru_nsignals = usage.c2rust_unnamed_10.ru_nsignals as uint64_t;
    (*rusage).ru_nvcsw = usage.c2rust_unnamed_11.ru_nvcsw as uint64_t;
    (*rusage).ru_nivcsw = usage.c2rust_unnamed_12.ru_nivcsw as uint64_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__open_cloexec(
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(path, flags | O_CLOEXEC);
    if fd == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn uv__slurp(
    mut filename: *const ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    let mut n: ssize_t = 0;
    let mut fd: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if len > 0 as size_t {
        } else {
            __assert_fail(
                b"len > 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1066 as ::core::ffi::c_uint,
                b"int uv__slurp(const char *, char *, size_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    fd = uv__open_cloexec(filename, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        return fd;
    }
    loop {
        n = read(
            fd,
            buf as *mut ::core::ffi::c_void,
            len.wrapping_sub(1 as size_t),
        );
        if !(n == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
            break;
        }
    }
    if uv__close_nocheckstdio(fd) != 0 {
        abort();
    }
    if n < 0 as ssize_t {
        return -*__errno_location();
    }
    *buf.offset(n as isize) = '\0' as i32 as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uv__dup2_cloexec(
    mut oldfd: ::core::ffi::c_int,
    mut newfd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    r = dup3(oldfd, newfd, O_CLOEXEC);
    if r == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    return r;
}
pub(crate) unsafe fn uv_os_homedir(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut pwd: uv_passwd_t = uv_passwd_s {
        username: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        uid: 0,
        gid: 0,
        shell: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        homedir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut len: size_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    r = uv_os_getenv(
        b"HOME\0" as *const u8 as *const ::core::ffi::c_char,
        buffer,
        size,
    );
    if r != UV_ENOENT as ::core::ffi::c_int {
        return r;
    }
    r = uv_os_get_passwd(&raw mut pwd);
    if r != 0 as ::core::ffi::c_int {
        return r;
    }
    len = strlen(pwd.homedir);
    if len >= *size {
        *size = len.wrapping_add(1 as size_t);
        uv_os_free_passwd(&raw mut pwd);
        return UV_ENOBUFS as ::core::ffi::c_int;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        pwd.homedir as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    *size = len;
    uv_os_free_passwd(&raw mut pwd);
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_tmpdir(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut buf: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    if buffer.is_null() || size.is_null() || *size == 0 as size_t {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    buf = getenv(b"TMPDIR\0" as *const u8 as *const ::core::ffi::c_char);
    if buf.is_null() {
        buf = getenv(b"TMP\0" as *const u8 as *const ::core::ffi::c_char);
        if buf.is_null() {
            buf = getenv(b"TEMP\0" as *const u8 as *const ::core::ffi::c_char);
            if buf.is_null() {
                buf = getenv(b"TEMPDIR\0" as *const u8 as *const ::core::ffi::c_char);
                if buf.is_null() {
                    buf = b"/tmp\0" as *const u8 as *const ::core::ffi::c_char;
                }
            }
        }
    }
    len = strlen(buf);
    if len >= *size {
        *size = len.wrapping_add(1 as size_t);
        return UV_ENOBUFS as ::core::ffi::c_int;
    }
    if len > 1 as size_t
        && *buf.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == '/' as i32
    {
        len = len.wrapping_sub(1);
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        buf as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    *buffer.offset(len as isize) = '\0' as i32 as ::core::ffi::c_char;
    *size = len;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn uv__getpwuid_r(
    mut pwd: *mut uv_passwd_t,
    mut uid: uid_t,
) -> ::core::ffi::c_int {
    let mut pw: passwd = passwd {
        pw_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_uid: 0,
        pw_gid: 0,
        pw_gecos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pw_shell: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut result: *mut passwd = ::core::ptr::null_mut::<passwd>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bufsize: size_t = 0;
    let mut name_size: size_t = 0;
    let mut homedir_size: size_t = 0;
    let mut shell_size: size_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    if pwd.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    bufsize = 2000 as size_t;
    loop {
        buf = uv__malloc(bufsize) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        loop {
            r = getpwuid_r(uid as __uid_t, &raw mut pw, buf, bufsize, &raw mut result);
            if !(r == EINTR) {
                break;
            }
        }
        if r != 0 as ::core::ffi::c_int || result.is_null() {
            uv__free(buf as *mut ::core::ffi::c_void);
        }
        if r != ERANGE {
            break;
        }
        bufsize = bufsize.wrapping_mul(2 as size_t);
    }
    if r != 0 as ::core::ffi::c_int {
        return -r;
    }
    if result.is_null() {
        return UV_ENOENT as ::core::ffi::c_int;
    }
    name_size = strlen(pw.pw_name).wrapping_add(1 as size_t);
    homedir_size = strlen(pw.pw_dir).wrapping_add(1 as size_t);
    shell_size = strlen(pw.pw_shell).wrapping_add(1 as size_t);
    (*pwd).username = uv__malloc(
        name_size
            .wrapping_add(homedir_size)
            .wrapping_add(shell_size),
    ) as *mut ::core::ffi::c_char;
    if (*pwd).username.is_null() {
        uv__free(buf as *mut ::core::ffi::c_void);
        return UV_ENOMEM as ::core::ffi::c_int;
    }
    memcpy(
        (*pwd).username as *mut ::core::ffi::c_void,
        pw.pw_name as *const ::core::ffi::c_void,
        name_size,
    );
    (*pwd).homedir = (*pwd).username.offset(name_size as isize);
    memcpy(
        (*pwd).homedir as *mut ::core::ffi::c_void,
        pw.pw_dir as *const ::core::ffi::c_void,
        homedir_size,
    );
    (*pwd).shell = (*pwd).homedir.offset(homedir_size as isize);
    memcpy(
        (*pwd).shell as *mut ::core::ffi::c_void,
        pw.pw_shell as *const ::core::ffi::c_void,
        shell_size,
    );
    (*pwd).uid = pw.pw_uid as ::core::ffi::c_ulong;
    (*pwd).gid = pw.pw_gid as ::core::ffi::c_ulong;
    uv__free(buf as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_get_group(
    mut grp: *mut uv_group_t,
    mut gid: uv_uid_t,
) -> ::core::ffi::c_int {
    let mut gp: group = group {
        gr_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        gr_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        gr_gid: 0,
        gr_mem: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    };
    let mut result: *mut group = ::core::ptr::null_mut::<group>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut gr_mem: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut bufsize: size_t = 0;
    let mut name_size: size_t = 0;
    let mut members: ::core::ffi::c_long = 0;
    let mut mem_size: size_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    if grp.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    bufsize = 2000 as size_t;
    loop {
        buf = uv__malloc(bufsize) as *mut ::core::ffi::c_char;
        if buf.is_null() {
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        loop {
            r = getgrgid_r(gid as __gid_t, &raw mut gp, buf, bufsize, &raw mut result);
            if !(r == EINTR) {
                break;
            }
        }
        if r != 0 as ::core::ffi::c_int || result.is_null() {
            uv__free(buf as *mut ::core::ffi::c_void);
        }
        if r != ERANGE {
            break;
        }
        bufsize = bufsize.wrapping_mul(2 as size_t);
    }
    if r != 0 as ::core::ffi::c_int {
        return -r;
    }
    if result.is_null() {
        return UV_ENOENT as ::core::ffi::c_int;
    }
    name_size = strlen(gp.gr_name).wrapping_add(1 as size_t);
    members = 0 as ::core::ffi::c_long;
    mem_size = ::core::mem::size_of::<*mut ::core::ffi::c_char>() as usize as size_t;
    r = 0 as ::core::ffi::c_int;
    while !(*gp.gr_mem.offset(r as isize)).is_null() {
        mem_size = (mem_size as ::core::ffi::c_ulong).wrapping_add(
            strlen(*gp.gr_mem.offset(r as isize))
                .wrapping_add(1 as size_t)
                .wrapping_add(::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
                as ::core::ffi::c_ulong,
        ) as size_t as size_t;
        members += 1;
        r += 1;
    }
    gr_mem = uv__malloc(name_size.wrapping_add(mem_size)) as *mut ::core::ffi::c_char;
    if gr_mem.is_null() {
        uv__free(buf as *mut ::core::ffi::c_void);
        return UV_ENOMEM as ::core::ffi::c_int;
    }
    (*grp).members = gr_mem as *mut *mut ::core::ffi::c_char;
    let ref mut fresh1 = *(*grp).members.offset(members as isize);
    *fresh1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
    gr_mem = (*grp)
        .members
        .offset((members + 1 as ::core::ffi::c_long) as isize)
        as *mut *mut ::core::ffi::c_char as *mut ::core::ffi::c_char;
    r = 0 as ::core::ffi::c_int;
    while (r as ::core::ffi::c_long) < members {
        let ref mut fresh2 = *(*grp).members.offset(r as isize);
        *fresh2 = gr_mem;
        strcpy(gr_mem, *gp.gr_mem.offset(r as isize));
        gr_mem = gr_mem.offset(strlen(gr_mem).wrapping_add(1 as size_t) as isize);
        r += 1;
    }
    '_c2rust_label: {
        if gr_mem == ((*grp).members as *mut ::core::ffi::c_char).offset(mem_size as isize) {
        } else {
            __assert_fail(
                b"gr_mem == (char*)grp->members + mem_size\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1342 as ::core::ffi::c_uint,
                b"int uv_os_get_group(uv_group_t *, uv_uid_t)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*grp).groupname = gr_mem;
    memcpy(
        (*grp).groupname as *mut ::core::ffi::c_void,
        gp.gr_name as *const ::core::ffi::c_void,
        name_size,
    );
    gr_mem = gr_mem.offset(name_size as isize);
    (*grp).gid = gp.gr_gid as ::core::ffi::c_ulong;
    uv__free(buf as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_get_passwd(mut pwd: *mut uv_passwd_t) -> ::core::ffi::c_int {
    return uv__getpwuid_r(pwd, geteuid() as uid_t);
}
pub(crate) unsafe fn uv_os_get_passwd2(
    mut pwd: *mut uv_passwd_t,
    mut uid: uv_uid_t,
) -> ::core::ffi::c_int {
    return uv__getpwuid_r(pwd, uid as uid_t);
}
pub(crate) unsafe fn uv_translate_sys_error(
    mut sys_errno: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return if sys_errno <= 0 as ::core::ffi::c_int {
        sys_errno
    } else {
        -sys_errno
    };
}
pub(crate) unsafe fn uv_os_environ(
    mut envitems: *mut *mut uv_env_item_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut cnt: ::core::ffi::c_int = 0;
    let mut envitem: *mut uv_env_item_t = ::core::ptr::null_mut::<uv_env_item_t>();
    *envitems = ::core::ptr::null_mut::<uv_env_item_t>();
    *count = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while !(*environ.offset(i as isize)).is_null() {
        i += 1;
    }
    *envitems = uv__calloc(
        i as size_t,
        ::core::mem::size_of::<uv_env_item_t>() as size_t,
    ) as *mut uv_env_item_t;
    if (*envitems).is_null() {
        return UV_ENOMEM as ::core::ffi::c_int;
    }
    j = 0 as ::core::ffi::c_int;
    cnt = 0 as ::core::ffi::c_int;
    loop {
        if !(j < i) {
            current_block = 5689001924483802034;
            break;
        }
        let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if (*environ.offset(j as isize)).is_null() {
            current_block = 5689001924483802034;
            break;
        }
        buf = uv__strdup(*environ.offset(j as isize));
        if buf.is_null() {
            current_block = 2025861165427811997;
            break;
        }
        ptr = strchr(buf, '=' as i32);
        if ptr.is_null() {
            uv__free(buf as *mut ::core::ffi::c_void);
        } else {
            *ptr = '\0' as i32 as ::core::ffi::c_char;
            envitem = (*envitems).offset(cnt as isize) as *mut uv_env_item_t;
            (*envitem).name = buf;
            (*envitem).value = ptr.offset(1 as ::core::ffi::c_int as isize);
            cnt += 1;
        }
        j += 1;
    }
    match current_block {
        2025861165427811997 => {
            i = 0 as ::core::ffi::c_int;
            while i < cnt {
                envitem = (*envitems).offset(cnt as isize) as *mut uv_env_item_t;
                uv__free((*envitem).name as *mut ::core::ffi::c_void);
                i += 1;
            }
            uv__free(*envitems as *mut ::core::ffi::c_void);
            *envitems = ::core::ptr::null_mut::<uv_env_item_t>();
            *count = 0 as ::core::ffi::c_int;
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        _ => {
            *count = cnt;
            return 0 as ::core::ffi::c_int;
        }
    };
}
pub(crate) unsafe fn uv_os_getenv(
    mut name: *const ::core::ffi::c_char,
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut var: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: size_t = 0;
    if name.is_null() || buffer.is_null() || size.is_null() || *size == 0 as size_t {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    var = getenv(name);
    if var.is_null() {
        return UV_ENOENT as ::core::ffi::c_int;
    }
    len = strlen(var);
    if len >= *size {
        *size = len.wrapping_add(1 as size_t);
        return UV_ENOBUFS as ::core::ffi::c_int;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        var as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    *size = len;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_setenv(
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if name.is_null() || value.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if setenv(name, value, 1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_unsetenv(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    if name.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if unsetenv(name) != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_gethostname(
    mut buffer: *mut ::core::ffi::c_char,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_char; 65] = [0; 65];
    let mut len: size_t = 0;
    if buffer.is_null() || size.is_null() || *size == 0 as size_t {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if gethostname(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 65]>() as size_t,
    ) != 0 as ::core::ffi::c_int
    {
        return -*__errno_location();
    }
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 65]>() as usize).wrapping_sub(1 as usize)
        as usize] = '\0' as i32 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char);
    if len >= *size {
        *size = len.wrapping_add(1 as size_t);
        return UV_ENOBUFS as ::core::ffi::c_int;
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len.wrapping_add(1 as size_t),
    );
    *size = len;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_get_osfhandle(mut fd: ::core::ffi::c_int) -> uv_os_fd_t {
    return fd as uv_os_fd_t;
}
pub(crate) unsafe fn uv_open_osfhandle(mut os_fd: uv_os_fd_t) -> ::core::ffi::c_int {
    return os_fd as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_getpid() -> uv_pid_t {
    return getpid() as uv_pid_t;
}
pub(crate) unsafe fn uv_os_getppid() -> uv_pid_t {
    return getppid() as uv_pid_t;
}
pub(crate) unsafe fn uv_cpumask_size() -> ::core::ffi::c_int {
    return CPU_SETSIZE;
}
pub(crate) unsafe fn uv_os_getpriority(
    mut pid: uv_pid_t,
    mut priority: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    if priority.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    r = getpriority(PRIO_PROCESS, pid as id_t);
    if r == -(1 as ::core::ffi::c_int) && *__errno_location() != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    *priority = r;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_setpriority(
    mut pid: uv_pid_t,
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if priority < UV_PRIORITY_HIGHEST || priority > UV_PRIORITY_LOW {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if setpriority(PRIO_PROCESS, pid as id_t, priority) != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_thread_getpriority(
    mut tid: uv_thread_t,
    mut priority: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut policy: ::core::ffi::c_int = 0;
    let mut param: sched_param = sched_param { sched_priority: 0 };
    let mut pid: pid_t = syscall(SYS_gettid as ::core::ffi::c_long) as pid_t;
    if priority.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    r = pthread_getschedparam(tid as pthread_t, &raw mut policy, &raw mut param);
    if r != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    if SCHED_OTHER == policy && pthread_equal(tid as pthread_t, pthread_self()) != 0 {
        *__errno_location() = 0 as ::core::ffi::c_int;
        r = getpriority(PRIO_PROCESS, pid as id_t);
        if r == -(1 as ::core::ffi::c_int) && *__errno_location() != 0 as ::core::ffi::c_int {
            return -*__errno_location();
        }
        *priority = r;
        return 0 as ::core::ffi::c_int;
    }
    *priority = param.sched_priority;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_nice_for_calling_thread(
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut nice: ::core::ffi::c_int = 0;
    if priority < UV_THREAD_PRIORITY_LOWEST as ::core::ffi::c_int
        || priority > UV_THREAD_PRIORITY_HIGHEST as ::core::ffi::c_int
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    let mut pid: pid_t = syscall(SYS_gettid as ::core::ffi::c_long) as pid_t;
    nice = 0 as ::core::ffi::c_int - priority * 2 as ::core::ffi::c_int;
    r = setpriority(PRIO_PROCESS, pid as id_t, nice);
    if r != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_thread_setpriority(
    mut tid: uv_thread_t,
    mut priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut range: ::core::ffi::c_int = 0;
    let mut prio: ::core::ffi::c_int = 0;
    let mut policy: ::core::ffi::c_int = 0;
    let mut param: sched_param = sched_param { sched_priority: 0 };
    if priority < UV_THREAD_PRIORITY_LOWEST as ::core::ffi::c_int
        || priority > UV_THREAD_PRIORITY_HIGHEST as ::core::ffi::c_int
    {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    r = pthread_getschedparam(tid as pthread_t, &raw mut policy, &raw mut param);
    if r != 0 as ::core::ffi::c_int {
        return -*__errno_location();
    }
    if SCHED_OTHER == policy && pthread_equal(tid as pthread_t, pthread_self()) != 0 {
        return set_nice_for_calling_thread(priority);
    }
    min = sched_get_priority_min(policy);
    max = sched_get_priority_max(policy);
    if min == -(1 as ::core::ffi::c_int) || max == -(1 as ::core::ffi::c_int) {
        return -*__errno_location();
    }
    range = max - min;
    match priority {
        2 => {
            prio = max;
        }
        1 => {
            prio = min + range * 3 as ::core::ffi::c_int / 4 as ::core::ffi::c_int;
        }
        0 => {
            prio = min + range / 2 as ::core::ffi::c_int;
        }
        -1 => {
            prio = min + range / 4 as ::core::ffi::c_int;
        }
        -2 => {
            prio = min;
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    if param.sched_priority != prio {
        param.sched_priority = prio;
        r = pthread_setschedparam(tid as pthread_t, policy, &raw mut param);
        if r != 0 as ::core::ffi::c_int {
            return -*__errno_location();
        }
    }
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_os_uname(mut buffer: *mut uv_utsname_t) -> ::core::ffi::c_int {
    let mut buf: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    let mut r: ::core::ffi::c_int = 0;
    if buffer.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if uname(&raw mut buf) == -(1 as ::core::ffi::c_int) {
        r = -*__errno_location();
    } else {
        r = uv__strscpy(
            &raw mut (*buffer).sysname as *mut ::core::ffi::c_char,
            &raw mut buf.sysname as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
        ) as ::core::ffi::c_int;
        if !(r == UV_E2BIG as ::core::ffi::c_int) {
            r = uv__strscpy(
                &raw mut (*buffer).release as *mut ::core::ffi::c_char,
                &raw mut buf.release as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            ) as ::core::ffi::c_int;
            if !(r == UV_E2BIG as ::core::ffi::c_int) {
                r = uv__strscpy(
                    &raw mut (*buffer).version as *mut ::core::ffi::c_char,
                    &raw mut buf.version as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                ) as ::core::ffi::c_int;
                if !(r == UV_E2BIG as ::core::ffi::c_int) {
                    r = uv__strscpy(
                        &raw mut (*buffer).machine as *mut ::core::ffi::c_char,
                        &raw mut buf.machine as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
                    ) as ::core::ffi::c_int;
                    if !(r == UV_E2BIG as ::core::ffi::c_int) {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    (*buffer).sysname[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*buffer).release[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*buffer).version[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*buffer).machine[0 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn uv__getsockpeername(
    mut handle: *const uv_handle_t,
    mut func: uv__peersockfunc,
    mut name: *mut sockaddr,
    mut namelen: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut socklen: socklen_t = 0;
    let mut fd: uv_os_fd_t = 0;
    let mut r: ::core::ffi::c_int = 0;
    r = uv_fileno(handle, &raw mut fd);
    if r < 0 as ::core::ffi::c_int {
        return r;
    }
    socklen = *namelen as socklen_t;
    if func.expect("non-null function pointer")(fd as ::core::ffi::c_int, name, &raw mut socklen)
        != 0
    {
        return -*__errno_location();
    }
    *namelen = socklen as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_gettimeofday(mut tv: *mut uv_timeval64_t) -> ::core::ffi::c_int {
    let mut time: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if tv.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if gettimeofday(
        &raw mut time,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    ) != 0 as ::core::ffi::c_int
    {
        return -*__errno_location();
    }
    (*tv).tv_sec = time.tv_sec as int64_t;
    (*tv).tv_usec = time.tv_usec as int32_t;
    return 0 as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_sleep(mut msec: ::core::ffi::c_uint) {
    let mut timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rc: ::core::ffi::c_int = 0;
    timeout.tv_sec = msec.wrapping_div(1000 as ::core::ffi::c_uint) as __time_t;
    timeout.tv_nsec = msec
        .wrapping_rem(1000 as ::core::ffi::c_uint)
        .wrapping_mul(1000 as ::core::ffi::c_uint)
        .wrapping_mul(1000 as ::core::ffi::c_uint) as __syscall_slong_t;
    loop {
        rc = nanosleep(&raw mut timeout, &raw mut timeout);
        if !(rc == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
            break;
        }
    }
    '_c2rust_label: {
        if rc == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"rc == 0\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/yans/safelibs/port-libuv/original/src/unix/core.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                1788 as ::core::ffi::c_uint,
                b"void uv_sleep(unsigned int)\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn uv__search_path(
    mut prog: *const ::core::ffi::c_char,
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: *mut size_t,
) -> ::core::ffi::c_int {
    let mut abspath: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut abspath_size: size_t = 0;
    let mut trypath: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut cloned_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut path_env: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut itr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if buf.is_null() || buflen.is_null() || *buflen == 0 as size_t {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    if !strchr(prog, '/' as i32).is_null() {
        if realpath(prog, &raw mut abspath as *mut ::core::ffi::c_char)
            != &raw mut abspath as *mut ::core::ffi::c_char
        {
            return -*__errno_location();
        }
        abspath_size = strlen(&raw mut abspath as *mut ::core::ffi::c_char);
        *buflen = (*buflen).wrapping_sub(1 as size_t);
        if *buflen > abspath_size {
            *buflen = abspath_size;
        }
        memcpy(
            buf as *mut ::core::ffi::c_void,
            &raw mut abspath as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            *buflen,
        );
        *buf.offset(*buflen as isize) = '\0' as i32 as ::core::ffi::c_char;
        return 0 as ::core::ffi::c_int;
    }
    cloned_path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    token = ::core::ptr::null_mut::<::core::ffi::c_char>();
    path_env = getenv(b"PATH\0" as *const u8 as *const ::core::ffi::c_char);
    if path_env.is_null() {
        return UV_EINVAL as ::core::ffi::c_int;
    }
    cloned_path = uv__strdup(path_env);
    if cloned_path.is_null() {
        return UV_ENOMEM as ::core::ffi::c_int;
    }
    token = uv__strtok(
        cloned_path,
        b":\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut itr,
    );
    while !token.is_null() {
        snprintf(
            &raw mut trypath as *mut ::core::ffi::c_char,
            (::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t)
                .wrapping_sub(1 as size_t),
            b"%s/%s\0" as *const u8 as *const ::core::ffi::c_char,
            token,
            prog,
        );
        if realpath(
            &raw mut trypath as *mut ::core::ffi::c_char,
            &raw mut abspath as *mut ::core::ffi::c_char,
        ) == &raw mut abspath as *mut ::core::ffi::c_char
        {
            if access(&raw mut abspath as *mut ::core::ffi::c_char, X_OK) == 0 as ::core::ffi::c_int
            {
                abspath_size = strlen(&raw mut abspath as *mut ::core::ffi::c_char);
                *buflen = (*buflen).wrapping_sub(1 as size_t);
                if *buflen > abspath_size {
                    *buflen = abspath_size;
                }
                memcpy(
                    buf as *mut ::core::ffi::c_void,
                    &raw mut abspath as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    *buflen,
                );
                *buf.offset(*buflen as isize) = '\0' as i32 as ::core::ffi::c_char;
                uv__free(cloned_path as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
        }
        token = uv__strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut itr,
        );
    }
    uv__free(cloned_path as *mut ::core::ffi::c_void);
    return UV_EINVAL as ::core::ffi::c_int;
}
pub(crate) unsafe fn uv_available_parallelism() -> ::core::ffi::c_uint {
    let mut set: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    let mut rc: ::core::ffi::c_long = 0;
    memset(
        &raw mut set as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
    );
    if 0 as ::core::ffi::c_int
        == sched_getaffinity(
            0 as __pid_t,
            ::core::mem::size_of::<cpu_set_t>() as size_t,
            &raw mut set,
        )
    {
        rc = __sched_cpucount(::core::mem::size_of::<cpu_set_t>() as size_t, &raw mut set)
            as ::core::ffi::c_long;
    } else {
        rc = sysconf(_SC_NPROCESSORS_ONLN as ::core::ffi::c_int);
    }
    if rc < 1 as ::core::ffi::c_long {
        rc = 1 as ::core::ffi::c_long;
    }
    return rc as ::core::ffi::c_uint;
}
