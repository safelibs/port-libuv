use crate::abi::linux_x86_64 as abi;
use crate::threading::threadpool;
use std::os::raw::c_int;

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn start(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getnameinfo_t,
    cb: abi::uv_getnameinfo_cb,
    addr: *const abi::sockaddr,
    flags: c_int,
) -> c_int {
    unsafe { unsafe { threadpool::getnameinfo(loop_, req, cb, addr, flags) } }
}
