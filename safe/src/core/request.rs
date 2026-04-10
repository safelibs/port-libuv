use crate::abi::linux_x86_64 as abi;
use crate::core::{
    allocator, loop_state, queue, remove_request_record, RequestKind, RequestRecord,
};
use std::mem::offset_of;
use std::os::raw::{c_char, c_int};

#[inline]
fn c_name(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr().cast()
}

pub(crate) fn req_type_name_ptr(req_type: abi::uv_req_type) -> *const c_char {
    match req_type {
        abi::uv_req_type_UV_REQ => c_name(b"req\0"),
        abi::uv_req_type_UV_CONNECT => c_name(b"connect\0"),
        abi::uv_req_type_UV_WRITE => c_name(b"write\0"),
        abi::uv_req_type_UV_SHUTDOWN => c_name(b"shutdown\0"),
        abi::uv_req_type_UV_UDP_SEND => c_name(b"udp_send\0"),
        abi::uv_req_type_UV_FS => c_name(b"fs\0"),
        abi::uv_req_type_UV_WORK => c_name(b"work\0"),
        abi::uv_req_type_UV_GETADDRINFO => c_name(b"getaddrinfo\0"),
        abi::uv_req_type_UV_GETNAMEINFO => c_name(b"getnameinfo\0"),
        abi::uv_req_type_UV_RANDOM => c_name(b"random\0"),
        _ => std::ptr::null(),
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn queue_work(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_work_t,
    work_cb: abi::uv_work_cb,
    after_work_cb: abi::uv_after_work_cb,
) -> c_int {
    unsafe {
        if loop_.is_null() || req.is_null() || work_cb.is_none() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let state = unsafe { &*loop_state(loop_) };
        let mut inner = state.inner.lock().unwrap();
        if !super::find_request_record(&inner, req.cast()).is_null() {
            return abi::uv_errno_t_UV_EBUSY;
        }

        let record = unsafe {
            allocator::alloc_value(RequestRecord {
                next: std::ptr::null_mut(),
                req: req.cast(),
                kind: RequestKind::Work as u8,
                _reserved: [0; 7],
            })
        };
        if record.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        unsafe {
            super::push_request_record(&mut inner, record);
            (*req).type_ = abi::uv_req_type_UV_WORK;
            (*req).loop_ = loop_;
            (*req).work_cb = work_cb;
            (*req).after_work_cb = after_work_cb;
            (*req).work_req.loop_ = loop_;
            (*req).work_req.work = None;
            (*req).work_req.done = None;
            queue::init(std::ptr::addr_of_mut!((*req).work_req.wq));
            (*loop_).active_reqs.count += 1;
        }
        drop(inner);

        let req_addr = req as usize;
        std::thread::spawn(move || unsafe {
            let req = req_addr as *mut abi::uv_work_t;
            if let Some(cb) = (*req).work_cb {
                cb(req);
            }

            let loop_ = (*req).loop_;
            let state = &*loop_state(loop_);
            let _guard = state.inner.lock().unwrap();
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).pending_queue),
                std::ptr::addr_of_mut!((*req).work_req.wq),
            );
            state.wake.notify_one();
        });

        0
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn run_pending(loop_: *mut abi::uv_loop_t) {
    unsafe {
        let state = unsafe { &*loop_state(loop_) };
        let mut local = abi::uv__queue::default();
        unsafe {
            queue::init(std::ptr::addr_of_mut!(local));
        }

        {
            let _guard = state.inner.lock().unwrap();
            unsafe {
                queue::move_queue(
                    std::ptr::addr_of_mut!((*loop_).pending_queue),
                    std::ptr::addr_of_mut!(local),
                );
            }
        }

        while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
            let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
            let work = unsafe {
                node.cast::<u8>()
                    .sub(offset_of!(abi::uv__work, wq))
                    .cast::<abi::uv__work>()
            };
            let req = unsafe {
                work.cast::<u8>()
                    .sub(offset_of!(abi::uv_work_t, work_req))
                    .cast::<abi::uv_work_t>()
            };

            unsafe {
                queue::remove(node);
                queue::init(node);
            }

            {
                let mut inner = state.inner.lock().unwrap();
                let record = unsafe { remove_request_record(&mut inner, req.cast()) };
                if !record.is_null() {
                    unsafe {
                        allocator::free_bytes(record.cast());
                    }
                }
            }

            unsafe {
                (*loop_).active_reqs.count -= 1;
                if let Some(cb) = (*req).after_work_cb {
                    cb(req, 0);
                }
            }
        }
    }
}
