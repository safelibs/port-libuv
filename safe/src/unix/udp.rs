use crate::abi::linux_x86_64 as abi;
use std::os::raw::{c_char, c_int, c_uint};

mod raw {
    use super::*;

    unsafe extern "C" {
        #[link_name = "uv_internal_uv_udp_bind"]
        pub fn uv_udp_bind(
            handle: *mut abi::uv_udp_t,
            addr: *const abi::sockaddr,
            flags: c_uint,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_connect"]
        pub fn uv_udp_connect(handle: *mut abi::uv_udp_t, addr: *const abi::sockaddr) -> c_int;
        #[link_name = "uv_internal_uv_udp_getpeername"]
        pub fn uv_udp_getpeername(
            handle: *const abi::uv_udp_t,
            name: *mut abi::sockaddr,
            namelen: *mut c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_getsockname"]
        pub fn uv_udp_getsockname(
            handle: *const abi::uv_udp_t,
            name: *mut abi::sockaddr,
            namelen: *mut c_int,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_init"]
        pub fn uv_udp_init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_udp_t) -> c_int;
        #[link_name = "uv_internal_uv_udp_init_ex"]
        pub fn uv_udp_init_ex(
            loop_: *mut abi::uv_loop_t,
            handle: *mut abi::uv_udp_t,
            flags: c_uint,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_open"]
        pub fn uv_udp_open(handle: *mut abi::uv_udp_t, sock: abi::uv_os_sock_t) -> c_int;
        #[link_name = "uv_internal_uv_udp_recv_start"]
        pub fn uv_udp_recv_start(
            handle: *mut abi::uv_udp_t,
            alloc_cb: abi::uv_alloc_cb,
            recv_cb: abi::uv_udp_recv_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_recv_stop"]
        pub fn uv_udp_recv_stop(handle: *mut abi::uv_udp_t) -> c_int;
        #[link_name = "uv_internal_uv_udp_send"]
        pub fn uv_udp_send(
            req: *mut abi::uv_udp_send_t,
            handle: *mut abi::uv_udp_t,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
            addr: *const abi::sockaddr,
            send_cb: abi::uv_udp_send_cb,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_broadcast"]
        pub fn uv_udp_set_broadcast(handle: *mut abi::uv_udp_t, on: c_int) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_membership"]
        pub fn uv_udp_set_membership(
            handle: *mut abi::uv_udp_t,
            multicast_addr: *const c_char,
            interface_addr: *const c_char,
            membership: abi::uv_membership,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_multicast_interface"]
        pub fn uv_udp_set_multicast_interface(
            handle: *mut abi::uv_udp_t,
            interface_addr: *const c_char,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_multicast_loop"]
        pub fn uv_udp_set_multicast_loop(handle: *mut abi::uv_udp_t, on: c_int) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_multicast_ttl"]
        pub fn uv_udp_set_multicast_ttl(handle: *mut abi::uv_udp_t, ttl: c_int) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_source_membership"]
        pub fn uv_udp_set_source_membership(
            handle: *mut abi::uv_udp_t,
            multicast_addr: *const c_char,
            interface_addr: *const c_char,
            source_addr: *const c_char,
            membership: abi::uv_membership,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_set_ttl"]
        pub fn uv_udp_set_ttl(handle: *mut abi::uv_udp_t, ttl: c_int) -> c_int;
        #[link_name = "uv_internal_uv_udp_try_send"]
        pub fn uv_udp_try_send(
            handle: *mut abi::uv_udp_t,
            bufs: *const abi::uv_buf_t,
            nbufs: c_uint,
            addr: *const abi::sockaddr,
        ) -> c_int;
        #[link_name = "uv_internal_uv_udp_using_recvmmsg"]
        pub fn uv_udp_using_recvmmsg(handle: *const abi::uv_udp_t) -> c_int;
    }
}

pub(crate) unsafe fn bind(
    handle: *mut abi::uv_udp_t,
    addr: *const abi::sockaddr,
    flags: c_uint,
) -> c_int {
    unsafe { raw::uv_udp_bind(handle, addr, flags) }
}

pub(crate) unsafe fn connect(handle: *mut abi::uv_udp_t, addr: *const abi::sockaddr) -> c_int {
    unsafe { raw::uv_udp_connect(handle, addr) }
}

pub(crate) unsafe fn getpeername(
    handle: *const abi::uv_udp_t,
    name: *mut abi::sockaddr,
    namelen: *mut c_int,
) -> c_int {
    unsafe { raw::uv_udp_getpeername(handle, name, namelen) }
}

pub(crate) unsafe fn getsockname(
    handle: *const abi::uv_udp_t,
    name: *mut abi::sockaddr,
    namelen: *mut c_int,
) -> c_int {
    unsafe { raw::uv_udp_getsockname(handle, name, namelen) }
}

pub(crate) unsafe fn init(loop_: *mut abi::uv_loop_t, handle: *mut abi::uv_udp_t) -> c_int {
    unsafe { raw::uv_udp_init(loop_, handle) }
}

pub(crate) unsafe fn init_ex(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_udp_t,
    flags: c_uint,
) -> c_int {
    unsafe { raw::uv_udp_init_ex(loop_, handle, flags) }
}

pub(crate) unsafe fn open(handle: *mut abi::uv_udp_t, sock: abi::uv_os_sock_t) -> c_int {
    unsafe { raw::uv_udp_open(handle, sock) }
}

pub(crate) unsafe fn recv_start(
    handle: *mut abi::uv_udp_t,
    alloc_cb: abi::uv_alloc_cb,
    recv_cb: abi::uv_udp_recv_cb,
) -> c_int {
    unsafe { raw::uv_udp_recv_start(handle, alloc_cb, recv_cb) }
}

pub(crate) unsafe fn recv_stop(handle: *mut abi::uv_udp_t) -> c_int {
    unsafe { raw::uv_udp_recv_stop(handle) }
}

pub(crate) unsafe fn send(
    req: *mut abi::uv_udp_send_t,
    handle: *mut abi::uv_udp_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    addr: *const abi::sockaddr,
    send_cb: abi::uv_udp_send_cb,
) -> c_int {
    unsafe { raw::uv_udp_send(req, handle, bufs, nbufs, addr, send_cb) }
}

pub(crate) unsafe fn set_broadcast(handle: *mut abi::uv_udp_t, on: c_int) -> c_int {
    unsafe { raw::uv_udp_set_broadcast(handle, on) }
}

pub(crate) unsafe fn set_membership(
    handle: *mut abi::uv_udp_t,
    multicast_addr: *const c_char,
    interface_addr: *const c_char,
    membership: abi::uv_membership,
) -> c_int {
    unsafe { raw::uv_udp_set_membership(handle, multicast_addr, interface_addr, membership) }
}

pub(crate) unsafe fn set_multicast_interface(
    handle: *mut abi::uv_udp_t,
    interface_addr: *const c_char,
) -> c_int {
    unsafe { raw::uv_udp_set_multicast_interface(handle, interface_addr) }
}

pub(crate) unsafe fn set_multicast_loop(handle: *mut abi::uv_udp_t, on: c_int) -> c_int {
    unsafe { raw::uv_udp_set_multicast_loop(handle, on) }
}

pub(crate) unsafe fn set_multicast_ttl(handle: *mut abi::uv_udp_t, ttl: c_int) -> c_int {
    unsafe { raw::uv_udp_set_multicast_ttl(handle, ttl) }
}

pub(crate) unsafe fn set_source_membership(
    handle: *mut abi::uv_udp_t,
    multicast_addr: *const c_char,
    interface_addr: *const c_char,
    source_addr: *const c_char,
    membership: abi::uv_membership,
) -> c_int {
    unsafe {
        raw::uv_udp_set_source_membership(
            handle,
            multicast_addr,
            interface_addr,
            source_addr,
            membership,
        )
    }
}

pub(crate) unsafe fn set_ttl(handle: *mut abi::uv_udp_t, ttl: c_int) -> c_int {
    unsafe { raw::uv_udp_set_ttl(handle, ttl) }
}

pub(crate) unsafe fn try_send(
    handle: *mut abi::uv_udp_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    addr: *const abi::sockaddr,
) -> c_int {
    unsafe { raw::uv_udp_try_send(handle, bufs, nbufs, addr) }
}

pub(crate) unsafe fn using_recvmmsg(handle: *const abi::uv_udp_t) -> c_int {
    unsafe { raw::uv_udp_using_recvmmsg(handle) }
}
