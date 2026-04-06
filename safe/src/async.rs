use crate::abi;
use crate::allocator;
use crate::bindings::*;
use crate::handle::{
    handle_init, handle_start, handle_stop, queue_empty, queue_head, queue_insert_tail,
    queue_move, queue_remove,
};
use crate::linux::epoll::{uv__io_init_impl, uv__io_start_impl, uv__io_stop_impl};
use std::mem::offset_of;
use std::sync::atomic::{AtomicI32, Ordering};

#[inline]
unsafe fn async_from_queue(q: *mut uv__queue) -> *mut uv_async_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_async_t, queue))
        .cast::<uv_async_t>()
}

#[inline]
unsafe fn pending_atomic(handle: *mut uv_async_t) -> &'static AtomicI32 {
    &*std::ptr::addr_of!((*handle).pending).cast::<AtomicI32>()
}

#[inline]
unsafe fn busy_atomic(handle: *mut uv_async_t) -> &'static AtomicI32 {
    &*std::ptr::addr_of!((*handle).u.fd).cast::<AtomicI32>()
}

unsafe fn uv__async_send(loop_: *mut uv_loop_t) {
    let value: u64 = 1;
    let fd = if (*loop_).async_wfd != -1 {
        (*loop_).async_wfd
    } else {
        (*loop_).async_io_watcher.fd
    };

    loop {
        let rc = libc::write(
            fd,
            std::ptr::addr_of!(value).cast::<libc::c_void>(),
            std::mem::size_of::<u64>(),
        );
        if rc == std::mem::size_of::<u64>() as isize {
            return;
        }
        if rc == -1 && *allocator::errno_location() == libc::EINTR {
            continue;
        }
        if rc == -1 {
            let err = *allocator::errno_location();
            if err == libc::EAGAIN || err == libc::EWOULDBLOCK {
                return;
            }
        }
        libc::abort();
    }
}

unsafe fn uv__async_start(loop_: *mut uv_loop_t) -> libc::c_int {
    if (*loop_).async_io_watcher.fd != -1 {
        return 0;
    }

    let fd = libc::eventfd(0, libc::EFD_CLOEXEC | libc::EFD_NONBLOCK);
    if fd < 0 {
        return allocator::last_error();
    }

    uv__io_init_impl(
        std::ptr::addr_of_mut!((*loop_).async_io_watcher),
        Some(uv__async_io),
        fd,
    );
    uv__io_start_impl(
        loop_,
        std::ptr::addr_of_mut!((*loop_).async_io_watcher),
        libc::POLLIN as libc::c_uint,
    );
    (*loop_).async_wfd = -1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_async_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_async_t,
    async_cb: uv_async_cb,
) -> libc::c_int {
    let err = uv__async_start(loop_);
    if err != 0 {
        return err;
    }

    handle_init(loop_, handle.cast(), uv_handle_type_UV_ASYNC);
    (*handle).async_cb = async_cb;
    (*handle).pending = 0;
    (*handle).u.fd = 0;

    queue_insert_tail(
        std::ptr::addr_of_mut!((*loop_).async_handles),
        std::ptr::addr_of_mut!((*handle).queue),
    );
    handle_start(handle.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_async_send(handle: *mut uv_async_t) -> libc::c_int {
    let pending = pending_atomic(handle);
    let busy = busy_atomic(handle);

    if pending.load(Ordering::Relaxed) != 0 {
        return 0;
    }

    busy.fetch_add(1, Ordering::Relaxed);
    if pending.swap(1, Ordering::AcqRel) == 0 {
        uv__async_send((*handle).loop_);
    }
    busy.fetch_sub(1, Ordering::Relaxed);

    0
}

unsafe fn uv__async_spin(handle: *mut uv_async_t) {
    let pending = pending_atomic(handle);
    let busy = busy_atomic(handle);
    pending.store(1, Ordering::SeqCst);

    loop {
        for _ in 0..997 {
            if busy.load(Ordering::Acquire) == 0 {
                return;
            }
            std::hint::spin_loop();
        }
        libc::sched_yield();
    }
}

pub(crate) unsafe extern "C" fn uv__async_close_impl(handle: *mut uv_async_t) {
    uv__async_spin(handle);
    queue_remove(std::ptr::addr_of_mut!((*handle).queue));
    handle_stop(handle.cast());
}

unsafe extern "C" fn uv__async_io(
    loop_: *mut uv_loop_t,
    w: *mut uv__io_t,
    _events: libc::c_uint,
) {
    debug_assert!(std::ptr::eq(w, std::ptr::addr_of_mut!((*loop_).async_io_watcher)));

    let mut value: u64 = 0;
    loop {
        let rc = libc::read(
            (*w).fd,
            std::ptr::addr_of_mut!(value).cast::<libc::c_void>(),
            std::mem::size_of::<u64>(),
        );
        if rc == -1 {
            let err = *allocator::errno_location();
            if err == libc::EAGAIN || err == libc::EWOULDBLOCK {
                break;
            }
            if err == libc::EINTR {
                continue;
            }
            libc::abort();
        }
        if rc == 0 {
            break;
        }
    }

    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).async_handles),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = async_from_queue(q);

        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).async_handles), q);

        if pending_atomic(handle).swap(0, Ordering::AcqRel) == 0 {
            continue;
        }

        if let Some(cb) = (*handle).async_cb {
            cb(handle);
        }
    }
}

pub(crate) unsafe extern "C" fn uv__async_stop_impl(loop_: *mut uv_loop_t) {
    if (*loop_).async_io_watcher.fd == -1 {
        return;
    }

    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).async_handles),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = async_from_queue(q);

        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).async_handles), q);
        uv__async_spin(handle);
    }

    if (*loop_).async_wfd != -1 {
        if (*loop_).async_wfd != (*loop_).async_io_watcher.fd {
            abi::uv__close((*loop_).async_wfd);
        }
        (*loop_).async_wfd = -1;
    }

    uv__io_stop_impl(
        loop_,
        std::ptr::addr_of_mut!((*loop_).async_io_watcher),
        libc::POLLIN as libc::c_uint,
    );
    abi::uv__close((*loop_).async_io_watcher.fd);
    (*loop_).async_io_watcher.fd = -1;
}

pub(crate) unsafe extern "C" fn uv__async_fork_impl(loop_: *mut uv_loop_t) -> libc::c_int {
    if (*loop_).async_io_watcher.fd == -1 {
        return 0;
    }

    let mut queue = std::mem::zeroed::<uv__queue>();
    queue_move(
        std::ptr::addr_of_mut!((*loop_).async_handles),
        std::ptr::addr_of_mut!(queue),
    );

    while !queue_empty(std::ptr::addr_of!(queue)) {
        let q = queue_head(std::ptr::addr_of!(queue));
        let handle = async_from_queue(q);

        queue_remove(q);
        queue_insert_tail(std::ptr::addr_of_mut!((*loop_).async_handles), q);
        (*handle).pending = 0;
        (*handle).u.fd = 0;
    }

    if (*loop_).async_wfd != -1 {
        if (*loop_).async_wfd != (*loop_).async_io_watcher.fd {
            abi::uv__close((*loop_).async_wfd);
        }
        (*loop_).async_wfd = -1;
    }

    uv__io_stop_impl(
        loop_,
        std::ptr::addr_of_mut!((*loop_).async_io_watcher),
        libc::POLLIN as libc::c_uint,
    );
    abi::uv__close((*loop_).async_io_watcher.fd);
    (*loop_).async_io_watcher.fd = -1;

    uv__async_start(loop_)
}
