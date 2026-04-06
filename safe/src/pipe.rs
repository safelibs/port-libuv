use crate::bindings::*;

mod private {
    use crate::bindings::*;

    unsafe extern "C" {
        #[link_name = "uv_phase5_private_uv_pipe"]
        pub(super) fn uv_pipe(
            fds: *mut uv_file,
            read_flags: libc::c_int,
            write_flags: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_bind"]
        pub(super) fn uv_pipe_bind(
            handle: *mut uv_pipe_t,
            name: *const libc::c_char,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_bind2"]
        pub(super) fn uv_pipe_bind2(
            handle: *mut uv_pipe_t,
            name: *const libc::c_char,
            namelen: usize,
            flags: libc::c_uint,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_chmod"]
        pub(super) fn uv_pipe_chmod(handle: *mut uv_pipe_t, flags: libc::c_int) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_connect"]
        pub(super) fn uv_pipe_connect(
            req: *mut uv_connect_t,
            handle: *mut uv_pipe_t,
            name: *const libc::c_char,
            cb: uv_connect_cb,
        );
        #[link_name = "uv_phase5_private_uv_pipe_connect2"]
        pub(super) fn uv_pipe_connect2(
            req: *mut uv_connect_t,
            handle: *mut uv_pipe_t,
            name: *const libc::c_char,
            namelen: usize,
            flags: libc::c_uint,
            cb: uv_connect_cb,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_getpeername"]
        pub(super) fn uv_pipe_getpeername(
            handle: *const uv_pipe_t,
            buffer: *mut libc::c_char,
            size: *mut usize,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_getsockname"]
        pub(super) fn uv_pipe_getsockname(
            handle: *const uv_pipe_t,
            buffer: *mut libc::c_char,
            size: *mut usize,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_init"]
        pub(super) fn uv_pipe_init(
            loop_: *mut uv_loop_t,
            handle: *mut uv_pipe_t,
            ipc: libc::c_int,
        ) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_open"]
        pub(super) fn uv_pipe_open(handle: *mut uv_pipe_t, file: uv_file) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_pending_count"]
        pub(super) fn uv_pipe_pending_count(handle: *mut uv_pipe_t) -> libc::c_int;
        #[link_name = "uv_phase5_private_uv_pipe_pending_instances"]
        pub(super) fn uv_pipe_pending_instances(handle: *mut uv_pipe_t, count: libc::c_int);
        #[link_name = "uv_phase5_private_uv_pipe_pending_type"]
        pub(super) fn uv_pipe_pending_type(handle: *mut uv_pipe_t) -> uv_handle_type;
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe(
    fds: *mut uv_file,
    read_flags: libc::c_int,
    write_flags: libc::c_int,
) -> libc::c_int {
    private::uv_pipe(fds, read_flags, write_flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_init(
    loop_: *mut uv_loop_t,
    handle: *mut uv_pipe_t,
    ipc: libc::c_int,
) -> libc::c_int {
    private::uv_pipe_init(loop_, handle, ipc)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_open(handle: *mut uv_pipe_t, file: uv_file) -> libc::c_int {
    private::uv_pipe_open(handle, file)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_bind(
    handle: *mut uv_pipe_t,
    name: *const libc::c_char,
) -> libc::c_int {
    private::uv_pipe_bind(handle, name)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_bind2(
    handle: *mut uv_pipe_t,
    name: *const libc::c_char,
    namelen: usize,
    flags: libc::c_uint,
) -> libc::c_int {
    private::uv_pipe_bind2(handle, name, namelen, flags)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_connect(
    req: *mut uv_connect_t,
    handle: *mut uv_pipe_t,
    name: *const libc::c_char,
    cb: uv_connect_cb,
) {
    private::uv_pipe_connect(req, handle, name, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_connect2(
    req: *mut uv_connect_t,
    handle: *mut uv_pipe_t,
    name: *const libc::c_char,
    namelen: usize,
    flags: libc::c_uint,
    cb: uv_connect_cb,
) -> libc::c_int {
    private::uv_pipe_connect2(req, handle, name, namelen, flags, cb)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_getsockname(
    handle: *const uv_pipe_t,
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    private::uv_pipe_getsockname(handle, buffer, size)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_getpeername(
    handle: *const uv_pipe_t,
    buffer: *mut libc::c_char,
    size: *mut usize,
) -> libc::c_int {
    private::uv_pipe_getpeername(handle, buffer, size)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_pending_instances(handle: *mut uv_pipe_t, count: libc::c_int) {
    private::uv_pipe_pending_instances(handle, count)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_pending_count(handle: *mut uv_pipe_t) -> libc::c_int {
    private::uv_pipe_pending_count(handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_pending_type(handle: *mut uv_pipe_t) -> uv_handle_type {
    private::uv_pipe_pending_type(handle)
}

#[no_mangle]
pub unsafe extern "C" fn uv_pipe_chmod(handle: *mut uv_pipe_t, flags: libc::c_int) -> libc::c_int {
    private::uv_pipe_chmod(handle, flags)
}
