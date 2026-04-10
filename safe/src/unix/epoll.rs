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
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fscanf(__stream: *mut FILE, __format: *const ::core::ffi::c_char, ...)
        -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn perror(__s: *const ::core::ffi::c_char);
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn getpagesize() -> ::core::ffi::c_int;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn uv__calloc(count: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__strdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn uv__malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn uv__free(ptr: *mut ::core::ffi::c_void);
    fn uv__metrics_update_idle_time(loop_0: *mut uv_loop_t);
    fn uv__metrics_set_provider_entry_time(loop_0: *mut uv_loop_t);
    fn atoi(__nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn abort() -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn memmem(
        __haystack: *const ::core::ffi::c_void,
        __haystacklen: size_t,
        __needle: *const ::core::ffi::c_void,
        __needlelen: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn uv__close(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn uv__io_init(w: *mut uv__io_t, cb: uv__io_cb, fd: ::core::ffi::c_int);
    fn uv__io_start(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__io_stop(loop_0: *mut uv_loop_t, w: *mut uv__io_t, events: ::core::ffi::c_uint);
    fn uv__slurp(
        filename: *const ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn uv__open_file(path: *const ::core::ffi::c_char) -> *mut FILE;
    fn uv__fs_post(loop_0: *mut uv_loop_t, req: *mut uv_fs_t);
    fn getifaddrs(__ifap: *mut *mut ifaddrs) -> ::core::ffi::c_int;
    fn freeifaddrs(__ifa: *mut ifaddrs);
    fn epoll_create1(__flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn epoll_ctl(
        __epfd: ::core::ffi::c_int,
        __op: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __event: *mut epoll_event,
    ) -> ::core::ffi::c_int;
    fn epoll_pwait(
        __epfd: ::core::ffi::c_int,
        __events: *mut epoll_event,
        __maxevents: ::core::ffi::c_int,
        __timeout: ::core::ffi::c_int,
        __ss: *const __sigset_t,
    ) -> ::core::ffi::c_int;
    fn inotify_init1(__flags: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inotify_add_watch(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __mask: uint32_t,
    ) -> ::core::ffi::c_int;
    fn inotify_rm_watch(__fd: ::core::ffi::c_int, __wd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn mmap(
        __addr: *mut ::core::ffi::c_void,
        __len: size_t,
        __prot: ::core::ffi::c_int,
        __flags: ::core::ffi::c_int,
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
    ) -> *mut ::core::ffi::c_void;
    fn munmap(__addr: *mut ::core::ffi::c_void, __len: size_t) -> ::core::ffi::c_int;
    fn prctl(__option: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn sysinfo(__info: *mut sysinfo) -> ::core::ffi::c_int;
    fn gnu_dev_makedev(__major: ::core::ffi::c_uint, __minor: ::core::ffi::c_uint) -> __dev_t;
    fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __mode_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __clock_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
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
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__queue {
    pub next: *mut uv__queue,
    pub prev: *mut uv__queue,
}
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __u16 = ::core::ffi::c_ushort;
pub type __u32 = ::core::ffi::c_uint;
pub type __kernel_long_t = ::core::ffi::c_long;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
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
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed_0 = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed_0 = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_0 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_0 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_0 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_0 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_0 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_0 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_0 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_0 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_0 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_0 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_0 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_0 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_0 = 236;
pub const _SC_IPV6: C2RustUnnamed_0 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_0 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_0 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_0 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_0 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_0 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_0 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_0 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_0 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_0 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_0 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_0 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_0 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_0 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_0 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_0 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_0 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_0 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_0 = 182;
pub const _SC_TRACE: C2RustUnnamed_0 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_0 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_0 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_0 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_0 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_0 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_0 = 175;
pub const _SC_STREAMS: C2RustUnnamed_0 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_0 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_0 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_0 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_0 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_0 = 169;
pub const _SC_2_PBS: C2RustUnnamed_0 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_0 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_0 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_0 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_0 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_0 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_0 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_0 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_0 = 160;
pub const _SC_SPAWN: C2RustUnnamed_0 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_0 = 158;
pub const _SC_SHELL: C2RustUnnamed_0 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_0 = 156;
pub const _SC_REGEXP: C2RustUnnamed_0 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_0 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_0 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_0 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_0 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_0 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_0 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_0 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_0 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_0 = 146;
pub const _SC_PIPE: C2RustUnnamed_0 = 145;
pub const _SC_FIFO: C2RustUnnamed_0 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_0 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_0 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_0 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_0 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_0 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_0 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_0 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_0 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_0 = 135;
pub const _SC_BASE: C2RustUnnamed_0 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_0 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_0 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_0 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_0 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_0 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_0 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_0 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_0 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_0 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_0 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_0 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_0 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_0 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_0 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_0 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_0 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_0 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_0 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_0 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_0 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_0 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_0 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_0 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_0 = 110;
pub const _SC_NZERO: C2RustUnnamed_0 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_0 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_0 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_0 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_0 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_0 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_0 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_0 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_0 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_0 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_0 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_0 = 98;
pub const _SC_2_UPE: C2RustUnnamed_0 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_0 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_0 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_0 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_0 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_0 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_0 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_0 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_0 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_0 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_0 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_0 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_0 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_0 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_0 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_0 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_0 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_0 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_0 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_0 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_0 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_0 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_0 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_0 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_0 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_0 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_0 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_0 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_0 = 68;
pub const _SC_THREADS: C2RustUnnamed_0 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_0 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_0 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_0 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_0 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_0 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_0 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_0 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_0 = 60;
pub const _SC_SELECT: C2RustUnnamed_0 = 59;
pub const _SC_POLL: C2RustUnnamed_0 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_0 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_0 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_0 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_0 = 54;
pub const _SC_PII: C2RustUnnamed_0 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_0 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_0 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_0 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_0 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_0 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_0 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_0 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_0 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_0 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_0 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_0 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_0 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_0 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_0 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_0 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_0 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_0 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_0 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_0 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_0 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_0 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_0 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_0 = 30;
pub const _SC_VERSION: C2RustUnnamed_0 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_0 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_0 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_0 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_0 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_0 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_0 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_0 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_0 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_0 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_0 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_0 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_0 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_0 = 16;
pub const _SC_FSYNC: C2RustUnnamed_0 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_0 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_0 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_0 = 12;
pub const _SC_TIMERS: C2RustUnnamed_0 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_0 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_0 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_0 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_0 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_0 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_0 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_0 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_0 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_0 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_0 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_0 = 0;
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
pub struct uv_fs_event_s {
    pub data: *mut ::core::ffi::c_void,
    pub loop_0: *mut uv_loop_t,
    pub type_0: uv_handle_type,
    pub close_cb: uv_close_cb,
    pub handle_queue: uv__queue,
    pub u: C2RustUnnamed_8,
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
pub union C2RustUnnamed_8 {
    pub fd: ::core::ffi::c_int,
    pub reserved: [*mut ::core::ffi::c_void; 4],
}
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
pub struct uv_interface_address_s {
    pub name: *mut ::core::ffi::c_char,
    pub phys_addr: [::core::ffi::c_char; 6],
    pub is_internal: ::core::ffi::c_int,
    pub address: C2RustUnnamed_10,
    pub netmask: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub netmask4: sockaddr_in,
    pub netmask6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub address4: sockaddr_in,
    pub address6: sockaddr_in6,
}
pub type uv_interface_address_t = uv_interface_address_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv_metrics_s {
    pub loop_count: uint64_t,
    pub events: uint64_t,
    pub events_waiting: uint64_t,
    pub reserved: [*mut uint64_t; 13],
}
pub type uv_metrics_t = uv_metrics_s;
pub type C2RustUnnamed_11 = ::core::ffi::c_uint;
pub const UV_METRICS_IDLE_TIME: C2RustUnnamed_11 = 1;
pub const UV_LOOP_BLOCK_SIGNAL: C2RustUnnamed_11 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu {
    pub freq: ::core::ffi::c_ulonglong,
    pub user: ::core::ffi::c_ulonglong,
    pub nice: ::core::ffi::c_ulonglong,
    pub sys: ::core::ffi::c_ulonglong,
    pub idle: ::core::ffi::c_ulonglong,
    pub irq: ::core::ffi::c_ulonglong,
    pub model: ::core::ffi::c_uint,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_ll {
    pub sll_family: ::core::ffi::c_ushort,
    pub sll_protocol: ::core::ffi::c_ushort,
    pub sll_ifindex: ::core::ffi::c_int,
    pub sll_hatype: ::core::ffi::c_ushort,
    pub sll_pkttype: ::core::ffi::c_uchar,
    pub sll_halen: ::core::ffi::c_uchar,
    pub sll_addr: [::core::ffi::c_uchar; 8],
}
pub const UV__EXCLUDE_IFPHYS: C2RustUnnamed_16 = 0;
pub const IFF_RUNNING: C2RustUnnamed_21 = 64;
pub const IFF_UP: C2RustUnnamed_21 = 1;
pub const IFF_LOOPBACK: C2RustUnnamed_21 = 8;
pub const UV__EXCLUDE_IFADDR: C2RustUnnamed_16 = 1;
pub type uv_fs_event = ::core::ffi::c_uint;
pub const UV_CHANGE: uv_fs_event = 2;
pub const UV_RENAME: uv_fs_event = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sysinfo {
    pub uptime: __kernel_long_t,
    pub loads: [__kernel_ulong_t; 3],
    pub totalram: __kernel_ulong_t,
    pub freeram: __kernel_ulong_t,
    pub sharedram: __kernel_ulong_t,
    pub bufferram: __kernel_ulong_t,
    pub totalswap: __kernel_ulong_t,
    pub freeswap: __kernel_ulong_t,
    pub procs: __u16,
    pub pad: __u16,
    pub totalhigh: __kernel_ulong_t,
    pub freehigh: __kernel_ulong_t,
    pub mem_unit: __u32,
    pub _f: [::core::ffi::c_char; 0],
}
pub const UV_HANDLE_REF: C2RustUnnamed_14 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct watcher_list {
    pub entry: C2RustUnnamed_13,
    pub watchers: uv__queue,
    pub iterating: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub wd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub rbe_left: *mut watcher_list,
    pub rbe_right: *mut watcher_list,
    pub rbe_parent: *mut watcher_list,
    pub rbe_color: ::core::ffi::c_int,
}
pub const UV_HANDLE_ACTIVE: C2RustUnnamed_14 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct watcher_root {
    pub rbh_root: *mut watcher_list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inotify_event {
    pub wd: ::core::ffi::c_int,
    pub mask: uint32_t,
    pub cookie: uint32_t,
    pub len: uint32_t,
    pub name: [::core::ffi::c_char; 0],
}
pub const IN_CLOEXEC: C2RustUnnamed_22 = 524288;
pub const IN_NONBLOCK: C2RustUnnamed_22 = 2048;
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type C2RustUnnamed_14 = ::core::ffi::c_uint;
pub const UV_HANDLE_REAP: C2RustUnnamed_14 = 268435456;
pub const UV_HANDLE_POLL_SLOW: C2RustUnnamed_14 = 16777216;
pub const UV_SIGNAL_ONE_SHOT: C2RustUnnamed_14 = 33554432;
pub const UV_SIGNAL_ONE_SHOT_DISPATCHED: C2RustUnnamed_14 = 16777216;
pub const UV_HANDLE_TTY_SAVED_ATTRIBUTES: C2RustUnnamed_14 = 134217728;
pub const UV_HANDLE_TTY_SAVED_POSITION: C2RustUnnamed_14 = 67108864;
pub const UV_HANDLE_TTY_RAW: C2RustUnnamed_14 = 33554432;
pub const UV_HANDLE_TTY_READABLE: C2RustUnnamed_14 = 16777216;
pub const UV_HANDLE_PIPESERVER: C2RustUnnamed_14 = 33554432;
pub const UV_HANDLE_NON_OVERLAPPED_PIPE: C2RustUnnamed_14 = 16777216;
pub const UV_HANDLE_UDP_RECVMMSG: C2RustUnnamed_14 = 67108864;
pub const UV_HANDLE_UDP_CONNECTED: C2RustUnnamed_14 = 33554432;
pub const UV_HANDLE_UDP_PROCESSING: C2RustUnnamed_14 = 16777216;
pub const UV_HANDLE_SHARED_TCP_SOCKET: C2RustUnnamed_14 = 268435456;
pub const UV_HANDLE_TCP_ACCEPT_STATE_CHANGING: C2RustUnnamed_14 = 134217728;
pub const UV_HANDLE_TCP_SINGLE_ACCEPT: C2RustUnnamed_14 = 67108864;
pub const UV_HANDLE_TCP_KEEPALIVE: C2RustUnnamed_14 = 33554432;
pub const UV_HANDLE_TCP_NODELAY: C2RustUnnamed_14 = 16777216;
pub const UV_HANDLE_IPV6: C2RustUnnamed_14 = 4194304;
pub const UV_HANDLE_CANCELLATION_PENDING: C2RustUnnamed_14 = 2097152;
pub const UV_HANDLE_BLOCKING_WRITES: C2RustUnnamed_14 = 1048576;
pub const UV_HANDLE_EMULATE_IOCP: C2RustUnnamed_14 = 524288;
pub const UV_HANDLE_ZERO_READ: C2RustUnnamed_14 = 262144;
pub const UV_HANDLE_SYNC_BYPASS_IOCP: C2RustUnnamed_14 = 131072;
pub const UV_HANDLE_READ_PENDING: C2RustUnnamed_14 = 65536;
pub const UV_HANDLE_WRITABLE: C2RustUnnamed_14 = 32768;
pub const UV_HANDLE_READABLE: C2RustUnnamed_14 = 16384;
pub const UV_HANDLE_BOUND: C2RustUnnamed_14 = 8192;
pub const UV_HANDLE_READING: C2RustUnnamed_14 = 4096;
pub const UV_HANDLE_READ_EOF: C2RustUnnamed_14 = 2048;
pub const UV_HANDLE_READ_PARTIAL: C2RustUnnamed_14 = 1024;
pub const UV_HANDLE_SHUT: C2RustUnnamed_14 = 512;
pub const UV_HANDLE_CONNECTION: C2RustUnnamed_14 = 128;
pub const UV_HANDLE_LISTENING: C2RustUnnamed_14 = 64;
pub const UV_HANDLE_ENDGAME_QUEUED: C2RustUnnamed_14 = 32;
pub const UV_HANDLE_INTERNAL: C2RustUnnamed_14 = 16;
pub const UV_HANDLE_CLOSED: C2RustUnnamed_14 = 2;
pub const UV_HANDLE_CLOSING: C2RustUnnamed_14 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__loop_metrics_s {
    pub metrics: uv_metrics_t,
    pub provider_entry_time: uint64_t,
    pub provider_idle_time: uint64_t,
    pub lock: uv_mutex_t,
}
pub type uv__loop_metrics_t = uv__loop_metrics_s;
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
pub type uv__loop_internal_fields_t = uv__loop_internal_fields_s;
pub type C2RustUnnamed_15 = ::core::ffi::c_uint;
pub const UV_LOOP_REAP_CHILDREN: C2RustUnnamed_15 = 2;
pub const UV_LOOP_BLOCK_SIGPROF: C2RustUnnamed_15 = 1;
pub type C2RustUnnamed_16 = ::core::ffi::c_uint;
pub type uv_clocktype_t = ::core::ffi::c_uint;
pub const UV_CLOCK_FAST: uv_clocktype_t = 1;
pub const UV_CLOCK_PRECISE: uv_clocktype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__statx_timestamp {
    pub tv_sec: int64_t,
    pub tv_nsec: uint32_t,
    pub unused0: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__statx {
    pub stx_mask: uint32_t,
    pub stx_blksize: uint32_t,
    pub stx_attributes: uint64_t,
    pub stx_nlink: uint32_t,
    pub stx_uid: uint32_t,
    pub stx_gid: uint32_t,
    pub stx_mode: uint16_t,
    pub unused0: uint16_t,
    pub stx_ino: uint64_t,
    pub stx_size: uint64_t,
    pub stx_blocks: uint64_t,
    pub stx_attributes_mask: uint64_t,
    pub stx_atime: uv__statx_timestamp,
    pub stx_btime: uv__statx_timestamp,
    pub stx_ctime: uv__statx_timestamp,
    pub stx_mtime: uv__statx_timestamp,
    pub stx_rdev_major: uint32_t,
    pub stx_rdev_minor: uint32_t,
    pub stx_dev_major: uint32_t,
    pub stx_dev_minor: uint32_t,
    pub unused1: [uint64_t; 14],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut ::core::ffi::c_void,
    pub fd: ::core::ffi::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_uring_sqe {
    pub opcode: uint8_t,
    pub flags: uint8_t,
    pub ioprio: uint16_t,
    pub fd: int32_t,
    pub c2rust_unnamed: C2RustUnnamed_20,
    pub c2rust_unnamed_0: C2RustUnnamed_19,
    pub len: uint32_t,
    pub c2rust_unnamed_1: C2RustUnnamed_18,
    pub user_data: uint64_t,
    pub c2rust_unnamed_2: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub buf_index: uint16_t,
    pub pad: [uint64_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
    pub rw_flags: uint32_t,
    pub fsync_flags: uint32_t,
    pub open_flags: uint32_t,
    pub statx_flags: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub addr: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub off: uint64_t,
    pub addr2: uint64_t,
}
pub const UV__IORING_OP_EPOLL_CTL: C2RustUnnamed_25 = 29;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_uring_cqe {
    pub user_data: uint64_t,
    pub res: int32_t,
    pub flags: uint32_t,
}
pub const UV__IORING_ENTER_GETEVENTS: C2RustUnnamed_26 = 1;
pub const UV__IORING_SQ_CQ_OVERFLOW: C2RustUnnamed_27 = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__invalidate {
    pub prep: *mut [epoll_event; 256],
    pub events: *mut epoll_event,
    pub nfds: ::core::ffi::c_int,
}
pub const UV__MKDIRAT_SYMLINKAT_LINKAT: C2RustUnnamed_28 = 1;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_cqring_offsets {
    pub head: uint32_t,
    pub tail: uint32_t,
    pub ring_mask: uint32_t,
    pub ring_entries: uint32_t,
    pub overflow: uint32_t,
    pub cqes: uint32_t,
    pub reserved0: uint64_t,
    pub reserved1: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_uring_params {
    pub sq_entries: uint32_t,
    pub cq_entries: uint32_t,
    pub flags: uint32_t,
    pub sq_thread_cpu: uint32_t,
    pub sq_thread_idle: uint32_t,
    pub features: uint32_t,
    pub reserved: [uint32_t; 4],
    pub sq_off: uv__io_sqring_offsets,
    pub cq_off: uv__io_cqring_offsets,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uv__io_sqring_offsets {
    pub head: uint32_t,
    pub tail: uint32_t,
    pub ring_mask: uint32_t,
    pub ring_entries: uint32_t,
    pub flags: uint32_t,
    pub dropped: uint32_t,
    pub array: uint32_t,
    pub reserved0: uint32_t,
    pub reserved1: uint64_t,
}
pub const UV__RING_SETUP_SQ_POLL: C2RustUnnamed_23 = 2;
pub const UV__IORING_FEAT_NODROP: C2RustUnnamed_24 = 2;
pub const UV__IORING_FEAT_SINGLE_MMAP: C2RustUnnamed_24 = 1;
pub const UV__IORING_FEAT_RSRC_TAGS: C2RustUnnamed_24 = 1024;
pub const UV__IORING_ENTER_SQ_WAKEUP: C2RustUnnamed_26 = 2;
pub const UV__IORING_SQ_NEED_WAKEUP: C2RustUnnamed_27 = 1;
pub const UV__IORING_OP_CLOSE: C2RustUnnamed_25 = 19;
pub const UV__IORING_OP_FSYNC: C2RustUnnamed_25 = 3;
pub const UV__IORING_OP_LINKAT: C2RustUnnamed_25 = 39;
pub const UV__IORING_OP_MKDIRAT: C2RustUnnamed_25 = 37;
pub const UV__IORING_OP_OPENAT: C2RustUnnamed_25 = 18;
pub const UV__IORING_OP_WRITEV: C2RustUnnamed_25 = 2;
pub const UV__IORING_OP_READV: C2RustUnnamed_25 = 1;
pub const UV__IORING_OP_RENAMEAT: C2RustUnnamed_25 = 35;
pub const UV__IORING_OP_STATX: C2RustUnnamed_25 = 21;
pub const UV__IORING_OP_SYMLINKAT: C2RustUnnamed_25 = 38;
pub const UV__IORING_OP_UNLINKAT: C2RustUnnamed_25 = 36;
pub type C2RustUnnamed_21 = ::core::ffi::c_uint;
pub const IFF_DYNAMIC: C2RustUnnamed_21 = 32768;
pub const IFF_AUTOMEDIA: C2RustUnnamed_21 = 16384;
pub const IFF_PORTSEL: C2RustUnnamed_21 = 8192;
pub const IFF_MULTICAST: C2RustUnnamed_21 = 4096;
pub const IFF_SLAVE: C2RustUnnamed_21 = 2048;
pub const IFF_MASTER: C2RustUnnamed_21 = 1024;
pub const IFF_ALLMULTI: C2RustUnnamed_21 = 512;
pub const IFF_PROMISC: C2RustUnnamed_21 = 256;
pub const IFF_NOARP: C2RustUnnamed_21 = 128;
pub const IFF_NOTRAILERS: C2RustUnnamed_21 = 32;
pub const IFF_POINTOPOINT: C2RustUnnamed_21 = 16;
pub const IFF_DEBUG: C2RustUnnamed_21 = 4;
pub const IFF_BROADCAST: C2RustUnnamed_21 = 2;
pub type C2RustUnnamed_22 = ::core::ffi::c_uint;
pub type C2RustUnnamed_23 = ::core::ffi::c_uint;
pub type C2RustUnnamed_24 = ::core::ffi::c_uint;
pub type C2RustUnnamed_25 = ::core::ffi::c_uint;
pub type C2RustUnnamed_26 = ::core::ffi::c_uint;
pub type C2RustUnnamed_27 = ::core::ffi::c_uint;
pub type C2RustUnnamed_28 = ::core::ffi::c_uint;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const EOPNOTSUPP: ::core::ffi::c_int = 95 as ::core::ffi::c_int;
pub const EOWNERDEAD: ::core::ffi::c_int = 130 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const AT_FDCWD: ::core::ffi::c_int = -(100 as ::core::ffi::c_int);
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PF_PACKET: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const SIGPROF: ::core::ffi::c_int = 27 as ::core::ffi::c_int;
pub const __IOV_MAX: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const IOV_MAX: ::core::ffi::c_int = __IOV_MAX;
pub const RB_BLACK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const RB_RED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RB_NEGINF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const CLOCK_MONOTONIC_COARSE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const CLOCK_BOOTTIME: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
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
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLPRI: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const POLLOUT: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const POLLRDHUP: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const MAP_POPULATE: ::core::ffi::c_int = 0x8000 as ::core::ffi::c_int;
pub const UV__POLLRDHUP: ::core::ffi::c_int = POLLRDHUP;
pub const UV__POLLPRI: ::core::ffi::c_int = POLLPRI;
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__update_time(mut loop_0: *mut uv_loop_t) {
    unsafe {
        (*loop_0).time = uv__hrtime(UV_CLOCK_FAST).wrapping_div(1000000 as uint64_t);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__basename_r(mut path: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        s = strrchr(path, '/' as i32);
        if s.is_null() {
            return path as *mut ::core::ffi::c_char;
        }
        return s.offset(1 as ::core::ffi::c_int as isize);
    }
}
pub const PROT_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PROT_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const MAP_SHARED: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const EPOLL_CTL_ADD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EPOLL_CTL_DEL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EPOLL_CTL_MOD: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IN_MODIFY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const IN_ATTRIB: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const IN_MOVED_FROM: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const IN_MOVED_TO: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const IN_CREATE: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const IN_DELETE: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const IN_DELETE_SELF: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const IN_MOVE_SELF: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const __NR_getrandom: ::core::ffi::c_int = 318 as ::core::ffi::c_int;
pub const __NR_copy_file_range: ::core::ffi::c_int = 326 as ::core::ffi::c_int;
pub const __NR_statx: ::core::ffi::c_int = 332 as ::core::ffi::c_int;
pub const __NR_ring_setup: ::core::ffi::c_int = 425 as ::core::ffi::c_int;
pub const __NR_ring_enter: ::core::ffi::c_int = 426 as ::core::ffi::c_int;
pub const __NR_io_uring_register: ::core::ffi::c_int = 427 as ::core::ffi::c_int;
pub const MAP_FAILED: *mut ::core::ffi::c_void =
    -(1 as ::core::ffi::c_int) as *mut ::core::ffi::c_void;
pub const PR_SET_NAME: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_INSERT_COLOR(
    mut head: *mut watcher_root,
    mut elm: *mut watcher_list,
) {
    unsafe {
        let mut parent: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut gparent: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut tmp: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        loop {
            parent = (*elm).entry.rbe_parent;
            if !(!parent.is_null() && (*parent).entry.rbe_color == RB_RED) {
                break;
            }
            gparent = (*parent).entry.rbe_parent;
            if parent == (*gparent).entry.rbe_left {
                tmp = (*gparent).entry.rbe_right;
                if !tmp.is_null() && (*tmp).entry.rbe_color == RB_RED {
                    (*tmp).entry.rbe_color = RB_BLACK;
                    (*parent).entry.rbe_color = RB_BLACK;
                    (*gparent).entry.rbe_color = RB_RED;
                    elm = gparent;
                } else {
                    if (*parent).entry.rbe_right == elm {
                        tmp = (*parent).entry.rbe_right;
                        (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                        if !(*parent).entry.rbe_right.is_null() {
                            (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                        }
                        (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                                (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                            } else {
                                (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                            }
                        } else {
                            (*head).rbh_root = tmp;
                        }
                        (*tmp).entry.rbe_left = parent;
                        (*parent).entry.rbe_parent = tmp;
                        !(*tmp).entry.rbe_parent.is_null();
                        tmp = parent;
                        parent = elm;
                        elm = tmp;
                    }
                    (*parent).entry.rbe_color = RB_BLACK;
                    (*gparent).entry.rbe_color = RB_RED;
                    tmp = (*gparent).entry.rbe_left;
                    (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*gparent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent;
                    }
                    (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                            (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_right = gparent;
                    (*gparent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                }
            } else {
                tmp = (*gparent).entry.rbe_left;
                if !tmp.is_null() && (*tmp).entry.rbe_color == RB_RED {
                    (*tmp).entry.rbe_color = RB_BLACK;
                    (*parent).entry.rbe_color = RB_BLACK;
                    (*gparent).entry.rbe_color = RB_RED;
                    elm = gparent;
                } else {
                    if (*parent).entry.rbe_left == elm {
                        tmp = (*parent).entry.rbe_left;
                        (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                        if !(*parent).entry.rbe_left.is_null() {
                            (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                        }
                        (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                        if !(*tmp).entry.rbe_parent.is_null() {
                            if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                                (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                            } else {
                                (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                            }
                        } else {
                            (*head).rbh_root = tmp;
                        }
                        (*tmp).entry.rbe_right = parent;
                        (*parent).entry.rbe_parent = tmp;
                        !(*tmp).entry.rbe_parent.is_null();
                        tmp = parent;
                        parent = elm;
                        elm = tmp;
                    }
                    (*parent).entry.rbe_color = RB_BLACK;
                    (*gparent).entry.rbe_color = RB_RED;
                    tmp = (*gparent).entry.rbe_right;
                    (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*gparent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent;
                    }
                    (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                            (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_left = gparent;
                    (*gparent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                }
            }
        }
        (*(*head).rbh_root).entry.rbe_color = RB_BLACK;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_MINMAX(
    mut head: *mut watcher_root,
    mut val: ::core::ffi::c_int,
) -> *mut watcher_list {
    unsafe {
        let mut tmp: *mut watcher_list = (*head).rbh_root;
        let mut parent: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        while !tmp.is_null() {
            parent = tmp;
            if val < 0 as ::core::ffi::c_int {
                tmp = (*tmp).entry.rbe_left;
            } else {
                tmp = (*tmp).entry.rbe_right;
            }
        }
        return parent;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_REMOVE_COLOR(
    mut head: *mut watcher_root,
    mut parent: *mut watcher_list,
    mut elm: *mut watcher_list,
) {
    unsafe {
        let mut tmp: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        while (elm.is_null() || (*elm).entry.rbe_color == RB_BLACK) && elm != (*head).rbh_root {
            if (*parent).entry.rbe_left == elm {
                tmp = (*parent).entry.rbe_right;
                if (*tmp).entry.rbe_color == RB_RED {
                    (*tmp).entry.rbe_color = RB_BLACK;
                    (*parent).entry.rbe_color = RB_RED;
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_right;
                }
                if ((*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == RB_BLACK)
                    && ((*tmp).entry.rbe_right.is_null()
                        || (*(*tmp).entry.rbe_right).entry.rbe_color == RB_BLACK)
                {
                    (*tmp).entry.rbe_color = RB_RED;
                    elm = parent;
                    parent = (*elm).entry.rbe_parent;
                } else {
                    if (*tmp).entry.rbe_right.is_null()
                        || (*(*tmp).entry.rbe_right).entry.rbe_color == RB_BLACK
                    {
                        let mut oleft: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
                        oleft = (*tmp).entry.rbe_left;
                        if !oleft.is_null() {
                            (*oleft).entry.rbe_color = RB_BLACK;
                        }
                        (*tmp).entry.rbe_color = RB_RED;
                        oleft = (*tmp).entry.rbe_left;
                        (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                        if !(*tmp).entry.rbe_left.is_null() {
                            (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp;
                        }
                        (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                        if !(*oleft).entry.rbe_parent.is_null() {
                            if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                                (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft;
                            } else {
                                (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft;
                            }
                        } else {
                            (*head).rbh_root = oleft;
                        }
                        (*oleft).entry.rbe_right = tmp;
                        (*tmp).entry.rbe_parent = oleft;
                        !(*oleft).entry.rbe_parent.is_null();
                        tmp = (*parent).entry.rbe_right;
                    }
                    (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                    (*parent).entry.rbe_color = RB_BLACK;
                    if !(*tmp).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_color = RB_BLACK;
                    }
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !(*parent).entry.rbe_right.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    elm = (*head).rbh_root;
                    break;
                }
            } else {
                tmp = (*parent).entry.rbe_left;
                if (*tmp).entry.rbe_color == RB_RED {
                    (*tmp).entry.rbe_color = RB_BLACK;
                    (*parent).entry.rbe_color = RB_RED;
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    tmp = (*parent).entry.rbe_left;
                }
                if ((*tmp).entry.rbe_left.is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == RB_BLACK)
                    && ((*tmp).entry.rbe_right.is_null()
                        || (*(*tmp).entry.rbe_right).entry.rbe_color == RB_BLACK)
                {
                    (*tmp).entry.rbe_color = RB_RED;
                    elm = parent;
                    parent = (*elm).entry.rbe_parent;
                } else {
                    if (*tmp).entry.rbe_left.is_null()
                        || (*(*tmp).entry.rbe_left).entry.rbe_color == RB_BLACK
                    {
                        let mut oright: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
                        oright = (*tmp).entry.rbe_right;
                        if !oright.is_null() {
                            (*oright).entry.rbe_color = RB_BLACK;
                        }
                        (*tmp).entry.rbe_color = RB_RED;
                        oright = (*tmp).entry.rbe_right;
                        (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                        if !(*tmp).entry.rbe_right.is_null() {
                            (*(*oright).entry.rbe_left).entry.rbe_parent = tmp;
                        }
                        (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                        if !(*oright).entry.rbe_parent.is_null() {
                            if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                                (*(*tmp).entry.rbe_parent).entry.rbe_left = oright;
                            } else {
                                (*(*tmp).entry.rbe_parent).entry.rbe_right = oright;
                            }
                        } else {
                            (*head).rbh_root = oright;
                        }
                        (*oright).entry.rbe_left = tmp;
                        (*tmp).entry.rbe_parent = oright;
                        !(*oright).entry.rbe_parent.is_null();
                        tmp = (*parent).entry.rbe_left;
                    }
                    (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                    (*parent).entry.rbe_color = RB_BLACK;
                    if !(*tmp).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_color = RB_BLACK;
                    }
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !(*parent).entry.rbe_left.is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !(*tmp).entry.rbe_parent.is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !(*tmp).entry.rbe_parent.is_null();
                    elm = (*head).rbh_root;
                    break;
                }
            }
        }
        if !elm.is_null() {
            (*elm).entry.rbe_color = RB_BLACK;
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_REMOVE(
    mut head: *mut watcher_root,
    mut elm: *mut watcher_list,
) -> *mut watcher_list {
    unsafe {
        let mut current_block: u64;
        let mut child: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut parent: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut old: *mut watcher_list = elm;
        let mut color: ::core::ffi::c_int = 0;
        if (*elm).entry.rbe_left.is_null() {
            child = (*elm).entry.rbe_right;
            current_block = 7245201122033322888;
        } else if (*elm).entry.rbe_right.is_null() {
            child = (*elm).entry.rbe_left;
            current_block = 7245201122033322888;
        } else {
            let mut left: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
            elm = (*elm).entry.rbe_right;
            loop {
                left = (*elm).entry.rbe_left;
                if left.is_null() {
                    break;
                }
                elm = left;
            }
            child = (*elm).entry.rbe_right;
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child;
                } else {
                    (*parent).entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
            if (*elm).entry.rbe_parent == old {
                parent = elm;
            }
            (*elm).entry = (*old).entry;
            if !(*old).entry.rbe_parent.is_null() {
                if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                    (*(*old).entry.rbe_parent).entry.rbe_left = elm;
                } else {
                    (*(*old).entry.rbe_parent).entry.rbe_right = elm;
                }
            } else {
                (*head).rbh_root = elm;
            }
            (*(*old).entry.rbe_left).entry.rbe_parent = elm;
            if !(*old).entry.rbe_right.is_null() {
                (*(*old).entry.rbe_right).entry.rbe_parent = elm;
            }
            if !parent.is_null() {
                left = parent;
                loop {
                    left = (*left).entry.rbe_parent;
                    if left.is_null() {
                        break;
                    }
                }
            }
            current_block = 16785959853902426992;
        }
        match current_block {
            7245201122033322888 => {
                parent = (*elm).entry.rbe_parent;
                color = (*elm).entry.rbe_color;
                if !child.is_null() {
                    (*child).entry.rbe_parent = parent;
                }
                if !parent.is_null() {
                    if (*parent).entry.rbe_left == elm {
                        (*parent).entry.rbe_left = child;
                    } else {
                        (*parent).entry.rbe_right = child;
                    }
                } else {
                    (*head).rbh_root = child;
                }
            }
            _ => {}
        }
        if color == RB_BLACK {
            watcher_root_RB_REMOVE_COLOR(head, parent, child);
        }
        return old;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_INSERT(
    mut head: *mut watcher_root,
    mut elm: *mut watcher_list,
) -> *mut watcher_list {
    unsafe {
        let mut tmp: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut parent: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut comp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        tmp = (*head).rbh_root;
        while !tmp.is_null() {
            parent = tmp;
            comp = compare_watchers(elm, parent);
            if comp < 0 as ::core::ffi::c_int {
                tmp = (*tmp).entry.rbe_left;
            } else if comp > 0 as ::core::ffi::c_int {
                tmp = (*tmp).entry.rbe_right;
            } else {
                return tmp;
            }
        }
        (*elm).entry.rbe_parent = parent;
        (*elm).entry.rbe_right = ::core::ptr::null_mut::<watcher_list>();
        (*elm).entry.rbe_left = (*elm).entry.rbe_right;
        (*elm).entry.rbe_color = RB_RED;
        if !parent.is_null() {
            if comp < 0 as ::core::ffi::c_int {
                (*parent).entry.rbe_left = elm;
            } else {
                (*parent).entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        watcher_root_RB_INSERT_COLOR(head, elm);
        return ::core::ptr::null_mut::<watcher_list>();
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_FIND(
    mut head: *mut watcher_root,
    mut elm: *mut watcher_list,
) -> *mut watcher_list {
    unsafe {
        let mut tmp: *mut watcher_list = (*head).rbh_root;
        let mut comp: ::core::ffi::c_int = 0;
        while !tmp.is_null() {
            comp = compare_watchers(elm, tmp);
            if comp < 0 as ::core::ffi::c_int {
                tmp = (*tmp).entry.rbe_left;
            } else if comp > 0 as ::core::ffi::c_int {
                tmp = (*tmp).entry.rbe_right;
            } else {
                return tmp;
            }
        }
        return ::core::ptr::null_mut::<watcher_list>();
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn watcher_root_RB_NEXT(mut elm: *mut watcher_list) -> *mut watcher_list {
    unsafe {
        if !(*elm).entry.rbe_right.is_null() {
            elm = (*elm).entry.rbe_right;
            while !(*elm).entry.rbe_left.is_null() {
                elm = (*elm).entry.rbe_left;
            }
        } else if !(*elm).entry.rbe_parent.is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
        {
            elm = (*elm).entry.rbe_parent;
        } else {
            while !(*elm).entry.rbe_parent.is_null()
                && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
            {
                elm = (*elm).entry.rbe_parent;
            }
            elm = (*elm).entry.rbe_parent;
        }
        return elm;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__inotify_watchers(mut loop_0: *mut uv_loop_t) -> *mut watcher_root {
    unsafe {
        return &raw mut (*loop_0).inotify_watchers as *mut watcher_root;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__fs_copy_file_range(
    mut fd_in: ::core::ffi::c_int,
    mut off_in: *mut off_t,
    mut fd_out: ::core::ffi::c_int,
    mut off_out: *mut off_t,
    mut len: size_t,
    mut flags: ::core::ffi::c_uint,
) -> ssize_t {
    unsafe {
        return syscall(
            __NR_copy_file_range as ::core::ffi::c_long,
            fd_in,
            off_in,
            fd_out,
            off_out,
            len,
            flags,
        ) as ssize_t;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__statx(
    mut dirfd: ::core::ffi::c_int,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut mask: ::core::ffi::c_uint,
    mut statxbuf: *mut uv__statx,
) -> ::core::ffi::c_int {
    unsafe {
        let mut rc: ::core::ffi::c_int = 0;
        rc = syscall(
            __NR_statx as ::core::ffi::c_long,
            dirfd,
            path,
            flags,
            mask,
            statxbuf,
        ) as ::core::ffi::c_int;
        rc >= 0 as ::core::ffi::c_int;
        return rc;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__getrandom(
    mut buf: *mut ::core::ffi::c_void,
    mut buflen: size_t,
    mut flags: ::core::ffi::c_uint,
) -> ssize_t {
    unsafe {
        let mut rc: ssize_t = 0;
        rc = syscall(__NR_getrandom as ::core::ffi::c_long, buf, buflen, flags) as ssize_t;
        rc >= 0 as ssize_t;
        return rc;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__ring_setup(
    mut entries: ::core::ffi::c_int,
    mut params: *mut uv__io_uring_params,
) -> ::core::ffi::c_int {
    unsafe {
        return syscall(__NR_ring_setup as ::core::ffi::c_long, entries, params)
            as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__ring_enter(
    mut fd: ::core::ffi::c_int,
    mut to_submit: ::core::ffi::c_uint,
    mut min_complete: ::core::ffi::c_uint,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        return syscall(
            __NR_ring_enter as ::core::ffi::c_long,
            fd,
            to_submit,
            min_complete,
            flags,
            NULL,
            0 as ::core::ffi::c_long,
        ) as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__io_uring_register(
    mut fd: ::core::ffi::c_int,
    mut opcode: ::core::ffi::c_uint,
    mut arg: *mut ::core::ffi::c_void,
    mut nargs: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        return syscall(
            __NR_io_uring_register as ::core::ffi::c_long,
            fd,
            opcode,
            arg,
            nargs,
        ) as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__iou_init(
    mut epollfd: ::core::ffi::c_int,
    mut iou: *mut uv__iou,
    mut entries: uint32_t,
    mut flags: uint32_t,
) {
    unsafe {
        let mut current_block: u64;
        let mut params: uv__io_uring_params = uv__io_uring_params {
            sq_entries: 0,
            cq_entries: 0,
            flags: 0,
            sq_thread_cpu: 0,
            sq_thread_idle: 0,
            features: 0,
            reserved: [0; 4],
            sq_off: uv__io_sqring_offsets {
                head: 0,
                tail: 0,
                ring_mask: 0,
                ring_entries: 0,
                flags: 0,
                dropped: 0,
                array: 0,
                reserved0: 0,
                reserved1: 0,
            },
            cq_off: uv__io_cqring_offsets {
                head: 0,
                tail: 0,
                ring_mask: 0,
                ring_entries: 0,
                overflow: 0,
                cqes: 0,
                reserved0: 0,
                reserved1: 0,
            },
        };
        let mut e: epoll_event = epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        };
        let mut cqlen: size_t = 0;
        let mut sqlen: size_t = 0;
        let mut maxlen: size_t = 0;
        let mut sqelen: size_t = 0;
        let mut i: uint32_t = 0;
        let mut sq: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut sqe: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut ringfd: ::core::ffi::c_int = 0;
        sq = MAP_FAILED as *mut ::core::ffi::c_char;
        sqe = MAP_FAILED as *mut ::core::ffi::c_char;
        if uv__use_io_uring() == 0 {
            return;
        }
        memset(
            &raw mut params as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uv__io_uring_params>() as size_t,
        );
        params.flags = flags;
        if flags & UV__RING_SETUP_SQ_POLL as ::core::ffi::c_int as uint32_t != 0 {
            params.sq_thread_idle = 10 as uint32_t;
        }
        ringfd = uv__ring_setup(entries as ::core::ffi::c_int, &raw mut params);
        if ringfd == -(1 as ::core::ffi::c_int) {
            return;
        }
        if !(params.features & UV__IORING_FEAT_RSRC_TAGS as ::core::ffi::c_int as uint32_t == 0) {
            if !(params.features & UV__IORING_FEAT_SINGLE_MMAP as ::core::ffi::c_int as uint32_t
                == 0)
            {
                if !(params.features & UV__IORING_FEAT_NODROP as ::core::ffi::c_int as uint32_t
                    == 0)
                {
                    sqlen = (params.sq_off.array as usize).wrapping_add(
                        (params.sq_entries as usize)
                            .wrapping_mul(::core::mem::size_of::<uint32_t>() as usize),
                    ) as size_t;
                    cqlen = (params.cq_off.cqes as usize).wrapping_add(
                        (params.cq_entries as usize)
                            .wrapping_mul(::core::mem::size_of::<uv__io_uring_cqe>() as usize),
                    ) as size_t;
                    maxlen = if sqlen < cqlen { cqlen } else { sqlen };
                    sqelen = (params.sq_entries as usize)
                        .wrapping_mul(::core::mem::size_of::<uv__io_uring_sqe>() as usize)
                        as size_t;
                    sq = mmap(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        maxlen,
                        PROT_READ | PROT_WRITE,
                        MAP_SHARED | MAP_POPULATE,
                        ringfd,
                        0 as __off64_t,
                    ) as *mut ::core::ffi::c_char;
                    sqe = mmap(
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                        sqelen,
                        PROT_READ | PROT_WRITE,
                        MAP_SHARED | MAP_POPULATE,
                        ringfd,
                        0x10000000 as __off64_t,
                    ) as *mut ::core::ffi::c_char;
                    if !(sq == MAP_FAILED as *mut ::core::ffi::c_char
                        || sqe == MAP_FAILED as *mut ::core::ffi::c_char)
                    {
                        if flags & UV__RING_SETUP_SQ_POLL as ::core::ffi::c_int as uint32_t != 0 {
                            memset(
                                &raw mut e as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<epoll_event>() as size_t,
                            );
                            e.events = POLLIN as uint32_t;
                            e.data.fd = ringfd;
                            if epoll_ctl(epollfd, EPOLL_CTL_ADD, ringfd, &raw mut e) != 0 {
                                current_block = 18074796990203347541;
                            } else {
                                current_block = 14401909646449704462;
                            }
                        } else {
                            current_block = 14401909646449704462;
                        }
                        match current_block {
                            18074796990203347541 => {}
                            _ => {
                                (*iou).sqhead =
                                    sq.offset(params.sq_off.head as isize) as *mut uint32_t;
                                (*iou).sqtail =
                                    sq.offset(params.sq_off.tail as isize) as *mut uint32_t;
                                (*iou).sqmask =
                                    *(sq.offset(params.sq_off.ring_mask as isize) as *mut uint32_t);
                                (*iou).sqarray =
                                    sq.offset(params.sq_off.array as isize) as *mut uint32_t;
                                (*iou).sqflags =
                                    sq.offset(params.sq_off.flags as isize) as *mut uint32_t;
                                (*iou).cqhead =
                                    sq.offset(params.cq_off.head as isize) as *mut uint32_t;
                                (*iou).cqtail =
                                    sq.offset(params.cq_off.tail as isize) as *mut uint32_t;
                                (*iou).cqmask =
                                    *(sq.offset(params.cq_off.ring_mask as isize) as *mut uint32_t);
                                (*iou).sq = sq as *mut ::core::ffi::c_void;
                                (*iou).cqe = sq.offset(params.cq_off.cqes as isize)
                                    as *mut ::core::ffi::c_void;
                                (*iou).sqe = sqe as *mut ::core::ffi::c_void;
                                (*iou).sqlen = sqlen;
                                (*iou).cqlen = cqlen;
                                (*iou).maxlen = maxlen;
                                (*iou).sqelen = sqelen;
                                (*iou).ringfd = ringfd;
                                (*iou).in_flight = 0 as uint32_t;
                                (*iou).flags = 0 as uint32_t;
                                if uv__kernel_version()
                                    >= 0x50f00 as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    (*iou).flags |= UV__MKDIRAT_SYMLINKAT_LINKAT
                                        as ::core::ffi::c_int
                                        as uint32_t;
                                }
                                i = 0 as uint32_t;
                                while i <= (*iou).sqmask {
                                    *(*iou).sqarray.offset(i as isize) = i;
                                    i = i.wrapping_add(1);
                                }
                                return;
                            }
                        }
                    }
                }
            }
        }
        if sq != MAP_FAILED as *mut ::core::ffi::c_char {
            munmap(sq as *mut ::core::ffi::c_void, maxlen);
        }
        if sqe != MAP_FAILED as *mut ::core::ffi::c_char {
            munmap(sqe as *mut ::core::ffi::c_void, sqelen);
        }
        uv__close(ringfd);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__iou_delete(mut iou: *mut uv__iou) {
    unsafe {
        if (*iou).ringfd != -(1 as ::core::ffi::c_int) {
            munmap((*iou).sq, (*iou).maxlen);
            munmap((*iou).sqe, (*iou).sqelen);
            uv__close((*iou).ringfd);
            (*iou).ringfd = -(1 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__platform_loop_init(mut loop_0: *mut uv_loop_t) -> ::core::ffi::c_int {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        lfields = (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
        (*lfields).ctl.ringfd = -(1 as ::core::ffi::c_int);
        (*lfields).iou.ringfd = -(1 as ::core::ffi::c_int);
        (*loop_0).inotify_watchers = NULL;
        (*loop_0).inotify_fd = -(1 as ::core::ffi::c_int);
        (*loop_0).backend_fd = epoll_create1(O_CLOEXEC);
        if (*loop_0).backend_fd == -(1 as ::core::ffi::c_int) {
            return -*__errno_location();
        }
        uv__iou_init(
            (*loop_0).backend_fd,
            &raw mut (*lfields).iou,
            64 as uint32_t,
            UV__RING_SETUP_SQ_POLL as ::core::ffi::c_int as uint32_t,
        );
        uv__iou_init(
            (*loop_0).backend_fd,
            &raw mut (*lfields).ctl,
            256 as uint32_t,
            0 as uint32_t,
        );
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__io_fork(mut loop_0: *mut uv_loop_t) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        let mut root: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        root = (*uv__inotify_watchers(loop_0)).rbh_root;
        uv__close((*loop_0).backend_fd);
        (*loop_0).backend_fd = -(1 as ::core::ffi::c_int);
        uv__platform_loop_delete(loop_0);
        err = uv__platform_loop_init(loop_0);
        if err != 0 {
            return err;
        }
        return uv__inotify_fork(loop_0, root);
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__platform_loop_delete(mut loop_0: *mut uv_loop_t) {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        lfields = (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
        uv__iou_delete(&raw mut (*lfields).ctl);
        uv__iou_delete(&raw mut (*lfields).iou);
        if (*loop_0).inotify_fd != -(1 as ::core::ffi::c_int) {
            uv__io_stop(
                loop_0,
                &raw mut (*loop_0).inotify_read_watcher,
                POLLIN as ::core::ffi::c_uint,
            );
            uv__close((*loop_0).inotify_fd);
            (*loop_0).inotify_fd = -(1 as ::core::ffi::c_int);
        }
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__platform_invalidate_fd(
    mut loop_0: *mut uv_loop_t,
    mut fd: ::core::ffi::c_int,
) {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        let mut inv: *mut uv__invalidate = ::core::ptr::null_mut::<uv__invalidate>();
        let mut dummy: epoll_event = epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        };
        let mut i: ::core::ffi::c_int = 0;
        lfields = (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
        inv = (*lfields).inv as *mut uv__invalidate;
        if !inv.is_null() {
            i = 0 as ::core::ffi::c_int;
            while i < (*inv).nfds {
                if (*(*inv).events.offset(i as isize)).data.fd == fd {
                    (*(*inv).events.offset(i as isize)).data.fd = -(1 as ::core::ffi::c_int);
                }
                i += 1;
            }
        }
        memset(
            &raw mut dummy as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<epoll_event>() as size_t,
        );
        if inv.is_null() {
            epoll_ctl((*loop_0).backend_fd, EPOLL_CTL_DEL, fd, &raw mut dummy);
        } else {
            uv__epoll_ctl_prep(
                (*loop_0).backend_fd,
                &raw mut (*lfields).ctl,
                (*inv).prep,
                EPOLL_CTL_DEL,
                fd,
                &raw mut dummy,
            );
        };
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__io_check_fd(
    mut loop_0: *mut uv_loop_t,
    mut fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut e: epoll_event = epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        };
        let mut rc: ::core::ffi::c_int = 0;
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<epoll_event>() as size_t,
        );
        e.events = POLLIN as uint32_t;
        e.data.fd = -(1 as ::core::ffi::c_int);
        rc = 0 as ::core::ffi::c_int;
        if epoll_ctl((*loop_0).backend_fd, EPOLL_CTL_ADD, fd, &raw mut e) != 0 {
            if *__errno_location() != EEXIST {
                rc = -*__errno_location();
            }
        }
        if rc == 0 as ::core::ffi::c_int {
            if epoll_ctl((*loop_0).backend_fd, EPOLL_CTL_DEL, fd, &raw mut e) != 0 {
                abort();
            }
        }
        return rc;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__iou_get_sqe(
    mut iou: *mut uv__iou,
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> *mut uv__io_uring_sqe {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut head: uint32_t = 0;
        let mut tail: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut slot: uint32_t = 0;
        if (*iou).ringfd == -(1 as ::core::ffi::c_int) {
            return ::core::ptr::null_mut::<uv__io_uring_sqe>();
        }
        head = crate::upstream_support::atomics::atomic_load_acquire_u32(
            (*iou).sqhead as *mut uint32_t,
        );
        tail = *(*iou).sqtail;
        mask = (*iou).sqmask;
        if head & mask == tail.wrapping_add(1 as uint32_t) & mask {
            return ::core::ptr::null_mut::<uv__io_uring_sqe>();
        }
        slot = tail & mask;
        sqe = (*iou).sqe as *mut uv__io_uring_sqe;
        sqe = sqe.offset(slot as isize) as *mut uv__io_uring_sqe;
        memset(
            sqe as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<uv__io_uring_sqe>() as size_t,
        );
        (*sqe).user_data = req as uintptr_t as uint64_t;
        (*req).work_req.loop_0 = loop_0 as *mut uv_loop_s;
        (*req).work_req.work = None;
        (*req).work_req.done = None;
        uv__queue_init(&raw mut (*req).work_req.wq);
        (*loop_0).active_reqs.count = (*loop_0).active_reqs.count.wrapping_add(1);
        (*iou).in_flight = (*iou).in_flight.wrapping_add(1);
        return sqe;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__iou_submit(mut iou: *mut uv__iou) {
    unsafe {
        let mut flags: uint32_t = 0;
        crate::upstream_support::atomics::atomic_store_release_u32(
            (*iou).sqtail as *mut uint32_t,
            (*(*iou).sqtail).wrapping_add(1 as uint32_t),
        );
        flags = crate::upstream_support::atomics::atomic_load_acquire_u32(
            (*iou).sqflags as *mut uint32_t,
        );
        if flags & UV__IORING_SQ_NEED_WAKEUP as ::core::ffi::c_int as uint32_t != 0 {
            if uv__ring_enter(
                (*iou).ringfd,
                0 as ::core::ffi::c_uint,
                0 as ::core::ffi::c_uint,
                UV__IORING_ENTER_SQ_WAKEUP as ::core::ffi::c_int as ::core::ffi::c_uint,
            ) != 0
            {
                if *__errno_location() != EOWNERDEAD {
                    perror(
                        b"libuv: ring_enter(wakeup)\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
        }
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_close(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        let mut kv: ::core::ffi::c_int = 0;
        kv = uv__kernel_version() as ::core::ffi::c_int;
        if kv < 0x50f5a as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if kv >= 0x50a00 as ::core::ffi::c_int && kv < 0x60100 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).fd = (*req).file as int32_t;
        (*sqe).opcode = UV__IORING_OP_CLOSE as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_fsync_or_fdatasync(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
    mut fsync_flags: uint32_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).fd = (*req).file as int32_t;
        (*sqe).c2rust_unnamed_1.fsync_flags = fsync_flags;
        (*sqe).opcode = UV__IORING_OP_FSYNC as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_link(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        if (*iou).flags & UV__MKDIRAT_SYMLINKAT_LINKAT as ::core::ffi::c_int as uint32_t == 0 {
            return 0 as ::core::ffi::c_int;
        }
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).c2rust_unnamed.addr2 = (*req).new_path as uintptr_t as uint64_t;
        (*sqe).len = AT_FDCWD as uint32_t;
        (*sqe).opcode = UV__IORING_OP_LINKAT as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_mkdir(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        if (*iou).flags & UV__MKDIRAT_SYMLINKAT_LINKAT as ::core::ffi::c_int as uint32_t == 0 {
            return 0 as ::core::ffi::c_int;
        }
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).len = (*req).mode as uint32_t;
        (*sqe).opcode = UV__IORING_OP_MKDIRAT as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_open(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).len = (*req).mode as uint32_t;
        (*sqe).opcode = UV__IORING_OP_OPENAT as ::core::ffi::c_int as uint8_t;
        (*sqe).c2rust_unnamed_1.open_flags = ((*req).flags | O_CLOEXEC) as uint32_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_rename(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).c2rust_unnamed.addr2 = (*req).new_path as uintptr_t as uint64_t;
        (*sqe).len = AT_FDCWD as uint32_t;
        (*sqe).opcode = UV__IORING_OP_RENAMEAT as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_symlink(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        if (*iou).flags & UV__MKDIRAT_SYMLINKAT_LINKAT as ::core::ffi::c_int as uint32_t == 0 {
            return 0 as ::core::ffi::c_int;
        }
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).c2rust_unnamed.addr2 = (*req).new_path as uintptr_t as uint64_t;
        (*sqe).opcode = UV__IORING_OP_SYMLINKAT as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_unlink(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).opcode = UV__IORING_OP_UNLINKAT as ::core::ffi::c_int as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_read_or_write(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
    mut is_read: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        if (*req).nbufs > IOV_MAX as ::core::ffi::c_uint {
            if is_read != 0 {
                (*req).nbufs = IOV_MAX as ::core::ffi::c_uint;
            } else {
                return 0 as ::core::ffi::c_int;
            }
        }
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*sqe).c2rust_unnamed_0.addr = (*req).bufs as uintptr_t as uint64_t;
        (*sqe).fd = (*req).file as int32_t;
        (*sqe).len = (*req).nbufs as uint32_t;
        (*sqe).c2rust_unnamed.off = (if (*req).off < 0 as off_t {
            -(1 as ::core::ffi::c_int) as off_t
        } else {
            (*req).off
        }) as uint64_t;
        (*sqe).opcode = (if is_read != 0 {
            UV__IORING_OP_READV as ::core::ffi::c_int
        } else {
            UV__IORING_OP_WRITEV as ::core::ffi::c_int
        }) as uint8_t;
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__iou_fs_statx(
    mut loop_0: *mut uv_loop_t,
    mut req: *mut uv_fs_t,
    mut is_fstat: ::core::ffi::c_int,
    mut is_lstat: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut statxbuf: *mut uv__statx = ::core::ptr::null_mut::<uv__statx>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        statxbuf = uv__malloc(::core::mem::size_of::<uv__statx>() as size_t) as *mut uv__statx;
        if statxbuf.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        iou = &raw mut (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).iou;
        sqe = uv__iou_get_sqe(iou, loop_0, req);
        if sqe.is_null() {
            uv__free(statxbuf as *mut ::core::ffi::c_void);
            return 0 as ::core::ffi::c_int;
        }
        (*req).ptr = statxbuf as *mut ::core::ffi::c_void;
        (*sqe).c2rust_unnamed_0.addr = (*req).path as uintptr_t as uint64_t;
        (*sqe).c2rust_unnamed.addr2 = statxbuf as uintptr_t as uint64_t;
        (*sqe).fd = AT_FDCWD as int32_t;
        (*sqe).len = 0xfff as uint32_t;
        (*sqe).opcode = UV__IORING_OP_STATX as ::core::ffi::c_int as uint8_t;
        if is_fstat != 0 {
            (*sqe).c2rust_unnamed_0.addr =
                b"\0" as *const u8 as *const ::core::ffi::c_char as uintptr_t as uint64_t;
            (*sqe).fd = (*req).file as int32_t;
            (*sqe).c2rust_unnamed_1.statx_flags |= 0x1000 as uint32_t;
        }
        if is_lstat != 0 {
            (*sqe).c2rust_unnamed_1.statx_flags |= AT_SYMLINK_NOFOLLOW as uint32_t;
        }
        uv__iou_submit(iou);
        return 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__statx_to_stat(mut statxbuf: *const uv__statx, mut buf: *mut uv_stat_t) {
    unsafe {
        (*buf).st_dev = gnu_dev_makedev(
            (*statxbuf).stx_dev_major as ::core::ffi::c_uint,
            (*statxbuf).stx_dev_minor as ::core::ffi::c_uint,
        ) as uint64_t;
        (*buf).st_mode = (*statxbuf).stx_mode as uint64_t;
        (*buf).st_nlink = (*statxbuf).stx_nlink as uint64_t;
        (*buf).st_uid = (*statxbuf).stx_uid as uint64_t;
        (*buf).st_gid = (*statxbuf).stx_gid as uint64_t;
        (*buf).st_rdev = gnu_dev_makedev(
            (*statxbuf).stx_rdev_major as ::core::ffi::c_uint,
            (*statxbuf).stx_rdev_minor as ::core::ffi::c_uint,
        ) as uint64_t;
        (*buf).st_ino = (*statxbuf).stx_ino;
        (*buf).st_size = (*statxbuf).stx_size;
        (*buf).st_blksize = (*statxbuf).stx_blksize as uint64_t;
        (*buf).st_blocks = (*statxbuf).stx_blocks;
        (*buf).st_atim.tv_sec = (*statxbuf).stx_atime.tv_sec as ::core::ffi::c_long;
        (*buf).st_atim.tv_nsec = (*statxbuf).stx_atime.tv_nsec as ::core::ffi::c_long;
        (*buf).st_mtim.tv_sec = (*statxbuf).stx_mtime.tv_sec as ::core::ffi::c_long;
        (*buf).st_mtim.tv_nsec = (*statxbuf).stx_mtime.tv_nsec as ::core::ffi::c_long;
        (*buf).st_ctim.tv_sec = (*statxbuf).stx_ctime.tv_sec as ::core::ffi::c_long;
        (*buf).st_ctim.tv_nsec = (*statxbuf).stx_ctime.tv_nsec as ::core::ffi::c_long;
        (*buf).st_birthtim.tv_sec = (*statxbuf).stx_btime.tv_sec as ::core::ffi::c_long;
        (*buf).st_birthtim.tv_nsec = (*statxbuf).stx_btime.tv_nsec as ::core::ffi::c_long;
        (*buf).st_flags = 0 as uint64_t;
        (*buf).st_gen = 0 as uint64_t;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__iou_fs_statx_post(mut req: *mut uv_fs_t) {
    unsafe {
        let mut statxbuf: *mut uv__statx = ::core::ptr::null_mut::<uv__statx>();
        let mut buf: *mut uv_stat_t = ::core::ptr::null_mut::<uv_stat_t>();
        buf = &raw mut (*req).statbuf;
        statxbuf = (*req).ptr as *mut uv__statx;
        (*req).ptr = NULL;
        if (*req).result == 0 as ssize_t {
            uv__statx_to_stat(statxbuf, buf);
            (*req).ptr = buf as *mut ::core::ffi::c_void;
        }
        uv__free(statxbuf as *mut ::core::ffi::c_void);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__poll_io_uring(mut loop_0: *mut uv_loop_t, mut iou: *mut uv__iou) {
    unsafe {
        let mut cqe: *mut uv__io_uring_cqe = ::core::ptr::null_mut::<uv__io_uring_cqe>();
        let mut e: *mut uv__io_uring_cqe = ::core::ptr::null_mut::<uv__io_uring_cqe>();
        let mut req: *mut uv_fs_t = ::core::ptr::null_mut::<uv_fs_t>();
        let mut head: uint32_t = 0;
        let mut tail: uint32_t = 0;
        let mut mask: uint32_t = 0;
        let mut i: uint32_t = 0;
        let mut flags: uint32_t = 0;
        let mut nevents: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        head = *(*iou).cqhead;
        tail = crate::upstream_support::atomics::atomic_load_acquire_u32(
            (*iou).cqtail as *mut uint32_t,
        );
        mask = (*iou).cqmask;
        cqe = (*iou).cqe as *mut uv__io_uring_cqe;
        nevents = 0 as ::core::ffi::c_int;
        i = head;
        while i != tail {
            e = cqe.offset((i & mask) as isize) as *mut uv__io_uring_cqe;
            req = (*e).user_data as uintptr_t as *mut uv_fs_t;
            '_c2rust_label: {
                if (*req).type_0 as ::core::ffi::c_uint
                    == UV_FS as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                } else {
                    __assert_fail(
                        b"req->type == UV_FS\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1156 as ::core::ffi::c_uint,
                        b"void uv__poll_io_uring(uv_loop_t *, struct uv__iou *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if (*loop_0).active_reqs.count > 0 as ::core::ffi::c_uint {
                } else {
                    __assert_fail(
                        b"uv__has_active_reqs(loop)\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1158 as ::core::ffi::c_uint,
                        b"void uv__poll_io_uring(uv_loop_t *, struct uv__iou *)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            (*loop_0).active_reqs.count = (*loop_0).active_reqs.count.wrapping_sub(1);
            (*iou).in_flight = (*iou).in_flight.wrapping_sub(1);
            if (*e).res == -(EOPNOTSUPP as int32_t) {
                uv__fs_post(loop_0, req);
            } else {
                (*req).result = (*e).res as ssize_t;
                match (*req).fs_type as ::core::ffi::c_int {
                    8 | 7 | 6 => {
                        uv__iou_fs_statx_post(req);
                    }
                    _ => {}
                }
                uv__metrics_update_idle_time(loop_0);
                (*req).cb.expect("non-null function pointer")(req);
                nevents += 1;
            }
            i = i.wrapping_add(1);
        }
        crate::upstream_support::atomics::atomic_store_release_u32(
            (*iou).cqhead as *mut uint32_t,
            tail,
        );
        flags = crate::upstream_support::atomics::atomic_load_acquire_u32(
            (*iou).sqflags as *mut uint32_t,
        );
        if flags & UV__IORING_SQ_CQ_OVERFLOW as ::core::ffi::c_int as uint32_t != 0 {
            loop {
                rc = uv__ring_enter(
                    (*iou).ringfd,
                    0 as ::core::ffi::c_uint,
                    0 as ::core::ffi::c_uint,
                    UV__IORING_ENTER_GETEVENTS as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                if !(rc == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                    break;
                }
            }
            if rc < 0 as ::core::ffi::c_int {
                perror(
                    b"libuv: ring_enter(getevents)\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
        }
        let ref mut fresh7 = (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t))
            .loop_metrics
            .metrics
            .events;
        *fresh7 = (*fresh7).wrapping_add(nevents as uint64_t);
        if (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t)).current_timeout
            == 0 as ::core::ffi::c_int
        {
            let ref mut fresh8 = (*((*loop_0).internal_fields as *mut uv__loop_internal_fields_t))
                .loop_metrics
                .metrics
                .events_waiting;
            *fresh8 = (*fresh8).wrapping_add(nevents as uint64_t);
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__epoll_ctl_prep(
    mut epollfd: ::core::ffi::c_int,
    mut ctl: *mut uv__iou,
    mut events: *mut [epoll_event; 256],
    mut op: ::core::ffi::c_int,
    mut fd: ::core::ffi::c_int,
    mut e: *mut epoll_event,
) {
    unsafe {
        let mut sqe: *mut uv__io_uring_sqe = ::core::ptr::null_mut::<uv__io_uring_sqe>();
        let mut pe: *mut epoll_event = ::core::ptr::null_mut::<epoll_event>();
        let mut mask: uint32_t = 0;
        let mut slot: uint32_t = 0;
        if (*ctl).ringfd == -(1 as ::core::ffi::c_int) {
            if epoll_ctl(epollfd, op, fd, e) == 0 {
                return;
            }
            if op == EPOLL_CTL_DEL {
                return;
            }
            if op != EPOLL_CTL_ADD {
                abort();
            }
            if *__errno_location() != EEXIST {
                abort();
            }
            if epoll_ctl(epollfd, EPOLL_CTL_MOD, fd, e) == 0 {
                return;
            }
            abort();
        } else {
            mask = (*ctl).sqmask;
            let fresh6 = *(*ctl).sqtail;
            *(*ctl).sqtail = (*(*ctl).sqtail).wrapping_add(1);
            slot = fresh6 & mask;
            pe = (&raw mut *events as *mut epoll_event).offset(slot as isize) as *mut epoll_event;
            *pe = *e;
            sqe = (*ctl).sqe as *mut uv__io_uring_sqe;
            sqe = sqe.offset(slot as isize) as *mut uv__io_uring_sqe;
            memset(
                sqe as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<uv__io_uring_sqe>() as size_t,
            );
            (*sqe).c2rust_unnamed_0.addr = pe as uintptr_t as uint64_t;
            (*sqe).fd = epollfd as int32_t;
            (*sqe).len = op as uint32_t;
            (*sqe).c2rust_unnamed.off = fd as uint64_t;
            (*sqe).opcode = UV__IORING_OP_EPOLL_CTL as ::core::ffi::c_int as uint8_t;
            (*sqe).user_data = ((op as uint32_t | slot << 2 as ::core::ffi::c_int) as int64_t
                | (fd as int64_t) << 32 as ::core::ffi::c_int)
                as uint64_t;
            if *(*ctl).sqhead & mask == *(*ctl).sqtail & mask {
                uv__epoll_ctl_flush(epollfd, ctl, events);
            }
        };
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__epoll_ctl_flush(
    mut epollfd: ::core::ffi::c_int,
    mut ctl: *mut uv__iou,
    mut events: *mut [epoll_event; 256],
) {
    unsafe {
        let mut oldevents: [epoll_event; 256] = [epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        }; 256];
        let mut cqe: *mut uv__io_uring_cqe = ::core::ptr::null_mut::<uv__io_uring_cqe>();
        let mut oldslot: uint32_t = 0;
        let mut slot: uint32_t = 0;
        let mut n: uint32_t = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0;
        let mut rc: ::core::ffi::c_int = 0;
        '_c2rust_label: {
            if (*ctl).ringfd != -(1 as ::core::ffi::c_int) {
            } else {
                __assert_fail(
                    b"ctl->ringfd != -1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1276 as ::core::ffi::c_uint,
                    b"void uv__epoll_ctl_flush(int, struct uv__iou *, struct epoll_event (*)[256])\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if *(*ctl).sqhead != *(*ctl).sqtail {
            } else {
                __assert_fail(
                    b"*ctl->sqhead != *ctl->sqtail\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1277 as ::core::ffi::c_uint,
                    b"void uv__epoll_ctl_flush(int, struct uv__iou *, struct epoll_event (*)[256])\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        n = (*(*ctl).sqtail).wrapping_sub(*(*ctl).sqhead);
        loop {
            rc = uv__ring_enter(
                (*ctl).ringfd,
                n as ::core::ffi::c_uint,
                n as ::core::ffi::c_uint,
                UV__IORING_ENTER_GETEVENTS as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            if !(rc == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if rc < 0 as ::core::ffi::c_int {
            perror(b"libuv: ring_enter(getevents)\0" as *const u8 as *const ::core::ffi::c_char);
        }
        if rc != n as ::core::ffi::c_int {
            abort();
        }
        '_c2rust_label_1: {
            if *(*ctl).sqhead == *(*ctl).sqtail {
            } else {
                __assert_fail(
                    b"*ctl->sqhead == *ctl->sqtail\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1290 as ::core::ffi::c_uint,
                    b"void uv__epoll_ctl_flush(int, struct uv__iou *, struct epoll_event (*)[256])\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            }
        };
        memcpy(
            &raw mut oldevents as *mut epoll_event as *mut ::core::ffi::c_void,
            &raw mut *events as *mut epoll_event as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[epoll_event; 256]>() as size_t,
        );
        while *(*ctl).cqhead != *(*ctl).cqtail {
            let fresh5 = *(*ctl).cqhead;
            *(*ctl).cqhead = (*(*ctl).cqhead).wrapping_add(1);
            slot = fresh5 & (*ctl).cqmask;
            cqe = (*ctl).cqe as *mut uv__io_uring_cqe;
            cqe = cqe.offset(slot as isize) as *mut uv__io_uring_cqe;
            if (*cqe).res == 0 as int32_t {
                continue;
            }
            fd = ((*cqe).user_data >> 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
            op = (3 as uint64_t & (*cqe).user_data) as ::core::ffi::c_int;
            oldslot = (255 as uint64_t & (*cqe).user_data >> 2 as ::core::ffi::c_int) as uint32_t;
            if op == EPOLL_CTL_DEL {
                continue;
            }
            if op != EPOLL_CTL_ADD {
                abort();
            }
            if (*cqe).res != -(EEXIST as int32_t) {
                abort();
            }
            uv__epoll_ctl_prep(
                epollfd,
                ctl,
                events,
                EPOLL_CTL_MOD,
                fd,
                (&raw mut oldevents as *mut epoll_event).offset(oldslot as isize)
                    as *mut epoll_event,
            );
        }
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__io_poll(mut loop_0: *mut uv_loop_t, mut timeout: ::core::ffi::c_int) {
    unsafe {
        let mut lfields: *mut uv__loop_internal_fields_t =
            ::core::ptr::null_mut::<uv__loop_internal_fields_t>();
        let mut events: [epoll_event; 1024] = [epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        }; 1024];
        let mut prep: [epoll_event; 256] = [epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        }; 256];
        let mut inv: uv__invalidate = uv__invalidate {
            prep: ::core::ptr::null_mut::<[epoll_event; 256]>(),
            events: ::core::ptr::null_mut::<epoll_event>(),
            nfds: 0,
        };
        let mut pe: *mut epoll_event = ::core::ptr::null_mut::<epoll_event>();
        let mut e: epoll_event = epoll_event {
            events: 0,
            data: epoll_data {
                ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
        };
        let mut ctl: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        let mut iou: *mut uv__iou = ::core::ptr::null_mut::<uv__iou>();
        let mut real_timeout: ::core::ffi::c_int = 0;
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut w: *mut uv__io_t = ::core::ptr::null_mut::<uv__io_t>();
        let mut sigmask: *mut sigset_t = ::core::ptr::null_mut::<sigset_t>();
        let mut sigset: sigset_t = __sigset_t { __val: [0; 16] };
        let mut base: uint64_t = 0;
        let mut have_iou_events: ::core::ffi::c_int = 0;
        let mut have_signals: ::core::ffi::c_int = 0;
        let mut nevents: ::core::ffi::c_int = 0;
        let mut epollfd: ::core::ffi::c_int = 0;
        let mut count: ::core::ffi::c_int = 0;
        let mut nfds: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut op: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        let mut user_timeout: ::core::ffi::c_int = 0;
        let mut reset_timeout: ::core::ffi::c_int = 0;
        lfields = (*loop_0).internal_fields as *mut uv__loop_internal_fields_t;
        ctl = &raw mut (*lfields).ctl;
        iou = &raw mut (*lfields).iou;
        sigmask = ::core::ptr::null_mut::<sigset_t>();
        if (*loop_0).flags & UV_LOOP_BLOCK_SIGPROF as ::core::ffi::c_int as ::core::ffi::c_ulong
            != 0
        {
            sigemptyset(&raw mut sigset);
            sigaddset(&raw mut sigset, SIGPROF);
            sigmask = &raw mut sigset;
        }
        '_c2rust_label: {
            if timeout >= -(1 as ::core::ffi::c_int) {
            } else {
                __assert_fail(
                    b"timeout >= -1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    1369 as ::core::ffi::c_uint,
                    b"void uv__io_poll(uv_loop_t *, int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        base = (*loop_0).time;
        count = 48 as ::core::ffi::c_int;
        real_timeout = timeout;
        if (*lfields).flags & UV_METRICS_IDLE_TIME as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            reset_timeout = 1 as ::core::ffi::c_int;
            user_timeout = timeout;
            timeout = 0 as ::core::ffi::c_int;
        } else {
            reset_timeout = 0 as ::core::ffi::c_int;
            user_timeout = 0 as ::core::ffi::c_int;
        }
        epollfd = (*loop_0).backend_fd;
        memset(
            &raw mut e as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<epoll_event>() as size_t,
        );
        while uv__queue_empty(&raw mut (*loop_0).watcher_queue) == 0 {
            q = uv__queue_head(&raw mut (*loop_0).watcher_queue);
            w = (q as *mut ::core::ffi::c_char).offset(-(24 as ::core::ffi::c_ulong as isize))
                as *mut uv__io_t;
            uv__queue_remove(q);
            uv__queue_init(q);
            op = EPOLL_CTL_MOD;
            if (*w).events == 0 as ::core::ffi::c_uint {
                op = EPOLL_CTL_ADD;
            }
            (*w).events = (*w).pevents;
            e.events = (*w).pevents as uint32_t;
            e.data.fd = (*w).fd;
            uv__epoll_ctl_prep(epollfd, ctl, &raw mut prep, op, (*w).fd, &raw mut e);
        }
        inv.events = &raw mut events as *mut epoll_event;
        inv.prep = &raw mut prep as *mut [epoll_event; 256];
        inv.nfds = -(1 as ::core::ffi::c_int);
        loop {
            if (*loop_0).nfds == 0 as ::core::ffi::c_uint {
                if (*iou).in_flight == 0 as uint32_t {
                    break;
                }
            }
            if (*ctl).ringfd != -(1 as ::core::ffi::c_int) {
                while *(*ctl).sqhead != *(*ctl).sqtail {
                    uv__epoll_ctl_flush(epollfd, ctl, &raw mut prep);
                }
            }
            if timeout != 0 as ::core::ffi::c_int {
                uv__metrics_set_provider_entry_time(loop_0);
            }
            (*lfields).current_timeout = timeout;
            nfds = epoll_pwait(
                epollfd,
                &raw mut events as *mut epoll_event,
                (::core::mem::size_of::<[epoll_event; 1024]>() as usize)
                    .wrapping_div(::core::mem::size_of::<epoll_event>() as usize)
                    as ::core::ffi::c_int,
                timeout,
                sigmask,
            );
            let mut _saved_errno: ::core::ffi::c_int = *__errno_location();
            uv__update_time(loop_0);
            *__errno_location() = _saved_errno;
            if nfds == -(1 as ::core::ffi::c_int) {
                '_c2rust_label_0: {
                    if *__errno_location() == 4 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"errno == EINTR\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            1441 as ::core::ffi::c_uint,
                            b"void uv__io_poll(uv_loop_t *, int)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
            } else if nfds == 0 as ::core::ffi::c_int {
                '_c2rust_label_1: {
                    if timeout != -(1 as ::core::ffi::c_int) {
                    } else {
                        __assert_fail(
                            b"timeout != -1\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            1444 as ::core::ffi::c_uint,
                            b"void uv__io_poll(uv_loop_t *, int)\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
            if nfds == 0 as ::core::ffi::c_int || nfds == -(1 as ::core::ffi::c_int) {
                if reset_timeout != 0 as ::core::ffi::c_int {
                    timeout = user_timeout;
                    reset_timeout = 0 as ::core::ffi::c_int;
                } else if nfds == 0 as ::core::ffi::c_int {
                    return;
                }
            } else {
                have_iou_events = 0 as ::core::ffi::c_int;
                have_signals = 0 as ::core::ffi::c_int;
                nevents = 0 as ::core::ffi::c_int;
                inv.nfds = nfds;
                (*lfields).inv = &raw mut inv as *mut ::core::ffi::c_void;
                i = 0 as ::core::ffi::c_int;
                while i < nfds {
                    pe = (&raw mut events as *mut epoll_event).offset(i as isize);
                    fd = (*pe).data.fd;
                    if !(fd == -(1 as ::core::ffi::c_int)) {
                        if fd == (*iou).ringfd {
                            uv__poll_io_uring(loop_0, iou);
                            have_iou_events = 1 as ::core::ffi::c_int;
                        } else {
                            '_c2rust_label_2: {
                                if fd >= 0 as ::core::ffi::c_int {
                                } else {
                                    __assert_fail(
                                        b"fd >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                                        b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        1479 as ::core::ffi::c_uint,
                                        b"void uv__io_poll(uv_loop_t *, int)\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            '_c2rust_label_3: {
                                if (fd as ::core::ffi::c_uint) < (*loop_0).nwatchers {
                                } else {
                                    __assert_fail(
                                        b"(unsigned) fd < loop->nwatchers\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                        1480 as ::core::ffi::c_uint,
                                        b"void uv__io_poll(uv_loop_t *, int)\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            };
                            w = *(*loop_0).watchers.offset(fd as isize);
                            if w.is_null() {
                                uv__epoll_ctl_prep(
                                    epollfd,
                                    ctl,
                                    &raw mut prep,
                                    EPOLL_CTL_DEL,
                                    fd,
                                    pe,
                                );
                            } else {
                                (*pe).events = ((*pe).events as ::core::ffi::c_uint
                                    & ((*w).pevents
                                        | POLLERR as ::core::ffi::c_uint
                                        | POLLHUP as ::core::ffi::c_uint))
                                    as uint32_t;
                                if (*pe).events == POLLERR as uint32_t
                                    || (*pe).events == POLLHUP as uint32_t
                                {
                                    (*pe).events = ((*pe).events as ::core::ffi::c_uint
                                        | (*w).pevents
                                            & (POLLIN | POLLOUT | UV__POLLRDHUP | UV__POLLPRI)
                                                as ::core::ffi::c_uint)
                                        as uint32_t;
                                }
                                if (*pe).events != 0 as uint32_t {
                                    if w == &raw mut (*loop_0).signal_io_watcher {
                                        have_signals = 1 as ::core::ffi::c_int;
                                    } else {
                                        uv__metrics_update_idle_time(loop_0);
                                        (*w).cb.expect("non-null function pointer")(
                                            loop_0 as *mut uv_loop_s,
                                            w as *mut uv__io_s,
                                            (*pe).events as ::core::ffi::c_uint,
                                        );
                                    }
                                    nevents += 1;
                                }
                            }
                        }
                    }
                    i += 1;
                }
                let ref mut fresh3 = (*((*loop_0).internal_fields
                    as *mut uv__loop_internal_fields_t))
                    .loop_metrics
                    .metrics
                    .events;
                *fresh3 = (*fresh3).wrapping_add(nevents as uint64_t);
                if reset_timeout != 0 as ::core::ffi::c_int {
                    timeout = user_timeout;
                    reset_timeout = 0 as ::core::ffi::c_int;
                    let ref mut fresh4 = (*((*loop_0).internal_fields
                        as *mut uv__loop_internal_fields_t))
                        .loop_metrics
                        .metrics
                        .events_waiting;
                    *fresh4 = (*fresh4).wrapping_add(nevents as uint64_t);
                }
                if have_signals != 0 as ::core::ffi::c_int {
                    uv__metrics_update_idle_time(loop_0);
                    (*loop_0)
                        .signal_io_watcher
                        .cb
                        .expect("non-null function pointer")(
                        loop_0 as *mut uv_loop_s,
                        &raw mut (*loop_0).signal_io_watcher,
                        POLLIN as ::core::ffi::c_uint,
                    );
                }
                (*lfields).inv = NULL;
                if have_iou_events != 0 as ::core::ffi::c_int {
                    break;
                }
                if have_signals != 0 as ::core::ffi::c_int {
                    break;
                }
                if nevents != 0 as ::core::ffi::c_int {
                    if !(nfds as usize
                        == (::core::mem::size_of::<[epoll_event; 1024]>() as usize)
                            .wrapping_div(::core::mem::size_of::<epoll_event>() as usize)
                        && {
                            count -= 1;
                            count != 0 as ::core::ffi::c_int
                        })
                    {
                        break;
                    }
                    timeout = 0 as ::core::ffi::c_int;
                    continue;
                }
            }
            if timeout == 0 as ::core::ffi::c_int {
                break;
            }
            if timeout == -(1 as ::core::ffi::c_int) {
                continue;
            }
            '_c2rust_label_4: {
                if timeout > 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"timeout > 0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        1571 as ::core::ffi::c_uint,
                        b"void uv__io_poll(uv_loop_t *, int)\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            real_timeout = (real_timeout as uint64_t)
                .wrapping_sub((*loop_0).time.wrapping_sub(base))
                as ::core::ffi::c_int as ::core::ffi::c_int;
            if real_timeout <= 0 as ::core::ffi::c_int {
                break;
            }
            timeout = real_timeout;
        }
        if (*ctl).ringfd != -(1 as ::core::ffi::c_int) {
            while *(*ctl).sqhead != *(*ctl).sqtail {
                uv__epoll_ctl_flush(epollfd, ctl, &raw mut prep);
            }
        }
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__hrtime(mut type_0: uv_clocktype_t) -> uint64_t {
    unsafe {
        static mut fast_clock_id: clock_t = -(1 as ::core::ffi::c_int) as ::core::ffi::c_long;
        let mut t: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut clock_id: clock_t = 0;
        clock_id = CLOCK_MONOTONIC as clock_t;
        if !(type_0 as ::core::ffi::c_uint
            != UV_CLOCK_FAST as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            clock_id =
                crate::upstream_support::atomics::atomic_load_relaxed_long(&raw mut fast_clock_id);
            if !(clock_id != -(1 as ::core::ffi::c_int) as clock_t) {
                clock_id = CLOCK_MONOTONIC as clock_t;
                if 0 as ::core::ffi::c_int == clock_getres(CLOCK_MONOTONIC_COARSE, &raw mut t) {
                    if t.tv_nsec
                        <= (1 as ::core::ffi::c_int
                            * 1000 as ::core::ffi::c_int
                            * 1000 as ::core::ffi::c_int)
                            as __syscall_slong_t
                    {
                        clock_id = CLOCK_MONOTONIC_COARSE as clock_t;
                    }
                }
                crate::upstream_support::atomics::atomic_store_relaxed_long(
                    &raw mut fast_clock_id,
                    clock_id,
                );
            }
        }
        if clock_gettime(clock_id as clockid_t, &raw mut t) != 0 {
            return 0 as uint64_t;
        }
        return (t.tv_sec as uint64_t)
            .wrapping_mul(1e9f64 as uint64_t)
            .wrapping_add(t.tv_nsec as uint64_t);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_resident_set_memory(mut rss: *mut size_t) -> ::core::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut n: ssize_t = 0;
        let mut val: ::core::ffi::c_long = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_int = 0;
        loop {
            fd = open(
                b"/proc/self/stat\0" as *const u8 as *const ::core::ffi::c_char,
                O_RDONLY,
            );
            if !(fd == -(1 as ::core::ffi::c_int) && *__errno_location() == EINTR) {
                break;
            }
        }
        if fd == -(1 as ::core::ffi::c_int) {
            return -*__errno_location();
        }
        loop {
            n = read(
                fd,
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t)
                    .wrapping_sub(1 as size_t),
            );
            if !(n == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
        uv__close(fd);
        if n == -(1 as ::core::ffi::c_int) as ssize_t {
            return -*__errno_location();
        }
        buf[n as usize] = '\0' as i32 as ::core::ffi::c_char;
        s = strchr(&raw mut buf as *mut ::core::ffi::c_char, ' ' as i32);
        if !s.is_null() {
            s = s.offset(1 as ::core::ffi::c_int as isize);
            if !(*s as ::core::ffi::c_int != '(' as i32) {
                s = strchr(s, ')' as i32);
                if !s.is_null() {
                    i = 1 as ::core::ffi::c_int;
                    loop {
                        if !(i <= 22 as ::core::ffi::c_int) {
                            current_block = 6057473163062296781;
                            break;
                        }
                        s = strchr(s.offset(1 as ::core::ffi::c_int as isize), ' ' as i32);
                        if s.is_null() {
                            current_block = 3803716351181952111;
                            break;
                        }
                        i += 1;
                    }
                    match current_block {
                        3803716351181952111 => {}
                        _ => {
                            *__errno_location() = 0 as ::core::ffi::c_int;
                            val = strtol(
                                s,
                                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                                10 as ::core::ffi::c_int,
                            );
                            if !(*__errno_location() != 0 as ::core::ffi::c_int) {
                                if !(val < 0 as ::core::ffi::c_long) {
                                    *rss = (val * getpagesize() as ::core::ffi::c_long) as size_t;
                                    return 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
        return UV_EINVAL as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_uptime(mut uptime: *mut ::core::ffi::c_double) -> ::core::ffi::c_int {
    unsafe {
        let mut now: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut buf: [::core::ffi::c_char; 128] = [0; 128];
        if 0 as ::core::ffi::c_int
            == uv__slurp(
                b"/proc/uptime\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            )
        {
            if 1 as ::core::ffi::c_int
                == sscanf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b"%lf\0" as *const u8 as *const ::core::ffi::c_char,
                    uptime,
                )
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        if clock_gettime(CLOCK_BOOTTIME, &raw mut now) != 0 {
            return -*__errno_location();
        }
        *uptime = now.tv_sec as ::core::ffi::c_double;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_cpu_info(
    mut ci: *mut *mut uv_cpu_info_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut current_block: u64;
        static mut model_marker: [::core::ffi::c_char; 14] = unsafe {
            ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"model name\t: \0")
        };
        static mut parts: [::core::ffi::c_char; 1] =
            unsafe { ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0") };
        let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut found: ::core::ffi::c_int = 0;
        let mut n: ::core::ffi::c_int = 0;
        let mut i: ::core::ffi::c_uint = 0;
        let mut cpu_0: ::core::ffi::c_uint = 0;
        let mut maxcpu: ::core::ffi::c_uint = 0;
        let mut size: ::core::ffi::c_uint = 0;
        let mut skip: ::core::ffi::c_ulonglong = 0;
        let mut cpus: *mut [cpu; 8192] = ::core::ptr::null_mut::<[cpu; 8192]>();
        let mut c: *mut cpu = ::core::ptr::null_mut::<cpu>();
        let mut t: cpu = cpu {
            freq: 0,
            user: 0,
            nice: 0,
            sys: 0,
            idle: 0,
            irq: 0,
            model: 0,
        };
        let mut model: *mut [::core::ffi::c_char; 64] =
            ::core::ptr::null_mut::<[::core::ffi::c_char; 64]>();
        let mut bitmap: [::core::ffi::c_uchar; 1024] = [0; 1024];
        let mut models: [[::core::ffi::c_char; 64]; 8] = [[0; 64]; 8];
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        memset(
            &raw mut bitmap as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_uchar; 1024]>() as size_t,
        );
        memset(
            &raw mut models as *mut [::core::ffi::c_char; 64] as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[[::core::ffi::c_char; 64]; 8]>() as size_t,
        );
        snprintf(
            &raw mut *(&raw mut models as *mut [::core::ffi::c_char; 64])
                as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
        );
        maxcpu = 0 as ::core::ffi::c_uint;
        cpus = uv__calloc(
            (::core::mem::size_of::<[cpu; 8192]>() as size_t)
                .wrapping_div(::core::mem::size_of::<cpu>() as size_t),
            ::core::mem::size_of::<cpu>() as size_t,
        ) as *mut [cpu; 8192];
        if cpus.is_null() {
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        fp = uv__open_file(b"/proc/stat\0" as *const u8 as *const ::core::ffi::c_char);
        if fp.is_null() {
            uv__free(cpus as *mut ::core::ffi::c_void);
            return -*__errno_location();
        }
        if fgets(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            fp,
        )
        .is_null()
        {
            abort();
        }
        loop {
            memset(
                &raw mut t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<cpu>() as size_t,
            );
            n = fscanf(
                fp,
                b"cpu%u %llu %llu %llu %llu %llu %llu\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut cpu_0,
                &raw mut t.user,
                &raw mut t.nice,
                &raw mut t.sys,
                &raw mut t.idle,
                &raw mut skip,
                &raw mut t.irq,
            );
            if n != 7 as ::core::ffi::c_int {
                break;
            }
            if fgets(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
                fp,
            )
            .is_null()
            {
                abort();
            }
            if cpu_0 as usize
                >= (::core::mem::size_of::<[cpu; 8192]>() as usize)
                    .wrapping_div(::core::mem::size_of::<cpu>() as usize)
            {
                continue;
            }
            (*cpus)[cpu_0 as usize] = t;
            bitmap[(cpu_0 >> 3 as ::core::ffi::c_int) as usize] =
                (bitmap[(cpu_0 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << (cpu_0 & 7 as ::core::ffi::c_uint))
                    as ::core::ffi::c_uchar;
            if cpu_0 >= maxcpu {
                maxcpu = cpu_0.wrapping_add(1 as ::core::ffi::c_uint);
            }
        }
        fclose(fp);
        fp = uv__open_file(b"/proc/cpuinfo\0" as *const u8 as *const ::core::ffi::c_char);
        if fp.is_null() {
            current_block = 14217021294996891811;
        } else {
            current_block = 4761528863920922185;
        }
        loop {
            match current_block {
                14217021294996891811 => {
                    n = 0 as ::core::ffi::c_int;
                    break;
                }
                _ => {
                    if 1 as ::core::ffi::c_int
                        != fscanf(
                            fp,
                            b"processor\t: %u\n\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut cpu_0,
                        )
                    {
                        fclose(fp);
                        fp = ::core::ptr::null_mut::<FILE>();
                        current_block = 14217021294996891811;
                    } else {
                        found = 0 as ::core::ffi::c_int;
                        while found == 0
                            && !fgets(
                                &raw mut buf as *mut ::core::ffi::c_char,
                                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                                    as ::core::ffi::c_int,
                                fp,
                            )
                            .is_null()
                        {
                            found = (strncmp(
                                &raw mut buf as *mut ::core::ffi::c_char,
                                &raw const model_marker as *const ::core::ffi::c_char,
                                (::core::mem::size_of::<[::core::ffi::c_char; 14]>() as size_t)
                                    .wrapping_sub(1 as size_t),
                            ) == 0) as ::core::ffi::c_int;
                        }
                        if !(found == 0) {
                            p = (&raw mut buf as *mut ::core::ffi::c_char)
                                .offset(
                                    ::core::mem::size_of::<[::core::ffi::c_char; 14]>() as usize
                                        as isize,
                                )
                                .offset(-(1 as ::core::ffi::c_int as isize));
                            n = strcspn(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char)
                                as ::core::ffi::c_int;
                            if *(&raw const parts as *const ::core::ffi::c_char) != 0 {
                                p = memmem(
                                    &raw const parts as *const ::core::ffi::c_char
                                        as *const ::core::ffi::c_void,
                                    (::core::mem::size_of::<[::core::ffi::c_char; 1]>() as size_t)
                                        .wrapping_sub(1 as size_t),
                                    p as *const ::core::ffi::c_void,
                                    (n + 1 as ::core::ffi::c_int) as size_t,
                                ) as *mut ::core::ffi::c_char;
                                if p.is_null() {
                                    p = b"unknown\0" as *const u8 as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char;
                                } else {
                                    p = p.offset((n + 1 as ::core::ffi::c_int) as isize);
                                }
                                n = strcspn(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char)
                                    as ::core::ffi::c_int;
                            }
                            found = 0 as ::core::ffi::c_int;
                            model = &raw mut models as *mut [::core::ffi::c_char; 64]
                                as *mut [::core::ffi::c_char; 64];
                            while found == 0
                                && model
                                    < (&raw mut models as *mut [::core::ffi::c_char; 64]).offset(
                                        (::core::mem::size_of::<[[::core::ffi::c_char; 64]; 8]>()
                                            as usize)
                                            .wrapping_div(::core::mem::size_of::<
                                                [::core::ffi::c_char; 64],
                                            >(
                                            )
                                                as usize)
                                            as isize,
                                    )
                            {
                                found = (strncmp(
                                    p,
                                    &raw mut *model as *mut ::core::ffi::c_char,
                                    strlen(&raw mut *model as *mut ::core::ffi::c_char),
                                ) == 0)
                                    as ::core::ffi::c_int;
                                model = model.offset(1);
                            }
                            if !(found == 0) {
                                if *(&raw mut *model as *mut ::core::ffi::c_char)
                                    as ::core::ffi::c_int
                                    == '\0' as i32
                                {
                                    snprintf(
                                        &raw mut *model as *mut ::core::ffi::c_char,
                                        ::core::mem::size_of::<[::core::ffi::c_char; 64]>()
                                            as size_t,
                                        b"%.*s\0" as *const u8 as *const ::core::ffi::c_char,
                                        n,
                                        p,
                                    );
                                }
                                if cpu_0 < maxcpu {
                                    (*cpus)[cpu_0 as usize].model = model.offset_from(
                                        &raw mut models as *mut [::core::ffi::c_char; 64],
                                    )
                                        as ::core::ffi::c_long
                                        as ::core::ffi::c_uint;
                                }
                            }
                        }
                        while !fgets(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                                as ::core::ffi::c_int,
                            fp,
                        )
                        .is_null()
                        {
                            if *(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                                == '\n' as i32
                            {
                                break;
                            }
                        }
                        current_block = 4761528863920922185;
                    }
                }
            }
        }
        cpu_0 = 0 as ::core::ffi::c_uint;
        while cpu_0 < maxcpu {
            if !(bitmap[(cpu_0 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << (cpu_0 & 7 as ::core::ffi::c_uint)
                == 0)
            {
                n += 1;
                snprintf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                    b"/sys/devices/system/cpu/cpu%u/cpufreq/scaling_cur_freq\0" as *const u8
                        as *const ::core::ffi::c_char,
                    cpu_0,
                );
                fp = uv__open_file(&raw mut buf as *mut ::core::ffi::c_char);
                if !fp.is_null() {
                    if 1 as ::core::ffi::c_int
                        != fscanf(
                            fp,
                            b"%llu\0" as *const u8 as *const ::core::ffi::c_char,
                            &raw mut (*(&raw mut *cpus as *mut cpu).offset(cpu_0 as isize)).freq,
                        )
                    {
                        abort();
                    }
                    fclose(fp);
                    fp = ::core::ptr::null_mut::<FILE>();
                }
            }
            cpu_0 = cpu_0.wrapping_add(1);
        }
        size = (n as usize)
            .wrapping_mul(::core::mem::size_of::<uv_cpu_info_t>() as usize)
            .wrapping_add(::core::mem::size_of::<[[::core::ffi::c_char; 64]; 8]>() as usize)
            as ::core::ffi::c_uint;
        *ci = uv__malloc(size as size_t) as *mut uv_cpu_info_t;
        *count = 0 as ::core::ffi::c_int;
        if (*ci).is_null() {
            uv__free(cpus as *mut ::core::ffi::c_void);
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        *count = n;
        p = memcpy(
            (*ci).offset(n as isize) as *mut ::core::ffi::c_void,
            &raw mut models as *mut [::core::ffi::c_char; 64] as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[[::core::ffi::c_char; 64]; 8]>() as size_t,
        ) as *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_uint;
        cpu_0 = 0 as ::core::ffi::c_uint;
        while cpu_0 < maxcpu {
            if !(bitmap[(cpu_0 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << (cpu_0 & 7 as ::core::ffi::c_uint)
                == 0)
            {
                c = (&raw mut *cpus as *mut cpu).offset(cpu_0 as isize);
                let fresh0 = i;
                i = i.wrapping_add(1);
                *(*ci).offset(fresh0 as isize) = uv_cpu_info_s {
                    model: p.offset(
                        ((*c).model as usize).wrapping_mul(::core::mem::size_of::<
                            [::core::ffi::c_char; 64],
                        >() as usize) as isize,
                    ),
                    speed: (*c).freq.wrapping_div(1000 as ::core::ffi::c_ulonglong)
                        as ::core::ffi::c_int,
                    cpu_times: uv_cpu_times_s {
                        user: (10 as ::core::ffi::c_ulonglong).wrapping_mul((*c).user) as uint64_t,
                        nice: (10 as ::core::ffi::c_ulonglong).wrapping_mul((*c).nice) as uint64_t,
                        sys: (10 as ::core::ffi::c_ulonglong).wrapping_mul((*c).sys) as uint64_t,
                        idle: (10 as ::core::ffi::c_ulonglong).wrapping_mul((*c).idle) as uint64_t,
                        irq: (10 as ::core::ffi::c_ulonglong).wrapping_mul((*c).irq) as uint64_t,
                    },
                };
            }
            cpu_0 = cpu_0.wrapping_add(1);
        }
        uv__free(cpus as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__ifaddr_exclude(
    mut ent: *mut ifaddrs,
    mut exclude_type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        if !((*ent).ifa_flags & IFF_UP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            && (*ent).ifa_flags & IFF_RUNNING as ::core::ffi::c_int as ::core::ffi::c_uint != 0)
        {
            return 1 as ::core::ffi::c_int;
        }
        if (*ent).ifa_addr.is_null() {
            return 1 as ::core::ffi::c_int;
        }
        if (*(*ent).ifa_addr).sa_family as ::core::ffi::c_int == PF_PACKET {
            return exclude_type;
        }
        return (exclude_type == 0) as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_interface_addresses(
    mut addresses: *mut *mut uv_interface_address_t,
    mut count: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut addrs: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
        let mut ent: *mut ifaddrs = ::core::ptr::null_mut::<ifaddrs>();
        let mut address: *mut uv_interface_address_t =
            ::core::ptr::null_mut::<uv_interface_address_t>();
        let mut i: ::core::ffi::c_int = 0;
        let mut sll: *mut sockaddr_ll = ::core::ptr::null_mut::<sockaddr_ll>();
        *count = 0 as ::core::ffi::c_int;
        *addresses = ::core::ptr::null_mut::<uv_interface_address_t>();
        if getifaddrs(&raw mut addrs) != 0 {
            return -*__errno_location();
        }
        ent = addrs;
        while !ent.is_null() {
            if !(uv__ifaddr_exclude(ent, UV__EXCLUDE_IFADDR as ::core::ffi::c_int) != 0) {
                *count += 1;
            }
            ent = (*ent).ifa_next;
        }
        if *count == 0 as ::core::ffi::c_int {
            freeifaddrs(addrs);
            return 0 as ::core::ffi::c_int;
        }
        *addresses = uv__calloc(
            *count as size_t,
            ::core::mem::size_of::<uv_interface_address_t>() as size_t,
        ) as *mut uv_interface_address_t;
        if (*addresses).is_null() {
            freeifaddrs(addrs);
            return UV_ENOMEM as ::core::ffi::c_int;
        }
        address = *addresses;
        ent = addrs;
        while !ent.is_null() {
            if !(uv__ifaddr_exclude(ent, UV__EXCLUDE_IFADDR as ::core::ffi::c_int) != 0) {
                (*address).name = uv__strdup((*ent).ifa_name);
                if (*(*ent).ifa_addr).sa_family as ::core::ffi::c_int == AF_INET6 {
                    (*address).address.address6 = *((*ent).ifa_addr as *mut sockaddr_in6);
                } else {
                    (*address).address.address4 = *((*ent).ifa_addr as *mut sockaddr_in);
                }
                if (*(*ent).ifa_netmask).sa_family as ::core::ffi::c_int == AF_INET6 {
                    (*address).netmask.netmask6 = *((*ent).ifa_netmask as *mut sockaddr_in6);
                } else {
                    (*address).netmask.netmask4 = *((*ent).ifa_netmask as *mut sockaddr_in);
                }
                (*address).is_internal = ((*ent).ifa_flags
                    & IFF_LOOPBACK as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0) as ::core::ffi::c_int;
                address = address.offset(1);
            }
            ent = (*ent).ifa_next;
        }
        ent = addrs;
        while !ent.is_null() {
            if !(uv__ifaddr_exclude(ent, UV__EXCLUDE_IFPHYS as ::core::ffi::c_int) != 0) {
                address = *addresses;
                i = 0 as ::core::ffi::c_int;
                while i < *count {
                    let mut namelen: size_t = strlen((*ent).ifa_name);
                    if strncmp((*address).name, (*ent).ifa_name, namelen) == 0 as ::core::ffi::c_int
                        && (*(*address).name.offset(namelen as isize) as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            || *(*address).name.offset(namelen as isize) as ::core::ffi::c_int
                                == ':' as i32)
                    {
                        sll = (*ent).ifa_addr as *mut sockaddr_ll;
                        memcpy(
                            &raw mut (*address).phys_addr as *mut ::core::ffi::c_char
                                as *mut ::core::ffi::c_void,
                            &raw mut (*sll).sll_addr as *mut ::core::ffi::c_uchar
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[::core::ffi::c_char; 6]>() as size_t,
                        );
                    }
                    address = address.offset(1);
                    i += 1;
                }
            }
            ent = (*ent).ifa_next;
        }
        freeifaddrs(addrs);
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_free_interface_addresses(
    mut addresses: *mut uv_interface_address_t,
    mut count: ::core::ffi::c_int,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < count {
            uv__free((*addresses.offset(i as isize)).name as *mut ::core::ffi::c_void);
            i += 1;
        }
        uv__free(addresses as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__set_process_title(mut title: *const ::core::ffi::c_char) {
    unsafe {
        prctl(PR_SET_NAME, title);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__read_proc_meminfo(mut what: *const ::core::ffi::c_char) -> uint64_t {
    unsafe {
        let mut rc: uint64_t = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
        if uv__slurp(
            b"/proc/meminfo\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
        ) != 0
        {
            return 0 as uint64_t;
        }
        p = strstr(&raw mut buf as *mut ::core::ffi::c_char, what);
        if p.is_null() {
            return 0 as uint64_t;
        }
        p = p.offset(strlen(what) as isize);
        rc = 0 as uint64_t;
        sscanf(
            p,
            b"%lu kB\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut rc,
        );
        return rc.wrapping_mul(1024 as uint64_t);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_get_free_memory() -> uint64_t {
    unsafe {
        let mut info: sysinfo = sysinfo {
            uptime: 0,
            loads: [0; 3],
            totalram: 0,
            freeram: 0,
            sharedram: 0,
            bufferram: 0,
            totalswap: 0,
            freeswap: 0,
            procs: 0,
            pad: 0,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 0,
            _f: [0; 0],
        };
        let mut rc: uint64_t = 0;
        rc = uv__read_proc_meminfo(b"MemAvailable:\0" as *const u8 as *const ::core::ffi::c_char);
        if rc != 0 as uint64_t {
            return rc;
        }
        if 0 as ::core::ffi::c_int == sysinfo(&raw mut info) {
            return (info.freeram as uint64_t).wrapping_mul(info.mem_unit as uint64_t);
        }
        return 0 as uint64_t;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_get_total_memory() -> uint64_t {
    unsafe {
        let mut info: sysinfo = sysinfo {
            uptime: 0,
            loads: [0; 3],
            totalram: 0,
            freeram: 0,
            sharedram: 0,
            bufferram: 0,
            totalswap: 0,
            freeswap: 0,
            procs: 0,
            pad: 0,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 0,
            _f: [0; 0],
        };
        let mut rc: uint64_t = 0;
        rc = uv__read_proc_meminfo(b"MemTotal:\0" as *const u8 as *const ::core::ffi::c_char);
        if rc != 0 as uint64_t {
            return rc;
        }
        if 0 as ::core::ffi::c_int == sysinfo(&raw mut info) {
            return (info.totalram as uint64_t).wrapping_mul(info.mem_unit as uint64_t);
        }
        return 0 as uint64_t;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__read_uint64(mut filename: *const ::core::ffi::c_char) -> uint64_t {
    unsafe {
        let mut buf: [::core::ffi::c_char; 32] = [0; 32];
        let mut rc: uint64_t = 0;
        rc = 0 as uint64_t;
        if 0 as ::core::ffi::c_int
            == uv__slurp(
                filename,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t,
            )
        {
            if 1 as ::core::ffi::c_int
                != sscanf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut rc,
                )
            {
                if 0 as ::core::ffi::c_int
                    == strcmp(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"max\n\0" as *const u8 as *const ::core::ffi::c_char,
                    )
                {
                    rc = UINT64_MAX as uint64_t;
                }
            }
        }
        return rc;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__cgroup1_find_memory_controller(
    mut buf: *mut ::core::ffi::c_char,
    mut n: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        p = strchr(buf as *const ::core::ffi::c_char, ':' as i32);
        while !p.is_null()
            && strncmp(
                p,
                b":memory:\0" as *const u8 as *const ::core::ffi::c_char,
                8 as size_t,
            ) != 0
        {
            p = strchr(p, '\n' as i32);
            if !p.is_null() {
                p = strchr(p, ':' as i32);
            }
        }
        if !p.is_null() {
            p = p
                .offset(strlen(b":memory:/\0" as *const u8 as *const ::core::ffi::c_char) as isize);
            *n = strcspn(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char)
                as ::core::ffi::c_int;
        }
        return p;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__get_cgroup1_memory_limits(
    mut buf: *mut ::core::ffi::c_char,
    mut high: *mut uint64_t,
    mut max: *mut uint64_t,
) {
    unsafe {
        let mut current_block: u64;
        let mut filename: [::core::ffi::c_char; 4097] = [0; 4097];
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0;
        let mut cgroup1_max: uint64_t = 0;
        p = uv__cgroup1_find_memory_controller(buf, &raw mut n);
        if !p.is_null() {
            snprintf(
                &raw mut filename as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
                b"/sys/fs/cgroup/memory/%.*s/memory.soft_limit_in_bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
                n,
                p,
            );
            *high = uv__read_uint64(&raw mut filename as *mut ::core::ffi::c_char);
            snprintf(
                &raw mut filename as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
                b"/sys/fs/cgroup/memory/%.*s/memory.limit_in_bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
                n,
                p,
            );
            *max = uv__read_uint64(&raw mut filename as *mut ::core::ffi::c_char);
            if *high != 0 as uint64_t && *max != 0 as uint64_t {
                current_block = 13920661776873878326;
            } else {
                current_block = 4906268039856690917;
            }
        } else {
            current_block = 4906268039856690917;
        }
        match current_block {
            4906268039856690917 => {
                *high = uv__read_uint64(
                    b"/sys/fs/cgroup/memory/memory.soft_limit_in_bytes\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                *max = uv__read_uint64(
                    b"/sys/fs/cgroup/memory/memory.limit_in_bytes\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        cgroup1_max = (LONG_MAX
            & !(sysconf(_SC_PAGESIZE as ::core::ffi::c_int) - 1 as ::core::ffi::c_long))
            as uint64_t;
        if *high == cgroup1_max {
            *high = UINT64_MAX as uint64_t;
        }
        if *max == cgroup1_max {
            *max = UINT64_MAX as uint64_t;
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__get_cgroup2_memory_limits(
    mut buf: *mut ::core::ffi::c_char,
    mut high: *mut uint64_t,
    mut max: *mut uint64_t,
) {
    unsafe {
        let mut filename: [::core::ffi::c_char; 4097] = [0; 4097];
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0;
        p = buf.offset(strlen(b"0::/\0" as *const u8 as *const ::core::ffi::c_char) as isize)
            as *mut ::core::ffi::c_char;
        n = strcspn(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        snprintf(
            &raw mut filename as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
            b"/sys/fs/cgroup/%.*s/memory.max\0" as *const u8 as *const ::core::ffi::c_char,
            n,
            p,
        );
        *max = uv__read_uint64(&raw mut filename as *mut ::core::ffi::c_char);
        snprintf(
            &raw mut filename as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
            b"/sys/fs/cgroup/%.*s/memory.high\0" as *const u8 as *const ::core::ffi::c_char,
            n,
            p,
        );
        *high = uv__read_uint64(&raw mut filename as *mut ::core::ffi::c_char);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__get_cgroup_constrained_memory(mut buf: *mut ::core::ffi::c_char) -> uint64_t {
    unsafe {
        let mut high: uint64_t = 0;
        let mut max: uint64_t = 0;
        if strncmp(
            buf as *const ::core::ffi::c_char,
            b"0::/\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0
        {
            uv__get_cgroup1_memory_limits(buf, &raw mut high, &raw mut max);
        } else {
            uv__get_cgroup2_memory_limits(buf, &raw mut high, &raw mut max);
        }
        if high == 0 as uint64_t || max == 0 as uint64_t {
            return 0 as uint64_t;
        }
        return if high < max { high } else { max };
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_get_constrained_memory() -> uint64_t {
    unsafe {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        if uv__slurp(
            b"/proc/self/cgroup\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        ) != 0
        {
            return 0 as uint64_t;
        }
        return uv__get_cgroup_constrained_memory(&raw mut buf as *mut ::core::ffi::c_char);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__get_cgroup1_current_memory(mut buf: *mut ::core::ffi::c_char) -> uint64_t {
    unsafe {
        let mut filename: [::core::ffi::c_char; 4097] = [0; 4097];
        let mut current: uint64_t = 0;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0;
        p = uv__cgroup1_find_memory_controller(buf, &raw mut n);
        if !p.is_null() {
            snprintf(
                &raw mut filename as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
                b"/sys/fs/cgroup/memory/%.*s/memory.usage_in_bytes\0" as *const u8
                    as *const ::core::ffi::c_char,
                n,
                p,
            );
            current = uv__read_uint64(&raw mut filename as *mut ::core::ffi::c_char);
            if current != 0 as uint64_t {
                return current;
            }
        }
        return uv__read_uint64(
            b"/sys/fs/cgroup/memory/memory.usage_in_bytes\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__get_cgroup2_current_memory(mut buf: *mut ::core::ffi::c_char) -> uint64_t {
    unsafe {
        let mut filename: [::core::ffi::c_char; 4097] = [0; 4097];
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_int = 0;
        p = buf.offset(strlen(b"0::/\0" as *const u8 as *const ::core::ffi::c_char) as isize)
            as *mut ::core::ffi::c_char;
        n = strcspn(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        snprintf(
            &raw mut filename as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
            b"/sys/fs/cgroup/%.*s/memory.current\0" as *const u8 as *const ::core::ffi::c_char,
            n,
            p,
        );
        return uv__read_uint64(&raw mut filename as *mut ::core::ffi::c_char);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_get_available_memory() -> uint64_t {
    unsafe {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut constrained: uint64_t = 0;
        let mut current: uint64_t = 0;
        let mut total: uint64_t = 0;
        if uv__slurp(
            b"/proc/self/cgroup\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
        ) != 0
        {
            return 0 as uint64_t;
        }
        constrained = uv__get_cgroup_constrained_memory(&raw mut buf as *mut ::core::ffi::c_char);
        if constrained == 0 as uint64_t {
            return uv_get_free_memory();
        }
        total = uv_get_total_memory();
        if constrained > total {
            return uv_get_free_memory();
        }
        if strncmp(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"0::/\0" as *const u8 as *const ::core::ffi::c_char,
            4 as size_t,
        ) != 0
        {
            current = uv__get_cgroup1_current_memory(&raw mut buf as *mut ::core::ffi::c_char);
        } else {
            current = uv__get_cgroup2_current_memory(&raw mut buf as *mut ::core::ffi::c_char);
        }
        if constrained < current {
            return 0 as uint64_t;
        }
        return constrained.wrapping_sub(current);
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_loadavg(mut avg: *mut ::core::ffi::c_double) {
    unsafe {
        let mut info: sysinfo = sysinfo {
            uptime: 0,
            loads: [0; 3],
            totalram: 0,
            freeram: 0,
            sharedram: 0,
            bufferram: 0,
            totalswap: 0,
            freeswap: 0,
            procs: 0,
            pad: 0,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 0,
            _f: [0; 0],
        };
        let mut buf: [::core::ffi::c_char; 128] = [0; 128];
        if 0 as ::core::ffi::c_int
            == uv__slurp(
                b"/proc/loadavg\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
            )
        {
            if 3 as ::core::ffi::c_int
                == sscanf(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    b"%lf %lf %lf\0" as *const u8 as *const ::core::ffi::c_char,
                    avg.offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                    avg.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                    avg.offset(2 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_double,
                )
            {
                return;
            }
        }
        if sysinfo(&raw mut info) < 0 as ::core::ffi::c_int {
            return;
        }
        *avg.offset(0 as ::core::ffi::c_int as isize) =
            info.loads[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_double / 65536.0f64;
        *avg.offset(1 as ::core::ffi::c_int as isize) =
            info.loads[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_double / 65536.0f64;
        *avg.offset(2 as ::core::ffi::c_int as isize) =
            info.loads[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_double / 65536.0f64;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn compare_watchers(
    mut a: *const watcher_list,
    mut b: *const watcher_list,
) -> ::core::ffi::c_int {
    unsafe {
        if (*a).wd < (*b).wd {
            return -(1 as ::core::ffi::c_int);
        }
        if (*a).wd > (*b).wd {
            return 1 as ::core::ffi::c_int;
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn init_inotify(mut loop_0: *mut uv_loop_t) -> ::core::ffi::c_int {
    unsafe {
        let mut fd: ::core::ffi::c_int = 0;
        if (*loop_0).inotify_fd != -(1 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
        fd = inotify_init1(IN_NONBLOCK as ::core::ffi::c_int | IN_CLOEXEC as ::core::ffi::c_int);
        if fd < 0 as ::core::ffi::c_int {
            return -*__errno_location();
        }
        (*loop_0).inotify_fd = fd;
        uv__io_init(
            &raw mut (*loop_0).inotify_read_watcher,
            Some(
                uv__inotify_read
                    as unsafe extern "C" fn(
                        *mut uv_loop_t,
                        *mut uv__io_t,
                        ::core::ffi::c_uint,
                    ) -> (),
            ),
            (*loop_0).inotify_fd,
        );
        uv__io_start(
            loop_0,
            &raw mut (*loop_0).inotify_read_watcher,
            POLLIN as ::core::ffi::c_uint,
        );
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__inotify_fork(
    mut loop_0: *mut uv_loop_t,
    mut root: *mut watcher_list,
) -> ::core::ffi::c_int {
    unsafe {
        let mut err: ::core::ffi::c_int = 0;
        let mut tmp_watcher_list_iter: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut watcher_list: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut tmp_watcher_list: watcher_list = watcher_list {
            entry: C2RustUnnamed_13 {
                rbe_left: ::core::ptr::null_mut::<watcher_list>(),
                rbe_right: ::core::ptr::null_mut::<watcher_list>(),
                rbe_parent: ::core::ptr::null_mut::<watcher_list>(),
                rbe_color: 0,
            },
            watchers: uv__queue {
                next: ::core::ptr::null_mut::<uv__queue>(),
                prev: ::core::ptr::null_mut::<uv__queue>(),
            },
            iterating: 0,
            path: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            wd: 0,
        };
        let mut queue: uv__queue = uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        };
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut handle: *mut uv_fs_event_t = ::core::ptr::null_mut::<uv_fs_event_t>();
        let mut tmp_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if root.is_null() {
            return 0 as ::core::ffi::c_int;
        }
        (*loop_0).inotify_watchers = root as *mut ::core::ffi::c_void;
        uv__queue_init(&raw mut tmp_watcher_list.watchers);
        watcher_list = watcher_root_RB_MINMAX(uv__inotify_watchers(loop_0), RB_NEGINF);
        while !watcher_list.is_null() && {
            tmp_watcher_list_iter = watcher_root_RB_NEXT(watcher_list);
            watcher_list != NULL as *mut watcher_list
        } {
            (*watcher_list).iterating = 1 as ::core::ffi::c_int;
            uv__queue_move(&raw mut (*watcher_list).watchers, &raw mut queue);
            while uv__queue_empty(&raw mut queue) == 0 {
                q = uv__queue_head(&raw mut queue);
                handle = (q as *mut ::core::ffi::c_char)
                    .offset(-(112 as ::core::ffi::c_ulong as isize))
                    as *mut uv_fs_event_t;
                tmp_path = uv__strdup((*handle).path);
                '_c2rust_label: {
                    if !tmp_path.is_null() {
                    } else {
                        __assert_fail(
                            b"tmp_path != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            2353 as ::core::ffi::c_uint,
                            b"int uv__inotify_fork(uv_loop_t *, struct watcher_list *)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                uv__queue_remove(q);
                uv__queue_insert_tail(&raw mut (*watcher_list).watchers, q);
                uv_fs_event_stop(handle);
                uv__queue_insert_tail(
                    &raw mut tmp_watcher_list.watchers,
                    &raw mut (*handle).watchers,
                );
                (*handle).path = tmp_path;
            }
            (*watcher_list).iterating = 0 as ::core::ffi::c_int;
            maybe_free_watcher_list(watcher_list, loop_0);
            watcher_list = tmp_watcher_list_iter;
        }
        uv__queue_move(&raw mut tmp_watcher_list.watchers, &raw mut queue);
        while uv__queue_empty(&raw mut queue) == 0 {
            q = uv__queue_head(&raw mut queue);
            uv__queue_remove(q);
            handle = (q as *mut ::core::ffi::c_char).offset(-(112 as ::core::ffi::c_ulong as isize))
                as *mut uv_fs_event_t;
            tmp_path = (*handle).path;
            (*handle).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
            err = uv_fs_event_start(handle, (*handle).cb, tmp_path, 0 as ::core::ffi::c_uint);
            uv__free(tmp_path as *mut ::core::ffi::c_void);
            if err != 0 {
                return err;
            }
        }
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn find_watcher(
    mut loop_0: *mut uv_loop_t,
    mut wd: ::core::ffi::c_int,
) -> *mut watcher_list {
    unsafe {
        let mut w: watcher_list = watcher_list {
            entry: C2RustUnnamed_13 {
                rbe_left: ::core::ptr::null_mut::<watcher_list>(),
                rbe_right: ::core::ptr::null_mut::<watcher_list>(),
                rbe_parent: ::core::ptr::null_mut::<watcher_list>(),
                rbe_color: 0,
            },
            watchers: uv__queue {
                next: ::core::ptr::null_mut::<uv__queue>(),
                prev: ::core::ptr::null_mut::<uv__queue>(),
            },
            iterating: 0,
            path: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            wd: 0,
        };
        w.wd = wd;
        return watcher_root_RB_FIND(uv__inotify_watchers(loop_0), &raw mut w);
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn maybe_free_watcher_list(mut w: *mut watcher_list, mut loop_0: *mut uv_loop_t) {
    unsafe {
        if (*w).iterating == 0 && uv__queue_empty(&raw mut (*w).watchers) != 0 {
            watcher_root_RB_REMOVE(uv__inotify_watchers(loop_0), w);
            inotify_rm_watch((*loop_0).inotify_fd, (*w).wd);
            uv__free(w as *mut ::core::ffi::c_void);
        }
    }
}
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn uv__inotify_read(
    mut loop_0: *mut uv_loop_t,
    mut dummy: *mut uv__io_t,
    mut events: ::core::ffi::c_uint,
) {
    unsafe {
        let mut e: *const inotify_event = ::core::ptr::null::<inotify_event>();
        let mut w: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut h: *mut uv_fs_event_t = ::core::ptr::null_mut::<uv_fs_event_t>();
        let mut queue: uv__queue = uv__queue {
            next: ::core::ptr::null_mut::<uv__queue>(),
            prev: ::core::ptr::null_mut::<uv__queue>(),
        };
        let mut q: *mut uv__queue = ::core::ptr::null_mut::<uv__queue>();
        let mut path: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut size: ssize_t = 0;
        let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
        loop {
            loop {
                size = read(
                    (*loop_0).inotify_fd,
                    &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
                );
                if !(size == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR)
                {
                    break;
                }
            }
            if size == -(1 as ::core::ffi::c_int) as ssize_t {
                '_c2rust_label: {
                    if *__errno_location() == 11 as ::core::ffi::c_int
                        || *__errno_location() == 11 as ::core::ffi::c_int
                    {
                    } else {
                        __assert_fail(
                            b"errno == EAGAIN || errno == EWOULDBLOCK\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            2420 as ::core::ffi::c_uint,
                            b"void uv__inotify_read(uv_loop_t *, uv__io_t *, unsigned int)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                break;
            } else {
                '_c2rust_label_0: {
                    if size > 0 as ssize_t {
                    } else {
                        __assert_fail(
                            b"size > 0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                            2424 as ::core::ffi::c_uint,
                            b"void uv__inotify_read(uv_loop_t *, uv__io_t *, unsigned int)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                p = &raw mut buf as *mut ::core::ffi::c_char;
                while p
                    < (&raw mut buf as *mut ::core::ffi::c_char).offset(size as isize)
                        as *const ::core::ffi::c_char
                {
                    e = p as *const inotify_event;
                    events = 0 as ::core::ffi::c_uint;
                    if (*e).mask & (IN_ATTRIB | IN_MODIFY) as uint32_t != 0 {
                        events |= UV_CHANGE as ::core::ffi::c_int as ::core::ffi::c_uint;
                    }
                    if (*e).mask & !(IN_ATTRIB | IN_MODIFY) as uint32_t != 0 {
                        events |= UV_RENAME as ::core::ffi::c_int as ::core::ffi::c_uint;
                    }
                    w = find_watcher(loop_0, (*e).wd);
                    if !w.is_null() {
                        path = if (*e).len != 0 {
                            e.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_char
                        } else {
                            uv__basename_r((*w).path) as *const ::core::ffi::c_char
                        };
                        (*w).iterating = 1 as ::core::ffi::c_int;
                        uv__queue_move(&raw mut (*w).watchers, &raw mut queue);
                        while uv__queue_empty(&raw mut queue) == 0 {
                            q = uv__queue_head(&raw mut queue);
                            h = (q as *mut ::core::ffi::c_char)
                                .offset(-(112 as ::core::ffi::c_ulong as isize))
                                as *mut uv_fs_event_t;
                            uv__queue_remove(q);
                            uv__queue_insert_tail(&raw mut (*w).watchers, q);
                            (*h).cb.expect("non-null function pointer")(
                                h,
                                path,
                                events as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        (*w).iterating = 0 as ::core::ffi::c_int;
                        maybe_free_watcher_list(w, loop_0);
                    }
                    p = p.offset(
                        (::core::mem::size_of::<inotify_event>() as usize)
                            .wrapping_add((*e).len as usize) as isize,
                    );
                }
            }
        }
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_fs_event_init(
    mut loop_0: *mut uv_loop_t,
    mut handle: *mut uv_fs_event_t,
) -> ::core::ffi::c_int {
    unsafe {
        let ref mut fresh1 = (*(handle as *mut uv_handle_t)).loop_0;
        *fresh1 = loop_0;
        (*(handle as *mut uv_handle_t)).type_0 = UV_FS_EVENT;
        (*(handle as *mut uv_handle_t)).flags =
            UV_HANDLE_REF as ::core::ffi::c_int as ::core::ffi::c_uint;
        uv__queue_insert_tail(
            &raw mut (*loop_0).handle_queue,
            &raw mut (*(handle as *mut uv_handle_t)).handle_queue,
        );
        let ref mut fresh2 = (*(handle as *mut uv_handle_t)).next_closing;
        *fresh2 = ::core::ptr::null_mut::<uv_handle_t>();
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_fs_event_start(
    mut handle: *mut uv_fs_event_t,
    mut cb: uv_fs_event_cb,
    mut path: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    unsafe {
        let mut w: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        let mut loop_0: *mut uv_loop_t = ::core::ptr::null_mut::<uv_loop_t>();
        let mut len: size_t = 0;
        let mut events: ::core::ffi::c_int = 0;
        let mut err: ::core::ffi::c_int = 0;
        let mut wd: ::core::ffi::c_int = 0;
        if (*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint
        {
            return UV_EINVAL as ::core::ffi::c_int;
        }
        loop_0 = (*handle).loop_0;
        err = init_inotify(loop_0);
        if err != 0 {
            return err;
        }
        events = IN_ATTRIB
            | IN_CREATE
            | IN_MODIFY
            | IN_DELETE
            | IN_DELETE_SELF
            | IN_MOVE_SELF
            | IN_MOVED_FROM
            | IN_MOVED_TO;
        wd = inotify_add_watch((*loop_0).inotify_fd, path, events as uint32_t);
        if wd == -(1 as ::core::ffi::c_int) {
            return -*__errno_location();
        }
        w = find_watcher(loop_0, wd);
        if w.is_null() {
            len = strlen(path).wrapping_add(1 as size_t);
            w = uv__malloc((::core::mem::size_of::<watcher_list>() as size_t).wrapping_add(len))
                as *mut watcher_list;
            if w.is_null() {
                return UV_ENOMEM as ::core::ffi::c_int;
            }
            (*w).wd = wd;
            (*w).path = memcpy(
                w.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                path as *const ::core::ffi::c_void,
                len,
            ) as *mut ::core::ffi::c_char;
            uv__queue_init(&raw mut (*w).watchers);
            (*w).iterating = 0 as ::core::ffi::c_int;
            watcher_root_RB_INSERT(uv__inotify_watchers(loop_0), w);
        }
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
        uv__queue_insert_tail(&raw mut (*w).watchers, &raw mut (*handle).watchers);
        (*handle).path = (*w).path;
        (*handle).cb = cb;
        (*handle).wd = wd;
        return 0 as ::core::ffi::c_int;
    }
}
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uv_fs_event_stop(mut handle: *mut uv_fs_event_t) -> ::core::ffi::c_int {
    unsafe {
        let mut w: *mut watcher_list = ::core::ptr::null_mut::<watcher_list>();
        if !((*handle).flags & UV_HANDLE_ACTIVE as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0 as ::core::ffi::c_uint)
        {
            return 0 as ::core::ffi::c_int;
        }
        w = find_watcher((*handle).loop_0, (*handle).wd);
        '_c2rust_label: {
            if !w.is_null() {
            } else {
                __assert_fail(
                    b"w != NULL\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/yans/safelibs/port-libuv/original/src/unix/linux.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    2548 as ::core::ffi::c_uint,
                    b"int uv_fs_event_stop(uv_fs_event_t *)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*handle).wd = -(1 as ::core::ffi::c_int);
        (*handle).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        uv__queue_remove(&raw mut (*handle).watchers);
        maybe_free_watcher_list(w, (*handle).loop_0);
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__fs_event_close(mut handle: *mut uv_fs_event_t) {
    unsafe {
        uv_fs_event_stop(handle);
    }
}
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;

#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__kernel_version() -> ::core::ffi::c_uint {unsafe{0}}

#[no_mangle]
// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
pub extern "C" fn uv__use_io_uring() -> ::core::ffi::c_int {unsafe{0}}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn backend_fd(
    loop_: *const crate::abi::linux_x86_64::uv_loop_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::unix_core::uv_backend_fd(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn backend_timeout(
    loop_: *const crate::abi::linux_x86_64::uv_loop_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::unix_core::uv_backend_timeout(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn close(
    handle: *mut crate::abi::linux_x86_64::uv_handle_t,
    close_cb: crate::abi::linux_x86_64::uv_close_cb,
) {
    unsafe {
        unsafe {
            crate::upstream_support::unix_core::uv_close(
                handle.cast(),
                std::mem::transmute::<_, crate::upstream_support::unix_core::uv_close_cb>(close_cb),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn default_loop() -> *mut crate::abi::linux_x86_64::uv_loop_t {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_default_loop().cast() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn handle_ref(handle: *mut crate::abi::linux_x86_64::uv_handle_t) {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_ref(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn handle_unref(handle: *mut crate::abi::linux_x86_64::uv_handle_t) {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_unref(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn has_ref(
    handle: *const crate::abi::linux_x86_64::uv_handle_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_has_ref(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn is_active(
    handle: *const crate::abi::linux_x86_64::uv_handle_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::unix_core::uv_is_active(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn is_closing(
    handle: *const crate::abi::linux_x86_64::uv_handle_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::unix_core::uv_is_closing(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_alive(
    loop_: *const crate::abi::linux_x86_64::uv_loop_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::unix_core::uv_loop_alive(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_close(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_loop_close(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_configure(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    option: crate::abi::linux_x86_64::uv_loop_option,
    arg: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            crate::upstream_support::uv_common::uv_loop_configure(
                loop_.cast(),
                option as crate::upstream_support::uv_common::uv_loop_option,
                arg,
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_delete(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_loop_delete(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_fork(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::unix::fork::loop_fork(loop_) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_init(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::unix_loop::uv_loop_init(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_new() -> *mut crate::abi::linux_x86_64::uv_loop_t {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_loop_new().cast() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn metrics_idle_time(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) -> u64 {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_metrics_idle_time(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn metrics_info(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    metrics: *mut crate::abi::linux_x86_64::uv_metrics_t,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe { crate::upstream_support::uv_common::uv_metrics_info(loop_.cast(), metrics.cast()) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn record_work_queue_events(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    count: u64,
) {
    unsafe {
        if loop_.is_null() || count == 0 {
            return;
        }

        let fields = unsafe {
            (*loop_)
                .internal_fields
                .cast::<crate::upstream_support::uv_common::uv__loop_internal_fields_t>()
        };
        if fields.is_null() {
            return;
        }

        unsafe {
            let metrics = &mut (*fields).loop_metrics.metrics;
            metrics.events = metrics.events.wrapping_add(count);
            if (*fields).current_timeout == 0 {
                metrics.events_waiting = metrics.events_waiting.wrapping_add(count);
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn now(loop_: *const crate::abi::linux_x86_64::uv_loop_t) -> u64 {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_now(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn print_active_handles(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    stream: *mut crate::abi::linux_x86_64::FILE,
) {
    unsafe {
        unsafe {
            crate::upstream_support::uv_common::uv_print_active_handles(loop_.cast(), stream.cast())
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn print_all_handles(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    stream: *mut crate::abi::linux_x86_64::FILE,
) {
    unsafe {
        unsafe {
            crate::upstream_support::uv_common::uv_print_all_handles(loop_.cast(), stream.cast())
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn run(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    mode: crate::abi::linux_x86_64::uv_run_mode,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            crate::upstream_support::unix_core::uv_run(
                loop_.cast(),
                std::mem::transmute::<_, crate::upstream_support::unix_core::uv_run_mode>(mode),
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn stop(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_stop(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn timer_again(
    handle: *mut crate::abi::linux_x86_64::uv_timer_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::timer::uv_timer_again(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn timer_get_due_in(handle: *const crate::abi::linux_x86_64::uv_timer_t) -> u64 {
    unsafe { unsafe { crate::upstream_support::timer::uv_timer_get_due_in(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn timer_init(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    handle: *mut crate::abi::linux_x86_64::uv_timer_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::timer::uv_timer_init(loop_.cast(), handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn timer_start(
    handle: *mut crate::abi::linux_x86_64::uv_timer_t,
    cb: crate::abi::linux_x86_64::uv_timer_cb,
    timeout: u64,
    repeat: u64,
) -> ::std::os::raw::c_int {
    unsafe {
        unsafe {
            crate::upstream_support::timer::uv_timer_start(
                handle.cast(),
                std::mem::transmute::<_, crate::upstream_support::timer::uv_timer_cb>(cb),
                timeout,
                repeat,
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn timer_stop(
    handle: *mut crate::abi::linux_x86_64::uv_timer_t,
) -> ::std::os::raw::c_int {
    unsafe { unsafe { crate::upstream_support::timer::uv_timer_stop(handle.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn update_time(loop_: *mut crate::abi::linux_x86_64::uv_loop_t) {
    unsafe { unsafe { crate::upstream_support::unix_core::uv_update_time(loop_.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn walk(
    loop_: *mut crate::abi::linux_x86_64::uv_loop_t,
    walk_cb: crate::abi::linux_x86_64::uv_walk_cb,
    arg: *mut ::std::ffi::c_void,
) {
    unsafe {
        unsafe {
            crate::upstream_support::uv_common::uv_walk(
                loop_.cast(),
                std::mem::transmute::<_, crate::upstream_support::uv_common::uv_walk_cb>(walk_cb),
                arg,
            )
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn library_shutdown() {
    unsafe { unsafe { crate::upstream_support::uv_common::uv_library_shutdown() } }
}
