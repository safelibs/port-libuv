use crate::abi::linux_x86_64 as abi;
use libc::{self, c_char, c_int};
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::Mutex;

// SAFETY(syscall_ffi): these tty helpers are raw libc and libuv C interfaces.
unsafe extern "C" {
    // SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
    fn uv__close(fd: c_int) -> c_int;
    // SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
    fn uv__dup2_cloexec(fd: c_int, target: c_int) -> c_int;
    // SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
    fn uv__nonblock_ioctl(fd: c_int, set: c_int) -> c_int;
    // SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
    fn uv__open_cloexec(path: *const c_char, flags: c_int) -> c_int;
    // SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
    fn uv__stream_init(
        loop_: *mut abi::uv_loop_t,
        stream: *mut abi::uv_stream_t,
        type_: abi::uv_handle_type,
    );
    // SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
    fn uv__stream_open(stream: *mut abi::uv_stream_t, fd: c_int, flags: c_int) -> c_int;
}

static ORIG_TERMIOS_FD: AtomicI32 = AtomicI32::new(-1);
static ORIG_TERMIOS: Mutex<Option<abi::termios>> = Mutex::new(None);
static VTERM_STATE: AtomicU32 = AtomicU32::new(abi::uv_tty_vtermstate_t_UV_TTY_SUPPORTED);

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn tty_is_slave(fd: c_int) -> bool {
    let mut dummy = 0;
    unsafe { libc::ioctl(fd, libc::TIOCGPTN as _, &mut dummy) != 0 }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn tcsetattr_retry(fd: c_int, how: c_int, term: *const abi::termios) -> c_int {
    unsafe {
        loop {
            let rc = unsafe { libc::tcsetattr(fd, how, term.cast()) };
            if rc == 0 {
                return 0;
            }
            let err = unsafe { *libc::__errno_location() };
            if err != libc::EINTR {
                return -err;
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_vterm_state(state: *mut abi::uv_tty_vtermstate_t) -> c_int {
    unsafe {
        if state.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            *state = VTERM_STATE.load(Ordering::Relaxed);
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn set_vterm_state(state: abi::uv_tty_vtermstate_t) {
    unsafe {
        VTERM_STATE.store(state, Ordering::Relaxed);
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_winsize(
    handle: *mut abi::uv_tty_t,
    width: *mut c_int,
    height: *mut c_int,
) -> c_int {
    unsafe {
        if handle.is_null() || width.is_null() || height.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut ws = std::mem::MaybeUninit::<libc::winsize>::uninit();
        if unsafe {
            libc::ioctl(
                (*handle).io_watcher.fd,
                libc::TIOCGWINSZ as _,
                ws.as_mut_ptr(),
            )
        } != 0
        {
            return -unsafe { *libc::__errno_location() };
        }

        let ws = unsafe { ws.assume_init() };
        unsafe {
            *width = ws.ws_col as c_int;
            *height = ws.ws_row as c_int;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tty_t,
    fd: abi::uv_file,
    _readable: c_int,
) -> c_int {
    unsafe {
        if loop_.is_null() || handle.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let kind = unsafe { crate::unix::fd::guess_handle(fd) };
        if kind == abi::uv_handle_type_UV_FILE || kind == abi::uv_handle_type_UV_UNKNOWN_HANDLE {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let saved_flags = loop {
            let rc = unsafe { libc::fcntl(fd, libc::F_GETFL) };
            if rc >= 0 {
                break rc;
            }
            let err = unsafe { *libc::__errno_location() };
            if err != libc::EINTR {
                return -err;
            }
        };

        let mode = saved_flags & libc::O_ACCMODE;
        let mut stream_flags = 0;
        let mut open_fd = fd;
        let mut reopened = -1;

        if kind == abi::uv_handle_type_UV_TTY {
            let mut path = [0 as c_char; 256];
            if tty_is_slave(fd)
                && unsafe { libc::ttyname_r(fd, path.as_mut_ptr(), path.len()) } == 0
            {
                let rc = unsafe { uv__open_cloexec(path.as_ptr(), mode | libc::O_NOCTTY) };
                if rc >= 0 {
                    reopened = rc;
                    let dup_rc = unsafe { uv__dup2_cloexec(reopened, fd) };
                    if dup_rc < 0 && dup_rc != abi::uv_errno_t_UV_EINVAL {
                        unsafe {
                            uv__close(reopened);
                        }
                        return dup_rc;
                    }
                    open_fd = reopened;
                } else if mode != libc::O_RDONLY {
                    stream_flags |= crate::unix::stream::UV_HANDLE_BLOCKING_WRITES as c_int;
                }
            } else if mode != libc::O_RDONLY {
                stream_flags |= crate::unix::stream::UV_HANDLE_BLOCKING_WRITES as c_int;
            }
        }

        unsafe {
            uv__stream_init(loop_, handle.cast(), abi::uv_handle_type_UV_TTY);
        }

        if stream_flags & crate::unix::stream::UV_HANDLE_BLOCKING_WRITES as c_int == 0 {
            let rc = unsafe { uv__nonblock_ioctl(open_fd, 1) };
            if rc != 0 {
                if reopened >= 0 {
                    unsafe {
                        uv__close(reopened);
                    }
                }
                return rc;
            }
        }

        if mode != libc::O_WRONLY {
            stream_flags |= crate::unix::stream::UV_HANDLE_READABLE as c_int;
        }
        if mode != libc::O_RDONLY {
            stream_flags |= crate::unix::stream::UV_HANDLE_WRITABLE as c_int;
        }

        let rc = unsafe { uv__stream_open(handle.cast(), open_fd, stream_flags) };
        if rc != 0 {
            if reopened >= 0 {
                unsafe {
                    uv__close(reopened);
                }
            }
            return rc;
        }

        unsafe {
            (*handle).mode = abi::uv_tty_mode_t_UV_TTY_MODE_NORMAL as c_int;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn reset_mode() -> c_int {
    unsafe {
        let fd = ORIG_TERMIOS_FD.load(Ordering::Acquire);
        if fd < 0 {
            return 0;
        }

        let guard = ORIG_TERMIOS.lock().unwrap();
        let Some(term) = guard.as_ref() else {
            return 0;
        };

        let rc = unsafe { tcsetattr_retry(fd, libc::TCSANOW, term) };
        if rc == 0 {
            ORIG_TERMIOS_FD.store(-1, Ordering::Release);
        }
        rc
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn set_mode(handle: *mut abi::uv_tty_t, mode: abi::uv_tty_mode_t) -> c_int {
    unsafe {
        if handle.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let fd = unsafe { (*handle).io_watcher.fd };
        if fd < 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut term = std::mem::MaybeUninit::<abi::termios>::uninit();
        if unsafe { libc::tcgetattr(fd, term.as_mut_ptr().cast()) } != 0 {
            return -unsafe { *libc::__errno_location() };
        }
        let mut term = unsafe { term.assume_init() };

        {
            let mut guard = ORIG_TERMIOS.lock().unwrap();
            if guard.is_none() {
                *guard = Some(term);
                ORIG_TERMIOS_FD.store(fd, Ordering::Release);
            }
        }
        unsafe {
            (*handle).orig_termios = term;
        }

        match mode {
            abi::uv_tty_mode_t_UV_TTY_MODE_NORMAL => {
                if let Some(saved) = ORIG_TERMIOS.lock().unwrap().as_ref().copied() {
                    term = saved;
                }
            }
            abi::uv_tty_mode_t_UV_TTY_MODE_RAW | abi::uv_tty_mode_t_UV_TTY_MODE_IO => unsafe {
                libc::cfmakeraw((&mut term as *mut abi::termios).cast());
                term.c_oflag |= libc::OPOST as libc::tcflag_t;
            },
            _ => return abi::uv_errno_t_UV_EINVAL,
        }

        let rc = unsafe { tcsetattr_retry(fd, libc::TCSANOW, &term) };
        if rc == 0 {
            unsafe {
                (*handle).mode = mode as c_int;
            }
        }
        rc
    }
}
