use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_fileno"]
        pub(super) fn uv_fileno(handle: *const uv_handle_t, fd: *mut uv_os_fd_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_socketpair"]
        pub(super) fn uv_socketpair(
            type_: libc::c_int,
            protocol: libc::c_int,
            sockets: *mut uv_os_sock_t,
            flags0: libc::c_int,
            flags1: libc::c_int,
        ) -> libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_send_buffer_size(
    handle: *mut uv_handle_t,
    value: *mut libc::c_int,
) -> libc::c_int {
    crate::legacy::uv_send_buffer_size(handle, value)
}

#[no_mangle]
pub unsafe extern "C" fn uv_recv_buffer_size(
    handle: *mut uv_handle_t,
    value: *mut libc::c_int,
) -> libc::c_int {
    crate::legacy::uv_recv_buffer_size(handle, value)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fileno(handle: *const uv_handle_t, fd: *mut uv_os_fd_t) -> libc::c_int {
    private::uv_fileno(handle, fd)
}

#[no_mangle]
pub unsafe extern "C" fn uv_socketpair(
    type_: libc::c_int,
    protocol: libc::c_int,
    sockets: *mut uv_os_sock_t,
    flags0: libc::c_int,
    flags1: libc::c_int,
) -> libc::c_int {
    private::uv_socketpair(type_, protocol, sockets, flags0, flags1)
}
