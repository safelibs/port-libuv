use crate::abi::linux_x86_64 as abi;
use crate::upstream_support;
use std::os::raw::{c_char, c_int};

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn exepath(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe {
        let mut n: libc::ssize_t;

        if buffer.is_null() || size.is_null() || unsafe { *size } == 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        n = unsafe { (*size).saturating_sub(1) as libc::ssize_t };
        if n > 0 {
            n = unsafe { libc::readlink(c"/proc/self/exe".as_ptr(), buffer.cast(), n as usize) };
            if n < 0 {
                return -unsafe { *libc::__errno_location() };
            }
        }

        unsafe {
            *buffer.add(n as usize) = 0;
            *size = n as usize;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cwd(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_cwd(buffer.cast(), size.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn chdir(dir: *const c_char) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_chdir(dir.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cpu_info(cpu_infos: *mut *mut abi::uv_cpu_info_t, count: *mut c_int) -> c_int {
    unsafe { unsafe { crate::unix::epoll::uv_cpu_info(cpu_infos.cast(), count) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn free_cpu_info(cpu_infos: *mut abi::uv_cpu_info_t, count: c_int) {
    unsafe { unsafe { upstream_support::uv_common::uv_free_cpu_info(cpu_infos.cast(), count) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loadavg(avg: *mut f64) {
    unsafe { unsafe { crate::unix::epoll::uv_loadavg(avg.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_free_memory() -> u64 {
    unsafe { unsafe { crate::unix::epoll::uv_get_free_memory() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_total_memory() -> u64 {
    unsafe { unsafe { crate::unix::epoll::uv_get_total_memory() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_constrained_memory() -> u64 {
    unsafe { unsafe { crate::unix::epoll::uv_get_constrained_memory() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_available_memory() -> u64 {
    unsafe { unsafe { crate::unix::epoll::uv_get_available_memory() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn gettimeofday(tv: *mut abi::uv_timeval64_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_gettimeofday(tv.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn getrusage(rusage: *mut abi::uv_rusage_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_getrusage(rusage.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn resident_set_memory(rss: *mut usize) -> c_int {
    unsafe { unsafe { crate::unix::epoll::uv_resident_set_memory(rss.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn uptime(uptime: *mut f64) -> c_int {
    unsafe { unsafe { crate::unix::epoll::uv_uptime(uptime) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_osfhandle(fd: c_int) -> abi::uv_os_fd_t {
    unsafe { unsafe { upstream_support::unix_core::uv_get_osfhandle(fd) as abi::uv_os_fd_t } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn open_osfhandle(os_fd: abi::uv_os_fd_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_open_osfhandle(os_fd as _) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_environ(envitems: *mut *mut abi::uv_env_item_t, count: *mut c_int) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_environ(envitems.cast(), count) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_free_environ(envitems: *mut abi::uv_env_item_t, count: c_int) {
    unsafe { unsafe { upstream_support::uv_common::uv_os_free_environ(envitems.cast(), count) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_free_group(grp: *mut abi::uv_group_t) {
    unsafe { unsafe { upstream_support::uv_common::uv_os_free_group(grp.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_free_passwd(pwd: *mut abi::uv_passwd_t) {
    unsafe { unsafe { upstream_support::uv_common::uv_os_free_passwd(pwd.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_get_group(grp: *mut abi::uv_group_t, gid: abi::uv_uid_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_get_group(grp.cast(), gid as _) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_get_passwd(pwd: *mut abi::uv_passwd_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_get_passwd(pwd.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_get_passwd2(pwd: *mut abi::uv_passwd_t, uid: abi::uv_uid_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_get_passwd2(pwd.cast(), uid as _) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_getenv(name: *const c_char, buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe {
        unsafe {
            upstream_support::unix_core::uv_os_getenv(name.cast(), buffer.cast(), size.cast())
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_setenv(name: *const c_char, value: *const c_char) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_setenv(name.cast(), value.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_unsetenv(name: *const c_char) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_unsetenv(name.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_gethostname(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_gethostname(buffer.cast(), size.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_getpid() -> abi::uv_pid_t {
    unsafe { unsafe { upstream_support::unix_core::uv_os_getpid() as abi::uv_pid_t } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_getppid() -> abi::uv_pid_t {
    unsafe { unsafe { upstream_support::unix_core::uv_os_getppid() as abi::uv_pid_t } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_getpriority(pid: abi::uv_pid_t, priority: *mut c_int) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_getpriority(pid as _, priority) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_setpriority(pid: abi::uv_pid_t, priority: c_int) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_setpriority(pid as _, priority) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_homedir(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_homedir(buffer.cast(), size.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_tmpdir(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_tmpdir(buffer.cast(), size.cast()) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn os_uname(buffer: *mut abi::uv_utsname_t) -> c_int {
    unsafe { unsafe { upstream_support::unix_core::uv_os_uname(buffer.cast()) } }
}
