use crate::{abi::linux_x86_64 as abi, stub};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_accept(
    server: *mut abi::uv_stream_t,
    client: *mut abi::uv_stream_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_accept")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_async_init(
    arg1: *mut abi::uv_loop_t,
    async_: *mut abi::uv_async_t,
    async_cb: abi::uv_async_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_async_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_async_send(async_: *mut abi::uv_async_t) -> ::std::os::raw::c_int {
    stub::status("uv_async_send")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_available_parallelism() -> ::std::os::raw::c_uint {
    unsafe { stub::zeroed::<::std::os::raw::c_uint>("uv_available_parallelism") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_barrier_destroy(barrier: *mut abi::uv_barrier_t) {
    stub::void("uv_barrier_destroy");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_barrier_init(
    barrier: *mut abi::uv_barrier_t,
    count: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_barrier_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_barrier_wait(barrier: *mut abi::uv_barrier_t) -> ::std::os::raw::c_int {
    stub::status("uv_barrier_wait")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cancel(req: *mut abi::uv_req_t) -> ::std::os::raw::c_int {
    stub::status("uv_cancel")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_chdir(dir: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    stub::status("uv_chdir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_check_init(
    arg1: *mut abi::uv_loop_t,
    check: *mut abi::uv_check_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_check_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_check_start(
    check: *mut abi::uv_check_t,
    cb: abi::uv_check_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_check_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_check_stop(check: *mut abi::uv_check_t) -> ::std::os::raw::c_int {
    stub::status("uv_check_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_clock_gettime(
    clock_id: abi::uv_clock_id,
    ts: *mut abi::uv_timespec64_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_clock_gettime")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_close(handle: *mut abi::uv_handle_t, close_cb: abi::uv_close_cb) {
    stub::void("uv_close");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_broadcast(cond: *mut abi::uv_cond_t) {
    stub::void("uv_cond_broadcast");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_destroy(cond: *mut abi::uv_cond_t) {
    stub::void("uv_cond_destroy");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_init(cond: *mut abi::uv_cond_t) -> ::std::os::raw::c_int {
    stub::status("uv_cond_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_signal(cond: *mut abi::uv_cond_t) {
    stub::void("uv_cond_signal");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_timedwait(
    cond: *mut abi::uv_cond_t,
    mutex: *mut abi::uv_mutex_t,
    timeout: u64,
) -> ::std::os::raw::c_int {
    stub::status("uv_cond_timedwait")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_wait(cond: *mut abi::uv_cond_t, mutex: *mut abi::uv_mutex_t) {
    stub::void("uv_cond_wait");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cpu_info(
    cpu_infos: *mut *mut abi::uv_cpu_info_t,
    count: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_cpu_info")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cwd(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_cwd")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_default_loop() -> *mut abi::uv_loop_t {
    unsafe { stub::zeroed::<*mut abi::uv_loop_t>("uv_default_loop") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_disable_stdio_inheritance() {
    stub::void("uv_disable_stdio_inheritance");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlclose(lib: *mut abi::uv_lib_t) {
    stub::void("uv_dlclose");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlerror(lib: *const abi::uv_lib_t) -> *const ::std::os::raw::c_char {
    unsafe { stub::zeroed::<*const ::std::os::raw::c_char>("uv_dlerror") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlopen(
    filename: *const ::std::os::raw::c_char,
    lib: *mut abi::uv_lib_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_dlopen")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlsym(
    lib: *mut abi::uv_lib_t,
    name: *const ::std::os::raw::c_char,
    ptr: *mut *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    stub::status("uv_dlsym")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_err_name(err: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
    unsafe { stub::zeroed::<*const ::std::os::raw::c_char>("uv_err_name") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_err_name_r(
    err: ::std::os::raw::c_int,
    buf: *mut ::std::os::raw::c_char,
    buflen: usize,
) -> *mut ::std::os::raw::c_char {
    unsafe { stub::zeroed::<*mut ::std::os::raw::c_char>("uv_err_name_r") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_exepath(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_exepath")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fileno(
    handle: *const abi::uv_handle_t,
    fd: *mut abi::uv_os_fd_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_fileno")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_free_cpu_info(
    cpu_infos: *mut abi::uv_cpu_info_t,
    count: ::std::os::raw::c_int,
) {
    stub::void("uv_free_cpu_info");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_free_interface_addresses(
    addresses: *mut abi::uv_interface_address_t,
    count: ::std::os::raw::c_int,
) {
    stub::void("uv_free_interface_addresses");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_freeaddrinfo(ai: *mut abi::addrinfo) {
    stub::void("uv_freeaddrinfo");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_access(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_access")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_chmod(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_chmod")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_chown(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    uid: abi::uv_uid_t,
    gid: abi::uv_gid_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_chown")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_close(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_close")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_closedir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    dir: *mut abi::uv_dir_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_closedir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_copyfile(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    new_path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_copyfile")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_getpath(
    handle: *mut abi::uv_fs_event_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_event_getpath")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_fs_event_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_event_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_start(
    handle: *mut abi::uv_fs_event_t,
    cb: abi::uv_fs_event_cb,
    path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_event_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_stop(
    handle: *mut abi::uv_fs_event_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_event_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fchmod(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_fchmod")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fchown(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    uid: abi::uv_uid_t,
    gid: abi::uv_gid_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_fchown")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fdatasync(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_fdatasync")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fstat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_fstat")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fsync(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_fsync")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_ftruncate(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_ftruncate")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_futime(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    atime: f64,
    mtime: f64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_futime")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_lchown(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    uid: abi::uv_uid_t,
    gid: abi::uv_gid_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_lchown")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_link(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    new_path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_link")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_lstat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_lstat")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_lutime(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    atime: f64,
    mtime: f64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_lutime")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_mkdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_mkdir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_mkdtemp(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    tpl: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_mkdtemp")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_mkstemp(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    tpl: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_mkstemp")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_open(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_open")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_opendir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_opendir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_getpath(
    handle: *mut abi::uv_fs_poll_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_poll_getpath")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_fs_poll_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_poll_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_start(
    handle: *mut abi::uv_fs_poll_t,
    poll_cb: abi::uv_fs_poll_cb,
    path: *const ::std::os::raw::c_char,
    interval: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_poll_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_stop(handle: *mut abi::uv_fs_poll_t) -> ::std::os::raw::c_int {
    stub::status("uv_fs_poll_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_read(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_read")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_readdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    dir: *mut abi::uv_dir_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_readdir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_readlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_readlink")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_realpath(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_realpath")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_rename(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    new_path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_rename")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_req_cleanup(req: *mut abi::uv_fs_t) {
    stub::void("uv_fs_req_cleanup");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_rmdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_rmdir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_scandir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_scandir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_scandir_next(
    req: *mut abi::uv_fs_t,
    ent: *mut abi::uv_dirent_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_scandir_next")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_sendfile(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    out_fd: abi::uv_file,
    in_fd: abi::uv_file,
    in_offset: i64,
    length: usize,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_sendfile")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_stat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_stat")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_statfs(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_statfs")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_symlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    new_path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_symlink")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_unlink")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_utime(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    atime: f64,
    mtime: f64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_utime")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_write(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_fs_write")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_available_memory() -> u64 {
    unsafe { stub::zeroed::<u64>("uv_get_available_memory") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_constrained_memory() -> u64 {
    unsafe { stub::zeroed::<u64>("uv_get_constrained_memory") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_free_memory() -> u64 {
    unsafe { stub::zeroed::<u64>("uv_get_free_memory") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_osfhandle(fd: ::std::os::raw::c_int) -> abi::uv_os_fd_t {
    unsafe { stub::zeroed::<abi::uv_os_fd_t>("uv_get_osfhandle") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_process_title(
    buffer: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_get_process_title")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_total_memory() -> u64 {
    unsafe { stub::zeroed::<u64>("uv_get_total_memory") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_getaddrinfo(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getaddrinfo_t,
    getaddrinfo_cb: abi::uv_getaddrinfo_cb,
    node: *const ::std::os::raw::c_char,
    service: *const ::std::os::raw::c_char,
    hints: *const abi::addrinfo,
) -> ::std::os::raw::c_int {
    stub::status("uv_getaddrinfo")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_getnameinfo(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getnameinfo_t,
    getnameinfo_cb: abi::uv_getnameinfo_cb,
    addr: *const abi::sockaddr,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_getnameinfo")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_getrusage(rusage: *mut abi::uv_rusage_t) -> ::std::os::raw::c_int {
    stub::status("uv_getrusage")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_gettimeofday(tv: *mut abi::uv_timeval64_t) -> ::std::os::raw::c_int {
    stub::status("uv_gettimeofday")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_guess_handle(file: abi::uv_file) -> abi::uv_handle_type {
    unsafe { stub::zeroed::<abi::uv_handle_type>("uv_guess_handle") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_hrtime() -> u64 {
    unsafe { stub::zeroed::<u64>("uv_hrtime") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_idle_init(
    arg1: *mut abi::uv_loop_t,
    idle: *mut abi::uv_idle_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_idle_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_idle_start(
    idle: *mut abi::uv_idle_t,
    cb: abi::uv_idle_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_idle_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_idle_stop(idle: *mut abi::uv_idle_t) -> ::std::os::raw::c_int {
    stub::status("uv_idle_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_if_indextoiid(
    ifindex: ::std::os::raw::c_uint,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_if_indextoiid")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_if_indextoname(
    ifindex: ::std::os::raw::c_uint,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_if_indextoname")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_inet_ntop(
    af: ::std::os::raw::c_int,
    src: *const ::std::os::raw::c_void,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_inet_ntop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_inet_pton(
    af: ::std::os::raw::c_int,
    src: *const ::std::os::raw::c_char,
    dst: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    stub::status("uv_inet_pton")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_interface_addresses(
    addresses: *mut *mut abi::uv_interface_address_t,
    count: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_interface_addresses")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip4_addr(
    ip: *const ::std::os::raw::c_char,
    port: ::std::os::raw::c_int,
    addr: *mut abi::sockaddr_in,
) -> ::std::os::raw::c_int {
    stub::status("uv_ip4_addr")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip4_name(
    src: *const abi::sockaddr_in,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_ip4_name")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip6_addr(
    ip: *const ::std::os::raw::c_char,
    port: ::std::os::raw::c_int,
    addr: *mut abi::sockaddr_in6,
) -> ::std::os::raw::c_int {
    stub::status("uv_ip6_addr")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip6_name(
    src: *const abi::sockaddr_in6,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_ip6_name")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip_name(
    src: *const abi::sockaddr,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_ip_name")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_create(key: *mut abi::uv_key_t) -> ::std::os::raw::c_int {
    stub::status("uv_key_create")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_delete(key: *mut abi::uv_key_t) {
    stub::void("uv_key_delete");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_get(key: *mut abi::uv_key_t) -> *mut ::std::os::raw::c_void {
    unsafe { stub::zeroed::<*mut ::std::os::raw::c_void>("uv_key_get") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_set(key: *mut abi::uv_key_t, value: *mut ::std::os::raw::c_void) {
    stub::void("uv_key_set");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_kill(
    pid: ::std::os::raw::c_int,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_kill")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_library_shutdown() {
    stub::void("uv_library_shutdown");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_listen(
    stream: *mut abi::uv_stream_t,
    backlog: ::std::os::raw::c_int,
    cb: abi::uv_connection_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_listen")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loadavg(avg: *mut f64) {
    stub::void("uv_loadavg");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_close(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    stub::status("uv_loop_close")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_configure(
    loop_: *mut abi::uv_loop_t,
    option: abi::uv_loop_option,
) -> ::std::os::raw::c_int {
    stub::status("uv_loop_configure")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_delete(arg1: *mut abi::uv_loop_t) {
    stub::void("uv_loop_delete");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_fork(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    stub::status("uv_loop_fork")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_init(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    stub::status("uv_loop_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_new() -> *mut abi::uv_loop_t {
    unsafe { stub::zeroed::<*mut abi::uv_loop_t>("uv_loop_new") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_metrics_idle_time(loop_: *mut abi::uv_loop_t) -> u64 {
    unsafe { stub::zeroed::<u64>("uv_metrics_idle_time") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_metrics_info(
    loop_: *mut abi::uv_loop_t,
    metrics: *mut abi::uv_metrics_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_metrics_info")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_destroy(handle: *mut abi::uv_mutex_t) {
    stub::void("uv_mutex_destroy");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_init(handle: *mut abi::uv_mutex_t) -> ::std::os::raw::c_int {
    stub::status("uv_mutex_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_init_recursive(
    handle: *mut abi::uv_mutex_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_mutex_init_recursive")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_lock(handle: *mut abi::uv_mutex_t) {
    stub::void("uv_mutex_lock");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_trylock(handle: *mut abi::uv_mutex_t) -> ::std::os::raw::c_int {
    stub::status("uv_mutex_trylock")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_unlock(handle: *mut abi::uv_mutex_t) {
    stub::void("uv_mutex_unlock");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_once(
    guard: *mut abi::uv_once_t,
    callback: ::std::option::Option<unsafe extern "C" fn()>,
) {
    stub::void("uv_once");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_open_osfhandle(os_fd: abi::uv_os_fd_t) -> ::std::os::raw::c_int {
    stub::status("uv_open_osfhandle")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_environ(
    envitems: *mut *mut abi::uv_env_item_t,
    count: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_environ")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_free_environ(
    envitems: *mut abi::uv_env_item_t,
    count: ::std::os::raw::c_int,
) {
    stub::void("uv_os_free_environ");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_free_group(grp: *mut abi::uv_group_t) {
    stub::void("uv_os_free_group");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_free_passwd(pwd: *mut abi::uv_passwd_t) {
    stub::void("uv_os_free_passwd");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_get_group(
    grp: *mut abi::uv_group_t,
    gid: abi::uv_uid_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_get_group")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_get_passwd(pwd: *mut abi::uv_passwd_t) -> ::std::os::raw::c_int {
    stub::status("uv_os_get_passwd")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_get_passwd2(
    pwd: *mut abi::uv_passwd_t,
    uid: abi::uv_uid_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_get_passwd2")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getenv(
    name: *const ::std::os::raw::c_char,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_getenv")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_gethostname(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_gethostname")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getpid() -> abi::uv_pid_t {
    unsafe { stub::zeroed::<abi::uv_pid_t>("uv_os_getpid") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getppid() -> abi::uv_pid_t {
    unsafe { stub::zeroed::<abi::uv_pid_t>("uv_os_getppid") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getpriority(
    pid: abi::uv_pid_t,
    priority: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_getpriority")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_homedir(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_homedir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_setenv(
    name: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_setenv")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_setpriority(
    pid: abi::uv_pid_t,
    priority: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_setpriority")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_tmpdir(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_tmpdir")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_uname(buffer: *mut abi::uv_utsname_t) -> ::std::os::raw::c_int {
    stub::status("uv_os_uname")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_unsetenv(
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    stub::status("uv_os_unsetenv")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe(
    fds: *mut abi::uv_file,
    read_flags: ::std::os::raw::c_int,
    write_flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_bind(
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_bind")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_bind2(
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    namelen: usize,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_bind2")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_chmod(
    handle: *mut abi::uv_pipe_t,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_chmod")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_connect(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    cb: abi::uv_connect_cb,
) {
    stub::void("uv_pipe_connect");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_connect2(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    namelen: usize,
    flags: ::std::os::raw::c_uint,
    cb: abi::uv_connect_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_connect2")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_getpeername(
    handle: *const abi::uv_pipe_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_getpeername")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_getsockname(
    handle: *const abi::uv_pipe_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_getsockname")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_pipe_t,
    ipc: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_open(
    arg1: *mut abi::uv_pipe_t,
    file: abi::uv_file,
) -> ::std::os::raw::c_int {
    stub::status("uv_pipe_open")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_pending_instances(
    handle: *mut abi::uv_pipe_t,
    count: ::std::os::raw::c_int,
) {
    stub::void("uv_pipe_pending_instances");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_pending_type(handle: *mut abi::uv_pipe_t) -> abi::uv_handle_type {
    unsafe { stub::zeroed::<abi::uv_handle_type>("uv_pipe_pending_type") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_poll_t,
    fd: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_poll_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_init_socket(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_poll_t,
    socket: abi::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_poll_init_socket")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_start(
    handle: *mut abi::uv_poll_t,
    events: ::std::os::raw::c_int,
    cb: abi::uv_poll_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_poll_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_stop(handle: *mut abi::uv_poll_t) -> ::std::os::raw::c_int {
    stub::status("uv_poll_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_prepare_init(
    arg1: *mut abi::uv_loop_t,
    prepare: *mut abi::uv_prepare_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_prepare_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_prepare_start(
    prepare: *mut abi::uv_prepare_t,
    cb: abi::uv_prepare_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_prepare_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_prepare_stop(prepare: *mut abi::uv_prepare_t) -> ::std::os::raw::c_int {
    stub::status("uv_prepare_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_print_active_handles(
    loop_: *mut abi::uv_loop_t,
    stream: *mut abi::FILE,
) {
    stub::void("uv_print_active_handles");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_print_all_handles(loop_: *mut abi::uv_loop_t, stream: *mut abi::FILE) {
    stub::void("uv_print_all_handles");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_process_kill(
    arg1: *mut abi::uv_process_t,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_process_kill")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_queue_work(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_work_t,
    work_cb: abi::uv_work_cb,
    after_work_cb: abi::uv_after_work_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_queue_work")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_random(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_random_t,
    buf: *mut ::std::os::raw::c_void,
    buflen: usize,
    flags: ::std::os::raw::c_uint,
    cb: abi::uv_random_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_random")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_read_start(
    arg1: *mut abi::uv_stream_t,
    alloc_cb: abi::uv_alloc_cb,
    read_cb: abi::uv_read_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_read_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_read_stop(arg1: *mut abi::uv_stream_t) -> ::std::os::raw::c_int {
    stub::status("uv_read_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_recv_buffer_size(
    handle: *mut abi::uv_handle_t,
    value: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_recv_buffer_size")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ref(arg1: *mut abi::uv_handle_t) {
    stub::void("uv_ref");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_replace_allocator(
    malloc_func: abi::uv_malloc_func,
    realloc_func: abi::uv_realloc_func,
    calloc_func: abi::uv_calloc_func,
    free_func: abi::uv_free_func,
) -> ::std::os::raw::c_int {
    stub::status("uv_replace_allocator")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_resident_set_memory(rss: *mut usize) -> ::std::os::raw::c_int {
    stub::status("uv_resident_set_memory")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_run(
    arg1: *mut abi::uv_loop_t,
    mode: abi::uv_run_mode,
) -> ::std::os::raw::c_int {
    stub::status("uv_run")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_destroy(rwlock: *mut abi::uv_rwlock_t) {
    stub::void("uv_rwlock_destroy");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_init(rwlock: *mut abi::uv_rwlock_t) -> ::std::os::raw::c_int {
    stub::status("uv_rwlock_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_rdlock(rwlock: *mut abi::uv_rwlock_t) {
    stub::void("uv_rwlock_rdlock");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_rdunlock(rwlock: *mut abi::uv_rwlock_t) {
    stub::void("uv_rwlock_rdunlock");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_tryrdlock(
    rwlock: *mut abi::uv_rwlock_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_rwlock_tryrdlock")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_trywrlock(
    rwlock: *mut abi::uv_rwlock_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_rwlock_trywrlock")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_wrlock(rwlock: *mut abi::uv_rwlock_t) {
    stub::void("uv_rwlock_wrlock");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_wrunlock(rwlock: *mut abi::uv_rwlock_t) {
    stub::void("uv_rwlock_wrunlock");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_destroy(sem: *mut abi::uv_sem_t) {
    stub::void("uv_sem_destroy");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_init(
    sem: *mut abi::uv_sem_t,
    value: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_sem_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_post(sem: *mut abi::uv_sem_t) {
    stub::void("uv_sem_post");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_trywait(sem: *mut abi::uv_sem_t) -> ::std::os::raw::c_int {
    stub::status("uv_sem_trywait")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_wait(sem: *mut abi::uv_sem_t) {
    stub::void("uv_sem_wait");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_send_buffer_size(
    handle: *mut abi::uv_handle_t,
    value: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_send_buffer_size")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_set_process_title(
    title: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    stub::status("uv_set_process_title")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_shutdown(
    req: *mut abi::uv_shutdown_t,
    handle: *mut abi::uv_stream_t,
    cb: abi::uv_shutdown_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_shutdown")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_signal_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_signal_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_start(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_signal_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_start_oneshot(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_signal_start_oneshot")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_stop(handle: *mut abi::uv_signal_t) -> ::std::os::raw::c_int {
    stub::status("uv_signal_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sleep(msec: ::std::os::raw::c_uint) {
    stub::void("uv_sleep");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_socketpair(
    type_: ::std::os::raw::c_int,
    protocol: ::std::os::raw::c_int,
    socket_vector: *mut abi::uv_os_sock_t,
    flags0: ::std::os::raw::c_int,
    flags1: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_socketpair")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_spawn(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_process_t,
    options: *const abi::uv_process_options_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_spawn")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_stop(arg1: *mut abi::uv_loop_t) {
    stub::void("uv_stop");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_stream_set_blocking(
    handle: *mut abi::uv_stream_t,
    blocking: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_stream_set_blocking")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_strerror(err: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
    unsafe { stub::zeroed::<*const ::std::os::raw::c_char>("uv_strerror") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_strerror_r(
    err: ::std::os::raw::c_int,
    buf: *mut ::std::os::raw::c_char,
    buflen: usize,
) -> *mut ::std::os::raw::c_char {
    unsafe { stub::zeroed::<*mut ::std::os::raw::c_char>("uv_strerror_r") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_bind(
    handle: *mut abi::uv_tcp_t,
    addr: *const abi::sockaddr,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_bind")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_close_reset(
    handle: *mut abi::uv_tcp_t,
    close_cb: abi::uv_close_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_close_reset")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_connect(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_tcp_t,
    addr: *const abi::sockaddr,
    cb: abi::uv_connect_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_connect")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_getpeername(
    handle: *const abi::uv_tcp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_getpeername")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_getsockname(
    handle: *const abi::uv_tcp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_getsockname")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tcp_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_init_ex(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tcp_t,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_init_ex")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_keepalive(
    handle: *mut abi::uv_tcp_t,
    enable: ::std::os::raw::c_int,
    delay: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_keepalive")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_nodelay(
    handle: *mut abi::uv_tcp_t,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_nodelay")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_open(
    handle: *mut abi::uv_tcp_t,
    sock: abi::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_open")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_simultaneous_accepts(
    handle: *mut abi::uv_tcp_t,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_tcp_simultaneous_accepts")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_create(
    tid: *mut abi::uv_thread_t,
    entry: abi::uv_thread_cb,
    arg: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    stub::status("uv_thread_create")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_create_ex(
    tid: *mut abi::uv_thread_t,
    params: *const abi::uv_thread_options_t,
    entry: abi::uv_thread_cb,
    arg: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    stub::status("uv_thread_create_ex")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_getaffinity(
    tid: *mut abi::uv_thread_t,
    cpumask: *mut ::std::os::raw::c_char,
    mask_size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_thread_getaffinity")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_getpriority(
    tid: abi::uv_thread_t,
    priority: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_thread_getpriority")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_join(tid: *mut abi::uv_thread_t) -> ::std::os::raw::c_int {
    stub::status("uv_thread_join")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_self() -> abi::uv_thread_t {
    unsafe { stub::zeroed::<abi::uv_thread_t>("uv_thread_self") }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_setaffinity(
    tid: *mut abi::uv_thread_t,
    cpumask: *mut ::std::os::raw::c_char,
    oldmask: *mut ::std::os::raw::c_char,
    mask_size: usize,
) -> ::std::os::raw::c_int {
    stub::status("uv_thread_setaffinity")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_setpriority(
    tid: abi::uv_thread_t,
    priority: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_thread_setpriority")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_again(handle: *mut abi::uv_timer_t) -> ::std::os::raw::c_int {
    stub::status("uv_timer_again")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_timer_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_timer_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_start(
    handle: *mut abi::uv_timer_t,
    cb: abi::uv_timer_cb,
    timeout: u64,
    repeat: u64,
) -> ::std::os::raw::c_int {
    stub::status("uv_timer_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_stop(handle: *mut abi::uv_timer_t) -> ::std::os::raw::c_int {
    stub::status("uv_timer_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_translate_sys_error(
    sys_errno: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_translate_sys_error")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_try_write(
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_try_write")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_try_write2(
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    send_handle: *mut abi::uv_stream_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_try_write2")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_get_vterm_state(
    state: *mut abi::uv_tty_vtermstate_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_tty_get_vterm_state")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_get_winsize(
    arg1: *mut abi::uv_tty_t,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_tty_get_winsize")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_init(
    arg1: *mut abi::uv_loop_t,
    arg2: *mut abi::uv_tty_t,
    fd: abi::uv_file,
    readable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_tty_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_reset_mode() -> ::std::os::raw::c_int {
    stub::status("uv_tty_reset_mode")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_set_mode(
    arg1: *mut abi::uv_tty_t,
    mode: abi::uv_tty_mode_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_tty_set_mode")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_set_vterm_state(state: abi::uv_tty_vtermstate_t) {
    stub::void("uv_tty_set_vterm_state");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_bind(
    handle: *mut abi::uv_udp_t,
    addr: *const abi::sockaddr,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_bind")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_connect(
    handle: *mut abi::uv_udp_t,
    addr: *const abi::sockaddr,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_connect")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_getpeername(
    handle: *const abi::uv_udp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_getpeername")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_getsockname(
    handle: *const abi::uv_udp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_getsockname")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_udp_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_init")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_init_ex(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_udp_t,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_init_ex")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_open(
    handle: *mut abi::uv_udp_t,
    sock: abi::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_open")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_recv_start(
    handle: *mut abi::uv_udp_t,
    alloc_cb: abi::uv_alloc_cb,
    recv_cb: abi::uv_udp_recv_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_recv_start")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_recv_stop(handle: *mut abi::uv_udp_t) -> ::std::os::raw::c_int {
    stub::status("uv_udp_recv_stop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_send(
    req: *mut abi::uv_udp_send_t,
    handle: *mut abi::uv_udp_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    addr: *const abi::sockaddr,
    send_cb: abi::uv_udp_send_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_send")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_broadcast(
    handle: *mut abi::uv_udp_t,
    on: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_broadcast")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_membership(
    handle: *mut abi::uv_udp_t,
    multicast_addr: *const ::std::os::raw::c_char,
    interface_addr: *const ::std::os::raw::c_char,
    membership: abi::uv_membership,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_membership")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_multicast_interface(
    handle: *mut abi::uv_udp_t,
    interface_addr: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_multicast_interface")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_multicast_loop(
    handle: *mut abi::uv_udp_t,
    on: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_multicast_loop")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_multicast_ttl(
    handle: *mut abi::uv_udp_t,
    ttl: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_multicast_ttl")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_source_membership(
    handle: *mut abi::uv_udp_t,
    multicast_addr: *const ::std::os::raw::c_char,
    interface_addr: *const ::std::os::raw::c_char,
    source_addr: *const ::std::os::raw::c_char,
    membership: abi::uv_membership,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_source_membership")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_ttl(
    handle: *mut abi::uv_udp_t,
    ttl: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_set_ttl")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_try_send(
    handle: *mut abi::uv_udp_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    addr: *const abi::sockaddr,
) -> ::std::os::raw::c_int {
    stub::status("uv_udp_try_send")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_unref(arg1: *mut abi::uv_handle_t) {
    stub::void("uv_unref");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_update_time(arg1: *mut abi::uv_loop_t) {
    stub::void("uv_update_time");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_uptime(uptime: *mut f64) -> ::std::os::raw::c_int {
    stub::status("uv_uptime")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_walk(
    loop_: *mut abi::uv_loop_t,
    walk_cb: abi::uv_walk_cb,
    arg: *mut ::std::os::raw::c_void,
) {
    stub::void("uv_walk");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_write(
    req: *mut abi::uv_write_t,
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    cb: abi::uv_write_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_write")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_write2(
    req: *mut abi::uv_write_t,
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    send_handle: *mut abi::uv_stream_t,
    cb: abi::uv_write_cb,
) -> ::std::os::raw::c_int {
    stub::status("uv_write2")
}
