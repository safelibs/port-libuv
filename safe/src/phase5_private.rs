use std::arch::global_asm;

global_asm!(
    r#"
.globl uv_phase5_private_uv_close
.hidden uv_phase5_private_uv_close
.type uv_phase5_private_uv_close, @function
uv_phase5_private_uv_close:
    jmp uv_close
.size uv_phase5_private_uv_close, .-uv_phase5_private_uv_close

.globl uv_phase5_private_uv_is_active
.hidden uv_phase5_private_uv_is_active
.type uv_phase5_private_uv_is_active, @function
uv_phase5_private_uv_is_active:
    jmp uv_is_active
.size uv_phase5_private_uv_is_active, .-uv_phase5_private_uv_is_active

.globl uv_phase5_private_uv__io_init
.hidden uv_phase5_private_uv__io_init
.type uv_phase5_private_uv__io_init, @function
uv_phase5_private_uv__io_init:
    jmp uv__io_init
.size uv_phase5_private_uv__io_init, .-uv_phase5_private_uv__io_init

.globl uv_phase5_private_uv__io_start
.hidden uv_phase5_private_uv__io_start
.type uv_phase5_private_uv__io_start, @function
uv_phase5_private_uv__io_start:
    jmp uv__io_start
.size uv_phase5_private_uv__io_start, .-uv_phase5_private_uv__io_start

.globl uv_phase5_private_uv__io_stop
.hidden uv_phase5_private_uv__io_stop
.type uv_phase5_private_uv__io_stop, @function
uv_phase5_private_uv__io_stop:
    jmp uv__io_stop
.size uv_phase5_private_uv__io_stop, .-uv_phase5_private_uv__io_stop

.globl uv_phase5_private_uv__io_close
.hidden uv_phase5_private_uv__io_close
.type uv_phase5_private_uv__io_close, @function
uv_phase5_private_uv__io_close:
    jmp uv__io_close
.size uv_phase5_private_uv__io_close, .-uv_phase5_private_uv__io_close

.globl uv_phase5_private_uv__io_feed
.hidden uv_phase5_private_uv__io_feed
.type uv_phase5_private_uv__io_feed, @function
uv_phase5_private_uv__io_feed:
    jmp uv__io_feed
.size uv_phase5_private_uv__io_feed, .-uv_phase5_private_uv__io_feed

.globl uv_phase5_private_uv__io_active
.hidden uv_phase5_private_uv__io_active
.type uv_phase5_private_uv__io_active, @function
uv_phase5_private_uv__io_active:
    jmp uv__io_active
.size uv_phase5_private_uv__io_active, .-uv_phase5_private_uv__io_active

.globl uv_phase5_private_uv__fd_exists
.hidden uv_phase5_private_uv__fd_exists
.type uv_phase5_private_uv__fd_exists, @function
uv_phase5_private_uv__fd_exists:
    jmp uv__fd_exists
.size uv_phase5_private_uv__fd_exists, .-uv_phase5_private_uv__fd_exists
"#
);
