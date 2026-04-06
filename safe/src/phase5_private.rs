use std::arch::global_asm;

global_asm!(
    r#"
.globl uv_phase5_private_uv__io_init
.type uv_phase5_private_uv__io_init, @function
uv_phase5_private_uv__io_init:
    jmp uv__io_init
.size uv_phase5_private_uv__io_init, .-uv_phase5_private_uv__io_init

.globl uv_phase5_private_uv__io_start
.type uv_phase5_private_uv__io_start, @function
uv_phase5_private_uv__io_start:
    jmp uv__io_start
.size uv_phase5_private_uv__io_start, .-uv_phase5_private_uv__io_start

.globl uv_phase5_private_uv__io_stop
.type uv_phase5_private_uv__io_stop, @function
uv_phase5_private_uv__io_stop:
    jmp uv__io_stop
.size uv_phase5_private_uv__io_stop, .-uv_phase5_private_uv__io_stop

.globl uv_phase5_private_uv__io_close
.type uv_phase5_private_uv__io_close, @function
uv_phase5_private_uv__io_close:
    jmp uv__io_close
.size uv_phase5_private_uv__io_close, .-uv_phase5_private_uv__io_close

.globl uv_phase5_private_uv__io_feed
.type uv_phase5_private_uv__io_feed, @function
uv_phase5_private_uv__io_feed:
    jmp uv__io_feed
.size uv_phase5_private_uv__io_feed, .-uv_phase5_private_uv__io_feed

.globl uv_phase5_private_uv__io_active
.type uv_phase5_private_uv__io_active, @function
uv_phase5_private_uv__io_active:
    jmp uv__io_active
.size uv_phase5_private_uv__io_active, .-uv_phase5_private_uv__io_active

.globl uv_phase5_private_uv__fd_exists
.type uv_phase5_private_uv__fd_exists, @function
uv_phase5_private_uv__fd_exists:
    jmp uv__fd_exists
.size uv_phase5_private_uv__fd_exists, .-uv_phase5_private_uv__fd_exists

.globl uv_phase5_private_uv_backend_fd
.type uv_phase5_private_uv_backend_fd, @function
uv_phase5_private_uv_backend_fd:
    jmp uv_backend_fd
.size uv_phase5_private_uv_backend_fd, .-uv_phase5_private_uv_backend_fd

.globl uv_phase5_private_uv_backend_timeout
.type uv_phase5_private_uv_backend_timeout, @function
uv_phase5_private_uv_backend_timeout:
    jmp uv_backend_timeout
.size uv_phase5_private_uv_backend_timeout, .-uv_phase5_private_uv_backend_timeout

.globl uv_phase5_private_uv_close
.type uv_phase5_private_uv_close, @function
uv_phase5_private_uv_close:
    jmp uv_close
.size uv_phase5_private_uv_close, .-uv_phase5_private_uv_close

.globl uv_phase5_private_uv_is_active
.type uv_phase5_private_uv_is_active, @function
uv_phase5_private_uv_is_active:
    jmp uv_is_active
.size uv_phase5_private_uv_is_active, .-uv_phase5_private_uv_is_active

.globl uv_phase5_private_uv_is_closing
.type uv_phase5_private_uv_is_closing, @function
uv_phase5_private_uv_is_closing:
    jmp uv_is_closing
.size uv_phase5_private_uv_is_closing, .-uv_phase5_private_uv_is_closing

.globl uv_phase5_private_uv_loop_alive
.type uv_phase5_private_uv_loop_alive, @function
uv_phase5_private_uv_loop_alive:
    jmp uv_loop_alive
.size uv_phase5_private_uv_loop_alive, .-uv_phase5_private_uv_loop_alive

.globl uv_phase5_private_uv_run
.type uv_phase5_private_uv_run, @function
uv_phase5_private_uv_run:
    jmp uv_run
.size uv_phase5_private_uv_run, .-uv_phase5_private_uv_run

.globl uv_phase5_private_uv_update_time
.type uv_phase5_private_uv_update_time, @function
uv_phase5_private_uv_update_time:
    jmp uv_update_time
.size uv_phase5_private_uv_update_time, .-uv_phase5_private_uv_update_time
"#
);
