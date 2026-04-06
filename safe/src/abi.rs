use crate::bindings::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct uv__loop_metrics_s {
    pub metrics: uv_metrics_t,
    pub provider_entry_time: u64,
    pub provider_idle_time: u64,
    pub lock: uv_mutex_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct uv__iou {
    pub sqhead: *mut u32,
    pub sqtail: *mut u32,
    pub sqarray: *mut u32,
    pub sqmask: u32,
    pub sqflags: *mut u32,
    pub cqhead: *mut u32,
    pub cqtail: *mut u32,
    pub cqmask: u32,
    pub sq: *mut libc::c_void,
    pub cqe: *mut libc::c_void,
    pub sqe: *mut libc::c_void,
    pub sqlen: usize,
    pub cqlen: usize,
    pub maxlen: usize,
    pub sqelen: usize,
    pub ringfd: libc::c_int,
    pub in_flight: u32,
    pub flags: u32,
}

impl Default for uv__iou {
    fn default() -> Self {
        Self {
            sqhead: std::ptr::null_mut(),
            sqtail: std::ptr::null_mut(),
            sqarray: std::ptr::null_mut(),
            sqmask: 0,
            sqflags: std::ptr::null_mut(),
            cqhead: std::ptr::null_mut(),
            cqtail: std::ptr::null_mut(),
            cqmask: 0,
            sq: std::ptr::null_mut(),
            cqe: std::ptr::null_mut(),
            sqe: std::ptr::null_mut(),
            sqlen: 0,
            cqlen: 0,
            maxlen: 0,
            sqelen: 0,
            ringfd: -1,
            in_flight: 0,
            flags: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct uv__loop_internal_fields_s {
    pub flags: libc::c_uint,
    pub loop_metrics: uv__loop_metrics_s,
    pub current_timeout: libc::c_int,
    pub ctl: uv__iou,
    pub iou: uv__iou,
    pub inv: *mut libc::c_void,
}

pub(crate) const UV_LOOP_BLOCK_SIGPROF: libc::c_ulong = 0x1;
pub(crate) const UV_LOOP_REAP_CHILDREN: libc::c_ulong = 0x2;
pub(crate) const UV_METRICS_IDLE_TIME_FLAG: libc::c_uint = 1;
pub(crate) const UV__POLLRDHUP: libc::c_uint = libc::POLLRDHUP as libc::c_uint;
pub(crate) const UV__POLLPRI: libc::c_uint = libc::POLLPRI as libc::c_uint;

unsafe extern "C" {
    pub(crate) fn uv__signal_global_once_init();
    pub(crate) fn uv__process_init(loop_: *mut uv_loop_t) -> libc::c_int;
    pub(crate) fn uv__wait_children(loop_: *mut uv_loop_t);
    pub(crate) fn uv__signal_loop_cleanup(loop_: *mut uv_loop_t);
    pub(crate) fn uv__signal_loop_fork(loop_: *mut uv_loop_t) -> libc::c_int;

    pub(crate) fn uv__pipe_close(handle: *mut uv_pipe_t);
    pub(crate) fn uv__stream_close(handle: *mut uv_stream_t);
    pub(crate) fn uv__tcp_close(handle: *mut uv_tcp_t);
    pub(crate) fn uv__udp_close(handle: *mut uv_udp_t);
    pub(crate) fn uv__process_close(handle: *mut uv_process_t);
    pub(crate) fn uv__fs_event_close(handle: *mut uv_fs_event_t);
    pub(crate) fn uv__poll_close(handle: *mut uv_poll_t);
    pub(crate) fn uv__fs_poll_close(handle: *mut uv_fs_poll_t);
    pub(crate) fn uv__signal_close(handle: *mut uv_signal_t);

    pub(crate) fn uv__stream_destroy(stream: *mut uv_stream_t);
    pub(crate) fn uv__udp_finish_close(handle: *mut uv_udp_t);
    pub(crate) fn uv__work_done(handle: *mut uv_async_t);

    pub(crate) fn uv__close(fd: libc::c_int) -> libc::c_int;
}
