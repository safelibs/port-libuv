use crate::{abi::linux_x86_64 as abi, core, unix, version};
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_uint};

mod generated;

#[inline(always)]
fn c_name(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr().cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_version() -> c_uint {
    version::version_hex()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_version_string() -> *const c_char {
    version::version_string_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_buf_init(base: *mut c_char, len: c_uint) -> abi::uv_buf_t {
    abi::uv_buf_t {
        base,
        len: len as usize,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_setup_args(argc: c_int, argv: *mut *mut c_char) -> *mut *mut c_char {
    unsafe { unix::process_title::setup_args(argc, argv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_size() -> usize {
    size_of::<abi::uv_loop_t>()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_handle_size(handle_type: abi::uv_handle_type) -> usize {
    match handle_type {
        abi::uv_handle_type_UV_ASYNC => size_of::<abi::uv_async_t>(),
        abi::uv_handle_type_UV_CHECK => size_of::<abi::uv_check_t>(),
        abi::uv_handle_type_UV_FS_EVENT => size_of::<abi::uv_fs_event_t>(),
        abi::uv_handle_type_UV_FS_POLL => size_of::<abi::uv_fs_poll_t>(),
        abi::uv_handle_type_UV_HANDLE => size_of::<abi::uv_handle_t>(),
        abi::uv_handle_type_UV_IDLE => size_of::<abi::uv_idle_t>(),
        abi::uv_handle_type_UV_NAMED_PIPE => size_of::<abi::uv_pipe_t>(),
        abi::uv_handle_type_UV_POLL => size_of::<abi::uv_poll_t>(),
        abi::uv_handle_type_UV_PREPARE => size_of::<abi::uv_prepare_t>(),
        abi::uv_handle_type_UV_PROCESS => size_of::<abi::uv_process_t>(),
        abi::uv_handle_type_UV_STREAM => size_of::<abi::uv_stream_t>(),
        abi::uv_handle_type_UV_TCP => size_of::<abi::uv_tcp_t>(),
        abi::uv_handle_type_UV_TIMER => size_of::<abi::uv_timer_t>(),
        abi::uv_handle_type_UV_TTY => size_of::<abi::uv_tty_t>(),
        abi::uv_handle_type_UV_UDP => size_of::<abi::uv_udp_t>(),
        abi::uv_handle_type_UV_SIGNAL => size_of::<abi::uv_signal_t>(),
        abi::uv_handle_type_UV_FILE => size_of::<abi::uv_file>(),
        _ => usize::MAX,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_req_size(req_type: abi::uv_req_type) -> usize {
    match req_type {
        abi::uv_req_type_UV_REQ => size_of::<abi::uv_req_t>(),
        abi::uv_req_type_UV_CONNECT => size_of::<abi::uv_connect_t>(),
        abi::uv_req_type_UV_WRITE => size_of::<abi::uv_write_t>(),
        abi::uv_req_type_UV_SHUTDOWN => size_of::<abi::uv_shutdown_t>(),
        abi::uv_req_type_UV_UDP_SEND => size_of::<abi::uv_udp_send_t>(),
        abi::uv_req_type_UV_FS => size_of::<abi::uv_fs_t>(),
        abi::uv_req_type_UV_WORK => size_of::<abi::uv_work_t>(),
        abi::uv_req_type_UV_GETADDRINFO => size_of::<abi::uv_getaddrinfo_t>(),
        abi::uv_req_type_UV_GETNAMEINFO => size_of::<abi::uv_getnameinfo_t>(),
        abi::uv_req_type_UV_RANDOM => size_of::<abi::uv_random_t>(),
        _ => usize::MAX,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_get_data(loop_: *const abi::uv_loop_t) -> *mut std::ffi::c_void {
    if loop_.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { (*loop_).data }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_set_data(loop_: *mut abi::uv_loop_t, data: *mut std::ffi::c_void) {
    if !loop_.is_null() {
        unsafe {
            (*loop_).data = data;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_handle_get_type(
    handle: *const abi::uv_handle_t,
) -> abi::uv_handle_type {
    if handle.is_null() {
        return abi::uv_handle_type_UV_UNKNOWN_HANDLE;
    }
    unsafe { (*handle).type_ }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_handle_get_data(
    handle: *const abi::uv_handle_t,
) -> *mut std::ffi::c_void {
    if handle.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { (*handle).data }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_handle_get_loop(
    handle: *const abi::uv_handle_t,
) -> *mut abi::uv_loop_t {
    if handle.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { (*handle).loop_ }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_handle_set_data(
    handle: *mut abi::uv_handle_t,
    data: *mut std::ffi::c_void,
) {
    if !handle.is_null() {
        unsafe {
            (*handle).data = data;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_req_get_data(req: *const abi::uv_req_t) -> *mut std::ffi::c_void {
    if req.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { (*req).data }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_req_set_data(req: *mut abi::uv_req_t, data: *mut std::ffi::c_void) {
    if !req.is_null() {
        unsafe {
            (*req).data = data;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_req_get_type(req: *const abi::uv_req_t) -> abi::uv_req_type {
    if req.is_null() {
        return abi::uv_req_type_UV_UNKNOWN_REQ;
    }
    unsafe { (*req).type_ }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_handle_type_name(handle_type: abi::uv_handle_type) -> *const c_char {
    core::handle::handle_type_name_ptr(handle_type)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_req_type_name(req_type: abi::uv_req_type) -> *const c_char {
    core::request::req_type_name_ptr(req_type)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_stream_get_write_queue_size(stream: *const abi::uv_stream_t) -> usize {
    if stream.is_null() {
        return 0;
    }
    unsafe { (*stream).write_queue_size }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_get_send_queue_size(handle: *const abi::uv_udp_t) -> usize {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).send_queue_size }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_get_send_queue_count(handle: *const abi::uv_udp_t) -> usize {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).send_queue_count }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_set_repeat(handle: *mut abi::uv_timer_t, repeat: u64) {
    if !handle.is_null() {
        unsafe {
            (*handle).repeat = repeat;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_get_repeat(handle: *const abi::uv_timer_t) -> u64 {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).repeat }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_get_due_in(handle: *const abi::uv_timer_t) -> u64 {
    unsafe { unix::epoll::timer_get_due_in(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_process_get_pid(handle: *const abi::uv_process_t) -> abi::uv_pid_t {
    if handle.is_null() {
        return 0;
    }
    unsafe { (*handle).pid as abi::uv_pid_t }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_alive(loop_: *const abi::uv_loop_t) -> c_int {
    unsafe { unix::epoll::loop_alive(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_now(loop_: *const abi::uv_loop_t) -> u64 {
    unsafe { unix::epoll::now(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_backend_fd(loop_: *const abi::uv_loop_t) -> c_int {
    unsafe { unix::epoll::backend_fd(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_backend_timeout(loop_: *const abi::uv_loop_t) -> c_int {
    unsafe { unix::epoll::backend_timeout(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_is_active(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { unix::epoll::is_active(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_is_closing(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { unix::epoll::is_closing(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_is_readable(handle: *const abi::uv_stream_t) -> c_int {
    unsafe { unix::stream::is_readable(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_is_writable(handle: *const abi::uv_stream_t) -> c_int {
    unsafe { unix::stream::is_writable(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_has_ref(handle: *const abi::uv_handle_t) -> c_int {
    unsafe { unix::epoll::has_ref(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_using_recvmmsg(handle: *const abi::uv_udp_t) -> c_int {
    unsafe { unix::udp::using_recvmmsg(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_pending_count(handle: *mut abi::uv_pipe_t) -> c_int {
    unsafe { unix::pipe::pending_count(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cpumask_size() -> c_int {
    crate::threading::thread::cpumask_size()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_equal(
    t1: *const abi::uv_thread_t,
    t2: *const abi::uv_thread_t,
) -> c_int {
    unsafe { crate::threading::thread::equal(t1, t2) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_getcpu() -> c_int {
    unsafe { crate::threading::thread::getcpu() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_get_type(req: *const abi::uv_fs_t) -> abi::uv_fs_type {
    if req.is_null() {
        return 0;
    }
    unsafe { (*req).fs_type }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_get_result(req: *const abi::uv_fs_t) -> isize {
    if req.is_null() {
        return 0;
    }
    unsafe { (*req).result }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_get_system_error(req: *const abi::uv_fs_t) -> c_int {
    if req.is_null() {
        return 0;
    }
    let result = unsafe { (*req).result };
    if result >= 0 {
        0
    } else {
        result as c_int
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_get_ptr(req: *const abi::uv_fs_t) -> *mut std::ffi::c_void {
    if req.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { (*req).ptr }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_get_path(req: *const abi::uv_fs_t) -> *const c_char {
    if req.is_null() {
        return std::ptr::null();
    }
    unsafe { (*req).path }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_get_statbuf(req: *mut abi::uv_fs_t) -> *mut abi::uv_stat_t {
    if req.is_null() {
        return std::ptr::null_mut();
    }
    unsafe { std::ptr::addr_of_mut!((*req).statbuf) }
}
