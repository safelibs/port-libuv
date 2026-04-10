use crate::{abi::linux_x86_64 as abi, core, unix};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_accept(
    server: *mut abi::uv_stream_t,
    client: *mut abi::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::accept(server, client) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_async_init(
    arg1: *mut abi::uv_loop_t,
    async_: *mut abi::uv_async_t,
    async_cb: abi::uv_async_cb,
) -> ::std::os::raw::c_int {
    unsafe { crate::unix_async::init(arg1, async_, async_cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_async_send(async_: *mut abi::uv_async_t) -> ::std::os::raw::c_int {
    unsafe { crate::unix_async::send(async_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_available_parallelism() -> ::std::os::raw::c_uint {
    crate::core::time::available_parallelism_export()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_barrier_destroy(barrier: *mut abi::uv_barrier_t) {
    unsafe {
        crate::threading::sync::barrier_destroy(barrier);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_barrier_init(
    barrier: *mut abi::uv_barrier_t,
    count: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::barrier_init(barrier, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_barrier_wait(barrier: *mut abi::uv_barrier_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::barrier_wait(barrier) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cancel(req: *mut abi::uv_req_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::threadpool::cancel(req) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_chdir(dir: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    unsafe { unix::os::chdir(dir) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_check_init(
    arg1: *mut abi::uv_loop_t,
    check: *mut abi::uv_check_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::check_init(arg1, check) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_check_start(
    check: *mut abi::uv_check_t,
    cb: abi::uv_check_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::check_start(check, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_check_stop(check: *mut abi::uv_check_t) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::check_stop(check) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_clock_gettime(
    clock_id: abi::uv_clock_id,
    ts: *mut abi::uv_timespec64_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::core::time::clock_gettime_export(clock_id, ts) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_close(handle: *mut abi::uv_handle_t, close_cb: abi::uv_close_cb) {
    unsafe { unix::epoll::close(handle, close_cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_broadcast(cond: *mut abi::uv_cond_t) {
    unsafe {
        crate::threading::sync::cond_broadcast(cond);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_destroy(cond: *mut abi::uv_cond_t) {
    unsafe {
        crate::threading::sync::cond_destroy(cond);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_init(cond: *mut abi::uv_cond_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::cond_init(cond) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_signal(cond: *mut abi::uv_cond_t) {
    unsafe {
        crate::threading::sync::cond_signal(cond);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_timedwait(
    cond: *mut abi::uv_cond_t,
    mutex: *mut abi::uv_mutex_t,
    timeout: u64,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::cond_timedwait(cond, mutex, timeout) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cond_wait(cond: *mut abi::uv_cond_t, mutex: *mut abi::uv_mutex_t) {
    unsafe {
        crate::threading::sync::cond_wait(cond, mutex);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cpu_info(
    cpu_infos: *mut *mut abi::uv_cpu_info_t,
    count: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::cpu_info(cpu_infos, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_cwd(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::cwd(buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_default_loop() -> *mut abi::uv_loop_t {
    unsafe { unix::epoll::default_loop() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_disable_stdio_inheritance() {
    unsafe { unix::fd::disable_stdio_inheritance() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlclose(lib: *mut abi::uv_lib_t) {
    unsafe { unix::dl::dlclose(lib) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlerror(lib: *const abi::uv_lib_t) -> *const ::std::os::raw::c_char {
    unsafe { unix::dl::dlerror(lib) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlopen(
    filename: *const ::std::os::raw::c_char,
    lib: *mut abi::uv_lib_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::dl::dlopen(filename, lib) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_dlsym(
    lib: *mut abi::uv_lib_t,
    name: *const ::std::os::raw::c_char,
    ptr: *mut *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe { unix::dl::dlsym(lib, name, ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_err_name(err: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
    unsafe { core::error::err_name(err) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_err_name_r(
    err: ::std::os::raw::c_int,
    buf: *mut ::std::os::raw::c_char,
    buflen: usize,
) -> *mut ::std::os::raw::c_char {
    unsafe { core::error::err_name_r(err, buf, buflen) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_exepath(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::exepath(buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fileno(
    handle: *const abi::uv_handle_t,
    fd: *mut abi::uv_os_fd_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::fileno(handle, fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_free_cpu_info(
    cpu_infos: *mut abi::uv_cpu_info_t,
    count: ::std::os::raw::c_int,
) {
    unsafe { unix::os::free_cpu_info(cpu_infos, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_free_interface_addresses(
    addresses: *mut abi::uv_interface_address_t,
    count: ::std::os::raw::c_int,
) {
    unsafe { unix::fd::free_interface_addresses(addresses, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_freeaddrinfo(ai: *mut abi::addrinfo) {
    unsafe {
        crate::threading::threadpool::freeaddrinfo(ai);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_access(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::access(loop_, req, path, mode, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_chmod(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::chmod(loop_, req, path, mode, cb) }
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
    unsafe { unix::fs::chown(loop_, req, path, uid, gid, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_close(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::close(loop_, req, file, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_closedir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    dir: *mut abi::uv_dir_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::closedir(loop_, req, dir, cb) }
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
    unsafe { unix::fs::copyfile(loop_, req, path, new_path, flags, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_getpath(
    handle: *mut abi::uv_fs_event_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_event::getpath(handle, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_fs_event_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_event::init(loop_, handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_start(
    handle: *mut abi::uv_fs_event_t,
    cb: abi::uv_fs_event_cb,
    path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_event::start(handle, cb, path, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_event_stop(
    handle: *mut abi::uv_fs_event_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_event::stop(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fchmod(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::fchmod(loop_, req, file, mode, cb) }
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
    unsafe { unix::fs::fchown(loop_, req, file, uid, gid, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fdatasync(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::fdatasync(loop_, req, file, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fstat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::fstat(loop_, req, file, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_fsync(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::fsync(loop_, req, file, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_ftruncate(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::ftruncate(loop_, req, file, offset, cb) }
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
    unsafe { unix::fs::futime(loop_, req, file, atime, mtime, cb) }
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
    unsafe { unix::fs::lchown(loop_, req, path, uid, gid, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_link(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    new_path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::link(loop_, req, path, new_path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_lstat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::lstat(loop_, req, path, cb) }
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
    unsafe { unix::fs::lutime(loop_, req, path, atime, mtime, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_mkdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    mode: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::mkdir(loop_, req, path, mode, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_mkdtemp(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    tpl: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::mkdtemp(loop_, req, tpl, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_mkstemp(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    tpl: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::mkstemp(loop_, req, tpl, cb) }
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
    unsafe { unix::fs::open(loop_, req, path, flags, mode, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_opendir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::opendir(loop_, req, path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_getpath(
    handle: *mut abi::uv_fs_poll_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_poll::getpath(handle, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_fs_poll_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_poll::init(loop_, handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_start(
    handle: *mut abi::uv_fs_poll_t,
    poll_cb: abi::uv_fs_poll_cb,
    path: *const ::std::os::raw::c_char,
    interval: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs_poll::start(handle, poll_cb, path, interval) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_poll_stop(handle: *mut abi::uv_fs_poll_t) -> ::std::os::raw::c_int {
    unsafe { unix::fs_poll::stop(handle) }
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
    unsafe { unix::fs::read(loop_, req, file, bufs, nbufs, offset, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_readdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    dir: *mut abi::uv_dir_t,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::readdir(loop_, req, dir, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_readlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::readlink(loop_, req, path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_realpath(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::realpath(loop_, req, path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_rename(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    new_path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::rename(loop_, req, path, new_path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_req_cleanup(req: *mut abi::uv_fs_t) {
    unsafe { unix::fs::req_cleanup(req) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_rmdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::rmdir(loop_, req, path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_scandir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    flags: ::std::os::raw::c_int,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::scandir(loop_, req, path, flags, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_scandir_next(
    req: *mut abi::uv_fs_t,
    ent: *mut abi::uv_dirent_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::scandir_next(req, ent) }
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
    unsafe { unix::fs::sendfile(loop_, req, out_fd, in_fd, in_offset, length, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_stat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::stat(loop_, req, path, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_statfs(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::statfs(loop_, req, path, cb) }
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
    unsafe { unix::fs::symlink(loop_, req, path, new_path, flags, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_fs_unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const ::std::os::raw::c_char,
    cb: abi::uv_fs_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::fs::unlink(loop_, req, path, cb) }
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
    unsafe { unix::fs::utime(loop_, req, path, atime, mtime, cb) }
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
    unsafe { unix::fs::write(loop_, req, file, bufs, nbufs, offset, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_available_memory() -> u64 {
    unsafe { unix::os::get_available_memory() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_constrained_memory() -> u64 {
    unsafe { unix::os::get_constrained_memory() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_free_memory() -> u64 {
    unsafe { unix::os::get_free_memory() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_osfhandle(fd: ::std::os::raw::c_int) -> abi::uv_os_fd_t {
    unsafe { unix::os::get_osfhandle(fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_process_title(
    buffer: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::process_title::get_process_title(buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_get_total_memory() -> u64 {
    unsafe { unix::os::get_total_memory() }
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
    unsafe {
        crate::threading::threadpool::getaddrinfo(loop_, req, getaddrinfo_cb, node, service, hints)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_getnameinfo(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_getnameinfo_t,
    getnameinfo_cb: abi::uv_getnameinfo_cb,
    addr: *const abi::sockaddr,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::getnameinfo::start(loop_, req, getnameinfo_cb, addr, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_getrusage(rusage: *mut abi::uv_rusage_t) -> ::std::os::raw::c_int {
    unsafe { unix::os::getrusage(rusage) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_gettimeofday(tv: *mut abi::uv_timeval64_t) -> ::std::os::raw::c_int {
    unsafe { unix::os::gettimeofday(tv) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_guess_handle(file: abi::uv_file) -> abi::uv_handle_type {
    unsafe { unix::fd::guess_handle(file) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_hrtime() -> u64 {
    unsafe { crate::core::time::hrtime_export() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_idle_init(
    arg1: *mut abi::uv_loop_t,
    idle: *mut abi::uv_idle_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::idle_init(arg1, idle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_idle_start(
    idle: *mut abi::uv_idle_t,
    cb: abi::uv_idle_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::idle_start(idle, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_idle_stop(idle: *mut abi::uv_idle_t) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::idle_stop(idle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_if_indextoiid(
    ifindex: ::std::os::raw::c_uint,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::getaddrinfo::if_indextoiid(ifindex, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_if_indextoname(
    ifindex: ::std::os::raw::c_uint,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::getaddrinfo::if_indextoname(ifindex, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_inet_ntop(
    af: ::std::os::raw::c_int,
    src: *const ::std::os::raw::c_void,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::inet_ntop(af, src, dst, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_inet_pton(
    af: ::std::os::raw::c_int,
    src: *const ::std::os::raw::c_char,
    dst: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::inet_pton(af, src, dst) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_interface_addresses(
    addresses: *mut *mut abi::uv_interface_address_t,
    count: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::interface_addresses(addresses, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip4_addr(
    ip: *const ::std::os::raw::c_char,
    port: ::std::os::raw::c_int,
    addr: *mut abi::sockaddr_in,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::ip4_addr(ip, port, addr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip4_name(
    src: *const abi::sockaddr_in,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::ip4_name(src, dst, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip6_addr(
    ip: *const ::std::os::raw::c_char,
    port: ::std::os::raw::c_int,
    addr: *mut abi::sockaddr_in6,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::ip6_addr(ip, port, addr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip6_name(
    src: *const abi::sockaddr_in6,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::ip6_name(src, dst, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ip_name(
    src: *const abi::sockaddr,
    dst: *mut ::std::os::raw::c_char,
    size: usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::ip_name(src, dst, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_create(key: *mut abi::uv_key_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::key_create(key) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_delete(key: *mut abi::uv_key_t) {
    unsafe {
        crate::threading::sync::key_delete(key);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_get(key: *mut abi::uv_key_t) -> *mut ::std::os::raw::c_void {
    unsafe { crate::threading::sync::key_get(key) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_key_set(key: *mut abi::uv_key_t, value: *mut ::std::os::raw::c_void) {
    unsafe {
        crate::threading::sync::key_set(key, value);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_kill(
    pid: ::std::os::raw::c_int,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::process::kill(pid, signum) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_library_shutdown() {
    unsafe { unix::epoll::library_shutdown() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_listen(
    stream: *mut abi::uv_stream_t,
    backlog: ::std::os::raw::c_int,
    cb: abi::uv_connection_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::listen(stream, backlog, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loadavg(avg: *mut f64) {
    unsafe { unix::os::loadavg(avg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_close(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::loop_close(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_configure(
    loop_: *mut abi::uv_loop_t,
    option: abi::uv_loop_option,
    arg: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::loop_configure(loop_, option, arg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_delete(arg1: *mut abi::uv_loop_t) {
    unsafe { unix::epoll::loop_delete(arg1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_fork(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::loop_fork(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_init(loop_: *mut abi::uv_loop_t) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::loop_init(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_loop_new() -> *mut abi::uv_loop_t {
    unsafe { unix::epoll::loop_new() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_metrics_idle_time(loop_: *mut abi::uv_loop_t) -> u64 {
    unsafe { unix::epoll::metrics_idle_time(loop_) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_metrics_info(
    loop_: *mut abi::uv_loop_t,
    metrics: *mut abi::uv_metrics_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::metrics_info(loop_, metrics) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_destroy(handle: *mut abi::uv_mutex_t) {
    unsafe {
        crate::threading::sync::mutex_destroy(handle);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_init(handle: *mut abi::uv_mutex_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::mutex_init(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_init_recursive(
    handle: *mut abi::uv_mutex_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::mutex_init_recursive(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_lock(handle: *mut abi::uv_mutex_t) {
    unsafe {
        crate::threading::sync::mutex_lock(handle);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_trylock(handle: *mut abi::uv_mutex_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::mutex_trylock(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_mutex_unlock(handle: *mut abi::uv_mutex_t) {
    unsafe {
        crate::threading::sync::mutex_unlock(handle);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_once(
    guard: *mut abi::uv_once_t,
    callback: ::std::option::Option<unsafe extern "C" fn()>,
) {
    unsafe {
        crate::threading::sync::once(guard, callback);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_open_osfhandle(os_fd: abi::uv_os_fd_t) -> ::std::os::raw::c_int {
    unsafe { unix::os::open_osfhandle(os_fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_environ(
    envitems: *mut *mut abi::uv_env_item_t,
    count: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_environ(envitems, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_free_environ(
    envitems: *mut abi::uv_env_item_t,
    count: ::std::os::raw::c_int,
) {
    unsafe { unix::os::os_free_environ(envitems, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_free_group(grp: *mut abi::uv_group_t) {
    unsafe { unix::os::os_free_group(grp) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_free_passwd(pwd: *mut abi::uv_passwd_t) {
    unsafe { unix::os::os_free_passwd(pwd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_get_group(
    grp: *mut abi::uv_group_t,
    gid: abi::uv_uid_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_get_group(grp, gid) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_get_passwd(pwd: *mut abi::uv_passwd_t) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_get_passwd(pwd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_get_passwd2(
    pwd: *mut abi::uv_passwd_t,
    uid: abi::uv_uid_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_get_passwd2(pwd, uid) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getenv(
    name: *const ::std::os::raw::c_char,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_getenv(name, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_gethostname(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_gethostname(buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getpid() -> abi::uv_pid_t {
    unsafe { unix::os::os_getpid() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getppid() -> abi::uv_pid_t {
    unsafe { unix::os::os_getppid() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_getpriority(
    pid: abi::uv_pid_t,
    priority: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_getpriority(pid, priority) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_homedir(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_homedir(buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_setenv(
    name: *const ::std::os::raw::c_char,
    value: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_setenv(name, value) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_setpriority(
    pid: abi::uv_pid_t,
    priority: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_setpriority(pid, priority) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_tmpdir(
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_tmpdir(buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_uname(buffer: *mut abi::uv_utsname_t) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_uname(buffer) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_os_unsetenv(
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { unix::os::os_unsetenv(name) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe(
    fds: *mut abi::uv_file,
    read_flags: ::std::os::raw::c_int,
    write_flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::pipe(fds, read_flags, write_flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_bind(
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::bind_pipe(handle, name) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_bind2(
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    namelen: usize,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::bind2(handle, name, namelen, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_chmod(
    handle: *mut abi::uv_pipe_t,
    flags: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::chmod_pipe(handle, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_connect(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_pipe_t,
    name: *const ::std::os::raw::c_char,
    cb: abi::uv_connect_cb,
) {
    unsafe { unix::pipe::connect_pipe(req, handle, name, cb) }
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
    unsafe { unix::pipe::connect2(req, handle, name, namelen, flags, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_getpeername(
    handle: *const abi::uv_pipe_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::getpeername_pipe(handle, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_getsockname(
    handle: *const abi::uv_pipe_t,
    buffer: *mut ::std::os::raw::c_char,
    size: *mut usize,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::getsockname_pipe(handle, buffer, size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_pipe_t,
    ipc: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::init(arg1, handle, ipc) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_open(
    arg1: *mut abi::uv_pipe_t,
    file: abi::uv_file,
) -> ::std::os::raw::c_int {
    unsafe { unix::pipe::open(arg1, file) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_pending_instances(
    handle: *mut abi::uv_pipe_t,
    count: ::std::os::raw::c_int,
) {
    unsafe { unix::pipe::pending_instances(handle, count) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_pipe_pending_type(handle: *mut abi::uv_pipe_t) -> abi::uv_handle_type {
    unsafe { unix::pipe::pending_type(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_poll_t,
    fd: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::poll::init(loop_, handle, fd) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_init_socket(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_poll_t,
    socket: abi::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::poll::init_socket(loop_, handle, socket) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_start(
    handle: *mut abi::uv_poll_t,
    events: ::std::os::raw::c_int,
    cb: abi::uv_poll_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::poll::start(handle, events, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_poll_stop(handle: *mut abi::uv_poll_t) -> ::std::os::raw::c_int {
    unsafe { unix::poll::stop(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_prepare_init(
    arg1: *mut abi::uv_loop_t,
    prepare: *mut abi::uv_prepare_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::prepare_init(arg1, prepare) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_prepare_start(
    prepare: *mut abi::uv_prepare_t,
    cb: abi::uv_prepare_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::prepare_start(prepare, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_prepare_stop(prepare: *mut abi::uv_prepare_t) -> ::std::os::raw::c_int {
    unsafe { unix::loop_watcher::prepare_stop(prepare) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_print_active_handles(
    loop_: *mut abi::uv_loop_t,
    stream: *mut abi::FILE,
) {
    unsafe { unix::epoll::print_active_handles(loop_, stream) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_print_all_handles(loop_: *mut abi::uv_loop_t, stream: *mut abi::FILE) {
    unsafe { unix::epoll::print_all_handles(loop_, stream) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_process_kill(
    arg1: *mut abi::uv_process_t,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::process::process_kill(arg1, signum) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_queue_work(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_work_t,
    work_cb: abi::uv_work_cb,
    after_work_cb: abi::uv_after_work_cb,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::threadpool::queue_work(loop_, req, work_cb, after_work_cb) }
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
    unsafe { crate::threading::random::random(loop_, req, buf, buflen, flags, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_read_start(
    arg1: *mut abi::uv_stream_t,
    alloc_cb: abi::uv_alloc_cb,
    read_cb: abi::uv_read_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::read_start(arg1, alloc_cb, read_cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_read_stop(arg1: *mut abi::uv_stream_t) -> ::std::os::raw::c_int {
    unsafe { unix::stream::read_stop(arg1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_recv_buffer_size(
    handle: *mut abi::uv_handle_t,
    value: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::recv_buffer_size(handle, value) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_ref(arg1: *mut abi::uv_handle_t) {
    unsafe { unix::epoll::handle_ref(arg1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_replace_allocator(
    malloc_func: abi::uv_malloc_func,
    realloc_func: abi::uv_realloc_func,
    calloc_func: abi::uv_calloc_func,
    free_func: abi::uv_free_func,
) -> ::std::os::raw::c_int {
    core::allocator::replace_allocator(malloc_func, realloc_func, calloc_func, free_func)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_resident_set_memory(rss: *mut usize) -> ::std::os::raw::c_int {
    unsafe { unix::os::resident_set_memory(rss) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_run(
    arg1: *mut abi::uv_loop_t,
    mode: abi::uv_run_mode,
) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::run(arg1, mode) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_destroy(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        crate::threading::sync::rwlock_destroy(rwlock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_init(rwlock: *mut abi::uv_rwlock_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::rwlock_init(rwlock) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_rdlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        crate::threading::sync::rwlock_rdlock(rwlock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_rdunlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        crate::threading::sync::rwlock_rdunlock(rwlock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_tryrdlock(
    rwlock: *mut abi::uv_rwlock_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::rwlock_tryrdlock(rwlock) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_trywrlock(
    rwlock: *mut abi::uv_rwlock_t,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::rwlock_trywrlock(rwlock) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_wrlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        crate::threading::sync::rwlock_wrlock(rwlock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_rwlock_wrunlock(rwlock: *mut abi::uv_rwlock_t) {
    unsafe {
        crate::threading::sync::rwlock_wrunlock(rwlock);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_destroy(sem: *mut abi::uv_sem_t) {
    unsafe {
        crate::threading::sync::sem_destroy(sem);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_init(
    sem: *mut abi::uv_sem_t,
    value: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::sem_init(sem, value) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_post(sem: *mut abi::uv_sem_t) {
    unsafe {
        crate::threading::sync::sem_post(sem);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_trywait(sem: *mut abi::uv_sem_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::sync::sem_trywait(sem) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sem_wait(sem: *mut abi::uv_sem_t) {
    unsafe {
        crate::threading::sync::sem_wait(sem);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_send_buffer_size(
    handle: *mut abi::uv_handle_t,
    value: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::send_buffer_size(handle, value) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_set_process_title(
    title: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { unix::process_title::set_process_title(title) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_shutdown(
    req: *mut abi::uv_shutdown_t,
    handle: *mut abi::uv_stream_t,
    cb: abi::uv_shutdown_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::shutdown_stream(req, handle, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_init(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_signal_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::signal::init(loop_, handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_start(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::signal::start_regular(handle, signal_cb, signum) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_start_oneshot(
    handle: *mut abi::uv_signal_t,
    signal_cb: abi::uv_signal_cb,
    signum: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::signal::start_oneshot(handle, signal_cb, signum) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_signal_stop(handle: *mut abi::uv_signal_t) -> ::std::os::raw::c_int {
    unsafe { unix::signal::stop(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_sleep(msec: ::std::os::raw::c_uint) {
    unsafe {
        crate::core::time::sleep_export(msec);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_socketpair(
    type_: ::std::os::raw::c_int,
    protocol: ::std::os::raw::c_int,
    socket_vector: *mut abi::uv_os_sock_t,
    flags0: ::std::os::raw::c_int,
    flags1: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::socketpair(type_, protocol, socket_vector, flags0, flags1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_spawn(
    loop_: *mut abi::uv_loop_t,
    handle: *mut abi::uv_process_t,
    options: *const abi::uv_process_options_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::process::spawn(loop_, handle, options) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_stop(arg1: *mut abi::uv_loop_t) {
    unsafe { unix::epoll::stop(arg1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_stream_set_blocking(
    handle: *mut abi::uv_stream_t,
    blocking: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::set_blocking(handle, blocking) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_strerror(err: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char {
    unsafe { core::error::strerror(err) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_strerror_r(
    err: ::std::os::raw::c_int,
    buf: *mut ::std::os::raw::c_char,
    buflen: usize,
) -> *mut ::std::os::raw::c_char {
    unsafe { core::error::strerror_r(err, buf, buflen) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_bind(
    handle: *mut abi::uv_tcp_t,
    addr: *const abi::sockaddr,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::bind_tcp(handle, addr, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_close_reset(
    handle: *mut abi::uv_tcp_t,
    close_cb: abi::uv_close_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::close_reset(handle, close_cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_connect(
    req: *mut abi::uv_connect_t,
    handle: *mut abi::uv_tcp_t,
    addr: *const abi::sockaddr,
    cb: abi::uv_connect_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::connect_tcp(req, handle, addr, cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_getpeername(
    handle: *const abi::uv_tcp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::getpeername_tcp(handle, name, namelen) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_getsockname(
    handle: *const abi::uv_tcp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::getsockname_tcp(handle, name, namelen) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tcp_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::init(arg1, handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_init_ex(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_tcp_t,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::init_ex(arg1, handle, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_keepalive(
    handle: *mut abi::uv_tcp_t,
    enable: ::std::os::raw::c_int,
    delay: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::keepalive(handle, enable, delay) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_nodelay(
    handle: *mut abi::uv_tcp_t,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::nodelay(handle, enable) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_open(
    handle: *mut abi::uv_tcp_t,
    sock: abi::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::open(handle, sock) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tcp_simultaneous_accepts(
    handle: *mut abi::uv_tcp_t,
    enable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::tcp::simultaneous_accepts(handle, enable) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_create(
    tid: *mut abi::uv_thread_t,
    entry: abi::uv_thread_cb,
    arg: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::create(tid, entry, arg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_create_ex(
    tid: *mut abi::uv_thread_t,
    params: *const abi::uv_thread_options_t,
    entry: abi::uv_thread_cb,
    arg: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::create_ex(tid, params, entry, arg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_getaffinity(
    tid: *mut abi::uv_thread_t,
    cpumask: *mut ::std::os::raw::c_char,
    mask_size: usize,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::getaffinity(tid, cpumask, mask_size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_getpriority(
    tid: abi::uv_thread_t,
    priority: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::getpriority(tid, priority) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_join(tid: *mut abi::uv_thread_t) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::join(tid) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_self() -> abi::uv_thread_t {
    unsafe { crate::threading::thread::self_thread() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_setaffinity(
    tid: *mut abi::uv_thread_t,
    cpumask: *mut ::std::os::raw::c_char,
    oldmask: *mut ::std::os::raw::c_char,
    mask_size: usize,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::setaffinity(tid, cpumask, oldmask, mask_size) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_thread_setpriority(
    tid: abi::uv_thread_t,
    priority: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { crate::threading::thread::setpriority(tid, priority) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_again(handle: *mut abi::uv_timer_t) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::timer_again(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_timer_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::timer_init(arg1, handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_start(
    handle: *mut abi::uv_timer_t,
    cb: abi::uv_timer_cb,
    timeout: u64,
    repeat: u64,
) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::timer_start(handle, cb, timeout, repeat) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_timer_stop(handle: *mut abi::uv_timer_t) -> ::std::os::raw::c_int {
    unsafe { unix::epoll::timer_stop(handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_translate_sys_error(
    sys_errno: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::fd::translate_sys_error(sys_errno) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_try_write(
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::try_write(handle, bufs, nbufs) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_try_write2(
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    send_handle: *mut abi::uv_stream_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::try_write2(handle, bufs, nbufs, send_handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_get_vterm_state(
    state: *mut abi::uv_tty_vtermstate_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::tty::get_vterm_state(state) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_get_winsize(
    arg1: *mut abi::uv_tty_t,
    width: *mut ::std::os::raw::c_int,
    height: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::tty::get_winsize(arg1, width, height) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_init(
    arg1: *mut abi::uv_loop_t,
    arg2: *mut abi::uv_tty_t,
    fd: abi::uv_file,
    readable: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::tty::init(arg1, arg2, fd, readable) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_reset_mode() -> ::std::os::raw::c_int {
    unsafe { unix::tty::reset_mode() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_set_mode(
    arg1: *mut abi::uv_tty_t,
    mode: abi::uv_tty_mode_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::tty::set_mode(arg1, mode) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_tty_set_vterm_state(state: abi::uv_tty_vtermstate_t) {
    unsafe { unix::tty::set_vterm_state(state) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_bind(
    handle: *mut abi::uv_udp_t,
    addr: *const abi::sockaddr,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::bind_udp(handle, addr, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_connect(
    handle: *mut abi::uv_udp_t,
    addr: *const abi::sockaddr,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::connect_udp(handle, addr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_getpeername(
    handle: *const abi::uv_udp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::getpeername_udp(handle, name, namelen) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_getsockname(
    handle: *const abi::uv_udp_t,
    name: *mut abi::sockaddr,
    namelen: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::getsockname_udp(handle, name, namelen) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_init(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_udp_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::init(arg1, handle) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_init_ex(
    arg1: *mut abi::uv_loop_t,
    handle: *mut abi::uv_udp_t,
    flags: ::std::os::raw::c_uint,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::init_ex(arg1, handle, flags) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_open(
    handle: *mut abi::uv_udp_t,
    sock: abi::uv_os_sock_t,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::open(handle, sock) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_recv_start(
    handle: *mut abi::uv_udp_t,
    alloc_cb: abi::uv_alloc_cb,
    recv_cb: abi::uv_udp_recv_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::recv_start(handle, alloc_cb, recv_cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_recv_stop(handle: *mut abi::uv_udp_t) -> ::std::os::raw::c_int {
    unsafe { unix::udp::recv_stop(handle) }
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
    unsafe { unix::udp::send(req, handle, bufs, nbufs, addr, send_cb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_broadcast(
    handle: *mut abi::uv_udp_t,
    on: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::set_broadcast(handle, on) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_membership(
    handle: *mut abi::uv_udp_t,
    multicast_addr: *const ::std::os::raw::c_char,
    interface_addr: *const ::std::os::raw::c_char,
    membership: abi::uv_membership,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::set_membership(handle, multicast_addr, interface_addr, membership) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_multicast_interface(
    handle: *mut abi::uv_udp_t,
    interface_addr: *const ::std::os::raw::c_char,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::set_multicast_interface(handle, interface_addr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_multicast_loop(
    handle: *mut abi::uv_udp_t,
    on: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::set_multicast_loop(handle, on) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_multicast_ttl(
    handle: *mut abi::uv_udp_t,
    ttl: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::set_multicast_ttl(handle, ttl) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_source_membership(
    handle: *mut abi::uv_udp_t,
    multicast_addr: *const ::std::os::raw::c_char,
    interface_addr: *const ::std::os::raw::c_char,
    source_addr: *const ::std::os::raw::c_char,
    membership: abi::uv_membership,
) -> ::std::os::raw::c_int {
    unsafe {
        unix::udp::set_source_membership(
            handle,
            multicast_addr,
            interface_addr,
            source_addr,
            membership,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_set_ttl(
    handle: *mut abi::uv_udp_t,
    ttl: ::std::os::raw::c_int,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::set_ttl(handle, ttl) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_udp_try_send(
    handle: *mut abi::uv_udp_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    addr: *const abi::sockaddr,
) -> ::std::os::raw::c_int {
    unsafe { unix::udp::try_send(handle, bufs, nbufs, addr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_unref(arg1: *mut abi::uv_handle_t) {
    unsafe { unix::epoll::handle_unref(arg1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_update_time(arg1: *mut abi::uv_loop_t) {
    unsafe { unix::epoll::update_time(arg1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_uptime(uptime: *mut f64) -> ::std::os::raw::c_int {
    unsafe { unix::os::uptime(uptime) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_walk(
    loop_: *mut abi::uv_loop_t,
    walk_cb: abi::uv_walk_cb,
    arg: *mut ::std::os::raw::c_void,
) {
    unsafe { unix::epoll::walk(loop_, walk_cb, arg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn uv_write(
    req: *mut abi::uv_write_t,
    handle: *mut abi::uv_stream_t,
    bufs: *const abi::uv_buf_t,
    nbufs: ::std::os::raw::c_uint,
    cb: abi::uv_write_cb,
) -> ::std::os::raw::c_int {
    unsafe { unix::stream::write_stream(req, handle, bufs, nbufs, cb) }
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
    unsafe { unix::stream::write2(req, handle, bufs, nbufs, send_handle, cb) }
}
