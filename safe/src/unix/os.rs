use crate::abi::linux_x86_64 as abi;
use crate::upstream_support;
use std::os::raw::{c_char, c_int};

pub(crate) unsafe fn exepath(buffer: *mut c_char, size: *mut usize) -> c_int {
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

pub(crate) unsafe fn cwd(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { upstream_support::unix_core::uv_cwd(buffer.cast(), size.cast()) }
}

pub(crate) unsafe fn chdir(dir: *const c_char) -> c_int {
    unsafe { upstream_support::unix_core::uv_chdir(dir.cast()) }
}

pub(crate) unsafe fn cpu_info(
    cpu_infos: *mut *mut abi::uv_cpu_info_t,
    count: *mut c_int,
) -> c_int {
    unsafe { crate::unix::epoll::uv_cpu_info(cpu_infos.cast(), count) }
}

pub(crate) unsafe fn free_cpu_info(cpu_infos: *mut abi::uv_cpu_info_t, count: c_int) {
    unsafe { upstream_support::uv_common::uv_free_cpu_info(cpu_infos.cast(), count) }
}

pub(crate) unsafe fn loadavg(avg: *mut f64) {
    unsafe { crate::unix::epoll::uv_loadavg(avg.cast()) }
}

pub(crate) unsafe fn get_free_memory() -> u64 {
    unsafe { crate::unix::epoll::uv_get_free_memory() }
}

pub(crate) unsafe fn get_total_memory() -> u64 {
    unsafe { crate::unix::epoll::uv_get_total_memory() }
}

pub(crate) unsafe fn get_constrained_memory() -> u64 {
    unsafe { crate::unix::epoll::uv_get_constrained_memory() }
}

pub(crate) unsafe fn get_available_memory() -> u64 {
    unsafe { crate::unix::epoll::uv_get_available_memory() }
}

pub(crate) unsafe fn gettimeofday(tv: *mut abi::uv_timeval64_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_gettimeofday(tv.cast()) }
}

pub(crate) unsafe fn getrusage(rusage: *mut abi::uv_rusage_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_getrusage(rusage.cast()) }
}

pub(crate) unsafe fn resident_set_memory(rss: *mut usize) -> c_int {
    unsafe { crate::unix::epoll::uv_resident_set_memory(rss.cast()) }
}

pub(crate) unsafe fn uptime(uptime: *mut f64) -> c_int {
    unsafe { crate::unix::epoll::uv_uptime(uptime) }
}

pub(crate) unsafe fn get_osfhandle(fd: c_int) -> abi::uv_os_fd_t {
    unsafe { upstream_support::unix_core::uv_get_osfhandle(fd) as abi::uv_os_fd_t }
}

pub(crate) unsafe fn open_osfhandle(os_fd: abi::uv_os_fd_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_open_osfhandle(os_fd as _) }
}

pub(crate) unsafe fn os_environ(
    envitems: *mut *mut abi::uv_env_item_t,
    count: *mut c_int,
) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_environ(envitems.cast(), count) }
}

pub(crate) unsafe fn os_free_environ(envitems: *mut abi::uv_env_item_t, count: c_int) {
    unsafe { upstream_support::uv_common::uv_os_free_environ(envitems.cast(), count) }
}

pub(crate) unsafe fn os_free_group(grp: *mut abi::uv_group_t) {
    unsafe { upstream_support::uv_common::uv_os_free_group(grp.cast()) }
}

pub(crate) unsafe fn os_free_passwd(pwd: *mut abi::uv_passwd_t) {
    unsafe { upstream_support::uv_common::uv_os_free_passwd(pwd.cast()) }
}

pub(crate) unsafe fn os_get_group(grp: *mut abi::uv_group_t, gid: abi::uv_uid_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_get_group(grp.cast(), gid as _) }
}

pub(crate) unsafe fn os_get_passwd(pwd: *mut abi::uv_passwd_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_get_passwd(pwd.cast()) }
}

pub(crate) unsafe fn os_get_passwd2(pwd: *mut abi::uv_passwd_t, uid: abi::uv_uid_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_get_passwd2(pwd.cast(), uid as _) }
}

pub(crate) unsafe fn os_getenv(name: *const c_char, buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_getenv(name.cast(), buffer.cast(), size.cast()) }
}

pub(crate) unsafe fn os_setenv(name: *const c_char, value: *const c_char) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_setenv(name.cast(), value.cast()) }
}

pub(crate) unsafe fn os_unsetenv(name: *const c_char) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_unsetenv(name.cast()) }
}

pub(crate) unsafe fn os_gethostname(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_gethostname(buffer.cast(), size.cast()) }
}

pub(crate) unsafe fn os_getpid() -> abi::uv_pid_t {
    unsafe { upstream_support::unix_core::uv_os_getpid() as abi::uv_pid_t }
}

pub(crate) unsafe fn os_getppid() -> abi::uv_pid_t {
    unsafe { upstream_support::unix_core::uv_os_getppid() as abi::uv_pid_t }
}

pub(crate) unsafe fn os_getpriority(pid: abi::uv_pid_t, priority: *mut c_int) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_getpriority(pid as _, priority) }
}

pub(crate) unsafe fn os_setpriority(pid: abi::uv_pid_t, priority: c_int) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_setpriority(pid as _, priority) }
}

pub(crate) unsafe fn os_homedir(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_homedir(buffer.cast(), size.cast()) }
}

pub(crate) unsafe fn os_tmpdir(buffer: *mut c_char, size: *mut usize) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_tmpdir(buffer.cast(), size.cast()) }
}

pub(crate) unsafe fn os_uname(buffer: *mut abi::uv_utsname_t) -> c_int {
    unsafe { upstream_support::unix_core::uv_os_uname(buffer.cast()) }
}
