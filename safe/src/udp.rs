use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_udp_getpeername"]
        pub(super) fn uv_udp_getpeername(
            handle: *const uv_udp_t,
            name: *mut sockaddr,
            namelen: *mut libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_getsockname"]
        pub(super) fn uv_udp_getsockname(
            handle: *const uv_udp_t,
            name: *mut sockaddr,
            namelen: *mut libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_open"]
        pub(super) fn uv_udp_open(handle: *mut uv_udp_t, sock: uv_os_sock_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_broadcast"]
        pub(super) fn uv_udp_set_broadcast(handle: *mut uv_udp_t, on: libc::c_int) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_membership"]
        pub(super) fn uv_udp_set_membership(
            handle: *mut uv_udp_t,
            multicast_addr: *const libc::c_char,
            interface_addr: *const libc::c_char,
            membership: uv_membership,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_multicast_interface"]
        pub(super) fn uv_udp_set_multicast_interface(
            handle: *mut uv_udp_t,
            interface_addr: *const libc::c_char,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_multicast_loop"]
        pub(super) fn uv_udp_set_multicast_loop(
            handle: *mut uv_udp_t,
            on: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_multicast_ttl"]
        pub(super) fn uv_udp_set_multicast_ttl(
            handle: *mut uv_udp_t,
            ttl: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_source_membership"]
        pub(super) fn uv_udp_set_source_membership(
            handle: *mut uv_udp_t,
            multicast_addr: *const libc::c_char,
            interface_addr: *const libc::c_char,
            source_addr: *const libc::c_char,
            membership: uv_membership,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_set_ttl"]
        pub(super) fn uv_udp_set_ttl(handle: *mut uv_udp_t, ttl: libc::c_int) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_udp_using_recvmmsg"]
        pub(super) fn uv_udp_using_recvmmsg(handle: *const uv_udp_t) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_init(loop_: *mut uv_loop_t, handle: *mut uv_udp_t) -> libc::c_int {
    crate::legacy::uv_udp_init(loop_, handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_init_ex(
    loop_: *mut uv_loop_t,
    handle: *mut uv_udp_t,
    flags: libc::c_uint,
) -> libc::c_int {
    crate::legacy::uv_udp_init_ex(loop_, handle, flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_open(handle: *mut uv_udp_t, sock: uv_os_sock_t) -> libc::c_int {
    private::uv_udp_open(handle, sock)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_bind(
    handle: *mut uv_udp_t,
    addr: *const sockaddr,
    flags: libc::c_uint,
) -> libc::c_int {
    crate::legacy::uv_udp_bind(handle, addr, flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_connect(
    handle: *mut uv_udp_t,
    addr: *const sockaddr,
) -> libc::c_int {
    crate::legacy::uv_udp_connect(handle, addr)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_getpeername(
    handle: *const uv_udp_t,
    name: *mut sockaddr,
    namelen: *mut libc::c_int,
) -> libc::c_int {
    private::uv_udp_getpeername(handle, name, namelen)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_getsockname(
    handle: *const uv_udp_t,
    name: *mut sockaddr,
    namelen: *mut libc::c_int,
) -> libc::c_int {
    private::uv_udp_getsockname(handle, name, namelen)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_membership(
    handle: *mut uv_udp_t,
    multicast_addr: *const libc::c_char,
    interface_addr: *const libc::c_char,
    membership: uv_membership,
) -> libc::c_int {
    private::uv_udp_set_membership(handle, multicast_addr, interface_addr, membership)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_source_membership(
    handle: *mut uv_udp_t,
    multicast_addr: *const libc::c_char,
    interface_addr: *const libc::c_char,
    source_addr: *const libc::c_char,
    membership: uv_membership,
) -> libc::c_int {
    private::uv_udp_set_source_membership(
        handle,
        multicast_addr,
        interface_addr,
        source_addr,
        membership,
    )
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_multicast_loop(
    handle: *mut uv_udp_t,
    on: libc::c_int,
) -> libc::c_int {
    private::uv_udp_set_multicast_loop(handle, on)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_multicast_ttl(
    handle: *mut uv_udp_t,
    ttl: libc::c_int,
) -> libc::c_int {
    private::uv_udp_set_multicast_ttl(handle, ttl)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_multicast_interface(
    handle: *mut uv_udp_t,
    interface_addr: *const libc::c_char,
) -> libc::c_int {
    private::uv_udp_set_multicast_interface(handle, interface_addr)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_broadcast(
    handle: *mut uv_udp_t,
    on: libc::c_int,
) -> libc::c_int {
    private::uv_udp_set_broadcast(handle, on)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_set_ttl(handle: *mut uv_udp_t, ttl: libc::c_int) -> libc::c_int {
    private::uv_udp_set_ttl(handle, ttl)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_send(
    req: *mut uv_udp_send_t,
    handle: *mut uv_udp_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    addr: *const sockaddr,
    send_cb: uv_udp_send_cb,
) -> libc::c_int {
    crate::legacy::uv_udp_send(req, handle, bufs, nbufs, addr, send_cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_try_send(
    handle: *mut uv_udp_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    addr: *const sockaddr,
) -> libc::c_int {
    crate::legacy::uv_udp_try_send(handle, bufs, nbufs, addr)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_recv_start(
    handle: *mut uv_udp_t,
    alloc_cb: uv_alloc_cb,
    recv_cb: uv_udp_recv_cb,
) -> libc::c_int {
    crate::legacy::uv_udp_recv_start(handle, alloc_cb, recv_cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_using_recvmmsg(handle: *const uv_udp_t) -> libc::c_int {
    private::uv_udp_using_recvmmsg(handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_udp_recv_stop(handle: *mut uv_udp_t) -> libc::c_int {
    crate::legacy::uv_udp_recv_stop(handle)
}
