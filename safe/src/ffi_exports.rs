use crate::bindings::*;
include!(concat!(env!("OUT_DIR"), "/ffi_exports_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/ffi_legacy_aliases_generated.rs"));

global_asm!(
    r#"
.globl uv__async_close
.hidden uv__async_close
.type uv__async_close, @function
uv__async_close:
    jmp {async_close_impl}
.size uv__async_close, .-uv__async_close

.globl uv__async_fork
.hidden uv__async_fork
.type uv__async_fork, @function
uv__async_fork:
    jmp {async_fork_impl}
.size uv__async_fork, .-uv__async_fork

.globl uv__async_stop
.hidden uv__async_stop
.type uv__async_stop, @function
uv__async_stop:
    jmp {async_stop_impl}
.size uv__async_stop, .-uv__async_stop

.globl uv__check_close
.hidden uv__check_close
.type uv__check_close, @function
uv__check_close:
    jmp {check_close_impl}
.size uv__check_close, .-uv__check_close

.globl uv__fd_exists
.hidden uv__fd_exists
.type uv__fd_exists, @function
uv__fd_exists:
    jmp {fd_exists_impl}
.size uv__fd_exists, .-uv__fd_exists

.globl uv__idle_close
.hidden uv__idle_close
.type uv__idle_close, @function
uv__idle_close:
    jmp {idle_close_impl}
.size uv__idle_close, .-uv__idle_close

.globl uv__io_active
.hidden uv__io_active
.type uv__io_active, @function
uv__io_active:
    jmp {io_active_impl}
.size uv__io_active, .-uv__io_active

.globl uv__io_check_fd
.hidden uv__io_check_fd
.type uv__io_check_fd, @function
uv__io_check_fd:
    jmp {io_check_fd_impl}
.size uv__io_check_fd, .-uv__io_check_fd

.globl uv__io_close
.hidden uv__io_close
.type uv__io_close, @function
uv__io_close:
    jmp {io_close_impl}
.size uv__io_close, .-uv__io_close

.globl uv__io_feed
.hidden uv__io_feed
.type uv__io_feed, @function
uv__io_feed:
    jmp {io_feed_impl}
.size uv__io_feed, .-uv__io_feed

.globl uv__io_fork
.hidden uv__io_fork
.type uv__io_fork, @function
uv__io_fork:
    jmp {io_fork_impl}
.size uv__io_fork, .-uv__io_fork

.globl uv__io_init
.hidden uv__io_init
.type uv__io_init, @function
uv__io_init:
    jmp {io_init_impl}
.size uv__io_init, .-uv__io_init

.globl uv__io_poll
.hidden uv__io_poll
.type uv__io_poll, @function
uv__io_poll:
    jmp {io_poll_impl}
.size uv__io_poll, .-uv__io_poll

.globl uv__io_start
.hidden uv__io_start
.type uv__io_start, @function
uv__io_start:
    jmp {io_start_impl}
.size uv__io_start, .-uv__io_start

.globl uv__io_stop
.hidden uv__io_stop
.type uv__io_stop, @function
uv__io_stop:
    jmp {io_stop_impl}
.size uv__io_stop, .-uv__io_stop

.globl uv__metrics_set_provider_entry_time
.hidden uv__metrics_set_provider_entry_time
.type uv__metrics_set_provider_entry_time, @function
uv__metrics_set_provider_entry_time:
    jmp {metrics_set_impl}
.size uv__metrics_set_provider_entry_time, .-uv__metrics_set_provider_entry_time

.globl uv__metrics_update_idle_time
.hidden uv__metrics_update_idle_time
.type uv__metrics_update_idle_time, @function
uv__metrics_update_idle_time:
    jmp {metrics_update_impl}
.size uv__metrics_update_idle_time, .-uv__metrics_update_idle_time

.globl uv__next_timeout
.hidden uv__next_timeout
.type uv__next_timeout, @function
uv__next_timeout:
    jmp {next_timeout_impl}
.size uv__next_timeout, .-uv__next_timeout

.globl uv__platform_invalidate_fd
.hidden uv__platform_invalidate_fd
.type uv__platform_invalidate_fd, @function
uv__platform_invalidate_fd:
    jmp {platform_invalidate_fd_impl}
.size uv__platform_invalidate_fd, .-uv__platform_invalidate_fd

.globl uv__platform_loop_delete
.hidden uv__platform_loop_delete
.type uv__platform_loop_delete, @function
uv__platform_loop_delete:
    jmp {platform_loop_delete_impl}
.size uv__platform_loop_delete, .-uv__platform_loop_delete

.globl uv__platform_loop_init
.hidden uv__platform_loop_init
.type uv__platform_loop_init, @function
uv__platform_loop_init:
    jmp {platform_loop_init_impl}
.size uv__platform_loop_init, .-uv__platform_loop_init

.globl uv__prepare_close
.hidden uv__prepare_close
.type uv__prepare_close, @function
uv__prepare_close:
    jmp {prepare_close_impl}
.size uv__prepare_close, .-uv__prepare_close

.globl uv__run_check
.hidden uv__run_check
.type uv__run_check, @function
uv__run_check:
    jmp {run_check_impl}
.size uv__run_check, .-uv__run_check

.globl uv__run_idle
.hidden uv__run_idle
.type uv__run_idle, @function
uv__run_idle:
    jmp {run_idle_impl}
.size uv__run_idle, .-uv__run_idle

.globl uv__run_prepare
.hidden uv__run_prepare
.type uv__run_prepare, @function
uv__run_prepare:
    jmp {run_prepare_impl}
.size uv__run_prepare, .-uv__run_prepare

.globl uv__run_timers
.hidden uv__run_timers
.type uv__run_timers, @function
uv__run_timers:
    jmp {run_timers_impl}
.size uv__run_timers, .-uv__run_timers

.globl uv__timer_close
.hidden uv__timer_close
.type uv__timer_close, @function
uv__timer_close:
    jmp {timer_close_impl}
.size uv__timer_close, .-uv__timer_close
"#,
    async_close_impl = sym crate::r#async::uv__async_close_impl,
    async_fork_impl = sym crate::r#async::uv__async_fork_impl,
    async_stop_impl = sym crate::r#async::uv__async_stop_impl,
    check_close_impl = sym crate::watchers::uv__check_close_impl,
    fd_exists_impl = sym crate::linux::epoll::uv__fd_exists_impl,
    idle_close_impl = sym crate::watchers::uv__idle_close_impl,
    io_active_impl = sym crate::linux::epoll::uv__io_active_impl,
    io_check_fd_impl = sym crate::linux::epoll::uv__io_check_fd_impl,
    io_close_impl = sym crate::linux::epoll::uv__io_close_impl,
    io_feed_impl = sym crate::linux::epoll::uv__io_feed_impl,
    io_fork_impl = sym crate::linux::epoll::uv__io_fork_impl,
    io_init_impl = sym crate::linux::epoll::uv__io_init_impl,
    io_poll_impl = sym crate::linux::epoll::uv__io_poll_impl,
    io_start_impl = sym crate::linux::epoll::uv__io_start_impl,
    io_stop_impl = sym crate::linux::epoll::uv__io_stop_impl,
    metrics_set_impl = sym crate::r#loop::uv__metrics_set_provider_entry_time_impl,
    metrics_update_impl = sym crate::r#loop::uv__metrics_update_idle_time_impl,
    next_timeout_impl = sym crate::timer::uv__next_timeout_impl,
    platform_invalidate_fd_impl = sym crate::linux::epoll::uv__platform_invalidate_fd_impl,
    platform_loop_delete_impl = sym crate::linux::epoll::uv__platform_loop_delete_impl,
    platform_loop_init_impl = sym crate::linux::epoll::uv__platform_loop_init_impl,
    prepare_close_impl = sym crate::watchers::uv__prepare_close_impl,
    run_check_impl = sym crate::watchers::uv__run_check_impl,
    run_idle_impl = sym crate::watchers::uv__run_idle_impl,
    run_prepare_impl = sym crate::watchers::uv__run_prepare_impl,
    run_timers_impl = sym crate::timer::uv__run_timers_impl,
    timer_close_impl = sym crate::timer::uv__timer_close_impl,
);
