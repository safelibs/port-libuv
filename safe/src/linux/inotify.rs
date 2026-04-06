use crate::bindings::*;
use crate::handle::{queue_empty, queue_head, queue_next};
use std::ffi::{CStr, CString};
use std::mem::offset_of;

#[repr(C)]
struct WatcherList {
    entry: WatcherEntry,
    watchers: uv__queue,
    path: *mut libc::c_char,
    wd: libc::c_int,
    iterating: libc::c_int,
}

#[repr(C)]
struct WatcherEntry {
    left: *mut WatcherList,
    right: *mut WatcherList,
    parent: *mut WatcherList,
    color: libc::c_int,
}

#[inline]
unsafe fn fs_event_from_queue(q: *mut uv__queue) -> *mut uv_fs_event_t {
    q.cast::<u8>()
        .sub(offset_of!(uv_fs_event_t, watchers))
        .cast::<uv_fs_event_t>()
}

unsafe fn collect_watcher_lists(root: *mut WatcherList) -> Vec<*mut WatcherList> {
    let mut lists = Vec::new();
    let mut stack = Vec::new();
    let mut current = root;

    while !current.is_null() || !stack.is_empty() {
        while !current.is_null() {
            stack.push(current);
            current = (*current).entry.left;
        }

        current = stack.pop().unwrap();
        lists.push(current);
        current = (*current).entry.right;
    }

    lists
}

pub(crate) unsafe fn uv__inotify_fork_impl(
    loop_: *mut uv_loop_t,
    root: *mut libc::c_void,
) -> libc::c_int {
    let root = root.cast::<WatcherList>();
    let mut restart = Vec::new();

    if root.is_null() {
        return 0;
    }

    (*loop_).inotify_watchers = root.cast();

    for watcher_list in collect_watcher_lists(root) {
        let head = std::ptr::addr_of_mut!((*watcher_list).watchers);
        let mut handles = Vec::new();
        let mut q = queue_head(head);
        while !std::ptr::eq(q, head) {
            handles.push(fs_event_from_queue(q));
            q = queue_next(q);
        }

        for handle in handles {
            let saved_path = if (*handle).path.is_null() {
                None
            } else {
                Some(CString::from(CStr::from_ptr((*handle).path)))
            };

            uv_fs_event_stop_impl(handle);
            restart.push((handle, saved_path));
        }
    }

    for (handle, saved_path) in restart {
        let Some(saved_path) = saved_path else {
            continue;
        };
        let err =
            uv_fs_event_start_impl(handle, (*handle).cb, saved_path.as_ptr(), 0 as libc::c_uint);
        if err != 0 {
            return err;
        }
    }

    debug_assert!(
        queue_empty(std::ptr::addr_of!((*loop_).watcher_queue)) || !(*loop_).watchers.is_null()
    );
    0
}

pub(crate) unsafe fn uv_fs_event_init_impl(
    loop_: *mut uv_loop_t,
    handle: *mut uv_fs_event_t,
) -> libc::c_int {
    crate::private_support::uv_fs_event_init(loop_, handle)
}

pub(crate) unsafe fn uv_fs_event_start_impl(
    handle: *mut uv_fs_event_t,
    cb: uv_fs_event_cb,
    path: *const libc::c_char,
    flags: libc::c_uint,
) -> libc::c_int {
    crate::private_support::uv_fs_event_start(handle, cb, path, flags)
}

pub(crate) unsafe fn uv_fs_event_stop_impl(handle: *mut uv_fs_event_t) -> libc::c_int {
    crate::private_support::uv_fs_event_stop(handle)
}

pub(crate) unsafe fn uv_fs_event_getpath_impl(
    handle: *mut uv_fs_event_t,
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    crate::private_support::uv_fs_event_getpath(handle, buffer, size)
}
