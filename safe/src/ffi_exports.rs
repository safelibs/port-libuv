use crate::bindings::*;
use std::arch::global_asm;

include!(concat!(env!("OUT_DIR"), "/ffi_exports_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/ffi_support_aliases_generated.rs"));

global_asm!(
    r#"
.globl uv__fd_exists
.type uv__fd_exists, @function
uv__fd_exists:
    jmp {fd_exists_impl}
.size uv__fd_exists, .-uv__fd_exists

.globl uv__io_check_fd
.type uv__io_check_fd, @function
uv__io_check_fd:
    jmp {io_check_fd_impl}
.size uv__io_check_fd, .-uv__io_check_fd

.globl uv__async_close
.type uv__async_close, @function
uv__async_close:
    jmp {async_close_impl}
.size uv__async_close, .-uv__async_close

.globl uv__async_fork
.type uv__async_fork, @function
uv__async_fork:
    jmp {async_fork_impl}
.size uv__async_fork, .-uv__async_fork

.globl uv__async_stop
.type uv__async_stop, @function
uv__async_stop:
    jmp {async_stop_impl}
.size uv__async_stop, .-uv__async_stop

.globl uv__io_active
.type uv__io_active, @function
uv__io_active:
    jmp {io_active_impl}
.size uv__io_active, .-uv__io_active

.globl uv__io_close
.type uv__io_close, @function
uv__io_close:
    jmp {io_close_impl}
.size uv__io_close, .-uv__io_close

.globl uv__io_feed
.type uv__io_feed, @function
uv__io_feed:
    jmp {io_feed_impl}
.size uv__io_feed, .-uv__io_feed

.globl uv__io_init
.type uv__io_init, @function
uv__io_init:
    jmp {io_init_impl}
.size uv__io_init, .-uv__io_init

.globl uv__io_start
.type uv__io_start, @function
uv__io_start:
    jmp {io_start_impl}
.size uv__io_start, .-uv__io_start

.globl uv__io_stop
.type uv__io_stop, @function
uv__io_stop:
    jmp {io_stop_impl}
.size uv__io_stop, .-uv__io_stop

.globl uv__platform_invalidate_fd
.type uv__platform_invalidate_fd, @function
uv__platform_invalidate_fd:
    jmp {platform_invalidate_fd_impl}
.size uv__platform_invalidate_fd, .-uv__platform_invalidate_fd
"#,
    fd_exists_impl = sym crate::linux::epoll::uv__fd_exists_impl,
    async_close_impl = sym crate::r#async::uv__async_close_impl,
    async_fork_impl = sym crate::r#async::uv__async_fork_impl,
    async_stop_impl = sym crate::r#async::uv__async_stop_impl,
    io_active_impl = sym crate::linux::epoll::uv__io_active_impl,
    io_check_fd_impl = sym crate::linux::epoll::uv__io_check_fd_impl,
    io_close_impl = sym crate::linux::epoll::uv__io_close_impl,
    io_feed_impl = sym crate::linux::epoll::uv__io_feed_impl,
    io_init_impl = sym crate::linux::epoll::uv__io_init_impl,
    io_start_impl = sym crate::linux::epoll::uv__io_start_impl,
    io_stop_impl = sym crate::linux::epoll::uv__io_stop_impl,
    platform_invalidate_fd_impl = sym crate::linux::epoll::uv__platform_invalidate_fd_impl,
);
