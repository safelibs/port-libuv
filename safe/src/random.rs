use crate::allocator::{self, UV_E2BIG, UV_EINVAL};
use crate::bindings::*;
use crate::request::{req_register, req_unregister};
use crate::threadpool::{self, WorkKind};
use std::mem::offset_of;

static mut DEVURANDOM_ONCE: uv_once_t = libc::PTHREAD_ONCE_INIT;
static mut DEVURANDOM_STATUS: libc::c_int = 0;

#[repr(C)]
struct SysctlArgs {
    name: *mut libc::c_int,
    nlen: libc::c_int,
    oldval: *mut libc::c_void,
    oldlenp: *mut usize,
    newval: *mut libc::c_void,
    newlen: usize,
    unused: [usize; 4],
}

#[inline]
unsafe fn random_req_from_work(w: *mut uv__work) -> *mut uv_random_t {
    w.cast::<u8>()
        .sub(offset_of!(uv_random_t, work_req))
        .cast::<uv_random_t>()
}

unsafe fn random_readpath(
    path: *const libc::c_char,
    buf: *mut libc::c_void,
    buflen: usize,
) -> libc::c_int {
    let fd = libc::open(path, libc::O_RDONLY | libc::O_CLOEXEC);
    if fd < 0 {
        return allocator::last_error();
    }

    let mut statbuf = std::mem::zeroed::<libc::stat>();
    if libc::fstat(fd, &mut statbuf) != 0 {
        let err = allocator::last_error();
        libc::close(fd);
        return err;
    }
    if (statbuf.st_mode & libc::S_IFMT) != libc::S_IFCHR {
        libc::close(fd);
        return uv_errno_t_UV_EIO;
    }

    let mut pos = 0usize;
    while pos != buflen {
        let n = loop {
            let rc = libc::read(fd, buf.cast::<u8>().add(pos).cast(), buflen - pos);
            if rc == -1 && *allocator::errno_location() == libc::EINTR {
                continue;
            }
            break rc;
        };

        if n == -1 {
            let err = allocator::last_error();
            libc::close(fd);
            return err;
        }
        if n == 0 {
            libc::close(fd);
            return uv_errno_t_UV_EIO;
        }
        pos += n as usize;
    }

    libc::close(fd);
    0
}

unsafe extern "C" fn random_devurandom_init() {
    let mut byte = 0u8;
    DEVURANDOM_STATUS = random_readpath(
        b"/dev/random\0".as_ptr().cast(),
        (&mut byte as *mut u8).cast(),
        1,
    );
}

unsafe fn random_devurandom(buf: *mut libc::c_void, buflen: usize) -> libc::c_int {
    uv_once(
        std::ptr::addr_of_mut!(DEVURANDOM_ONCE),
        Some(random_devurandom_init),
    );
    if DEVURANDOM_STATUS != 0 {
        return DEVURANDOM_STATUS;
    }
    random_readpath(b"/dev/urandom\0".as_ptr().cast(), buf, buflen)
}

unsafe fn random_getrandom(buf: *mut libc::c_void, buflen: usize) -> libc::c_int {
    let mut pos = 0usize;
    while pos != buflen {
        let mut n = buflen - pos;
        if n > 256 {
            n = 256;
        }

        let rc = loop {
            let rc = libc::getrandom(buf.cast::<u8>().add(pos).cast(), n, 0);
            if rc == -1 && *allocator::errno_location() == libc::EINTR {
                continue;
            }
            break rc;
        };

        if rc == -1 {
            let err = *allocator::errno_location();
            if err == libc::ENOSYS {
                return uv_errno_t_UV_ENOSYS;
            }
            return -err;
        }
        if rc == 0 {
            return uv_errno_t_UV_EIO;
        }
        pos += rc as usize;
    }

    0
}

#[cfg(all(
    target_os = "linux",
    not(any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    ))
))]
unsafe fn random_sysctl(buf: *mut libc::c_void, buflen: usize) -> libc::c_int {
    let mut name = [1 as libc::c_int, 40 as libc::c_int, 6 as libc::c_int];
    let mut uuid = [0u8; 16];
    let mut p = buf.cast::<u8>();
    let pe = p.add(buflen);

    while p < pe {
        let mut n = uuid.len();
        let mut args = SysctlArgs {
            name: name.as_mut_ptr(),
            nlen: name.len() as libc::c_int,
            oldval: uuid.as_mut_ptr().cast(),
            oldlenp: std::ptr::addr_of_mut!(n),
            newval: std::ptr::null_mut(),
            newlen: 0,
            unused: [0; 4],
        };

        let rc = libc::syscall(libc::SYS__sysctl as libc::c_long, &mut args);
        if rc == -1 {
            return allocator::last_error();
        }
        if n != uuid.len() {
            return uv_errno_t_UV_EIO;
        }

        uuid[6] = uuid[14];
        uuid[8] = uuid[15];

        let mut chunk = pe.offset_from(p) as usize;
        if chunk > 14 {
            chunk = 14;
        }
        std::ptr::copy_nonoverlapping(uuid.as_ptr(), p, chunk);
        p = p.add(chunk);
    }

    0
}

#[cfg(not(all(
    target_os = "linux",
    not(any(
        target_arch = "aarch64",
        target_arch = "riscv64",
        target_arch = "loongarch64"
    ))
)))]
unsafe fn random_sysctl(_buf: *mut libc::c_void, _buflen: usize) -> libc::c_int {
    uv_errno_t_UV_ENOSYS
}

unsafe fn random_fill(buf: *mut libc::c_void, buflen: usize) -> libc::c_int {
    if buflen == 0 {
        return 0;
    }

    let mut rc = random_getrandom(buf, buflen);
    if rc == uv_errno_t_UV_ENOSYS {
        rc = random_devurandom(buf, buflen);
    }
    match rc {
        x if x == uv_errno_t_UV_EACCES
            || x == uv_errno_t_UV_EIO
            || x == uv_errno_t_UV_ELOOP
            || x == uv_errno_t_UV_EMFILE
            || x == uv_errno_t_UV_ENFILE
            || x == uv_errno_t_UV_ENOENT
            || x == uv_errno_t_UV_EPERM =>
        {
            let sysctl_rc = random_sysctl(buf, buflen);
            if sysctl_rc != uv_errno_t_UV_ENOSYS {
                return sysctl_rc;
            }
        }
        _ => {}
    }
    rc
}

unsafe extern "C" fn uv__random_work(w: *mut uv__work) {
    let req = random_req_from_work(w);
    (*req).status = random_fill((*req).buf, (*req).buflen);
}

unsafe extern "C" fn uv__random_done(w: *mut uv__work, status: libc::c_int) {
    let req = random_req_from_work(w);
    req_unregister((*req).loop_, req.cast());

    let mut status = status;
    if status == 0 {
        status = (*req).status;
    }

    if let Some(cb) = (*req).cb {
        cb(req, status, (*req).buf, (*req).buflen);
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_random(
    loop_: *mut uv_loop_t,
    req: *mut uv_random_t,
    buf: *mut libc::c_void,
    buflen: usize,
    flags: libc::c_uint,
    cb: uv_random_cb,
) -> libc::c_int {
    if buflen > 0x7FFF_FFFF {
        return UV_E2BIG;
    }
    if flags != 0 {
        return UV_EINVAL;
    }
    if cb.is_none() {
        return random_fill(buf, buflen);
    }
    if loop_.is_null() || req.is_null() {
        return UV_EINVAL;
    }

    (*req).type_ = uv_req_type_UV_RANDOM;
    (*req).loop_ = loop_;
    (*req).status = 0;
    (*req).buf = buf;
    (*req).buflen = buflen;
    (*req).cb = cb;
    req_register(loop_, req.cast());
    threadpool::uv__work_submit(
        loop_,
        std::ptr::addr_of_mut!((*req).work_req),
        WorkKind::Cpu,
        Some(uv__random_work),
        Some(uv__random_done),
    );
    0
}
