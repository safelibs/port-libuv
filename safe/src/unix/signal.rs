use crate::abi::linux_x86_64 as abi;
use crate::core::queue;
use crate::upstream_support::unix_core;
use std::cell::UnsafeCell;
use std::mem::offset_of;
use std::os::raw::{c_int, c_uint, c_void};
use std::ptr;
use std::sync::Once;

const UV_HANDLE_CLOSING: u32 = crate::core::UV_HANDLE_CLOSING;
const UV_HANDLE_ACTIVE: u32 = crate::core::UV_HANDLE_ACTIVE;
const UV_HANDLE_REF: u32 = crate::core::UV_HANDLE_REF;
const UV_SIGNAL_ONE_SHOT: u32 = crate::upstream_support::unix_core::UV_SIGNAL_ONE_SHOT as u32;

#[repr(C)]
#[derive(Clone, Copy)]
struct SignalMsg {
    handle: *mut abi::uv_signal_t,
    signum: c_int,
}

struct SignalRegistry {
    watchers: UnsafeCell<Vec<WatcherKey>>,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct WatcherKey {
    signum: c_int,
    oneshot: bool,
    loop_ptr: usize,
    handle_ptr: usize,
}

// SAFETY(syscall_ffi): the registry stores stable libuv handle pointers and mutates only under the signal lock.
unsafe impl Sync for SignalRegistry {}

static REGISTRY: SignalRegistry = SignalRegistry {
    watchers: UnsafeCell::new(Vec::new()),
};
static GLOBAL_INIT: Once = Once::new();
static mut SIGNAL_LOCK_PIPEFD: [c_int; 2] = [-1, -1];

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn last_errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn watcher_key(handle: *mut abi::uv_signal_t) -> WatcherKey {
    unsafe {
        WatcherKey {
            signum: (*handle).signum,
            oneshot: ((*handle).flags & UV_SIGNAL_ONE_SHOT) != 0,
            loop_ptr: (*handle).loop_ as usize,
            handle_ptr: handle as usize,
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn handle_start(handle: *mut abi::uv_handle_t) {
    unsafe {
        if (*handle).flags & UV_HANDLE_ACTIVE != 0 {
            return;
        }

        (*handle).flags |= UV_HANDLE_ACTIVE;
        if (*handle).flags & UV_HANDLE_REF != 0 {
            (*(*handle).loop_).active_handles += 1;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn handle_stop(handle: *mut abi::uv_handle_t) {
    unsafe {
        if (*handle).flags & UV_HANDLE_ACTIVE == 0 {
            return;
        }

        (*handle).flags &= !UV_HANDLE_ACTIVE;
        if (*handle).flags & UV_HANDLE_REF != 0 {
            (*(*handle).loop_).active_handles -= 1;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn close_fd(fd: c_int) {
    unsafe {
        if fd >= 0 {
            let _ = unsafe { libc::close(fd) };
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn signal_unlock() -> bool {
    unsafe {
        let data = 42u8;
        loop {
            let rc = unsafe {
                libc::write(
                    SIGNAL_LOCK_PIPEFD[1],
                    std::ptr::addr_of!(data).cast::<c_void>(),
                    1,
                )
            };
            if rc >= 0 {
                return true;
            }
            if last_errno() != libc::EINTR {
                return false;
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn signal_lock() -> bool {
    unsafe {
        let mut data = 0u8;
        loop {
            let rc = unsafe {
                libc::read(
                    SIGNAL_LOCK_PIPEFD[0],
                    std::ptr::addr_of_mut!(data).cast::<c_void>(),
                    1,
                )
            };
            if rc >= 0 {
                return true;
            }
            if last_errno() != libc::EINTR {
                return false;
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn global_reinit() {
    unsafe {
        unsafe { cleanup_global() };

        if unsafe { libc::pipe2(SIGNAL_LOCK_PIPEFD.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
            unsafe { libc::abort() };
        }

        if !unsafe { signal_unlock() } {
            unsafe { libc::abort() };
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn after_fork_child() {
    unsafe {
        unsafe { global_reinit() };
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn ensure_global_init() {
    GLOBAL_INIT.call_once(|| {
        if unsafe { libc::pthread_atfork(None, None, Some(after_fork_child)) } != 0 {
            unsafe { libc::abort() };
        }
        unsafe { global_reinit() };
    });
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn block_and_lock(saved_sigmask: *mut libc::sigset_t) {
    unsafe {
        let mut mask = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
        if unsafe { libc::sigfillset(mask.as_mut_ptr()) } != 0 {
            unsafe { libc::abort() };
        }
        if unsafe { libc::sigemptyset(saved_sigmask) } != 0 {
            unsafe { libc::abort() };
        }
        if unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, mask.as_ptr(), saved_sigmask) } != 0 {
            unsafe { libc::abort() };
        }
        if !unsafe { signal_lock() } {
            unsafe { libc::abort() };
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn unlock_and_unblock(saved_sigmask: *const libc::sigset_t) {
    unsafe {
        if !unsafe { signal_unlock() } {
            unsafe { libc::abort() };
        }
        if unsafe { libc::pthread_sigmask(libc::SIG_SETMASK, saved_sigmask, ptr::null_mut()) } != 0
        {
            unsafe { libc::abort() };
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn signal_lower_bound(watchers: &[WatcherKey], signum: c_int) -> usize {
    watchers.partition_point(|candidate| candidate.signum < signum)
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn first_watcher_in(watchers: &[WatcherKey], signum: c_int) -> Option<WatcherKey> {
    let first = signal_lower_bound(watchers, signum);
    if first < watchers.len() && watchers[first].signum == signum {
        Some(watchers[first])
    } else {
        None
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn insert_watcher(handle: *mut abi::uv_signal_t) {
    let key = watcher_key(handle);
    let watchers = unsafe { &mut *REGISTRY.watchers.get() };

    if let Some(index) = watchers
        .iter()
        .position(|candidate| candidate.handle_ptr == handle as usize)
    {
        watchers.remove(index);
    }

    let insert_at = watchers.binary_search(&key).unwrap_or_else(|index| index);
    watchers.insert(insert_at, key);
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn first_watcher(signum: c_int) -> Option<WatcherKey> {
    let watchers = unsafe { &*REGISTRY.watchers.get() };
    first_watcher_in(watchers, signum)
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn reserve_watcher_slot() -> c_int {
    let watchers = unsafe { &mut *REGISTRY.watchers.get() };
    if watchers.try_reserve(1).is_err() {
        abi::uv_errno_t_UV_ENOMEM
    } else {
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn register_handler(signum: c_int, oneshot: bool) -> c_int {
    unsafe {
        let mut action = std::mem::MaybeUninit::<libc::sigaction>::zeroed();
        let action_ptr = action.as_mut_ptr();

        if unsafe { libc::sigfillset(std::ptr::addr_of_mut!((*action_ptr).sa_mask)) } != 0 {
            unsafe { libc::abort() };
        }

        unsafe {
            (*action_ptr).sa_sigaction = signal_handler as usize;
            (*action_ptr).sa_flags =
                libc::SA_RESTART | if oneshot { libc::SA_RESETHAND } else { 0 };
        }

        if unsafe { libc::sigaction(signum, action_ptr, ptr::null_mut()) } != 0 {
            -last_errno()
        } else {
            0
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn unregister_handler(signum: c_int) {
    unsafe {
        let mut action = std::mem::MaybeUninit::<libc::sigaction>::zeroed();
        let action_ptr = action.as_mut_ptr();

        unsafe {
            (*action_ptr).sa_sigaction = libc::SIG_DFL;
        }

        if unsafe { libc::sigaction(signum, action_ptr, ptr::null_mut()) } != 0 {
            unsafe { libc::abort() };
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn signal_handler(signum: c_int) {
    unsafe {
        let saved_errno = last_errno();
        let mut message = SignalMsg {
            handle: ptr::null_mut(),
            signum,
        };

        if !unsafe { signal_lock() } {
            unsafe {
                *libc::__errno_location() = saved_errno;
            }
            return;
        }

        let watchers = unsafe { &*REGISTRY.watchers.get() };
        let first = signal_lower_bound(watchers, signum);
        for &watcher in watchers[first..]
            .iter()
            .take_while(|watcher| watcher.signum == signum)
        {
            if watcher.signum != signum {
                continue;
            }

            let handle = watcher.handle_ptr as *mut abi::uv_signal_t;
            message.handle = handle;
            let rc = loop {
                let rc = unsafe {
                    libc::write(
                        (*(*handle).loop_).signal_pipefd[1],
                        std::ptr::addr_of!(message).cast::<c_void>(),
                        std::mem::size_of::<SignalMsg>(),
                    )
                };
                if rc == -1 && last_errno() == libc::EINTR {
                    continue;
                }
                break rc;
            };

            if rc == std::mem::size_of::<SignalMsg>() as isize {
                unsafe {
                    (*handle).caught_signals = (*handle).caught_signals.wrapping_add(1);
                }
            }
        }

        let _ = unsafe { signal_unlock() };
        unsafe {
            *libc::__errno_location() = saved_errno;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn signal_loop_once_init(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe {
        if unsafe { (*loop_).signal_pipefd[0] } != -1 {
            return 0;
        }

        let mut pipefd = [-1; 2];
        if unsafe { libc::pipe2(pipefd.as_mut_ptr(), libc::O_NONBLOCK | libc::O_CLOEXEC) } != 0 {
            return -last_errno();
        }

        unsafe {
            (*loop_).signal_pipefd = pipefd;
            unix_core::uv__io_init(
                std::ptr::addr_of_mut!((*loop_).signal_io_watcher).cast(),
                std::mem::transmute::<abi::uv__io_cb, crate::upstream_support::unix_core::uv__io_cb>(
                    Some(signal_event),
                ),
                pipefd[0],
            );
            unix_core::uv__io_start(
                loop_.cast(),
                std::ptr::addr_of_mut!((*loop_).signal_io_watcher).cast(),
                libc::POLLIN as c_uint,
            );
        }

        0
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn signal_event(
    loop_: *mut abi::uv_loop_t,
    watcher: *mut abi::uv__io_t,
    _events: c_uint,
) {
    unsafe {
        let _ = loop_;
        let _ = watcher;

        const MSGS_PER_READ: usize = 32;
        const MSG_SIZE: usize = std::mem::size_of::<SignalMsg>();

        let mut buffer = [0u8; MSGS_PER_READ * MSG_SIZE];
        let mut bytes = 0usize;

        loop {
            let rc = unsafe {
                libc::read(
                    (*loop_).signal_pipefd[0],
                    buffer[bytes..].as_mut_ptr().cast::<c_void>(),
                    buffer.len() - bytes,
                )
            };

            if rc == -1 && last_errno() == libc::EINTR {
                continue;
            }

            if rc == -1 && (last_errno() == libc::EAGAIN || last_errno() == libc::EWOULDBLOCK) {
                if bytes == 0 {
                    return;
                }
                continue;
            }

            if rc == -1 {
                unsafe { libc::abort() };
            }

            bytes += rc as usize;
            let end = (bytes / MSG_SIZE) * MSG_SIZE;
            let mut offset = 0usize;

            while offset < end {
                let message = unsafe {
                    std::ptr::read_unaligned(buffer.as_ptr().add(offset).cast::<SignalMsg>())
                };
                let handle = message.handle;

                if !handle.is_null() {
                    let should_dispatch = unsafe { message.signum == (*handle).signum };

                    if should_dispatch {
                        if unsafe { (*handle).flags & UV_HANDLE_CLOSING } != 0 {
                            unsafe { libc::abort() };
                        }
                        if let Some(cb) = unsafe { (*handle).signal_cb } {
                            unsafe {
                                cb(handle, (*handle).signum);
                            }
                        }
                    }

                    unsafe {
                        (*handle).dispatched_signals = (*handle).dispatched_signals.wrapping_add(1);
                    }

                    if unsafe { (*handle).flags & UV_SIGNAL_ONE_SHOT } != 0 {
                        let _ = unsafe { stop(handle) };
                    }
                }

                offset += MSG_SIZE;
            }

            bytes -= end;
            if bytes != 0 {
                buffer.copy_within(end..end + bytes, 0);
                continue;
            }

            if end < buffer.len() {
                return;
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn init_handle(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_signal_t) {
    unsafe {
        let data = unsafe { (*handle).data };
        unsafe {
            std::ptr::write_bytes(handle, 0, 1);
            (*handle).loop_ = loop_;
            (*handle).type_ = abi::uv_handle_type_UV_SIGNAL;
            (*handle).flags = UV_HANDLE_REF;
            (*handle).data = data;
            queue::init(std::ptr::addr_of_mut!((*handle).handle_queue));
            queue::insert_tail(
                std::ptr::addr_of_mut!((*loop_).handle_queue),
                std::ptr::addr_of_mut!((*handle).handle_queue),
            );
            (*handle).next_closing = ptr::null_mut();
            (*handle).close_cb = None;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn start(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: c_int,
    oneshot: bool,
) -> c_int {
    unsafe {
        if handle.is_null() || signal_cb.is_none() || signum == 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        if unsafe { (*handle).flags & UV_HANDLE_CLOSING } != 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        if signum == unsafe { (*handle).signum } {
            unsafe {
                (*handle).signal_cb = signal_cb;
            }
            return 0;
        }

        if unsafe { (*handle).signum } != 0 {
            let rc = unsafe { stop(handle) };
            if rc != 0 {
                return rc;
            }
        }

        let mut saved = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
        unsafe { block_and_lock(saved.as_mut_ptr()) };

        let rc = reserve_watcher_slot();
        if rc != 0 {
            unsafe { unlock_and_unblock(saved.as_ptr()) };
            return rc;
        }

        let first = unsafe { first_watcher(signum) };
        if first.is_none() || (!oneshot && first.unwrap().oneshot) {
            let rc = unsafe { register_handler(signum, oneshot) };
            if rc != 0 {
                unsafe { unlock_and_unblock(saved.as_ptr()) };
                return rc;
            }
        }

        unsafe {
            (*handle).signum = signum;
            if oneshot {
                (*handle).flags |= UV_SIGNAL_ONE_SHOT;
            } else {
                (*handle).flags &= !UV_SIGNAL_ONE_SHOT;
            }
            insert_watcher(handle);
        }

        unsafe { unlock_and_unblock(saved.as_ptr()) };
        unsafe {
            (*handle).signal_cb = signal_cb;
        }
        handle_start(handle.cast());
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn stop(handle: *mut abi::uv_signal_t) -> c_int {
    unsafe {
        if handle.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let signum = unsafe { (*handle).signum };
        if signum == 0 {
            return 0;
        }

        let mut saved = std::mem::MaybeUninit::<libc::sigset_t>::uninit();
        unsafe { block_and_lock(saved.as_mut_ptr()) };

        let removed_regular = unsafe { (*handle).flags & UV_SIGNAL_ONE_SHOT } == 0;
        let first = {
            let watchers = unsafe { &mut *REGISTRY.watchers.get() };
            let remove_at = watchers
                .iter()
                .position(|candidate| candidate.handle_ptr == handle as usize);
            let remove_at = match remove_at {
                Some(index) => index,
                None => {
                    unsafe { unlock_and_unblock(saved.as_ptr()) };
                    unsafe {
                        (*handle).signum = 0;
                        (*handle).flags &= !UV_SIGNAL_ONE_SHOT;
                    }
                    handle_stop(handle.cast());
                    return 0;
                }
            };
            watchers.remove(remove_at);
            first_watcher_in(watchers, signum)
        };
        if first.is_none() {
            unsafe { unregister_handler(signum) };
        } else if removed_regular && first.unwrap().oneshot {
            let rc = unsafe { register_handler(signum, true) };
            if rc != 0 {
                unsafe { libc::abort() };
            }
        }

        unsafe { unlock_and_unblock(saved.as_ptr()) };

        unsafe {
            (*handle).signum = 0;
            (*handle).flags &= !UV_SIGNAL_ONE_SHOT;
        }
        handle_stop(handle.cast());
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_signal_t) -> c_int {
    unsafe {
        if loop_.is_null() || handle.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        ensure_global_init();

        let rc = unsafe { signal_loop_once_init(loop_) };
        if rc != 0 {
            return rc;
        }

        unsafe { init_handle(loop_, handle) };
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn start_regular(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: c_int,
) -> c_int {
    unsafe { unsafe { start(handle, signal_cb, signum, false) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn start_oneshot(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: c_int,
) -> c_int {
    unsafe { unsafe { start(handle, signal_cb, signum, true) } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn close(handle: *mut abi::uv_signal_t) {
    unsafe {
        let _ = unsafe { stop(handle) };
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_fork(loop_: *mut abi::uv_loop_t) -> c_int {
    unsafe {
        if loop_.is_null() || unsafe { (*loop_).signal_pipefd[0] } == -1 {
            return 0;
        }

        unsafe {
            unix_core::uv__io_stop(
                loop_.cast(),
                std::ptr::addr_of_mut!((*loop_).signal_io_watcher).cast(),
                libc::POLLIN as c_uint,
            );
            close_fd((*loop_).signal_pipefd[0]);
            close_fd((*loop_).signal_pipefd[1]);
            (*loop_).signal_pipefd = [-1, -1];
        }

        let head = unsafe { std::ptr::addr_of_mut!((*loop_).handle_queue) };
        let mut node = unsafe { (*head).next };
        while !std::ptr::eq(node, head) {
            let handle = unsafe {
                node.cast::<u8>()
                    .sub(offset_of!(abi::uv_handle_t, handle_queue))
                    .cast::<abi::uv_handle_t>()
            };
            node = unsafe { (*node).next };

            if unsafe { (*handle).type_ } != abi::uv_handle_type_UV_SIGNAL {
                continue;
            }

            let signal = handle.cast::<abi::uv_signal_t>();
            unsafe {
                (*signal).caught_signals = 0;
                (*signal).dispatched_signals = 0;
            }
        }

        unsafe { signal_loop_once_init(loop_) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn loop_cleanup(loop_: *mut abi::uv_loop_t) {
    unsafe {
        if loop_.is_null() {
            return;
        }

        let head = unsafe { std::ptr::addr_of_mut!((*loop_).handle_queue) };
        let mut node = unsafe { (*head).next };
        while !std::ptr::eq(node, head) {
            let handle = unsafe {
                node.cast::<u8>()
                    .sub(offset_of!(abi::uv_handle_t, handle_queue))
                    .cast::<abi::uv_handle_t>()
            };
            node = unsafe { (*node).next };

            if unsafe { (*handle).type_ } == abi::uv_handle_type_UV_SIGNAL {
                let _ = unsafe { stop(handle.cast()) };
            }
        }

        if unsafe { (*loop_).signal_pipefd[0] } != -1 {
            unsafe {
                unix_core::uv__io_stop(
                    loop_.cast(),
                    std::ptr::addr_of_mut!((*loop_).signal_io_watcher).cast(),
                    libc::POLLIN as c_uint,
                );
                close_fd((*loop_).signal_pipefd[0]);
                close_fd((*loop_).signal_pipefd[1]);
                (*loop_).signal_pipefd = [-1, -1];
                (*loop_).signal_io_watcher.fd = -1;
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn global_once_init() {
    unsafe {
        ensure_global_init();
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn cleanup_global() {
    unsafe {
        unsafe {
            if SIGNAL_LOCK_PIPEFD[0] != -1 {
                close_fd(SIGNAL_LOCK_PIPEFD[0]);
                SIGNAL_LOCK_PIPEFD[0] = -1;
            }
            if SIGNAL_LOCK_PIPEFD[1] != -1 {
                close_fd(SIGNAL_LOCK_PIPEFD[1]);
                SIGNAL_LOCK_PIPEFD[1] = -1;
            }
        }
    }
}
