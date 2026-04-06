use crate::bindings::*;

#[no_mangle]
pub unsafe extern "C" fn uv_handle_size(type_: uv_handle_type) -> usize {
    match type_ {
        uv_handle_type_UV_ASYNC => std::mem::size_of::<uv_async_t>(),
        uv_handle_type_UV_CHECK => std::mem::size_of::<uv_check_t>(),
        uv_handle_type_UV_FS_EVENT => std::mem::size_of::<uv_fs_event_t>(),
        uv_handle_type_UV_FS_POLL => std::mem::size_of::<uv_fs_poll_t>(),
        uv_handle_type_UV_HANDLE => std::mem::size_of::<uv_handle_t>(),
        uv_handle_type_UV_IDLE => std::mem::size_of::<uv_idle_t>(),
        uv_handle_type_UV_NAMED_PIPE => std::mem::size_of::<uv_pipe_t>(),
        uv_handle_type_UV_POLL => std::mem::size_of::<uv_poll_t>(),
        uv_handle_type_UV_PREPARE => std::mem::size_of::<uv_prepare_t>(),
        uv_handle_type_UV_PROCESS => std::mem::size_of::<uv_process_t>(),
        uv_handle_type_UV_STREAM => std::mem::size_of::<uv_stream_t>(),
        uv_handle_type_UV_TCP => std::mem::size_of::<uv_tcp_t>(),
        uv_handle_type_UV_TIMER => std::mem::size_of::<uv_timer_t>(),
        uv_handle_type_UV_TTY => std::mem::size_of::<uv_tty_t>(),
        uv_handle_type_UV_UDP => std::mem::size_of::<uv_udp_t>(),
        uv_handle_type_UV_SIGNAL => std::mem::size_of::<uv_signal_t>(),
        _ => usize::MAX,
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_req_size(type_: uv_req_type) -> usize {
    match type_ {
        uv_req_type_UV_REQ => std::mem::size_of::<uv_req_t>(),
        uv_req_type_UV_CONNECT => std::mem::size_of::<uv_connect_t>(),
        uv_req_type_UV_WRITE => std::mem::size_of::<uv_write_t>(),
        uv_req_type_UV_SHUTDOWN => std::mem::size_of::<uv_shutdown_t>(),
        uv_req_type_UV_UDP_SEND => std::mem::size_of::<uv_udp_send_t>(),
        uv_req_type_UV_FS => std::mem::size_of::<uv_fs_t>(),
        uv_req_type_UV_WORK => std::mem::size_of::<uv_work_t>(),
        uv_req_type_UV_GETADDRINFO => std::mem::size_of::<uv_getaddrinfo_t>(),
        uv_req_type_UV_GETNAMEINFO => std::mem::size_of::<uv_getnameinfo_t>(),
        uv_req_type_UV_RANDOM => std::mem::size_of::<uv_random_t>(),
        _ => usize::MAX,
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_buf_init(base: *mut libc::c_char, len: libc::c_uint) -> uv_buf_t {
    uv_buf_t {
        base,
        len: len as usize,
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_handle_type_name(type_: uv_handle_type) -> *const libc::c_char {
    match type_ {
        uv_handle_type_UV_ASYNC => b"async\0".as_ptr().cast(),
        uv_handle_type_UV_CHECK => b"check\0".as_ptr().cast(),
        uv_handle_type_UV_FS_EVENT => b"fs_event\0".as_ptr().cast(),
        uv_handle_type_UV_FS_POLL => b"fs_poll\0".as_ptr().cast(),
        uv_handle_type_UV_HANDLE => b"handle\0".as_ptr().cast(),
        uv_handle_type_UV_IDLE => b"idle\0".as_ptr().cast(),
        uv_handle_type_UV_NAMED_PIPE => b"pipe\0".as_ptr().cast(),
        uv_handle_type_UV_POLL => b"poll\0".as_ptr().cast(),
        uv_handle_type_UV_PREPARE => b"prepare\0".as_ptr().cast(),
        uv_handle_type_UV_PROCESS => b"process\0".as_ptr().cast(),
        uv_handle_type_UV_STREAM => b"stream\0".as_ptr().cast(),
        uv_handle_type_UV_TCP => b"tcp\0".as_ptr().cast(),
        uv_handle_type_UV_TIMER => b"timer\0".as_ptr().cast(),
        uv_handle_type_UV_TTY => b"tty\0".as_ptr().cast(),
        uv_handle_type_UV_UDP => b"udp\0".as_ptr().cast(),
        uv_handle_type_UV_SIGNAL => b"signal\0".as_ptr().cast(),
        uv_handle_type_UV_FILE => b"file\0".as_ptr().cast(),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_handle_get_type(handle: *const uv_handle_t) -> uv_handle_type {
    (*handle).type_
}

#[no_mangle]
pub unsafe extern "C" fn uv_handle_get_data(handle: *const uv_handle_t) -> *mut libc::c_void {
    (*handle).data
}

#[no_mangle]
pub unsafe extern "C" fn uv_handle_get_loop(handle: *const uv_handle_t) -> *mut uv_loop_t {
    (*handle).loop_
}

#[no_mangle]
pub unsafe extern "C" fn uv_handle_set_data(handle: *mut uv_handle_t, data: *mut libc::c_void) {
    (*handle).data = data;
}

#[no_mangle]
pub unsafe extern "C" fn uv_req_type_name(type_: uv_req_type) -> *const libc::c_char {
    match type_ {
        uv_req_type_UV_REQ => b"req\0".as_ptr().cast(),
        uv_req_type_UV_CONNECT => b"connect\0".as_ptr().cast(),
        uv_req_type_UV_WRITE => b"write\0".as_ptr().cast(),
        uv_req_type_UV_SHUTDOWN => b"shutdown\0".as_ptr().cast(),
        uv_req_type_UV_UDP_SEND => b"udp_send\0".as_ptr().cast(),
        uv_req_type_UV_FS => b"fs\0".as_ptr().cast(),
        uv_req_type_UV_WORK => b"work\0".as_ptr().cast(),
        uv_req_type_UV_GETADDRINFO => b"getaddrinfo\0".as_ptr().cast(),
        uv_req_type_UV_GETNAMEINFO => b"getnameinfo\0".as_ptr().cast(),
        uv_req_type_UV_RANDOM => b"random\0".as_ptr().cast(),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_req_get_type(req: *const uv_req_t) -> uv_req_type {
    (*req).type_
}

#[no_mangle]
pub unsafe extern "C" fn uv_req_get_data(req: *const uv_req_t) -> *mut libc::c_void {
    (*req).data
}

#[no_mangle]
pub unsafe extern "C" fn uv_req_set_data(req: *mut uv_req_t, data: *mut libc::c_void) {
    (*req).data = data;
}

#[no_mangle]
pub unsafe extern "C" fn uv_stream_get_write_queue_size(stream: *const uv_stream_t) -> usize {
    (*stream).write_queue_size
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_get_send_queue_size(handle: *const uv_udp_t) -> usize {
    (*handle).send_queue_size
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_get_send_queue_count(handle: *const uv_udp_t) -> usize {
    (*handle).send_queue_count
}

#[no_mangle]
pub unsafe extern "C" fn uv_process_get_pid(proc: *const uv_process_t) -> uv_pid_t {
    (*proc).pid
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_get_type(req: *const uv_fs_t) -> uv_fs_type {
    (*req).fs_type
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_get_result(req: *const uv_fs_t) -> isize {
    (*req).result
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_get_ptr(req: *const uv_fs_t) -> *mut libc::c_void {
    (*req).ptr
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_get_path(req: *const uv_fs_t) -> *const libc::c_char {
    (*req).path
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_get_statbuf(req: *mut uv_fs_t) -> *mut uv_stat_t {
    &mut (*req).statbuf
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_get_data(loop_: *const uv_loop_t) -> *mut libc::c_void {
    (*loop_).data
}

#[no_mangle]
pub unsafe extern "C" fn uv_loop_set_data(loop_: *mut uv_loop_t, data: *mut libc::c_void) {
    (*loop_).data = data;
}
