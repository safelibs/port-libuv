use crate::abi::linux_x86_64 as abi;
use crate::core::{
    allocator, default_loop, loop_state, queue, remove_handle_record, timer, HandleKind,
    HandleRecord, LoopStateInner, UV_HANDLE_ACTIVE, UV_HANDLE_CLOSED, UV_HANDLE_CLOSING,
    UV_HANDLE_INTERNAL, UV_HANDLE_REF,
};
use crate::unix_async;
use std::mem::offset_of;
use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    fn fprintf(stream: *mut abi::FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut abi::FILE;
}

#[inline]
fn c_name(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr().cast()
}

pub(crate) fn handle_type_name_ptr(handle_type: abi::uv_handle_type) -> *const c_char {
    match handle_type {
        abi::uv_handle_type_UV_ASYNC => c_name(b"async\0"),
        abi::uv_handle_type_UV_CHECK => c_name(b"check\0"),
        abi::uv_handle_type_UV_FS_EVENT => c_name(b"fs_event\0"),
        abi::uv_handle_type_UV_FS_POLL => c_name(b"fs_poll\0"),
        abi::uv_handle_type_UV_HANDLE => c_name(b"handle\0"),
        abi::uv_handle_type_UV_IDLE => c_name(b"idle\0"),
        abi::uv_handle_type_UV_NAMED_PIPE => c_name(b"pipe\0"),
        abi::uv_handle_type_UV_POLL => c_name(b"poll\0"),
        abi::uv_handle_type_UV_PREPARE => c_name(b"prepare\0"),
        abi::uv_handle_type_UV_PROCESS => c_name(b"process\0"),
        abi::uv_handle_type_UV_STREAM => c_name(b"stream\0"),
        abi::uv_handle_type_UV_TCP => c_name(b"tcp\0"),
        abi::uv_handle_type_UV_TIMER => c_name(b"timer\0"),
        abi::uv_handle_type_UV_TTY => c_name(b"tty\0"),
        abi::uv_handle_type_UV_UDP => c_name(b"udp\0"),
        abi::uv_handle_type_UV_SIGNAL => c_name(b"signal\0"),
        abi::uv_handle_type_UV_FILE => c_name(b"file\0"),
        _ => std::ptr::null(),
    }
}

#[inline]
pub(crate) unsafe fn is_active(handle: *const abi::uv_handle_t) -> bool {
    !handle.is_null() && unsafe { ((*handle).flags & UV_HANDLE_ACTIVE) != 0 }
}

#[inline]
pub(crate) unsafe fn is_closing(handle: *const abi::uv_handle_t) -> bool {
    !handle.is_null() && unsafe { ((*handle).flags & (UV_HANDLE_CLOSING | UV_HANDLE_CLOSED)) != 0 }
}

#[inline]
pub(crate) unsafe fn has_ref(handle: *const abi::uv_handle_t) -> bool {
    !handle.is_null() && unsafe { ((*handle).flags & UV_HANDLE_REF) != 0 }
}

#[inline]
pub(crate) unsafe fn is_active_int(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { is_active(handle) as c_int }
}

#[inline]
pub(crate) unsafe fn is_closing_int(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { is_closing(handle) as c_int }
}

#[inline]
pub(crate) unsafe fn has_ref_int(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { has_ref(handle) as c_int }
}

pub(crate) unsafe fn handle_start(handle: *mut abi::uv_handle_t) {
    if unsafe { (*handle).flags & UV_HANDLE_ACTIVE } != 0 {
        return;
    }
    unsafe {
        (*handle).flags |= UV_HANDLE_ACTIVE;
        if (*handle).flags & UV_HANDLE_REF != 0 {
            (*(*handle).loop_).active_handles += 1;
        }
    }
}

pub(crate) unsafe fn handle_stop(handle: *mut abi::uv_handle_t) {
    if unsafe { (*handle).flags & UV_HANDLE_ACTIVE } == 0 {
        return;
    }
    unsafe {
        (*handle).flags &= !UV_HANDLE_ACTIVE;
        if (*handle).flags & UV_HANDLE_REF != 0 {
            (*(*handle).loop_).active_handles -= 1;
        }
    }
}

pub(crate) unsafe fn handle_ref(handle: *mut abi::uv_handle_t) {
    if handle.is_null() || unsafe { (*handle).flags & UV_HANDLE_REF } != 0 {
        return;
    }

    unsafe {
        (*handle).flags |= UV_HANDLE_REF;
        if (*handle).flags & UV_HANDLE_CLOSING == 0 && (*handle).flags & UV_HANDLE_ACTIVE != 0 {
            (*(*handle).loop_).active_handles += 1;
        }
    }
}

pub(crate) unsafe fn handle_unref(handle: *mut abi::uv_handle_t) {
    if handle.is_null() || unsafe { (*handle).flags & UV_HANDLE_REF } == 0 {
        return;
    }

    unsafe {
        (*handle).flags &= !UV_HANDLE_REF;
        if (*handle).flags & UV_HANDLE_CLOSING == 0 && (*handle).flags & UV_HANDLE_ACTIVE != 0 {
            (*(*handle).loop_).active_handles -= 1;
        }
    }
}

pub(crate) unsafe fn handle_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_handle_t,
    handle_type: abi::uv_handle_type,
    kind: HandleKind,
) -> c_int {
    if loop_.is_null() || handle.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }

    let data = unsafe { (*handle).data };
    let state = unsafe { &*loop_state(loop_) };
    let mut inner = state.inner.lock().unwrap();
    if !super::find_handle_record(&inner, handle).is_null() {
        return 0;
    }

    let record = unsafe {
        allocator::alloc_value(HandleRecord {
            next: std::ptr::null_mut(),
            handle,
            kind: kind as u8,
            _reserved: [0; 7],
            timer_generation: 0,
        })
    };
    if record.is_null() {
        return abi::uv_errno_t_UV_ENOMEM;
    }

    unsafe {
        super::push_handle_record(&mut inner, record);
        (*handle).loop_ = loop_;
        (*handle).type_ = handle_type;
        (*handle).close_cb = None;
        queue::init(std::ptr::addr_of_mut!((*handle).handle_queue));
        (*handle).next_closing = std::ptr::null_mut();
        (*handle).flags = UV_HANDLE_REF;
        (*handle).u.fd = 0;
        (*handle).data = data;
        queue::insert_tail(
            std::ptr::addr_of_mut!((*loop_).handle_queue),
            std::ptr::addr_of_mut!((*handle).handle_queue),
        );
    }

    0
}

pub(crate) unsafe fn prepare_init(
    loop_: *mut abi::uv_loop_t,
    prepare: *mut abi::uv_prepare_t,
) -> c_int {
    let rc = unsafe {
        handle_init(
            loop_,
            prepare.cast(),
            abi::uv_handle_type_UV_PREPARE,
            HandleKind::Prepare,
        )
    };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*prepare).prepare_cb = None;
        queue::init(std::ptr::addr_of_mut!((*prepare).queue));
    }
    0
}

pub(crate) unsafe fn prepare_start(
    prepare: *mut abi::uv_prepare_t,
    cb: abi::uv_prepare_cb,
) -> c_int {
    if prepare.is_null() || cb.is_none() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { is_active(prepare.cast()) } {
        return 0;
    }

    unsafe {
        (*prepare).prepare_cb = cb;
        queue::insert_head(
            std::ptr::addr_of_mut!((*(*prepare).loop_).prepare_handles),
            std::ptr::addr_of_mut!((*prepare).queue),
        );
        handle_start(prepare.cast());
    }
    0
}

pub(crate) unsafe fn prepare_stop(prepare: *mut abi::uv_prepare_t) -> c_int {
    if prepare.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { !is_active(prepare.cast()) } {
        unsafe {
            queue::init(std::ptr::addr_of_mut!((*prepare).queue));
        }
        return 0;
    }

    unsafe {
        queue::remove(std::ptr::addr_of_mut!((*prepare).queue));
        queue::init(std::ptr::addr_of_mut!((*prepare).queue));
        handle_stop(prepare.cast());
    }
    0
}

pub(crate) unsafe fn idle_init(loop_: *mut abi::uv_loop_t, idle: *mut abi::uv_idle_t) -> c_int {
    let rc = unsafe {
        handle_init(
            loop_,
            idle.cast(),
            abi::uv_handle_type_UV_IDLE,
            HandleKind::Idle,
        )
    };
    if rc != 0 {
        return rc;
    }
    unsafe {
        (*idle).idle_cb = None;
        queue::init(std::ptr::addr_of_mut!((*idle).queue));
    }
    0
}

pub(crate) unsafe fn idle_start(idle: *mut abi::uv_idle_t, cb: abi::uv_idle_cb) -> c_int {
    if idle.is_null() || cb.is_none() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { is_active(idle.cast()) } {
        return 0;
    }

    unsafe {
        (*idle).idle_cb = cb;
        queue::insert_head(
            std::ptr::addr_of_mut!((*(*idle).loop_).idle_handles),
            std::ptr::addr_of_mut!((*idle).queue),
        );
        handle_start(idle.cast());
    }
    0
}

pub(crate) unsafe fn idle_stop(idle: *mut abi::uv_idle_t) -> c_int {
    if idle.is_null() {
        return abi::uv_errno_t_UV_EINVAL;
    }
    if unsafe { !is_active(idle.cast()) } {
        unsafe {
            queue::init(std::ptr::addr_of_mut!((*idle).queue));
        }
        return 0;
    }

    unsafe {
        queue::remove(std::ptr::addr_of_mut!((*idle).queue));
        queue::init(std::ptr::addr_of_mut!((*idle).queue));
        handle_stop(idle.cast());
    }
    0
}

unsafe fn run_prepare_handles(loop_: *mut abi::uv_loop_t) {
    let mut local = abi::uv__queue::default();
    unsafe {
        queue::move_queue(
            std::ptr::addr_of_mut!((*loop_).prepare_handles),
            std::ptr::addr_of_mut!(local),
        );
    }

    while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
        let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
        let handle = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_prepare_t, queue))
                .cast::<abi::uv_prepare_t>()
        };
        unsafe {
            queue::remove(node);
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).prepare_handles),
                std::ptr::addr_of_mut!((*handle).queue),
            );
            if let Some(cb) = (*handle).prepare_cb {
                cb(handle);
            }
        }
    }
}

unsafe fn run_idle_handles(loop_: *mut abi::uv_loop_t) {
    let mut local = abi::uv__queue::default();
    unsafe {
        queue::move_queue(
            std::ptr::addr_of_mut!((*loop_).idle_handles),
            std::ptr::addr_of_mut!(local),
        );
    }

    while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
        let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
        let handle = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_idle_t, queue))
                .cast::<abi::uv_idle_t>()
        };
        unsafe {
            queue::remove(node);
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).idle_handles),
                std::ptr::addr_of_mut!((*handle).queue),
            );
            if let Some(cb) = (*handle).idle_cb {
                cb(handle);
            }
        }
    }
}

pub(crate) unsafe fn run_prepare(loop_: *mut abi::uv_loop_t) {
    unsafe { run_prepare_handles(loop_) };
}

pub(crate) unsafe fn run_idle(loop_: *mut abi::uv_loop_t) {
    unsafe { run_idle_handles(loop_) };
}

pub(crate) unsafe fn walk(
    loop_: *mut abi::uv_loop_t,
    walk_cb: abi::uv_walk_cb,
    arg: *mut std::ffi::c_void,
) {
    if loop_.is_null() || walk_cb.is_none() {
        return;
    }

    let mut local = abi::uv__queue::default();
    unsafe {
        queue::move_queue(
            std::ptr::addr_of_mut!((*loop_).handle_queue),
            std::ptr::addr_of_mut!(local),
        );
    }

    while unsafe { !queue::is_empty(std::ptr::addr_of!(local)) } {
        let node = unsafe { queue::head(std::ptr::addr_of_mut!(local)) };
        let handle = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_handle_t, handle_queue))
                .cast::<abi::uv_handle_t>()
        };
        unsafe {
            queue::remove(node);
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).handle_queue),
                std::ptr::addr_of_mut!((*handle).handle_queue),
            );
            if (*handle).flags & UV_HANDLE_INTERNAL == 0 {
                walk_cb.unwrap_unchecked()(handle, arg);
            }
        }
    }
}

unsafe fn type_label(handle: *mut abi::uv_handle_t) -> *const c_char {
    let ptr = handle_type_name_ptr(unsafe { (*handle).type_ });
    if ptr.is_null() {
        c_name(b"<unknown>\0")
    } else {
        ptr
    }
}

pub(crate) unsafe fn print_handles(
    mut loop_: *mut abi::uv_loop_t,
    only_active: bool,
    mut stream: *mut abi::FILE,
) {
    if loop_.is_null() {
        loop_ = unsafe { default_loop::default_loop() };
    }
    if loop_.is_null() {
        return;
    }
    if stream.is_null() {
        unsafe {
            stream = stderr;
        }
    }

    let head = unsafe { std::ptr::addr_of_mut!((*loop_).handle_queue) };
    let mut node = unsafe { (*head).next };
    while !std::ptr::eq(node, head) {
        let next = unsafe { (*node).next };
        let handle = unsafe {
            node.cast::<u8>()
                .sub(offset_of!(abi::uv_handle_t, handle_queue))
                .cast::<abi::uv_handle_t>()
        };

        if !only_active || unsafe { is_active(handle) } {
            let r = if unsafe { (*handle).flags & UV_HANDLE_REF } != 0 {
                b'R' as c_int
            } else {
                b'-' as c_int
            };
            let a = if unsafe { (*handle).flags & UV_HANDLE_ACTIVE } != 0 {
                b'A' as c_int
            } else {
                b'-' as c_int
            };
            let i = if unsafe { (*handle).flags & UV_HANDLE_INTERNAL } != 0 {
                b'I' as c_int
            } else {
                b'-' as c_int
            };
            unsafe {
                fprintf(
                    stream,
                    c_name(b"[%c%c%c] %-8s %p\n\0"),
                    r,
                    a,
                    i,
                    type_label(handle),
                    handle.cast::<std::ffi::c_void>(),
                );
            }
        }

        node = next;
    }
}

unsafe fn queue_close(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_handle_t) {
    unsafe {
        (*handle).next_closing = (*loop_).closing_handles;
        (*loop_).closing_handles = handle;
    }
}

pub(crate) unsafe fn close(handle: *mut abi::uv_handle_t, close_cb: abi::uv_close_cb) {
    if handle.is_null() || unsafe { is_closing(handle) } {
        return;
    }

    unsafe {
        (*handle).flags |= UV_HANDLE_CLOSING;
        (*handle).close_cb = close_cb;
    }

    match unsafe { (*handle).type_ } {
        abi::uv_handle_type_UV_TIMER => unsafe {
            timer::timer_close(handle.cast());
        },
        abi::uv_handle_type_UV_PREPARE => {
            let _ = unsafe { prepare_stop(handle.cast()) };
        }
        abi::uv_handle_type_UV_IDLE => {
            let _ = unsafe { idle_stop(handle.cast()) };
        }
        abi::uv_handle_type_UV_ASYNC => unsafe {
            unix_async::close(handle.cast());
        },
        _ => unsafe {
            handle_stop(handle);
        },
    }

    unsafe {
        queue_close((*handle).loop_, handle);
    }
}

unsafe fn finish_close(handle: *mut abi::uv_handle_t, inner: &mut LoopStateInner) {
    unsafe {
        (*handle).flags |= UV_HANDLE_CLOSED;
        handle_unref(handle);
        queue::remove(std::ptr::addr_of_mut!((*handle).handle_queue));
        queue::init(std::ptr::addr_of_mut!((*handle).handle_queue));
    }

    let record = unsafe { remove_handle_record(inner, handle) };
    if !record.is_null() {
        unsafe {
            allocator::free_bytes(record.cast());
        }
    }

    unsafe {
        if let Some(close_cb) = (*handle).close_cb {
            close_cb(handle);
        }
    }
}

pub(crate) unsafe fn run_closing_handles(loop_: *mut abi::uv_loop_t) {
    let state = unsafe { &*loop_state(loop_) };
    let mut inner = state.inner.lock().unwrap();

    let mut current = unsafe { (*loop_).closing_handles };
    unsafe {
        (*loop_).closing_handles = std::ptr::null_mut();
    }

    while !current.is_null() {
        let next = unsafe { (*current).next_closing };
        unsafe {
            (*current).next_closing = std::ptr::null_mut();
            finish_close(current, &mut inner);
        }
        current = next;
    }
}
