use crate::abi::linux_x86_64 as abi;
use crate::core::{handle, queue, UV_HANDLE_INTERNAL, UV_HANDLE_REF};
use crate::upstream_support::unix_core;
use libc::{self, c_int, c_uint};
use std::mem::offset_of;
use std::sync::atomic::{AtomicI32, Ordering};

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn pending_atomic<'a>(handle: *mut abi::uv_async_t) -> &'a AtomicI32 {
    unsafe { &*(std::ptr::addr_of!((*handle).pending).cast::<AtomicI32>()) }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn busy_atomic<'a>(handle: *mut abi::uv_async_t) -> &'a AtomicI32 {
    unsafe { &*(std::ptr::addr_of!((*handle).u.fd).cast::<AtomicI32>()) }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn async_handle_from_queue(node: *mut abi::uv__queue) -> *mut abi::uv_async_t {
    unsafe {
        unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_async_t, queue))
                .cast::<abi::uv_async_t>()
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn drain_wakeup_fd(loop_: *mut abi::uv_loop_t) {
    unsafe {
        let fd = unsafe { (*loop_).async_io_watcher.fd };
        if fd < 0 {
            return;
        }

        if unsafe { (*loop_).async_wfd } == -1 {
            let mut value = 0u64;
            loop {
                let rc = unsafe {
                    libc::read(
                        fd,
                        std::ptr::addr_of_mut!(value).cast(),
                        std::mem::size_of::<u64>(),
                    )
                };
                if rc == -1 {
                    let err = unsafe { *libc::__errno_location() };
                    if err == libc::EINTR {
                        continue;
                    }
                    if err == libc::EAGAIN || err == libc::EWOULDBLOCK {
                        break;
                    }
                    unsafe {
                        libc::abort();
                    }
                }
                break;
            }
            return;
        }

        let mut buf = [0u8; 1024];
        loop {
            let rc = unsafe { libc::read(fd, buf.as_mut_ptr().cast(), buf.len()) };
            if rc as usize == buf.len() {
                continue;
            }
            if rc == -1 {
                let err = unsafe { *libc::__errno_location() };
                if err == libc::EINTR {
                    continue;
                }
                if err == libc::EAGAIN || err == libc::EWOULDBLOCK {
                    break;
                }
                unsafe {
                    libc::abort();
                }
            }
            break;
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn async_io(loop_: *mut abi::uv_loop_t, _watcher: *mut abi::uv__io_t, _events: c_uint) {
    unsafe {
        unsafe {
            run(loop_);
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn set_nonblocking_cloexec(fd: c_int) -> bool {
    unsafe {
        if libc::fcntl(fd, libc::F_SETFL, libc::O_NONBLOCK) != 0 {
            return false;
        }
        if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) != 0 {
            return false;
        }
    }
    true
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn start_backend(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe {
        let mut read_fd = unsafe { libc::eventfd(0, libc::EFD_CLOEXEC | libc::EFD_NONBLOCK) };
        let mut write_fd = -1;

        if read_fd < 0 {
            let mut fds = [0; 2];
            if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
                return -unsafe { *libc::__errno_location() };
            }

            if !set_nonblocking_cloexec(fds[0]) || !set_nonblocking_cloexec(fds[1]) {
                unsafe {
                    libc::close(fds[0]);
                    libc::close(fds[1]);
                }
                return -unsafe { *libc::__errno_location() };
            }

            read_fd = fds[0];
            write_fd = fds[1];
        }

        unsafe {
            unix_core::uv__io_init(
                std::ptr::addr_of_mut!((*loop_).async_io_watcher).cast(),
                std::mem::transmute::<abi::uv__io_cb, crate::upstream_support::unix_core::uv__io_cb>(
                    Some(async_io),
                ),
                read_fd,
            );
            unix_core::uv__io_start(
                loop_.cast(),
                std::ptr::addr_of_mut!((*loop_).async_io_watcher).cast(),
                libc::POLLIN as c_uint,
            );
            (*loop_).async_wfd = write_fd;
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn init_handle(loop_: *mut abi::uv_loop_t, handle_ptr: *mut abi::uv_async_t) {
    let data = unsafe { (*handle_ptr).data };
    unsafe {
        std::ptr::write_bytes(handle_ptr, 0, 1);
        (*handle_ptr).loop_ = loop_;
        (*handle_ptr).type_ = abi::uv_handle_type_UV_ASYNC;
        (*handle_ptr).close_cb = None;
        (*handle_ptr).flags = UV_HANDLE_REF;
        (*handle_ptr).data = data;
        (*handle_ptr).next_closing = std::ptr::null_mut();
        queue::init(std::ptr::addr_of_mut!((*handle_ptr).handle_queue));
        queue::insert_tail(
            std::ptr::addr_of_mut!((*loop_).handle_queue),
            std::ptr::addr_of_mut!((*handle_ptr).handle_queue),
        );
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ensure_backend(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe {
        if loop_.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        if unsafe { (*loop_).async_io_watcher.fd } != -1 {
            return 0;
        }
        unsafe { start_backend(loop_) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init_internal(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe {
        let rc = unsafe { ensure_backend(loop_) };
        if rc != 0 {
            return rc;
        }

        unsafe {
            std::ptr::write_bytes(std::ptr::addr_of_mut!((*loop_).wq_async), 0, 1);
            (*loop_).wq_async.loop_ = loop_;
            (*loop_).wq_async.type_ = abi::uv_handle_type_UV_ASYNC;
            (*loop_).wq_async.flags = UV_HANDLE_INTERNAL;
            (*loop_).wq_async.async_cb = Some(crate::threading::threadpool::loop_wq_async_cb);
            queue::init(std::ptr::addr_of_mut!((*loop_).wq_async.handle_queue));
            queue::init(std::ptr::addr_of_mut!((*loop_).wq_async.queue));
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).handle_queue),
                std::ptr::addr_of_mut!((*loop_).wq_async.handle_queue),
            );
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).async_handles),
                std::ptr::addr_of_mut!((*loop_).wq_async.queue),
            );
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init(
    loop_: *mut abi::uv_loop_t,
    handle_ptr: *mut abi::uv_async_t,
    cb: abi::uv_async_cb,
) -> c_int {
    unsafe {
        if loop_.is_null() || handle_ptr.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let rc = unsafe { ensure_backend(loop_) };
        if rc != 0 {
            return rc;
        }

        init_handle(loop_, handle_ptr);

        unsafe {
            (*handle_ptr).async_cb = cb;
            (*handle_ptr).pending = 0;
            (*handle_ptr).u.fd = 0;
            queue::init(std::ptr::addr_of_mut!((*handle_ptr).queue));
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).async_handles),
                std::ptr::addr_of_mut!((*handle_ptr).queue),
            );
            handle::handle_start(handle_ptr.cast());
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn send_wakeup(loop_: *mut abi::uv_loop_t) {
    let mut buf_ptr: *const libc::c_void = b"\0".as_ptr().cast();
    let mut len = 1usize;
    let mut fd = unsafe { (*loop_).async_wfd };

    if fd == -1 {
        static VALUE: u64 = 1;
        buf_ptr = std::ptr::addr_of!(VALUE).cast();
        len = std::mem::size_of::<u64>();
        fd = unsafe { (*loop_).async_io_watcher.fd };
    }

    loop {
        let rc = unsafe { libc::write(fd, buf_ptr, len) };
        if rc == len as isize {
            return;
        }
        if rc == -1 {
            let err = unsafe { *libc::__errno_location() };
            if err == libc::EINTR {
                continue;
            }
            if err == libc::EAGAIN || err == libc::EWOULDBLOCK {
                return;
            }
        }
        unsafe {
            libc::abort();
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn send(handle_ptr: *mut abi::uv_async_t) -> c_int {
    unsafe {
        if handle_ptr.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let pending = pending_atomic(handle_ptr);
        let busy = busy_atomic(handle_ptr);

        if pending.load(Ordering::Relaxed) != 0 {
            return 0;
        }

        busy.fetch_add(1, Ordering::AcqRel);
        if pending.swap(1, Ordering::AcqRel) == 0 {
            send_wakeup(unsafe { (*handle_ptr).loop_ });
        }
        busy.fetch_sub(1, Ordering::AcqRel);

        0
    }
}

fn spin_until_idle(handle_ptr: *mut abi::uv_async_t) {
    let pending = pending_atomic(handle_ptr);
    let busy = busy_atomic(handle_ptr);
    pending.store(1, Ordering::Release);

    loop {
        if busy.load(Ordering::Acquire) == 0 {
            return;
        }
        std::hint::spin_loop();
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn close(handle_ptr: *mut abi::uv_async_t) {
    unsafe {
        if handle_ptr.is_null() {
            return;
        }
        spin_until_idle(handle_ptr);
        unsafe {
            queue::remove(std::ptr::addr_of_mut!((*handle_ptr).queue));
            queue::init(std::ptr::addr_of_mut!((*handle_ptr).queue));
            handle::handle_stop(handle_ptr.cast());
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn has_pending(loop_: *mut abi::uv_loop_t) -> bool {
    unsafe {
        if loop_.is_null() {
            return false;
        }
        let head = unsafe { std::ptr::addr_of!((*loop_).async_handles) };
        let mut node = unsafe { (*head).next };
        while !std::ptr::eq(node, head.cast_mut()) {
            let handle_ptr = unsafe { async_handle_from_queue(node) };
            if pending_atomic(handle_ptr).load(Ordering::Acquire) != 0 {
                return true;
            }
            node = unsafe { (*node).next };
        }
        false
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn run(loop_: *mut abi::uv_loop_t) {
    unsafe {
        if loop_.is_null() || unsafe { (*loop_).async_io_watcher.fd } == -1 {
            return;
        }

        unsafe {
            drain_wakeup_fd(loop_);
        }

        let mut local = abi::uv__queue::default();
        unsafe {
            queue::move_queue(
                std::ptr::addr_of_mut!((*loop_).async_handles),
                std::ptr::addr_of_mut!(local),
            );
        }

        while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
            let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
            let handle_ptr = unsafe { async_handle_from_queue(node) };
            unsafe {
                queue::remove(node);
                queue::insert_tail(
                    std::ptr::addr_of_mut!((*loop_).async_handles),
                    std::ptr::addr_of_mut!((*handle_ptr).queue),
                );
            }

            if pending_atomic(handle_ptr).swap(0, Ordering::AcqRel) == 0 {
                continue;
            }

            if let Some(cb) = unsafe { (*handle_ptr).async_cb } {
                unsafe {
                    cb(handle_ptr);
                }
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn shutdown(loop_: *mut abi::uv_loop_t) {
    unsafe {
        if loop_.is_null() || unsafe { (*loop_).async_io_watcher.fd } == -1 {
            return;
        }

        let mut local = abi::uv__queue::default();
        unsafe {
            queue::move_queue(
                std::ptr::addr_of_mut!((*loop_).async_handles),
                std::ptr::addr_of_mut!(local),
            );
        }

        while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
            let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
            let handle_ptr = unsafe { async_handle_from_queue(node) };
            unsafe {
                queue::remove(node);
                queue::insert_tail(
                    std::ptr::addr_of_mut!((*loop_).async_handles),
                    std::ptr::addr_of_mut!((*handle_ptr).queue),
                );
            }
            spin_until_idle(handle_ptr);
        }

        unsafe {
            if (*loop_).async_wfd != -1 && (*loop_).async_wfd != (*loop_).async_io_watcher.fd {
                libc::close((*loop_).async_wfd);
            }
            (*loop_).async_wfd = -1;
            unix_core::uv__io_stop(
                loop_.cast(),
                std::ptr::addr_of_mut!((*loop_).async_io_watcher).cast(),
                libc::POLLIN as c_uint,
            );
            libc::close((*loop_).async_io_watcher.fd);
            (*loop_).async_io_watcher.fd = -1;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fork(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe {
        if loop_.is_null() || unsafe { (*loop_).async_io_watcher.fd } == -1 {
            return 0;
        }

        let mut local = abi::uv__queue::default();
        unsafe {
            queue::move_queue(
                std::ptr::addr_of_mut!((*loop_).async_handles),
                std::ptr::addr_of_mut!(local),
            );
        }

        while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
            let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
            let handle_ptr = unsafe { async_handle_from_queue(node) };
            unsafe {
                queue::remove(node);
                queue::insert_tail(
                    std::ptr::addr_of_mut!((*loop_).async_handles),
                    std::ptr::addr_of_mut!((*handle_ptr).queue),
                );
                (*handle_ptr).pending = 0;
                (*handle_ptr).u.fd = 0;
            }
        }

        unsafe {
            if (*loop_).async_wfd != -1 && (*loop_).async_wfd != (*loop_).async_io_watcher.fd {
                libc::close((*loop_).async_wfd);
            }
            (*loop_).async_wfd = -1;
            unix_core::uv__io_stop(
                loop_.cast(),
                std::ptr::addr_of_mut!((*loop_).async_io_watcher).cast(),
                libc::POLLIN as c_uint,
            );
            libc::close((*loop_).async_io_watcher.fd);
            (*loop_).async_io_watcher.fd = -1;
        }

        unsafe { ensure_backend(loop_) }
    }
}
