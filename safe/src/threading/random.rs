use crate::abi::linux_x86_64 as abi;
use crate::core::queue;
use crate::threading::threadpool::{self, TaskClass};
use libc::{self, c_char, c_int, c_long, c_void};
use std::mem::offset_of;
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Once;

static DEVURANDOM_INIT: Once = Once::new();
static DEVURANDOM_STATUS: AtomicI32 = AtomicI32::new(0);

const SYS__SYSCTL_X86_64: c_long = 156;

#[repr(C)]
struct SysctlArgs {
    name: *mut c_int,
    nlen: c_int,
    oldval: *mut c_void,
    oldlenp: *mut usize,
    newval: *mut c_void,
    newlen: usize,
    unused: [u64; 4],
}

#[inline]
fn last_errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

#[inline]
fn uv_err(errno: c_int) -> c_int {
    if errno == 0 { 0 } else { -errno }
}

fn random_readpath(path: &[u8], buf: *mut c_void, buflen: usize) -> c_int {
    let fd = unsafe { libc::open(path.as_ptr().cast::<c_char>(), libc::O_RDONLY | libc::O_CLOEXEC) };
    if fd < 0 {
        return uv_err(last_errno());
    }

    let mut st: abi::stat = unsafe { std::mem::zeroed() };
    if unsafe { libc::fstat(fd, (&mut st as *mut abi::stat).cast()) } != 0 {
        let err = uv_err(last_errno());
        unsafe {
            libc::close(fd);
        }
        return err;
    }

    if (st.st_mode as u32 & libc::S_IFMT) != libc::S_IFCHR {
        unsafe {
            libc::close(fd);
        }
        return abi::uv_errno_t_UV_EIO;
    }

    let mut pos = 0usize;
    while pos < buflen {
        let rc = unsafe {
            libc::read(
                fd,
                buf.cast::<u8>().add(pos).cast(),
                buflen.saturating_sub(pos),
            )
        };
        if rc == -1 {
            let err = last_errno();
            if err == libc::EINTR {
                continue;
            }
            unsafe {
                libc::close(fd);
            }
            return uv_err(err);
        }
        if rc == 0 {
            unsafe {
                libc::close(fd);
            }
            return abi::uv_errno_t_UV_EIO;
        }
        pos += rc as usize;
    }

    unsafe {
        libc::close(fd);
    }
    0
}

fn random_devurandom_init() {
    let mut byte = 0u8;
    let status = random_readpath(b"/dev/random\0", std::ptr::addr_of_mut!(byte).cast(), 1);
    DEVURANDOM_STATUS.store(status, Ordering::Release);
}

fn random_devurandom(buf: *mut c_void, buflen: usize) -> c_int {
    DEVURANDOM_INIT.call_once(random_devurandom_init);
    let status = DEVURANDOM_STATUS.load(Ordering::Acquire);
    if status != 0 {
        return status;
    }
    random_readpath(b"/dev/urandom\0", buf, buflen)
}

fn random_getrandom(buf: *mut c_void, buflen: usize) -> c_int {
    let mut pos = 0usize;
    while pos < buflen {
        let mut chunk = buflen - pos;
        if chunk > 256 {
            chunk = 256;
        }

        let rc = unsafe {
            libc::syscall(
                libc::SYS_getrandom as c_long,
                buf.cast::<u8>().add(pos),
                chunk,
                0u32,
            )
        };
        if rc == -1 {
            let err = last_errno();
            if err == libc::EINTR {
                continue;
            }
            if err == libc::ENOSYS {
                return abi::uv_errno_t_UV_ENOSYS;
            }
            return uv_err(err);
        }
        if rc == 0 {
            return abi::uv_errno_t_UV_EIO;
        }
        pos += rc as usize;
    }
    0
}

fn random_sysctl(buf: *mut c_void, buflen: usize) -> c_int {
    let mut name = [1, 40, 6];
    let mut pos = 0usize;
    let mut uuid = [0u8; 16];

    while pos < buflen {
        let mut len = uuid.len();
        let mut args = SysctlArgs {
            name: name.as_mut_ptr(),
            nlen: name.len() as c_int,
            oldval: uuid.as_mut_ptr().cast(),
            oldlenp: std::ptr::addr_of_mut!(len),
            newval: ptr::null_mut(),
            newlen: 0,
            unused: [0; 4],
        };

        if unsafe { libc::syscall(SYS__SYSCTL_X86_64, std::ptr::addr_of_mut!(args)) } == -1 {
            return uv_err(last_errno());
        }

        if len != uuid.len() {
            return abi::uv_errno_t_UV_EIO;
        }

        uuid[6] = uuid[14];
        uuid[8] = uuid[15];
        let remaining = buflen - pos;
        let chunk = remaining.min(14);
        unsafe {
            ptr::copy_nonoverlapping(uuid.as_ptr(), buf.cast::<u8>().add(pos), chunk);
        }
        pos += chunk;
    }

    0
}

fn fill_random(buf: *mut c_void, buflen: usize) -> c_int {
    let mut rc = random_getrandom(buf, buflen);
    if rc == abi::uv_errno_t_UV_ENOSYS {
        rc = random_devurandom(buf, buflen);
    }

    match rc {
        abi::uv_errno_t_UV_EACCES
        | abi::uv_errno_t_UV_EIO
        | abi::uv_errno_t_UV_ELOOP
        | abi::uv_errno_t_UV_EMFILE
        | abi::uv_errno_t_UV_ENFILE
        | abi::uv_errno_t_UV_ENOENT
        | abi::uv_errno_t_UV_EPERM => random_sysctl(buf, buflen),
        _ => rc,
    }
}

unsafe extern "C" fn random_work(work: *mut abi::uv__work) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_random_t, work_req))
            .cast::<abi::uv_random_t>()
    };
    unsafe {
        (*req).status = fill_random((*req).buf, (*req).buflen);
    }
}

unsafe extern "C" fn random_done(work: *mut abi::uv__work, status: c_int) {
    let req = unsafe {
        work.cast::<u8>()
            .sub(offset_of!(abi::uv_random_t, work_req))
            .cast::<abi::uv_random_t>()
    };
    let final_status = if status == 0 {
        unsafe { (*req).status }
    } else {
        status
    };
    if let Some(cb) = unsafe { (*req).cb } {
        unsafe {
            cb(req, final_status, (*req).buf, (*req).buflen);
        }
    }
}

pub(crate) unsafe fn random(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_random_t,
    buf: *mut c_void,
    buflen: usize,
    flags: libc::c_uint,
    cb: abi::uv_random_cb,
) -> c_int {
    if buflen > 0x7fff_ffff {
        return abi::uv_errno_t_UV_E2BIG;
    }
    if flags != 0 {
        return abi::uv_errno_t_UV_EINVAL;
    }

    if cb.is_none() {
        return fill_random(buf, buflen);
    }

    if loop_.is_null() || req.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let data = unsafe { (*req).data };
    unsafe {
        std::ptr::write_bytes(req, 0, 1);
        (*req).data = data;
        (*req).type_ = abi::uv_req_type_UV_RANDOM;
        (*req).loop_ = loop_;
        (*req).status = 0;
        (*req).buf = buf;
        (*req).buflen = buflen;
        (*req).cb = cb;
        (*req).work_req.work = Some(random_work);
        (*req).work_req.done = Some(random_done);
        queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
    }

    unsafe { threadpool::submit(loop_, req.cast(), std::ptr::addr_of_mut!((*req).work_req), TaskClass::Cpu) }
}
