use crate::abi::{self, UV_LOOP_BLOCK_SIGPROF, UV__POLLPRI, UV__POLLRDHUP};
use crate::allocator;
use crate::bindings::*;
use crate::handle::{queue_empty, queue_head, queue_init, queue_insert_tail, queue_remove};
use crate::r#loop::{
    metrics_inc_events, metrics_inc_events_waiting, uv__metrics_set_provider_entry_time_impl,
    uv__metrics_update_idle_time_impl,
};
use crate::state::loop_fields;
use std::mem::{offset_of, size_of};

const MAX_EPOLL_EVENTS: usize = 1024;

#[repr(C)]
struct Invalidate {
    events: *mut libc::epoll_event,
    nfds: libc::c_int,
}

#[inline]
unsafe fn watcher_from_queue(q: *mut uv__queue) -> *mut uv__io_t {
    q.cast::<u8>()
        .sub(offset_of!(uv__io_t, watcher_queue))
        .cast::<uv__io_t>()
}

#[inline]
unsafe fn update_loop_time(loop_: *mut uv_loop_t) {
    (*loop_).time = uv_hrtime() / 1_000_000;
}

#[inline]
fn epoll_event(events: libc::c_uint, fd: libc::c_int) -> libc::epoll_event {
    libc::epoll_event {
        events,
        u64: fd as u64,
    }
}

unsafe fn next_power_of_two(mut value: usize) -> usize {
    if value <= 1 {
        return 1;
    }

    value -= 1;
    value |= value >> 1;
    value |= value >> 2;
    value |= value >> 4;
    value |= value >> 8;
    value |= value >> 16;
    if size_of::<usize>() >= 8 {
        value |= value >> 32;
    }
    value + 1
}

unsafe fn maybe_resize(loop_: *mut uv_loop_t, len: libc::c_uint) {
    if len <= (*loop_).nwatchers {
        return;
    }

    let (fake_watcher_list, fake_watcher_count) = if !(*loop_).watchers.is_null() {
        (
            *(*loop_).watchers.add((*loop_).nwatchers as usize),
            *(*loop_).watchers.add((*loop_).nwatchers as usize + 1),
        )
    } else {
        (std::ptr::null_mut(), std::ptr::null_mut())
    };

    let nwatchers = next_power_of_two(len as usize + 2) - 2;
    let watchers = allocator::reallocf(
        (*loop_).watchers.cast(),
        (nwatchers + 2) * size_of::<*mut uv__io_t>(),
    )
    .cast::<*mut uv__io_t>();
    if watchers.is_null() {
        libc::abort();
    }

    for i in (*loop_).nwatchers as usize..nwatchers {
        *watchers.add(i) = std::ptr::null_mut();
    }
    *watchers.add(nwatchers) = fake_watcher_list;
    *watchers.add(nwatchers + 1) = fake_watcher_count;

    (*loop_).watchers = watchers;
    (*loop_).nwatchers = nwatchers as libc::c_uint;
}

unsafe fn apply_watcher_queue(loop_: *mut uv_loop_t) {
    while !queue_empty(std::ptr::addr_of!((*loop_).watcher_queue)) {
        let q = queue_head(std::ptr::addr_of!((*loop_).watcher_queue));
        let w = watcher_from_queue(q);

        queue_remove(q);
        queue_init(q);

        let mut op = libc::EPOLL_CTL_MOD;
        if (*w).events == 0 {
            op = libc::EPOLL_CTL_ADD;
        }

        (*w).events = (*w).pevents;
        let mut event = epoll_event((*w).pevents, (*w).fd);
        if libc::epoll_ctl((*loop_).backend_fd, op, (*w).fd, &mut event) == 0 {
            continue;
        }

        if op == libc::EPOLL_CTL_ADD && *allocator::errno_location() == libc::EEXIST {
            if libc::epoll_ctl(
                (*loop_).backend_fd,
                libc::EPOLL_CTL_MOD,
                (*w).fd,
                &mut event,
            ) == 0
            {
                continue;
            }
        }

        libc::abort();
    }
}

pub(crate) unsafe extern "C" fn uv__platform_loop_init_impl(loop_: *mut uv_loop_t) -> libc::c_int {
    let lfields = loop_fields(loop_);

    (*loop_).inotify_watchers = std::ptr::null_mut();
    (*loop_).inotify_fd = -1;
    (*loop_).backend_fd = libc::epoll_create1(libc::EPOLL_CLOEXEC);
    if (*loop_).backend_fd == -1 {
        return allocator::last_error();
    }

    (*lfields).current_timeout = 0;
    (*lfields).ctl = Default::default();
    (*lfields).iou = Default::default();
    (*lfields).inv = std::ptr::null_mut();
    0
}

pub(crate) unsafe extern "C" fn uv__io_fork_impl(loop_: *mut uv_loop_t) -> libc::c_int {
    let inotify_watchers = (*loop_).inotify_watchers;

    if (*loop_).backend_fd != -1 {
        abi::uv__close((*loop_).backend_fd);
        (*loop_).backend_fd = -1;
    }

    uv__platform_loop_delete_impl(loop_);

    let err = uv__platform_loop_init_impl(loop_);
    if err != 0 {
        return err;
    }

    (*loop_).inotify_watchers = inotify_watchers;
    crate::linux::inotify::uv__inotify_fork_impl(loop_, inotify_watchers)
}

pub(crate) unsafe extern "C" fn uv__platform_loop_delete_impl(loop_: *mut uv_loop_t) {
    let lfields = loop_fields(loop_);

    (*lfields).ctl = Default::default();
    (*lfields).iou = Default::default();
    (*lfields).inv = std::ptr::null_mut();

    if (*loop_).inotify_fd != -1 {
        uv__io_stop_impl(
            loop_,
            std::ptr::addr_of_mut!((*loop_).inotify_read_watcher),
            libc::POLLIN as libc::c_uint,
        );
        abi::uv__close((*loop_).inotify_fd);
        (*loop_).inotify_fd = -1;
    }
}

pub(crate) unsafe extern "C" fn uv__platform_invalidate_fd_impl(
    loop_: *mut uv_loop_t,
    fd: libc::c_int,
) {
    let lfields = loop_fields(loop_);
    let inv = (*lfields).inv.cast::<Invalidate>();
    let mut dummy = epoll_event(0, 0);

    if !inv.is_null() {
        for i in 0..(*inv).nfds {
            let event = (*inv).events.add(i as usize);
            if (*event).u64 as libc::c_int == fd {
                (*event).u64 = (-1_i64) as u64;
            }
        }
    }

    if (*loop_).backend_fd != -1 {
        libc::epoll_ctl((*loop_).backend_fd, libc::EPOLL_CTL_DEL, fd, &mut dummy);
    }
}

pub(crate) unsafe extern "C" fn uv__io_check_fd_impl(
    loop_: *mut uv_loop_t,
    fd: libc::c_int,
) -> libc::c_int {
    let mut event = epoll_event(libc::POLLIN as libc::c_uint, -1);
    let mut rc = 0;

    if libc::epoll_ctl((*loop_).backend_fd, libc::EPOLL_CTL_ADD, fd, &mut event) != 0
        && *allocator::errno_location() != libc::EEXIST
    {
        rc = allocator::last_error();
    }

    if rc == 0 && libc::epoll_ctl((*loop_).backend_fd, libc::EPOLL_CTL_DEL, fd, &mut event) != 0 {
        libc::abort();
    }

    rc
}

pub(crate) unsafe extern "C" fn uv__io_poll_impl(loop_: *mut uv_loop_t, mut timeout: libc::c_int) {
    let lfields = loop_fields(loop_);
    let mut events = [epoll_event(0, 0); MAX_EPOLL_EVENTS];
    let mut inv = Invalidate {
        events: events.as_mut_ptr(),
        nfds: -1,
    };
    let mut sigset = std::mem::zeroed::<libc::sigset_t>();
    let mut sigmask: *const libc::sigset_t = std::ptr::null();
    let base = (*loop_).time;
    let mut count = 48;
    let mut real_timeout = timeout;
    let mut reset_timeout;
    let user_timeout;

    debug_assert!(timeout >= -1);

    if ((*loop_).flags & UV_LOOP_BLOCK_SIGPROF) != 0 {
        libc::sigemptyset(&mut sigset);
        libc::sigaddset(&mut sigset, libc::SIGPROF);
        sigmask = std::ptr::addr_of!(sigset);
    }

    if ((*lfields).flags & crate::abi::UV_METRICS_IDLE_TIME_FLAG) != 0 {
        reset_timeout = true;
        user_timeout = timeout;
        timeout = 0;
    } else {
        reset_timeout = false;
        user_timeout = 0;
    }

    apply_watcher_queue(loop_);

    loop {
        if (*loop_).nfds == 0 && (*lfields).iou.in_flight == 0 {
            break;
        }

        if timeout != 0 {
            uv__metrics_set_provider_entry_time_impl(loop_);
        }

        (*lfields).current_timeout = timeout;

        let nfds = libc::epoll_pwait(
            (*loop_).backend_fd,
            events.as_mut_ptr(),
            events.len() as libc::c_int,
            timeout,
            sigmask,
        );
        let saved_errno = if nfds == -1 {
            *allocator::errno_location()
        } else {
            0
        };
        update_loop_time(loop_);
        if nfds == -1 {
            *allocator::errno_location() = saved_errno;
        }

        if nfds == -1 {
            debug_assert_eq!(saved_errno, libc::EINTR);
        } else if nfds == 0 {
            debug_assert_ne!(timeout, -1);
        }

        if nfds == 0 || nfds == -1 {
            if reset_timeout {
                timeout = user_timeout;
                reset_timeout = false;
            } else if nfds == 0 {
                return;
            }

            if timeout == 0 {
                break;
            }
            if timeout == -1 {
                continue;
            }

            let elapsed = ((*loop_).time - base).min(libc::c_int::MAX as u64) as libc::c_int;
            real_timeout -= elapsed;
            if real_timeout <= 0 {
                break;
            }

            timeout = real_timeout;
            continue;
        }

        let mut have_iou_events = false;
        let mut have_signals = false;
        let mut nevents = 0;

        inv.nfds = nfds;
        (*lfields).inv = std::ptr::addr_of_mut!(inv).cast();

        for i in 0..nfds as usize {
            let pe = &mut events[i];
            let fd = pe.u64 as libc::c_int;

            if fd == -1 {
                continue;
            }

            if fd == (*lfields).iou.ringfd {
                have_iou_events = true;
                continue;
            }

            debug_assert!(fd >= 0);
            debug_assert!((fd as libc::c_uint) < (*loop_).nwatchers);

            let w = *(*loop_).watchers.add(fd as usize);
            if w.is_null() {
                let mut dummy = epoll_event(0, fd);
                libc::epoll_ctl((*loop_).backend_fd, libc::EPOLL_CTL_DEL, fd, &mut dummy);
                continue;
            }

            pe.events &=
                (*w).pevents | libc::POLLERR as libc::c_uint | libc::POLLHUP as libc::c_uint;

            if pe.events == libc::POLLERR as libc::c_uint
                || pe.events == libc::POLLHUP as libc::c_uint
            {
                pe.events |= (*w).pevents
                    & (libc::POLLIN as libc::c_uint
                        | libc::POLLOUT as libc::c_uint
                        | UV__POLLRDHUP
                        | UV__POLLPRI);
            }

            if pe.events == 0 {
                continue;
            }

            if std::ptr::eq(w, std::ptr::addr_of_mut!((*loop_).signal_io_watcher)) {
                have_signals = true;
            } else {
                uv__metrics_update_idle_time_impl(loop_);
                if let Some(cb) = (*w).cb {
                    cb(loop_, w, pe.events);
                }
            }

            nevents += 1;
        }

        metrics_inc_events(loop_, nevents as u64);
        if reset_timeout {
            timeout = user_timeout;
            reset_timeout = false;
            metrics_inc_events_waiting(loop_, nevents as u64);
        }

        if have_signals {
            uv__metrics_update_idle_time_impl(loop_);
            if let Some(cb) = (*loop_).signal_io_watcher.cb {
                cb(
                    loop_,
                    std::ptr::addr_of_mut!((*loop_).signal_io_watcher),
                    libc::POLLIN as libc::c_uint,
                );
            }
        }

        (*lfields).inv = std::ptr::null_mut();

        if have_iou_events || have_signals {
            break;
        }

        if nevents != 0 {
            if nfds as usize == events.len() {
                count -= 1;
                if count != 0 {
                    timeout = 0;
                    continue;
                }
            }
            break;
        }

        if timeout == 0 {
            break;
        }
        if timeout == -1 {
            continue;
        }

        let elapsed = ((*loop_).time - base).min(libc::c_int::MAX as u64) as libc::c_int;
        real_timeout -= elapsed;
        if real_timeout <= 0 {
            break;
        }
        timeout = real_timeout;
    }
}

pub(crate) unsafe extern "C" fn uv__io_init_impl(w: *mut uv__io_t, cb: uv__io_cb, fd: libc::c_int) {
    debug_assert!(cb.is_some());
    debug_assert!(fd >= -1);

    queue_init(std::ptr::addr_of_mut!((*w).pending_queue));
    queue_init(std::ptr::addr_of_mut!((*w).watcher_queue));
    (*w).cb = cb;
    (*w).fd = fd;
    (*w).events = 0;
    (*w).pevents = 0;
}

pub(crate) unsafe extern "C" fn uv__io_start_impl(
    loop_: *mut uv_loop_t,
    w: *mut uv__io_t,
    events: libc::c_uint,
) {
    debug_assert_eq!(
        events
            & !(libc::POLLIN as libc::c_uint
                | libc::POLLOUT as libc::c_uint
                | UV__POLLRDHUP
                | UV__POLLPRI),
        0
    );
    debug_assert_ne!(events, 0);
    debug_assert!((*w).fd >= 0);

    (*w).pevents |= events;
    maybe_resize(loop_, (*w).fd as libc::c_uint + 1);

    if (*w).events == (*w).pevents {
        return;
    }

    if queue_empty(std::ptr::addr_of!((*w).watcher_queue)) {
        queue_insert_tail(
            std::ptr::addr_of_mut!((*loop_).watcher_queue),
            std::ptr::addr_of_mut!((*w).watcher_queue),
        );
    }

    let slot = (*loop_).watchers.add((*w).fd as usize);
    if (*slot).is_null() {
        *slot = w;
        (*loop_).nfds += 1;
    }
}

pub(crate) unsafe extern "C" fn uv__io_stop_impl(
    loop_: *mut uv_loop_t,
    w: *mut uv__io_t,
    events: libc::c_uint,
) {
    debug_assert_eq!(
        events
            & !(libc::POLLIN as libc::c_uint
                | libc::POLLOUT as libc::c_uint
                | UV__POLLRDHUP
                | UV__POLLPRI),
        0
    );
    debug_assert_ne!(events, 0);

    if (*w).fd == -1 {
        return;
    }

    debug_assert!((*w).fd >= 0);
    if (*w).fd as libc::c_uint >= (*loop_).nwatchers {
        return;
    }

    (*w).pevents &= !events;

    if (*w).pevents == 0 {
        queue_remove(std::ptr::addr_of_mut!((*w).watcher_queue));
        queue_init(std::ptr::addr_of_mut!((*w).watcher_queue));
        (*w).events = 0;

        let slot = (*loop_).watchers.add((*w).fd as usize);
        if std::ptr::eq(w, *slot) {
            debug_assert!((*loop_).nfds > 0);
            *slot = std::ptr::null_mut();
            (*loop_).nfds -= 1;
        }
    } else if queue_empty(std::ptr::addr_of!((*w).watcher_queue)) {
        queue_insert_tail(
            std::ptr::addr_of_mut!((*loop_).watcher_queue),
            std::ptr::addr_of_mut!((*w).watcher_queue),
        );
    }
}

pub(crate) unsafe extern "C" fn uv__io_close_impl(loop_: *mut uv_loop_t, w: *mut uv__io_t) {
    uv__io_stop_impl(
        loop_,
        w,
        libc::POLLIN as libc::c_uint | libc::POLLOUT as libc::c_uint | UV__POLLRDHUP | UV__POLLPRI,
    );
    queue_remove(std::ptr::addr_of_mut!((*w).pending_queue));

    if (*w).fd != -1 {
        uv__platform_invalidate_fd_impl(loop_, (*w).fd);
    }
}

pub(crate) unsafe extern "C" fn uv__io_feed_impl(loop_: *mut uv_loop_t, w: *mut uv__io_t) {
    if queue_empty(std::ptr::addr_of!((*w).pending_queue)) {
        queue_insert_tail(
            std::ptr::addr_of_mut!((*loop_).pending_queue),
            std::ptr::addr_of_mut!((*w).pending_queue),
        );
    }
}

pub(crate) unsafe extern "C" fn uv__io_active_impl(
    w: *const uv__io_t,
    events: libc::c_uint,
) -> libc::c_int {
    debug_assert_eq!(
        events
            & !(libc::POLLIN as libc::c_uint
                | libc::POLLOUT as libc::c_uint
                | UV__POLLRDHUP
                | UV__POLLPRI),
        0
    );
    debug_assert_ne!(events, 0);
    (((*w).pevents & events) != 0) as libc::c_int
}

pub(crate) unsafe extern "C" fn uv__fd_exists_impl(
    loop_: *mut uv_loop_t,
    fd: libc::c_int,
) -> libc::c_int {
    ((fd as libc::c_uint) < (*loop_).nwatchers
        && !(*((*loop_).watchers.add(fd as usize))).is_null()) as libc::c_int
}
