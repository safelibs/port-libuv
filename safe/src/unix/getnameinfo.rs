use crate::abi::linux_x86_64 as abi;
use crate::threading::threadpool;
use std::os::raw::c_int;

pub(crate) unsafe fn start(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getnameinfo_t,
    cb: abi::uv_getnameinfo_cb,
    addr: *const abi::sockaddr,
    flags: c_int,
) -> c_int {
    unsafe { threadpool::getnameinfo(loop_, req, cb, addr, flags) }
}
