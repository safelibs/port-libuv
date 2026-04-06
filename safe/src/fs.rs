use crate::allocator::{self, UV_EINVAL, UV_ENOMEM};
use crate::bindings::*;
use crate::request::{req_register, req_unregister};
use crate::threadpool::{self, WorkKind};
use std::ffi::c_void;
use std::mem::offset_of;

const RUST_FS_MARKER_SLOT: usize = 0;
const RUST_FS_OWNED_BUFS_SLOT: usize = 1;

static RUST_FS_MARKER: u8 = 0;

#[inline]
unsafe fn fs_req_from_work(w: *mut uv__work) -> *mut uv_fs_t {
    w.cast::<u8>()
        .sub(offset_of!(uv_fs_t, work_req))
        .cast::<uv_fs_t>()
}

#[inline]
pub(crate) unsafe fn is_rust_fs_request(req: *mut uv_fs_t) -> bool {
    !req.is_null()
        && std::ptr::eq(
            (*req).reserved[RUST_FS_MARKER_SLOT].cast::<u8>(),
            std::ptr::addr_of!(RUST_FS_MARKER),
        )
}

#[inline]
unsafe fn mark_rust_fs_request(req: *mut uv_fs_t) {
    (*req).reserved[RUST_FS_MARKER_SLOT] = std::ptr::addr_of!(RUST_FS_MARKER)
        .cast_mut()
        .cast::<c_void>();
}

#[inline]
unsafe fn clear_rust_fs_request(req: *mut uv_fs_t) {
    (*req).reserved[RUST_FS_MARKER_SLOT] = std::ptr::null_mut();
    (*req).reserved[RUST_FS_OWNED_BUFS_SLOT] = std::ptr::null_mut();
}

#[inline]
unsafe fn set_owned_bufs(req: *mut uv_fs_t, bufs: *mut c_void) {
    (*req).reserved[RUST_FS_OWNED_BUFS_SLOT] = bufs;
}

#[inline]
unsafe fn owned_bufs(req: *mut uv_fs_t) -> *mut c_void {
    (*req).reserved[RUST_FS_OWNED_BUFS_SLOT]
}

unsafe fn to_uv_stat(src: &libc::stat, dst: *mut uv_stat_t) {
    (*dst).st_dev = src.st_dev as u64;
    (*dst).st_mode = src.st_mode as u64;
    (*dst).st_nlink = src.st_nlink as u64;
    (*dst).st_uid = src.st_uid as u64;
    (*dst).st_gid = src.st_gid as u64;
    (*dst).st_rdev = src.st_rdev as u64;
    (*dst).st_ino = src.st_ino as u64;
    (*dst).st_size = src.st_size as u64;
    (*dst).st_blksize = src.st_blksize as u64;
    (*dst).st_blocks = src.st_blocks as u64;
    (*dst).st_atim.tv_sec = src.st_atime as libc::c_long;
    (*dst).st_atim.tv_nsec = src.st_atime_nsec as libc::c_long;
    (*dst).st_mtim.tv_sec = src.st_mtime as libc::c_long;
    (*dst).st_mtim.tv_nsec = src.st_mtime_nsec as libc::c_long;
    (*dst).st_ctim.tv_sec = src.st_ctime as libc::c_long;
    (*dst).st_ctim.tv_nsec = src.st_ctime_nsec as libc::c_long;
    (*dst).st_birthtim.tv_sec = src.st_ctime as libc::c_long;
    (*dst).st_birthtim.tv_nsec = src.st_ctime_nsec as libc::c_long;
    (*dst).st_flags = 0;
    (*dst).st_gen = 0;
}

unsafe fn init_async_request(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    cb: uv_fs_cb,
    fs_type: uv_fs_type,
) {
    (*req).type_ = uv_req_type_UV_FS;
    (*req).fs_type = fs_type;
    (*req).loop_ = loop_;
    (*req).cb = cb;
    (*req).result = 0;
    (*req).ptr = std::ptr::null_mut();
    (*req).path = std::ptr::null();
    (*req).new_path = std::ptr::null();
    (*req).bufs = std::ptr::null_mut();
    (*req).nbufs = 0;
    mark_rust_fs_request(req);
    set_owned_bufs(req, std::ptr::null_mut());
}

unsafe fn duplicate_path(path: *const libc::c_char) -> Result<*const libc::c_char, libc::c_int> {
    let dup = allocator::strdup(path);
    if dup.is_null() {
        Err(UV_ENOMEM)
    } else {
        Ok(dup)
    }
}

unsafe fn duplicate_path_pair(
    path: *const libc::c_char,
    new_path: *const libc::c_char,
) -> Result<(*const libc::c_char, *const libc::c_char), libc::c_int> {
    let path_len = libc::strlen(path) + 1;
    let new_path_len = libc::strlen(new_path) + 1;
    let dup = allocator::malloc(path_len + new_path_len).cast::<u8>();
    if dup.is_null() {
        return Err(UV_ENOMEM);
    }

    let new_dup = dup.add(path_len);
    std::ptr::copy_nonoverlapping(path.cast::<u8>(), dup, path_len);
    std::ptr::copy_nonoverlapping(new_path.cast::<u8>(), new_dup, new_path_len);
    Ok((dup.cast(), new_dup.cast()))
}

unsafe fn duplicate_bufs(
    req: *mut uv_fs_t,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
) -> libc::c_int {
    if bufs.is_null() || nbufs == 0 {
        return UV_EINVAL;
    }

    (*req).nbufs = nbufs;
    if nbufs as usize <= (*req).bufsml.len() {
        std::ptr::copy_nonoverlapping(bufs, (*req).bufsml.as_mut_ptr(), nbufs as usize);
        (*req).bufs = (*req).bufsml.as_mut_ptr();
        return 0;
    }

    let size = nbufs as usize * std::mem::size_of::<uv_buf_t>();
    let dup = allocator::malloc(size).cast::<uv_buf_t>();
    if dup.is_null() {
        return UV_ENOMEM;
    }

    std::ptr::copy_nonoverlapping(bufs, dup, nbufs as usize);
    (*req).bufs = dup;
    set_owned_bufs(req, dup.cast());
    0
}

unsafe fn submit_request(loop_: *mut uv_loop_t, req: *mut uv_fs_t) -> libc::c_int {
    req_register(loop_, req.cast());
    threadpool::uv__work_submit(
        loop_,
        std::ptr::addr_of_mut!((*req).work_req),
        WorkKind::FastIo,
        Some(uv__fs_work),
        Some(uv__fs_done),
    );
    0
}

unsafe fn finalize_sync_fs_result(req: *mut uv_fs_t, rc: libc::c_int) {
    if rc < 0 {
        (*req).result = rc as isize;
    }
}

unsafe extern "C" fn uv__fs_work(w: *mut uv__work) {
    let req = fs_req_from_work(w);

    if (*req).fs_type == uv_fs_type_UV_FS_STAT {
        let mut statbuf = std::mem::zeroed::<libc::stat>();
        if libc::stat((*req).path, &mut statbuf) == 0 {
            to_uv_stat(&statbuf, std::ptr::addr_of_mut!((*req).statbuf));
            (*req).result = 0;
            (*req).ptr = std::ptr::addr_of_mut!((*req).statbuf).cast();
        } else {
            (*req).result = allocator::last_error() as isize;
            (*req).ptr = std::ptr::null_mut();
        }
        return;
    }

    let loop_ = (*req).loop_;
    let cb = (*req).cb;
    let path = (*req).path;
    let new_path = (*req).new_path;
    let file = (*req).file;
    let flags = (*req).flags;
    let mode = (*req).mode as libc::c_int;
    let nbufs = (*req).nbufs;
    let bufs = (*req).bufs as *const uv_buf_t;
    let off = (*req).off;
    let uid = (*req).uid;
    let gid = (*req).gid;
    let atime = (*req).atime;
    let mtime = (*req).mtime;
    let send_len = (*req).bufsml[0].len;

    (*req).cb = None;

    let rc = match (*req).fs_type {
        uv_fs_type_UV_FS_CHMOD => crate::legacy::uv_fs_chmod(loop_, req, path, mode, None),
        uv_fs_type_UV_FS_CHOWN => crate::legacy::uv_fs_chown(loop_, req, path, uid, gid, None),
        uv_fs_type_UV_FS_CLOSE => crate::legacy::uv_fs_close(loop_, req, file, None),
        uv_fs_type_UV_FS_FCHMOD => crate::legacy::uv_fs_fchmod(loop_, req, file, mode, None),
        uv_fs_type_UV_FS_FCHOWN => crate::legacy::uv_fs_fchown(loop_, req, file, uid, gid, None),
        uv_fs_type_UV_FS_FDATASYNC => crate::legacy::uv_fs_fdatasync(loop_, req, file, None),
        uv_fs_type_UV_FS_FSTAT => crate::legacy::uv_fs_fstat(loop_, req, file, None),
        uv_fs_type_UV_FS_FSYNC => crate::legacy::uv_fs_fsync(loop_, req, file, None),
        uv_fs_type_UV_FS_FTRUNCATE => crate::legacy::uv_fs_ftruncate(loop_, req, file, off, None),
        uv_fs_type_UV_FS_FUTIME => {
            crate::legacy::uv_fs_futime(loop_, req, file, atime, mtime, None)
        }
        uv_fs_type_UV_FS_LINK => crate::legacy::uv_fs_link(loop_, req, path, new_path, None),
        uv_fs_type_UV_FS_LSTAT => crate::legacy::uv_fs_lstat(loop_, req, path, None),
        uv_fs_type_UV_FS_MKDIR => crate::legacy::uv_fs_mkdir(loop_, req, path, mode, None),
        uv_fs_type_UV_FS_OPEN => crate::legacy::uv_fs_open(loop_, req, path, flags, mode, None),
        uv_fs_type_UV_FS_READ => {
            crate::legacy::uv_fs_read(loop_, req, file, bufs, nbufs, off, None)
        }
        uv_fs_type_UV_FS_READLINK => crate::legacy::uv_fs_readlink(loop_, req, path, None),
        uv_fs_type_UV_FS_REALPATH => crate::legacy::uv_fs_realpath(loop_, req, path, None),
        uv_fs_type_UV_FS_RENAME => crate::legacy::uv_fs_rename(loop_, req, path, new_path, None),
        uv_fs_type_UV_FS_SCANDIR => crate::legacy::uv_fs_scandir(loop_, req, path, flags, None),
        uv_fs_type_UV_FS_SENDFILE => {
            crate::legacy::uv_fs_sendfile(loop_, req, file, flags, off, send_len, None)
        }
        uv_fs_type_UV_FS_SYMLINK => {
            crate::legacy::uv_fs_symlink(loop_, req, path, new_path, flags as libc::c_int, None)
        }
        uv_fs_type_UV_FS_UNLINK => crate::legacy::uv_fs_unlink(loop_, req, path, None),
        uv_fs_type_UV_FS_UTIME => crate::legacy::uv_fs_utime(loop_, req, path, atime, mtime, None),
        uv_fs_type_UV_FS_WRITE => {
            crate::legacy::uv_fs_write(loop_, req, file, bufs, nbufs, off, None)
        }
        _ => uv_errno_t_UV_ENOSYS,
    };

    (*req).cb = cb;
    (*req).loop_ = loop_;
    finalize_sync_fs_result(req, rc);
}

unsafe extern "C" fn uv__fs_done(w: *mut uv__work, status: libc::c_int) {
    let req = fs_req_from_work(w);
    req_unregister((*req).loop_, req.cast());

    if status == uv_errno_t_UV_ECANCELED {
        (*req).result = uv_errno_t_UV_ECANCELED as isize;
    }

    if let Some(cb) = (*req).cb {
        cb(req);
    }
}

unsafe fn path_request(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    cb: uv_fs_cb,
    fs_type: uv_fs_type,
) -> libc::c_int {
    if req.is_null() || path.is_null() {
        return UV_EINVAL;
    }
    if cb.is_none() {
        return 0;
    }
    if loop_.is_null() {
        return UV_EINVAL;
    }

    init_async_request(loop_, req, cb, fs_type);
    match duplicate_path(path) {
        Ok(dup) => {
            (*req).path = dup;
            0
        }
        Err(err) => {
            clear_rust_fs_request(req);
            err
        }
    }
}

unsafe fn path_pair_request(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    new_path: *const libc::c_char,
    cb: uv_fs_cb,
    fs_type: uv_fs_type,
) -> libc::c_int {
    if req.is_null() || path.is_null() || new_path.is_null() {
        return UV_EINVAL;
    }
    if cb.is_none() {
        return 0;
    }
    if loop_.is_null() {
        return UV_EINVAL;
    }

    init_async_request(loop_, req, cb, fs_type);
    match duplicate_path_pair(path, new_path) {
        Ok((dup_path, dup_new_path)) => {
            (*req).path = dup_path;
            (*req).new_path = dup_new_path;
            0
        }
        Err(err) => {
            clear_rust_fs_request(req);
            err
        }
    }
}

unsafe fn file_request(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    cb: uv_fs_cb,
    fs_type: uv_fs_type,
) -> libc::c_int {
    if req.is_null() {
        return UV_EINVAL;
    }
    if cb.is_none() {
        return 0;
    }
    if loop_.is_null() {
        return UV_EINVAL;
    }

    init_async_request(loop_, req, cb, fs_type);
    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_chmod(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    mode: libc::c_int,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_chmod(loop_, req, path, mode, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_CHMOD);
    if rc != 0 {
        return rc;
    }
    (*req).mode = mode as mode_t;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_chown(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    uid: uv_uid_t,
    gid: uv_gid_t,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_chown(loop_, req, path, uid, gid, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_CHOWN);
    if rc != 0 {
        return rc;
    }
    (*req).uid = uid;
    (*req).gid = gid;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_close(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_close(loop_, req, file, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_CLOSE);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_fchmod(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    mode: libc::c_int,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_fchmod(loop_, req, file, mode, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FCHMOD);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    (*req).mode = mode as mode_t;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_fchown(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    uid: uv_uid_t,
    gid: uv_gid_t,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_fchown(loop_, req, file, uid, gid, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FCHOWN);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    (*req).uid = uid;
    (*req).gid = gid;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_fdatasync(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_fdatasync(loop_, req, file, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FDATASYNC);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_fstat(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_fstat(loop_, req, file, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FSTAT);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_fsync(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_fsync(loop_, req, file, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FSYNC);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_ftruncate(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    offset: i64,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_ftruncate(loop_, req, file, offset, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FTRUNCATE);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    (*req).off = offset;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_futime(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    atime: f64,
    mtime: f64,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_futime(loop_, req, file, atime, mtime, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_FUTIME);
    if rc != 0 {
        return rc;
    }
    (*req).file = file;
    (*req).atime = atime;
    (*req).mtime = mtime;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_link(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    new_path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_link(loop_, req, path, new_path, None);
    }
    let rc = path_pair_request(loop_, req, path, new_path, cb, uv_fs_type_UV_FS_LINK);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_lstat(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_lstat(loop_, req, path, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_LSTAT);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_mkdir(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    mode: libc::c_int,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_mkdir(loop_, req, path, mode, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_MKDIR);
    if rc != 0 {
        return rc;
    }
    (*req).mode = mode as mode_t;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_open(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    flags: libc::c_int,
    mode: libc::c_int,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_open(loop_, req, path, flags, mode, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_OPEN);
    if rc != 0 {
        return rc;
    }
    (*req).flags = flags;
    (*req).mode = mode as mode_t;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_read(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    offset: i64,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_read(loop_, req, file, bufs, nbufs, offset, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_READ);
    if rc != 0 {
        return rc;
    }
    let rc = duplicate_bufs(req, bufs, nbufs);
    if rc != 0 {
        clear_rust_fs_request(req);
        return rc;
    }
    (*req).file = file;
    (*req).off = offset;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_readlink(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_readlink(loop_, req, path, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_READLINK);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_realpath(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_realpath(loop_, req, path, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_REALPATH);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_rename(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    new_path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_rename(loop_, req, path, new_path, None);
    }
    let rc = path_pair_request(loop_, req, path, new_path, cb, uv_fs_type_UV_FS_RENAME);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_scandir(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    flags: libc::c_int,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_scandir(loop_, req, path, flags, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_SCANDIR);
    if rc != 0 {
        return rc;
    }
    (*req).flags = flags;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_sendfile(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    out_fd: uv_file,
    in_fd: uv_file,
    offset: i64,
    length: usize,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_sendfile(loop_, req, out_fd, in_fd, offset, length, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_SENDFILE);
    if rc != 0 {
        return rc;
    }
    (*req).file = out_fd;
    (*req).flags = in_fd;
    (*req).off = offset;
    (*req).bufsml[0].len = length;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_stat(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_stat(loop_, req, path, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_STAT);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_symlink(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    new_path: *const libc::c_char,
    flags: libc::c_int,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_symlink(loop_, req, path, new_path, flags, None);
    }
    let rc = path_pair_request(loop_, req, path, new_path, cb, uv_fs_type_UV_FS_SYMLINK);
    if rc != 0 {
        return rc;
    }
    (*req).flags = flags;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_unlink(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_unlink(loop_, req, path, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_UNLINK);
    if rc != 0 {
        return rc;
    }
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_utime(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    path: *const libc::c_char,
    atime: f64,
    mtime: f64,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_utime(loop_, req, path, atime, mtime, None);
    }
    let rc = path_request(loop_, req, path, cb, uv_fs_type_UV_FS_UTIME);
    if rc != 0 {
        return rc;
    }
    (*req).atime = atime;
    (*req).mtime = mtime;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_write(
    loop_: *mut uv_loop_t,
    req: *mut uv_fs_t,
    file: uv_file,
    bufs: *const uv_buf_t,
    nbufs: libc::c_uint,
    offset: i64,
    cb: uv_fs_cb,
) -> libc::c_int {
    if cb.is_none() {
        return crate::legacy::uv_fs_write(loop_, req, file, bufs, nbufs, offset, None);
    }
    let rc = file_request(loop_, req, cb, uv_fs_type_UV_FS_WRITE);
    if rc != 0 {
        return rc;
    }
    let rc = duplicate_bufs(req, bufs, nbufs);
    if rc != 0 {
        clear_rust_fs_request(req);
        return rc;
    }
    (*req).file = file;
    (*req).off = offset;
    submit_request(loop_, req)
}

#[no_mangle]
pub unsafe extern "C" fn uv_fs_req_cleanup(req: *mut uv_fs_t) {
    if req.is_null() {
        return;
    }

    if is_rust_fs_request(req) {
        let owned = owned_bufs(req);
        if !owned.is_null() && (*req).bufs.is_null() {
            allocator::free(owned);
        }
        clear_rust_fs_request(req);
    }

    crate::legacy::uv_fs_req_cleanup(req);
}
