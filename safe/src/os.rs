use crate::allocator::{
    self, last_error, uv_err, UV_E2BIG, UV_EFAULT, UV_EINVAL, UV_ENOBUFS, UV_ENOENT, UV_ENOMEM,
};
use crate::bindings::*;
use std::ffi::CStr;
use std::mem;
use std::ptr;

const UV_PATH_MAX: usize = 4096;
const NANOSEC: u64 = 1_000_000_000;

unsafe extern "C" {
    static mut environ: *mut *mut libc::c_char;
}

unsafe fn open_cloexec(path: *const libc::c_char) -> libc::c_int {
    let fd = libc::open(path, libc::O_RDONLY | libc::O_CLOEXEC);
    if fd < 0 {
        last_error()
    } else {
        fd
    }
}

unsafe fn slurp_cstr(path: *const libc::c_char, buf: &mut [u8]) -> Result<(), libc::c_int> {
    debug_assert!(!buf.is_empty());

    let fd = open_cloexec(path);
    if fd < 0 {
        return Err(fd);
    }

    let n = loop {
        let rc = libc::read(fd, buf.as_mut_ptr().cast(), buf.len() - 1);
        if rc >= 0 {
            break rc as usize;
        }
        if *allocator::errno_location() != libc::EINTR {
            libc::close(fd);
            return Err(last_error());
        }
    };

    libc::close(fd);
    buf[n] = 0;
    Ok(())
}

unsafe fn slurp_str(path: *const libc::c_char, buf: &mut [u8]) -> Result<&str, libc::c_int> {
    slurp_cstr(path, buf)?;
    Ok(CStr::from_ptr(buf.as_ptr().cast())
        .to_str()
        .expect("procfs content must be utf-8"))
}

unsafe fn alloc_buf(size: usize) -> *mut libc::c_char {
    allocator::malloc(size).cast()
}

unsafe fn uv_getpwuid_r_impl(pwd: *mut uv_passwd_t, uid: libc::uid_t) -> libc::c_int {
    if pwd.is_null() {
        return UV_EINVAL;
    }

    let mut bufsize = 2000usize;
    let (mut passwd, mut result);
    let mut temp_buf;
    let mut r;

    loop {
        temp_buf = alloc_buf(bufsize);
        if temp_buf.is_null() {
            return UV_ENOMEM;
        }

        passwd = mem::zeroed::<libc::passwd>();
        result = ptr::null_mut();

        loop {
            r = libc::getpwuid_r(uid, &mut passwd, temp_buf, bufsize, &mut result);
            if r != libc::EINTR {
                break;
            }
        }

        if r != 0 || result.is_null() {
            allocator::free(temp_buf.cast());
        }
        if r != libc::ERANGE {
            break;
        }
        bufsize *= 2;
    }

    if r != 0 {
        return uv_err(r);
    }
    if result.is_null() {
        return UV_ENOENT;
    }

    let name_size = libc::strlen(passwd.pw_name) + 1;
    let homedir_size = libc::strlen(passwd.pw_dir) + 1;
    let shell_size = libc::strlen(passwd.pw_shell) + 1;
    let username = alloc_buf(name_size + homedir_size + shell_size);
    if username.is_null() {
        allocator::free(temp_buf.cast());
        return UV_ENOMEM;
    }

    ptr::copy_nonoverlapping(passwd.pw_name, username, name_size);
    (*pwd).username = username;

    (*pwd).homedir = username.add(name_size);
    ptr::copy_nonoverlapping(passwd.pw_dir, (*pwd).homedir, homedir_size);

    (*pwd).shell = (*pwd).homedir.add(homedir_size);
    ptr::copy_nonoverlapping(passwd.pw_shell, (*pwd).shell, shell_size);

    (*pwd).uid = passwd.pw_uid as libc::c_ulong;
    (*pwd).gid = passwd.pw_gid as libc::c_ulong;

    allocator::free(temp_buf.cast());
    0
}

unsafe fn err_from_pthread(err: libc::c_int) -> libc::c_int {
    let errno = *allocator::errno_location();
    if errno != 0 {
        -errno
    } else {
        uv_err(err)
    }
}

unsafe fn current_tid() -> libc::pid_t {
    libc::syscall(libc::SYS_gettid) as libc::pid_t
}

unsafe fn read_uint64(path: &[u8]) -> u64 {
    let mut buf = [0u8; 32];
    if slurp_cstr(path.as_ptr().cast(), &mut buf).is_err() {
        return 0;
    }
    let text = CStr::from_ptr(buf.as_ptr().cast())
        .to_str()
        .expect("cgroup values must be utf-8")
        .trim();
    if text == "max" {
        u64::MAX
    } else {
        text.parse::<u64>().unwrap_or(0)
    }
}

unsafe fn read_proc_meminfo(label: &str) -> u64 {
    let mut buf = [0u8; 4096];
    let Ok(text) = slurp_str(b"/proc/meminfo\0".as_ptr().cast(), &mut buf) else {
        return 0;
    };
    let Some(start) = text.find(label) else {
        return 0;
    };
    let value = text[start + label.len()..]
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);
    value * 1024
}

fn cgroup1_memory_controller(text: &str) -> Option<&str> {
    for line in text.lines() {
        let mut parts = line.splitn(3, ':');
        let _ = parts.next()?;
        let controllers = parts.next()?;
        let path = parts.next()?;
        if controllers
            .split(',')
            .any(|controller| controller == "memory")
        {
            return Some(path.strip_prefix('/').unwrap_or(path));
        }
    }
    None
}

unsafe fn snprintf_path(buf: &mut [libc::c_char], fmt: &[u8], path: &str) {
    libc::snprintf(
        buf.as_mut_ptr(),
        buf.len(),
        fmt.as_ptr().cast(),
        path.len() as libc::c_int,
        path.as_ptr().cast::<libc::c_char>(),
    );
}

unsafe fn cgroup1_memory_limits(text: &str) -> (u64, u64) {
    let mut filename = [0i8; 4097];
    if let Some(path) = cgroup1_memory_controller(text) {
        snprintf_path(
            &mut filename,
            b"/sys/fs/cgroup/memory/%.*s/memory.soft_limit_in_bytes\0",
            path,
        );
        let high = read_uint64(CStr::from_ptr(filename.as_ptr()).to_bytes_with_nul());

        snprintf_path(
            &mut filename,
            b"/sys/fs/cgroup/memory/%.*s/memory.limit_in_bytes\0",
            path,
        );
        let max = read_uint64(CStr::from_ptr(filename.as_ptr()).to_bytes_with_nul());
        if high != 0 && max != 0 {
            let cgroup1_max = (libc::c_long::MAX as u64)
                & !((libc::sysconf(libc::_SC_PAGESIZE) as u64).saturating_sub(1));
            return (
                if high == cgroup1_max { u64::MAX } else { high },
                if max == cgroup1_max { u64::MAX } else { max },
            );
        }
    }

    let high = read_uint64(b"/sys/fs/cgroup/memory/memory.soft_limit_in_bytes\0");
    let max = read_uint64(b"/sys/fs/cgroup/memory/memory.limit_in_bytes\0");
    let cgroup1_max = (libc::c_long::MAX as u64)
        & !((libc::sysconf(libc::_SC_PAGESIZE) as u64).saturating_sub(1));
    (
        if high == cgroup1_max { u64::MAX } else { high },
        if max == cgroup1_max { u64::MAX } else { max },
    )
}

unsafe fn cgroup2_memory_limits(text: &str) -> (u64, u64) {
    let mut filename = [0i8; 4097];
    let path = text.trim_end().strip_prefix("0::/").unwrap_or_default();
    snprintf_path(&mut filename, b"/sys/fs/cgroup/%.*s/memory.max\0", path);
    let max = read_uint64(CStr::from_ptr(filename.as_ptr()).to_bytes_with_nul());
    snprintf_path(&mut filename, b"/sys/fs/cgroup/%.*s/memory.high\0", path);
    let high = read_uint64(CStr::from_ptr(filename.as_ptr()).to_bytes_with_nul());
    (high, max)
}

unsafe fn constrained_memory_from_text(text: &str) -> u64 {
    let (high, max) = if text.starts_with("0::/") {
        cgroup2_memory_limits(text)
    } else {
        cgroup1_memory_limits(text)
    };

    if high == 0 || max == 0 {
        0
    } else {
        high.min(max)
    }
}

unsafe fn cgroup1_current_memory(text: &str) -> u64 {
    let mut filename = [0i8; 4097];
    if let Some(path) = cgroup1_memory_controller(text) {
        snprintf_path(
            &mut filename,
            b"/sys/fs/cgroup/memory/%.*s/memory.usage_in_bytes\0",
            path,
        );
        let current = read_uint64(CStr::from_ptr(filename.as_ptr()).to_bytes_with_nul());
        if current != 0 {
            return current;
        }
    }

    read_uint64(b"/sys/fs/cgroup/memory/memory.usage_in_bytes\0")
}

unsafe fn cgroup2_current_memory(text: &str) -> u64 {
    let mut filename = [0i8; 4097];
    let path = text.trim_end().strip_prefix("0::/").unwrap_or_default();
    snprintf_path(&mut filename, b"/sys/fs/cgroup/%.*s/memory.current\0", path);
    read_uint64(CStr::from_ptr(filename.as_ptr()).to_bytes_with_nul())
}

#[no_mangle]
pub unsafe extern "C" fn uv_clock_gettime(
    clock_id: uv_clock_id,
    ts: *mut uv_timespec64_t,
) -> libc::c_int {
    if ts.is_null() {
        return UV_EFAULT;
    }

    let mut t = mem::zeroed::<libc::timespec>();
    let clock = match clock_id {
        uv_clock_id_UV_CLOCK_MONOTONIC => libc::CLOCK_MONOTONIC,
        uv_clock_id_UV_CLOCK_REALTIME => libc::CLOCK_REALTIME,
        _ => return UV_EINVAL,
    };

    if libc::clock_gettime(clock, &mut t) != 0 {
        return last_error();
    }

    (*ts).tv_sec = t.tv_sec as i64;
    (*ts).tv_nsec = t.tv_nsec as i32;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_hrtime() -> u64 {
    let mut ts = mem::zeroed::<libc::timespec>();
    if libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) != 0 {
        0
    } else {
        ts.tv_sec as u64 * NANOSEC + ts.tv_nsec as u64
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_cwd(buffer: *mut libc::c_char, size: *mut usize) -> libc::c_int {
    let mut scratch = [0i8; UV_PATH_MAX + 1];
    let mut used_scratch = false;

    if buffer.is_null() || size.is_null() {
        return UV_EINVAL;
    }

    if libc::getcwd(buffer, *size).is_null() {
        if *allocator::errno_location() != libc::ERANGE {
            return last_error();
        }

        if libc::getcwd(scratch.as_mut_ptr(), scratch.len()).is_null() {
            return last_error();
        }
        used_scratch = true;
    }

    let src = if used_scratch {
        scratch.as_mut_ptr()
    } else {
        buffer
    };

    let mut len = libc::strlen(src);
    if len > 1 && *src.add(len - 1) == b'/' as libc::c_char {
        len -= 1;
        *src.add(len) = 0;
    }

    *size = len;
    if used_scratch {
        *size += 1;
        UV_ENOBUFS
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_chdir(dir: *const libc::c_char) -> libc::c_int {
    if libc::chdir(dir) != 0 {
        last_error()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_disable_stdio_inheritance() {
    let mut fd = 0;
    loop {
        let rc = libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC);
        if rc != 0 && fd > 15 {
            break;
        }
        fd += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_exepath(buffer: *mut libc::c_char, size: *mut usize) -> libc::c_int {
    if buffer.is_null() || size.is_null() || *size == 0 {
        return UV_EINVAL;
    }

    let mut n = *size - 1;
    if n > 0 {
        let rc = libc::readlink(b"/proc/self/exe\0".as_ptr().cast(), buffer, n);
        if rc < 0 {
            return last_error();
        }
        n = rc as usize;
    } else {
        n = 0;
    }

    *buffer.add(n) = 0;
    *size = n;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_getrusage(rusage: *mut uv_rusage_t) -> libc::c_int {
    let mut usage = mem::zeroed::<libc::rusage>();
    if libc::getrusage(libc::RUSAGE_SELF, &mut usage) != 0 {
        return last_error();
    }

    (*rusage).ru_utime.tv_sec = usage.ru_utime.tv_sec;
    (*rusage).ru_utime.tv_usec = usage.ru_utime.tv_usec;
    (*rusage).ru_stime.tv_sec = usage.ru_stime.tv_sec;
    (*rusage).ru_stime.tv_usec = usage.ru_stime.tv_usec;
    (*rusage).ru_maxrss = usage.ru_maxrss as u64;
    (*rusage).ru_ixrss = usage.ru_ixrss as u64;
    (*rusage).ru_idrss = usage.ru_idrss as u64;
    (*rusage).ru_isrss = usage.ru_isrss as u64;
    (*rusage).ru_minflt = usage.ru_minflt as u64;
    (*rusage).ru_majflt = usage.ru_majflt as u64;
    (*rusage).ru_nswap = usage.ru_nswap as u64;
    (*rusage).ru_inblock = usage.ru_inblock as u64;
    (*rusage).ru_oublock = usage.ru_oublock as u64;
    (*rusage).ru_msgsnd = usage.ru_msgsnd as u64;
    (*rusage).ru_msgrcv = usage.ru_msgrcv as u64;
    (*rusage).ru_nsignals = usage.ru_nsignals as u64;
    (*rusage).ru_nvcsw = usage.ru_nvcsw as u64;
    (*rusage).ru_nivcsw = usage.ru_nivcsw as u64;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_homedir(buffer: *mut libc::c_char, size: *mut usize) -> libc::c_int {
    let mut pwd = mem::zeroed::<uv_passwd_t>();
    let r = uv_os_getenv(b"HOME\0".as_ptr().cast(), buffer, size);
    if r != UV_ENOENT {
        return r;
    }

    let r = uv_os_get_passwd(&mut pwd);
    if r != 0 {
        return r;
    }

    let len = libc::strlen(pwd.homedir);
    if len >= *size {
        *size = len + 1;
        uv_os_free_passwd(&mut pwd);
        return UV_ENOBUFS;
    }

    ptr::copy_nonoverlapping(pwd.homedir, buffer, len + 1);
    *size = len;
    uv_os_free_passwd(&mut pwd);
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_tmpdir(buffer: *mut libc::c_char, size: *mut usize) -> libc::c_int {
    if buffer.is_null() || size.is_null() || *size == 0 {
        return UV_EINVAL;
    }

    let mut path = libc::getenv(b"TMPDIR\0".as_ptr().cast());
    if path.is_null() {
        path = libc::getenv(b"TMP\0".as_ptr().cast());
    }
    if path.is_null() {
        path = libc::getenv(b"TEMP\0".as_ptr().cast());
    }
    if path.is_null() {
        path = libc::getenv(b"TEMPDIR\0".as_ptr().cast());
    }
    if path.is_null() {
        path = b"/tmp\0".as_ptr() as *mut libc::c_char;
    }

    let mut len = libc::strlen(path);
    if len >= *size {
        *size = len + 1;
        return UV_ENOBUFS;
    }
    if len > 1 && *path.add(len - 1) == b'/' as libc::c_char {
        len -= 1;
    }

    ptr::copy_nonoverlapping(path, buffer, len + 1);
    *buffer.add(len) = 0;
    *size = len;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_get_passwd(pwd: *mut uv_passwd_t) -> libc::c_int {
    uv_getpwuid_r_impl(pwd, libc::geteuid())
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_get_passwd2(pwd: *mut uv_passwd_t, uid: uv_uid_t) -> libc::c_int {
    uv_getpwuid_r_impl(pwd, uid)
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_free_passwd(pwd: *mut uv_passwd_t) {
    if pwd.is_null() {
        return;
    }

    allocator::free((*pwd).username.cast());
    (*pwd).username = ptr::null_mut();
    (*pwd).shell = ptr::null_mut();
    (*pwd).homedir = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_get_group(grp: *mut uv_group_t, gid: uv_uid_t) -> libc::c_int {
    if grp.is_null() {
        return UV_EINVAL;
    }

    let mut bufsize = 2000usize;
    let (mut group, mut result);
    let mut temp_buf;
    let mut r;

    loop {
        temp_buf = alloc_buf(bufsize);
        if temp_buf.is_null() {
            return UV_ENOMEM;
        }

        group = mem::zeroed::<libc::group>();
        result = ptr::null_mut();
        loop {
            r = libc::getgrgid_r(gid, &mut group, temp_buf, bufsize, &mut result);
            if r != libc::EINTR {
                break;
            }
        }

        if r != 0 || result.is_null() {
            allocator::free(temp_buf.cast());
        }
        if r != libc::ERANGE {
            break;
        }
        bufsize *= 2;
    }

    if r != 0 {
        return uv_err(r);
    }
    if result.is_null() {
        return UV_ENOENT;
    }

    let name_size = libc::strlen(group.gr_name) + 1;
    let mut members = 0usize;
    let mut mem_size = mem::size_of::<*mut libc::c_char>();
    while !(*group.gr_mem.add(members)).is_null() {
        mem_size += libc::strlen(*group.gr_mem.add(members)) + 1;
        mem_size += mem::size_of::<*mut libc::c_char>();
        members += 1;
    }

    let storage = alloc_buf(name_size + mem_size);
    if storage.is_null() {
        allocator::free(temp_buf.cast());
        return UV_ENOMEM;
    }

    (*grp).members = storage.cast();
    *(*grp).members.add(members) = ptr::null_mut();

    let mut cursor = (*grp).members.add(members + 1).cast::<libc::c_char>();
    for i in 0..members {
        let src = *group.gr_mem.add(i);
        let len = libc::strlen(src) + 1;
        *(*grp).members.add(i) = cursor;
        ptr::copy_nonoverlapping(src, cursor, len);
        cursor = cursor.add(len);
    }

    (*grp).groupname = cursor;
    ptr::copy_nonoverlapping(group.gr_name, cursor, name_size);
    (*grp).gid = group.gr_gid as libc::c_ulong;

    allocator::free(temp_buf.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_free_group(grp: *mut uv_group_t) {
    if grp.is_null() {
        return;
    }

    allocator::free((*grp).members.cast());
    (*grp).members = ptr::null_mut();
    (*grp).groupname = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn uv_translate_sys_error(sys_errno: libc::c_int) -> libc::c_int {
    if sys_errno <= 0 {
        sys_errno
    } else {
        -sys_errno
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_environ(
    envitems: *mut *mut uv_env_item_t,
    count: *mut libc::c_int,
) -> libc::c_int {
    if envitems.is_null() || count.is_null() {
        return UV_EINVAL;
    }

    *envitems = ptr::null_mut();
    *count = 0;

    let mut total = 0usize;
    while !(*environ.add(total)).is_null() {
        total += 1;
    }

    let items = allocator::calloc(total, mem::size_of::<uv_env_item_t>()).cast::<uv_env_item_t>();
    if items.is_null() {
        return UV_ENOMEM;
    }

    let mut written = 0usize;
    for i in 0..total {
        let entry = *environ.add(i);
        if entry.is_null() {
            break;
        }

        let buf = allocator::strdup(entry);
        if buf.is_null() {
            for j in 0..written {
                allocator::free((*items.add(j)).name.cast());
            }
            allocator::free(items.cast());
            return UV_ENOMEM;
        }

        let sep = libc::strchr(buf, '=' as libc::c_int);
        if sep.is_null() {
            allocator::free(buf.cast());
            continue;
        }

        *sep = 0;
        (*items.add(written)).name = buf;
        (*items.add(written)).value = sep.add(1);
        written += 1;
    }

    *envitems = items;
    *count = written as libc::c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_free_environ(envitems: *mut uv_env_item_t, count: libc::c_int) {
    for i in 0..count.max(0) as usize {
        allocator::free((*envitems.add(i)).name.cast());
    }
    allocator::free(envitems.cast());
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_getenv(
    name: *const libc::c_char,
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    if name.is_null() || buffer.is_null() || size.is_null() || *size == 0 {
        return UV_EINVAL;
    }

    let value = libc::getenv(name);
    if value.is_null() {
        return UV_ENOENT;
    }

    let len = libc::strlen(value);
    if len >= *size {
        *size = len + 1;
        return UV_ENOBUFS;
    }

    ptr::copy_nonoverlapping(value, buffer, len + 1);
    *size = len;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_setenv(
    name: *const libc::c_char,
    value: *const libc::c_char,
) -> libc::c_int {
    if name.is_null() || value.is_null() {
        return UV_EINVAL;
    }
    if libc::setenv(name, value, 1) != 0 {
        last_error()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_unsetenv(name: *const libc::c_char) -> libc::c_int {
    if name.is_null() {
        return UV_EINVAL;
    }
    if libc::unsetenv(name) != 0 {
        last_error()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_gethostname(
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    let mut buf = [0i8; UV_MAXHOSTNAMESIZE as usize];
    if buffer.is_null() || size.is_null() || *size == 0 {
        return UV_EINVAL;
    }

    if libc::gethostname(buf.as_mut_ptr(), buf.len()) != 0 {
        return last_error();
    }

    buf[buf.len() - 1] = 0;
    let len = libc::strlen(buf.as_ptr());
    if len >= *size {
        *size = len + 1;
        return UV_ENOBUFS;
    }

    ptr::copy_nonoverlapping(buf.as_ptr(), buffer, len + 1);
    *size = len;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_getpid() -> uv_pid_t {
    libc::getpid()
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_getppid() -> uv_pid_t {
    libc::getppid()
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_getpriority(
    pid: uv_pid_t,
    priority: *mut libc::c_int,
) -> libc::c_int {
    if priority.is_null() {
        return UV_EINVAL;
    }

    *allocator::errno_location() = 0;
    let r = libc::getpriority(libc::PRIO_PROCESS, pid as libc::id_t);
    if r == -1 && *allocator::errno_location() != 0 {
        return last_error();
    }

    *priority = r;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_setpriority(pid: uv_pid_t, priority: libc::c_int) -> libc::c_int {
    if !(UV_PRIORITY_HIGHEST..=UV_PRIORITY_LOW as libc::c_int).contains(&priority) {
        return UV_EINVAL;
    }

    if libc::setpriority(libc::PRIO_PROCESS, pid as libc::id_t, priority) != 0 {
        last_error()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_getpriority(
    tid: uv_thread_t,
    priority: *mut libc::c_int,
) -> libc::c_int {
    if priority.is_null() {
        return UV_EINVAL;
    }

    let mut policy = 0;
    let mut param = mem::zeroed::<libc::sched_param>();
    let r = libc::pthread_getschedparam(tid, &mut policy, &mut param);
    if r != 0 {
        return err_from_pthread(r);
    }

    if policy == libc::SCHED_OTHER && libc::pthread_equal(tid, libc::pthread_self()) != 0 {
        *allocator::errno_location() = 0;
        let r = libc::getpriority(libc::PRIO_PROCESS, current_tid() as libc::id_t);
        if r == -1 && *allocator::errno_location() != 0 {
            return last_error();
        }
        *priority = r;
        return 0;
    }

    *priority = param.sched_priority;
    0
}

unsafe fn set_nice_for_calling_thread(priority: libc::c_int) -> libc::c_int {
    if !(UV_THREAD_PRIORITY_LOWEST..=UV_THREAD_PRIORITY_HIGHEST).contains(&priority) {
        return UV_EINVAL;
    }

    let nice = -priority * 2;
    if libc::setpriority(libc::PRIO_PROCESS, current_tid() as libc::id_t, nice) != 0 {
        last_error()
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_thread_setpriority(
    tid: uv_thread_t,
    priority: libc::c_int,
) -> libc::c_int {
    if !(UV_THREAD_PRIORITY_LOWEST..=UV_THREAD_PRIORITY_HIGHEST).contains(&priority) {
        return UV_EINVAL;
    }

    let mut policy = 0;
    let mut param = mem::zeroed::<libc::sched_param>();
    let r = libc::pthread_getschedparam(tid, &mut policy, &mut param);
    if r != 0 {
        return err_from_pthread(r);
    }

    if policy == libc::SCHED_OTHER && libc::pthread_equal(tid, libc::pthread_self()) != 0 {
        return set_nice_for_calling_thread(priority);
    }

    let min = libc::sched_get_priority_min(policy);
    let max = libc::sched_get_priority_max(policy);
    if min == -1 || max == -1 {
        return last_error();
    }

    let range = max - min;
    let prio = match priority {
        UV_THREAD_PRIORITY_HIGHEST => max,
        UV_THREAD_PRIORITY_ABOVE_NORMAL => min + range * 3 / 4,
        UV_THREAD_PRIORITY_NORMAL => min + range / 2,
        UV_THREAD_PRIORITY_BELOW_NORMAL => min + range / 4,
        UV_THREAD_PRIORITY_LOWEST => min,
        _ => return 0,
    };

    if param.sched_priority != prio {
        param.sched_priority = prio;
        let r = libc::pthread_setschedparam(tid, policy, &param);
        if r != 0 {
            return err_from_pthread(r);
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_os_uname(buffer: *mut uv_utsname_t) -> libc::c_int {
    if buffer.is_null() {
        return UV_EINVAL;
    }

    let mut uts = mem::zeroed::<libc::utsname>();
    if libc::uname(&mut uts) != 0 {
        return last_error();
    }

    let mut r =
        crate::strings::uv__strscpy((*buffer).sysname.as_mut_ptr(), uts.sysname.as_ptr(), 256);
    if r == UV_E2BIG as isize {
        goto_error(buffer, UV_E2BIG)
    } else {
        r = crate::strings::uv__strscpy((*buffer).release.as_mut_ptr(), uts.release.as_ptr(), 256);
        if r == UV_E2BIG as isize {
            goto_error(buffer, UV_E2BIG)
        } else {
            r = crate::strings::uv__strscpy(
                (*buffer).version.as_mut_ptr(),
                uts.version.as_ptr(),
                256,
            );
            if r == UV_E2BIG as isize {
                goto_error(buffer, UV_E2BIG)
            } else {
                r = crate::strings::uv__strscpy(
                    (*buffer).machine.as_mut_ptr(),
                    uts.machine.as_ptr(),
                    256,
                );
                if r == UV_E2BIG as isize {
                    goto_error(buffer, UV_E2BIG)
                } else {
                    0
                }
            }
        }
    }
}

unsafe fn goto_error(buffer: *mut uv_utsname_t, code: libc::c_int) -> libc::c_int {
    (*buffer).sysname[0] = 0;
    (*buffer).release[0] = 0;
    (*buffer).version[0] = 0;
    (*buffer).machine[0] = 0;
    code
}

#[no_mangle]
pub unsafe extern "C" fn uv_gettimeofday(tv: *mut uv_timeval64_t) -> libc::c_int {
    if tv.is_null() {
        return UV_EINVAL;
    }

    let mut time = mem::zeroed::<libc::timeval>();
    if libc::gettimeofday(&mut time, ptr::null_mut()) != 0 {
        return last_error();
    }

    (*tv).tv_sec = time.tv_sec as i64;
    (*tv).tv_usec = time.tv_usec as i32;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_sleep(msec: libc::c_uint) {
    let mut timeout = libc::timespec {
        tv_sec: (msec / 1000) as libc::time_t,
        tv_nsec: ((msec % 1000) * 1_000_000) as libc::c_long,
    };

    loop {
        if libc::nanosleep(&timeout, &mut timeout) == 0 {
            return;
        }
        if *allocator::errno_location() != libc::EINTR {
            libc::abort();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_available_parallelism() -> libc::c_uint {
    let mut set = mem::zeroed::<libc::cpu_set_t>();
    libc::CPU_ZERO(&mut set);

    let mut rc = if libc::sched_getaffinity(0, mem::size_of::<libc::cpu_set_t>(), &mut set) == 0 {
        libc::CPU_COUNT(&set) as libc::c_long
    } else {
        libc::sysconf(libc::_SC_NPROCESSORS_ONLN)
    };

    if rc < 1 {
        rc = 1;
    }

    rc as libc::c_uint
}

#[no_mangle]
pub unsafe extern "C" fn uv_resident_set_memory(rss: *mut usize) -> libc::c_int {
    let mut buf = [0u8; 1024];
    let Ok(text) = slurp_str(b"/proc/self/stat\0".as_ptr().cast(), &mut buf) else {
        return UV_EINVAL;
    };

    let Some(first_space) = text.find(' ') else {
        return UV_EINVAL;
    };
    if text.as_bytes().get(first_space + 1) != Some(&b'(') {
        return UV_EINVAL;
    }
    let Some(close_paren_rel) = text[first_space + 1..].find(')') else {
        return UV_EINVAL;
    };

    let mut index = first_space + 1 + close_paren_rel;
    for _ in 0..22 {
        let Some(next_space) = text[index + 1..].find(' ') else {
            return UV_EINVAL;
        };
        index += next_space + 1;
    }

    let value = text[index..]
        .trim_start()
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<libc::c_long>().ok())
        .filter(|value| *value >= 0)
        .ok_or(UV_EINVAL);

    match value {
        Ok(value) => {
            *rss = value as usize * libc::sysconf(libc::_SC_PAGESIZE) as usize;
            0
        }
        Err(code) => code,
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_uptime(uptime: *mut f64) -> libc::c_int {
    let mut buf = [0u8; 128];
    if let Ok(text) = slurp_str(b"/proc/uptime\0".as_ptr().cast(), &mut buf) {
        if let Some(value) = text
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
        {
            *uptime = value;
            return 0;
        }
    }

    let mut now = mem::zeroed::<libc::timespec>();
    if libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut now) != 0 {
        return last_error();
    }

    *uptime = now.tv_sec as f64;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_get_free_memory() -> u64 {
    let mem = read_proc_meminfo("MemAvailable:");
    if mem != 0 {
        return mem;
    }

    let mut info = mem::zeroed::<libc::sysinfo>();
    if libc::sysinfo(&mut info) == 0 {
        info.freeram as u64 * info.mem_unit as u64
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_get_total_memory() -> u64 {
    let mem = read_proc_meminfo("MemTotal:");
    if mem != 0 {
        return mem;
    }

    let mut info = mem::zeroed::<libc::sysinfo>();
    if libc::sysinfo(&mut info) == 0 {
        info.totalram as u64 * info.mem_unit as u64
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_get_constrained_memory() -> u64 {
    let mut buf = [0u8; 1024];
    let Ok(text) = slurp_str(b"/proc/self/cgroup\0".as_ptr().cast(), &mut buf) else {
        return 0;
    };
    constrained_memory_from_text(text)
}

#[no_mangle]
pub unsafe extern "C" fn uv_get_available_memory() -> u64 {
    let mut buf = [0u8; 1024];
    let Ok(text) = slurp_str(b"/proc/self/cgroup\0".as_ptr().cast(), &mut buf) else {
        return 0;
    };

    let constrained = constrained_memory_from_text(text);
    if constrained == 0 {
        return uv_get_free_memory();
    }

    let total = uv_get_total_memory();
    if constrained > total {
        return uv_get_free_memory();
    }

    let current = if text.starts_with("0::/") {
        cgroup2_current_memory(text)
    } else {
        cgroup1_current_memory(text)
    };

    if constrained < current {
        0
    } else {
        constrained - current
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_loadavg(avg: *mut f64) {
    let mut buf = [0u8; 128];
    if let Ok(text) = slurp_str(b"/proc/loadavg\0".as_ptr().cast(), &mut buf) {
        let mut it = text.split_whitespace();
        if let (Some(a), Some(b), Some(c)) = (it.next(), it.next(), it.next()) {
            if let (Ok(a), Ok(b), Ok(c)) = (a.parse::<f64>(), b.parse::<f64>(), c.parse::<f64>()) {
                *avg.add(0) = a;
                *avg.add(1) = b;
                *avg.add(2) = c;
                return;
            }
        }
    }

    let mut info = mem::zeroed::<libc::sysinfo>();
    if libc::sysinfo(&mut info) < 0 {
        return;
    }

    *avg.add(0) = info.loads[0] as f64 / 65536.0;
    *avg.add(1) = info.loads[1] as f64 / 65536.0;
    *avg.add(2) = info.loads[2] as f64 / 65536.0;
}
