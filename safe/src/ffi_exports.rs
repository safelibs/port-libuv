use crate::bindings::*;
use std::arch::global_asm;

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
"#,
    async_close_impl = sym crate::r#async::uv__async_close_impl,
    async_fork_impl = sym crate::r#async::uv__async_fork_impl,
    async_stop_impl = sym crate::r#async::uv__async_stop_impl,
);
