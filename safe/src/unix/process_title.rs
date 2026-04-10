use crate::abi::linux_x86_64 as abi;
use crate::core::allocator;
use std::os::raw::{c_char, c_int};
use std::sync::{Mutex, OnceLock};

// SAFETY(syscall_ffi): `prctl` is a raw libc varargs interface used for Linux process naming.
unsafe extern "C" {
    fn prctl(option: c_int, ...) -> c_int;
}

#[derive(Default)]
struct ProcessTitleState {
    args_mem: *mut *mut c_char,
    title_ptr: *mut c_char,
    len: usize,
    cap: usize,
}

// SAFETY(syscall_ffi): the raw pointers live behind the global mutex and are only mutated while locked.
unsafe impl Send for ProcessTitleState {}

fn state() -> &'static Mutex<ProcessTitleState> {
    static STATE: OnceLock<Mutex<ProcessTitleState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(ProcessTitleState::default()))
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn strlen(ptr: *const c_char) -> usize {
    unsafe { libc::strlen(ptr) }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn set_process_name(title: *const c_char) {
    unsafe {
        #[cfg(target_os = "linux")]
        unsafe {
            let _ = prctl(libc::PR_SET_NAME, title);
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn setup_args(argc: c_int, argv: *mut *mut c_char) -> *mut *mut c_char {
    unsafe {
        if argc <= 0 || argv.is_null() {
            return argv;
        }

        let argc = argc as usize;
        let title_ptr = unsafe { *argv };
        let title_len = strlen(title_ptr);
        let mut copy_bytes = title_len + 1;

        for i in 1..argc {
            copy_bytes += strlen(unsafe { *argv.add(i) }) + 1;
        }

        let alloc_bytes = copy_bytes + (argc + 1) * std::mem::size_of::<*mut c_char>();
        let new_argv = unsafe { allocator::malloc_bytes(alloc_bytes) }.cast::<*mut c_char>();
        if new_argv.is_null() {
            return argv;
        }

        let mut dst = unsafe { new_argv.add(argc + 1).cast::<c_char>() };
        let mut copied_size = title_len + 1;
        for i in 0..argc {
            if i != 0 {
                copied_size = strlen(unsafe { *argv.add(i) }) + 1;
            }

            unsafe {
                std::ptr::copy_nonoverlapping(*argv.add(i), dst, copied_size);
                *new_argv.add(i) = dst;
                dst = dst.add(copied_size);
            }
        }

        unsafe {
            *new_argv.add(argc) = std::ptr::null_mut();
        }

        let cap = if argc == 0 {
            0
        } else {
            let last = unsafe { *argv.add(argc - 1) };
            let last_size = strlen(last) + 1;
            last as usize + last_size - title_ptr as usize
        };

        let mut guard = state().lock().unwrap();
        if !guard.args_mem.is_null() {
            unsafe {
                allocator::free_bytes(guard.args_mem.cast());
            }
        }

        guard.args_mem = new_argv;
        guard.title_ptr = title_ptr;
        guard.len = title_len;
        guard.cap = cap;

        new_argv
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn set_process_title(title: *const c_char) -> c_int {
    unsafe {
        if title.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut guard = state().lock().unwrap();
        if guard.args_mem.is_null() || guard.title_ptr.is_null() {
            return abi::uv_errno_t_UV_ENOBUFS;
        }

        let mut len = strlen(title);
        if len >= guard.cap {
            len = guard.cap.saturating_sub(1);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(title, guard.title_ptr, len);
            std::ptr::write_bytes(guard.title_ptr.add(len), 0, guard.cap.saturating_sub(len));
            set_process_name(guard.title_ptr);
        }

        guard.len = len;
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn get_process_title(buffer: *mut c_char, size: usize) -> c_int {
    unsafe {
        if buffer.is_null() || size == 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let guard = state().lock().unwrap();
        if guard.args_mem.is_null() || guard.title_ptr.is_null() {
            return abi::uv_errno_t_UV_ENOBUFS;
        }

        if size <= guard.len {
            return abi::uv_errno_t_UV_ENOBUFS;
        }

        unsafe {
            if guard.len != 0 {
                std::ptr::copy_nonoverlapping(guard.title_ptr, buffer, guard.len + 1);
            }
            *buffer.add(guard.len) = 0;
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cleanup() {
    unsafe {
        let mut guard = state().lock().unwrap();
        if !guard.args_mem.is_null() {
            unsafe {
                allocator::free_bytes(guard.args_mem.cast());
            }
        }

        *guard = ProcessTitleState::default();
    }
}
