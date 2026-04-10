use crate::abi::linux_x86_64 as abi;
use crate::core::queue;
use crate::threading::sync;

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn reset_loop_mutex(loop_: *mut abi::uv_loop_t) -> i32 {
    unsafe {
        unsafe {
            std::ptr::write_bytes(std::ptr::addr_of_mut!((*loop_).wq_mutex), 0, 1);
            sync::mutex_init_raw(std::ptr::addr_of_mut!((*loop_).wq_mutex), false)
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn reset_loop_rwlock(loop_: *mut abi::uv_loop_t) -> i32 {
    unsafe {
        unsafe {
            std::ptr::write_bytes(std::ptr::addr_of_mut!((*loop_).cloexec_lock), 0, 1);
            sync::rwlock_init_raw(std::ptr::addr_of_mut!((*loop_).cloexec_lock))
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn reset_threadpool_completion_state(loop_: *mut abi::uv_loop_t) {
    unsafe {
        unsafe {
            queue::init(std::ptr::addr_of_mut!((*loop_).wq));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_fork(loop_: *mut abi::uv_loop_t) -> i32 {
    unsafe {
        let rc = unsafe { crate::upstream_support::unix_loop::uv_loop_fork(loop_.cast()) };
        if rc != 0 {
            return rc;
        }

        let rc = unsafe { reset_loop_mutex(loop_) };
        if rc != 0 {
            return rc;
        }

        let rc = unsafe { reset_loop_rwlock(loop_) };
        if rc != 0 {
            return rc;
        }

        unsafe {
            reset_threadpool_completion_state(loop_);
        }

        0
    }
}
