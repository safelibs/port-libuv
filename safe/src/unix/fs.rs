use crate::abi::linux_x86_64 as abi;
use crate::core::{allocator, queue};
use crate::threading::threadpool::{self, TaskClass};
use libc;
use std::cmp::min;
use std::ffi::{CStr, CString};
use std::mem::{offset_of, zeroed};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::{self, null, null_mut};
use std::slice;

const SLOT_PRIMARY: usize = 0;
const SLOT_PATH: usize = 1;
const SLOT_NEW_PATH: usize = 2;
const SLOT_BUFS: usize = 3;
const SLOT_AUX: usize = 4;
const SLOT_EXTRA: usize = 5;

fn uv_err(errno: c_int) -> c_int {
    if errno <= 0 {
        errno
    } else {
        -errno
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn last_errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn get_iovmax() -> usize {
    let rc = unsafe { libc::sysconf(libc::_SC_IOV_MAX) };
    if rc > 0 {
        rc as usize
    } else {
        1024
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn clone_c_string(value: *const c_char) -> *mut c_char {
    if value.is_null() {
        return null_mut();
    }

    let bytes = unsafe { CStr::from_ptr(value).to_bytes_with_nul() };
    let ptr = unsafe { allocator::malloc_bytes(bytes.len()) }.cast::<c_char>();
    if ptr.is_null() {
        return null_mut();
    }

    unsafe {
        ptr::copy_nonoverlapping(value, ptr, bytes.len());
    }
    ptr
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn free_slot(req: *mut abi::uv_fs_t, slot: usize) {
    unsafe {
        let ptr = unsafe { (*req).reserved[slot] };
        if !ptr.is_null() {
            unsafe {
                allocator::free_bytes(ptr);
                (*req).reserved[slot] = null_mut();
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn cleanup_owned_allocations(req: *mut abi::uv_fs_t) {
    unsafe {
        for slot in 0..(*req).reserved.len() {
            unsafe {
                free_slot(req, slot);
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn reset_request_pointers(req: *mut abi::uv_fs_t) {
    unsafe {
        unsafe {
            (*req).path = null();
            (*req).new_path = null();
            (*req).bufs = null_mut();
            (*req).ptr = null_mut();
            (*req).nbufs = 0;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn fill_uv_stat(out: *mut abi::uv_stat_t, st: *const abi::stat) {
    unsafe {
        unsafe {
            (*out).st_dev = (*st).st_dev;
            (*out).st_mode = (*st).st_mode as u64;
            (*out).st_nlink = (*st).st_nlink;
            (*out).st_uid = (*st).st_uid as u64;
            (*out).st_gid = (*st).st_gid as u64;
            (*out).st_rdev = (*st).st_rdev;
            (*out).st_ino = (*st).st_ino;
            (*out).st_size = (*st).st_size as u64;
            (*out).st_blksize = (*st).st_blksize as u64;
            (*out).st_blocks = (*st).st_blocks as u64;
            (*out).st_flags = 0;
            (*out).st_gen = 0;
            (*out).st_atim = abi::uv_timespec_t {
                tv_sec: (*st).st_atim.tv_sec as _,
                tv_nsec: (*st).st_atim.tv_nsec as _,
            };
            (*out).st_mtim = abi::uv_timespec_t {
                tv_sec: (*st).st_mtim.tv_sec as _,
                tv_nsec: (*st).st_mtim.tv_nsec as _,
            };
            (*out).st_ctim = abi::uv_timespec_t {
                tv_sec: (*st).st_ctim.tv_sec as _,
                tv_nsec: (*st).st_ctim.tv_nsec as _,
            };
            (*out).st_birthtim = abi::uv_timespec_t::default();
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn fill_uv_statfs(out: *mut abi::uv_statfs_t, st: *const libc::statfs) {
    unsafe {
        unsafe {
            (*out).f_type = (*st).f_type as u64;
            (*out).f_bsize = (*st).f_bsize as u64;
            (*out).f_blocks = (*st).f_blocks as u64;
            (*out).f_bfree = (*st).f_bfree as u64;
            (*out).f_bavail = (*st).f_bavail as u64;
            (*out).f_files = (*st).f_files as u64;
            (*out).f_ffree = (*st).f_ffree as u64;
            (*out).f_spare = [0; 4];
        }
    }
}

fn dirent_type(dt: u8) -> abi::uv_dirent_type_t {
    match dt {
        libc::DT_DIR => abi::uv_dirent_type_t_UV_DIRENT_DIR,
        libc::DT_REG => abi::uv_dirent_type_t_UV_DIRENT_FILE,
        libc::DT_LNK => abi::uv_dirent_type_t_UV_DIRENT_LINK,
        libc::DT_FIFO => abi::uv_dirent_type_t_UV_DIRENT_FIFO,
        libc::DT_SOCK => abi::uv_dirent_type_t_UV_DIRENT_SOCKET,
        libc::DT_CHR => abi::uv_dirent_type_t_UV_DIRENT_CHAR,
        libc::DT_BLK => abi::uv_dirent_type_t_UV_DIRENT_BLOCK,
        _ => abi::uv_dirent_type_t_UV_DIRENT_UNKNOWN,
    }
}

fn fs_to_timespec(time: f64) -> libc::timespec {
    let mut sec = time.trunc() as libc::time_t;
    let mut nsec = ((time - sec as f64) * 1e9) as libc::c_long;
    nsec -= nsec % 1000;
    if nsec < 0 {
        nsec += 1_000_000_000;
        sec -= 1;
    }
    libc::timespec {
        tv_sec: sec,
        tv_nsec: nsec,
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn set_request_error(req: *mut abi::uv_fs_t, err: c_int) {
    unsafe {
        unsafe {
            (*req).result = uv_err(err) as isize;
            (*req).ptr = null_mut();
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn fs_req_init(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    fs_type: abi::uv_fs_type,
    cb: abi::uv_fs_cb,
) {
    unsafe {
        let data = unsafe { (*req).data };
        unsafe {
            ptr::write_bytes(req, 0, 1);
            (*req).data = data;
            (*req).type_ = abi::uv_req_type_UV_FS;
            (*req).fs_type = fs_type;
            (*req).loop_ = loop_;
            (*req).cb = cb;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn clone_path_into(req: *mut abi::uv_fs_t, path: *const c_char, slot: usize) -> c_int {
    unsafe {
        let cloned = clone_c_string(path);
        if path.is_null() || !cloned.is_null() {
            unsafe {
                (*req).path = cloned;
                (*req).reserved[slot] = cloned.cast();
            }
            return 0;
        }
        abi::uv_errno_t_UV_ENOMEM
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn clone_paths(req: *mut abi::uv_fs_t, path: *const c_char, new_path: *const c_char) -> c_int {
    unsafe {
        let rc = unsafe { clone_path_into(req, path, SLOT_PATH) };
        if rc != 0 {
            return rc;
        }

        let cloned = clone_c_string(new_path);
        if new_path.is_null() || !cloned.is_null() {
            unsafe {
                (*req).new_path = cloned;
                (*req).reserved[SLOT_NEW_PATH] = cloned.cast();
            }
            return 0;
        }

        unsafe {
            cleanup_owned_allocations(req);
            reset_request_pointers(req);
        }
        abi::uv_errno_t_UV_ENOMEM
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn copy_bufs(
    req: *mut abi::uv_fs_t,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    stable: bool,
) -> c_int {
    unsafe {
        if bufs.is_null() || nbufs == 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            (*req).nbufs = nbufs;
        }

        if !stable {
            unsafe {
                (*req).bufs = bufs.cast_mut();
            }
            return 0;
        }

        let nbufs = nbufs as usize;
        if nbufs <= unsafe { (*req).bufsml.len() } {
            unsafe {
                ptr::copy_nonoverlapping(bufs, (*req).bufsml.as_mut_ptr(), nbufs);
                (*req).bufs = (*req).bufsml.as_mut_ptr();
            }
            return 0;
        }

        let bytes = match nbufs.checked_mul(std::mem::size_of::<abi::uv_buf_t>()) {
            Some(bytes) => bytes,
            None => return abi::uv_errno_t_UV_ENOMEM,
        };
        let storage = unsafe { allocator::malloc_bytes(bytes) }.cast::<abi::uv_buf_t>();
        if storage.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        unsafe {
            ptr::copy_nonoverlapping(bufs, storage, nbufs);
            (*req).bufs = storage;
            (*req).reserved[SLOT_BUFS] = storage.cast();
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn finish_read_or_write(req: *mut abi::uv_fs_t) {
    unsafe {
        unsafe {
            free_slot(req, SLOT_BUFS);
            (*req).bufs = null_mut();
            (*req).nbufs = 0;
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn scandir_entries(req: *mut abi::uv_fs_t) -> *mut abi::uv_dirent_t {
    unsafe { unsafe { (*req).ptr.cast::<abi::uv_dirent_t>() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn readdir_cleanup(req: *mut abi::uv_fs_t) {
    unsafe {
        let dir = unsafe { (*req).ptr.cast::<abi::uv_dir_t>() };
        if dir.is_null() {
            return;
        }

        let dirents = unsafe { (*dir).dirents };
        let count = unsafe { (*req).result.max(0) as usize };
        unsafe {
            (*req).ptr = null_mut();
        }
        if dirents.is_null() {
            return;
        }

        for index in 0..count {
            let entry = unsafe { dirents.add(index) };
            let name = unsafe { (*entry).name.cast_mut() };
            if !name.is_null() {
                unsafe {
                    allocator::free_bytes(name.cast());
                    (*entry).name = null();
                }
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn make_scandir_result(
    req: *mut abi::uv_fs_t,
    entries: &mut Vec<(CString, abi::uv_dirent_type_t)>,
) -> c_int {
    unsafe {
        if entries.is_empty() {
            unsafe {
                (*req).result = 0;
                (*req).ptr = null_mut();
                (*req).nbufs = 0;
            }
            return 0;
        }

        entries.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

        let count = entries.len();
        let names_bytes = entries
            .iter()
            .fold(0usize, |sum, entry| sum + entry.0.as_bytes_with_nul().len());
        let dirent_bytes = match count.checked_mul(std::mem::size_of::<abi::uv_dirent_t>()) {
            Some(bytes) => bytes,
            None => return abi::uv_errno_t_UV_ENOMEM,
        };
        let total_bytes = match dirent_bytes.checked_add(names_bytes) {
            Some(bytes) => bytes,
            None => return abi::uv_errno_t_UV_ENOMEM,
        };
        let block = unsafe { allocator::malloc_bytes(total_bytes) }.cast::<u8>();
        if block.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        let array = block.cast::<abi::uv_dirent_t>();
        let mut cursor = unsafe { block.add(dirent_bytes) };
        for (index, (name, type_)) in entries.iter().enumerate() {
            let bytes = name.as_bytes_with_nul();
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), cursor, bytes.len());
                (*array.add(index)).name = cursor.cast();
                (*array.add(index)).type_ = *type_;
                cursor = cursor.add(bytes.len());
            }
        }

        unsafe {
            (*req).reserved[SLOT_AUX] = block.cast();
            (*req).ptr = array.cast();
            (*req).result = count as isize;
            (*req).nbufs = 0;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn read_directory_entries(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let dir = unsafe { libc::opendir((*req).path) };
        if dir.is_null() {
            return uv_err(last_errno());
        }

        let mut entries = Vec::new();
        loop {
            unsafe {
                *libc::__errno_location() = 0;
            }
            let dent = unsafe { libc::readdir(dir) };
            if dent.is_null() {
                let err = last_errno();
                unsafe {
                    libc::closedir(dir);
                }
                if err == 0 {
                    return unsafe { make_scandir_result(req, &mut entries) };
                }
                return uv_err(err);
            }

            let name = unsafe { CStr::from_ptr((*dent).d_name.as_ptr()) };
            let bytes = name.to_bytes();
            if bytes == b"." || bytes == b".." {
                continue;
            }

            let Some(name) = CString::new(bytes).ok() else {
                continue;
            };
            entries.push((name, dirent_type(unsafe { (*dent).d_type })));
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn readlink_pathmax(path: *const c_char) -> usize {
    unsafe {
        let rc = unsafe { libc::pathconf(path, libc::_PC_PATH_MAX) };
        if rc > 0 {
            return rc as usize;
        }
        if libc::PATH_MAX > 0 {
            libc::PATH_MAX as usize
        } else {
            4096
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn alloc_readlink_result(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let mut maxlen = unsafe { readlink_pathmax((*req).path) };
        if maxlen == 0 {
            maxlen = 4096;
        }

        let buf = unsafe { allocator::malloc_bytes(maxlen + 1) }.cast::<c_char>();
        if buf.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        let len = unsafe { libc::readlink((*req).path, buf, maxlen) };
        if len < 0 {
            unsafe {
                allocator::free_bytes(buf.cast());
            }
            return uv_err(last_errno());
        }

        unsafe {
            *buf.add(len as usize) = 0;
            (*req).reserved[SLOT_AUX] = buf.cast();
            (*req).ptr = buf.cast();
            (*req).result = 0;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn alloc_realpath_result(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let resolved = unsafe { libc::realpath((*req).path, null_mut()) };
        if resolved.is_null() {
            return uv_err(last_errno());
        }

        unsafe {
            (*req).reserved[SLOT_AUX] = resolved.cast();
            (*req).ptr = resolved.cast();
            (*req).result = 0;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn alloc_statfs_result(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let out = unsafe { allocator::alloc_zeroed::<abi::uv_statfs_t>() };
        if out.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        let mut st: libc::statfs = unsafe { zeroed() };
        if unsafe { libc::statfs((*req).path, &mut st) } != 0 {
            unsafe {
                allocator::free_bytes(out.cast());
            }
            return uv_err(last_errno());
        }

        unsafe {
            fill_uv_statfs(out, &st);
            (*req).reserved[SLOT_AUX] = out.cast();
            (*req).ptr = out.cast();
            (*req).result = 0;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_opendir(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let dir = unsafe { allocator::alloc_zeroed::<abi::uv_dir_t>() };
        if dir.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }

        let stream = unsafe { libc::opendir((*req).path) };
        if stream.is_null() {
            unsafe {
                allocator::free_bytes(dir.cast());
                (*req).ptr = null_mut();
            }
            return uv_err(last_errno());
        }

        unsafe {
            (*dir).dir = stream.cast();
            (*req).ptr = dir.cast();
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_readdir(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let dir = unsafe { (*req).ptr.cast::<abi::uv_dir_t>() };
        if dir.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let stream = unsafe { (*dir).dir.cast::<libc::DIR>() };
        let dirents = unsafe { (*dir).dirents };
        let limit = unsafe { (*dir).nentries };
        if stream.is_null() || dirents.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        for index in 0..limit {
            unsafe {
                (*dirents.add(index)).name = null();
                (*dirents.add(index)).type_ = abi::uv_dirent_type_t_UV_DIRENT_UNKNOWN;
            }
        }

        let mut count = 0usize;
        while count < limit {
            unsafe {
                *libc::__errno_location() = 0;
            }
            let dent = unsafe { libc::readdir(stream) };
            if dent.is_null() {
                let err = last_errno();
                if err == 0 {
                    return count as c_int;
                }

                unsafe {
                    (*req).result = count as isize;
                    readdir_cleanup(req);
                }
                return uv_err(err);
            }

            let name = unsafe { CStr::from_ptr((*dent).d_name.as_ptr()) };
            let bytes = name.to_bytes();
            if bytes == b"." || bytes == b".." {
                continue;
            }

            let cloned = clone_c_string(name.as_ptr());
            if cloned.is_null() {
                unsafe {
                    (*req).result = count as isize;
                    readdir_cleanup(req);
                }
                return abi::uv_errno_t_UV_ENOMEM;
            }

            unsafe {
                (*dirents.add(count)).name = cloned;
                (*dirents.add(count)).type_ = dirent_type((*dent).d_type);
            }
            count += 1;
        }

        count as c_int
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_closedir(req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        let dir = unsafe { (*req).ptr.cast::<abi::uv_dir_t>() };
        if dir.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let stream = unsafe { (*dir).dir.cast::<libc::DIR>() };
        if !stream.is_null() {
            unsafe {
                libc::closedir(stream);
                (*dir).dir = null_mut();
            }
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_sendfile(req: *mut abi::uv_fs_t) -> isize {
    unsafe {
        let in_fd = unsafe { (*req).flags };
        let out_fd = unsafe { (*req).file };
        let mut off = unsafe { (*req).off as i64 as libc::off_t };
        let len = unsafe { (*req).bufsml[0].len };

        let rc = unsafe { libc::sendfile(out_fd, in_fd, &mut off, len) };
        if rc >= 0 {
            unsafe {
                (*req).off = off as abi::off_t;
            }
            rc as isize
        } else {
            uv_err(last_errno()) as isize
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn close_fd(fd: c_int) -> c_int {
    unsafe {
        let rc = unsafe { libc::close(fd) };
        if rc == 0 {
            0
        } else {
            let err = last_errno();
            if err == libc::EINTR || err == libc::EINPROGRESS {
                0
            } else {
                uv_err(err)
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn is_cifs_or_smb(fd: c_int) -> bool {
    unsafe {
        let mut st: libc::statfs = unsafe { zeroed() };
        if unsafe { libc::fstatfs(fd, &mut st) } != 0 {
            return false;
        }

        matches!(st.f_type as u64, 0x0000_517B | 0xFE53_4D42 | 0xFF53_4D42)
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn copy_regular_data(srcfd: c_int, dstfd: c_int, size: libc::off_t) -> c_int {
    unsafe {
        let mut offset: libc::off_t = 0;
        let mut remaining = size;
        let mut buffer = [0u8; 65_536];

        while remaining > 0 {
            let chunk = min(remaining as usize, buffer.len());
            let read_rc = loop {
                let rc = unsafe { libc::pread(srcfd, buffer.as_mut_ptr().cast(), chunk, offset) };
                if rc < 0 && last_errno() == libc::EINTR {
                    continue;
                }
                break rc;
            };

            if read_rc < 0 {
                return uv_err(last_errno());
            }
            if read_rc == 0 {
                break;
            }

            let mut written = 0usize;
            while written < read_rc as usize {
                let write_rc = loop {
                    let rc = unsafe {
                        libc::write(
                            dstfd,
                            buffer[written..read_rc as usize].as_ptr().cast(),
                            read_rc as usize - written,
                        )
                    };
                    if rc < 0 && last_errno() == libc::EINTR {
                        continue;
                    }
                    break rc;
                };

                if write_rc < 0 {
                    return uv_err(last_errno());
                }
                written += write_rc as usize;
            }

            offset += read_rc as libc::off_t;
            remaining -= read_rc as libc::off_t;
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn finish_copyfile(srcfd: c_int, dstfd: c_int, mut err: c_int) -> isize {
    unsafe {
        if dstfd >= 0 {
            let close_err = unsafe { close_fd(dstfd) };
            if err == 0 {
                err = close_err;
            }
        }
        if srcfd >= 0 {
            let close_err = unsafe { close_fd(srcfd) };
            if err == 0 {
                err = close_err;
            }
        }
        err as isize
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_copyfile(req: *mut abi::uv_fs_t) -> isize {
    unsafe {
        let mut srcfd = -1;
        let mut dstfd = -1;
        let mut src_stat: abi::stat = unsafe { zeroed() };
        let mut dst_stat: abi::stat = unsafe { zeroed() };

        srcfd = unsafe { libc::open((*req).path, libc::O_RDONLY) };
        if srcfd < 0 {
            return uv_err(last_errno()) as isize;
        }

        if unsafe { libc::fstat(srcfd, (&mut src_stat as *mut abi::stat).cast()) } != 0 {
            return unsafe { finish_copyfile(srcfd, dstfd, uv_err(last_errno())) };
        }

        let mut dst_flags = libc::O_WRONLY | libc::O_CREAT;
        if unsafe { (*req).flags } & abi::UV_FS_COPYFILE_EXCL as c_int != 0 {
            dst_flags |= libc::O_EXCL;
        }

        dstfd = unsafe { libc::open((*req).new_path, dst_flags, src_stat.st_mode as libc::mode_t) };
        if dstfd < 0 {
            return unsafe { finish_copyfile(srcfd, dstfd, uv_err(last_errno())) };
        }

        if unsafe { (*req).flags } & abi::UV_FS_COPYFILE_EXCL as c_int == 0 {
            if unsafe { libc::fstat(dstfd, (&mut dst_stat as *mut abi::stat).cast()) } != 0 {
                return unsafe { finish_copyfile(srcfd, dstfd, uv_err(last_errno())) };
            }

            if src_stat.st_dev == dst_stat.st_dev && src_stat.st_ino == dst_stat.st_ino {
                return unsafe { finish_copyfile(srcfd, dstfd, 0) };
            }

            if unsafe { libc::ftruncate(dstfd, 0) } != 0 {
                let err = uv_err(last_errno());
                if err != abi::uv_errno_t_UV_EACCES || dst_stat.st_size > 0 {
                    return unsafe { finish_copyfile(srcfd, dstfd, err) };
                }
            }
        }

        if unsafe { libc::fchmod(dstfd, src_stat.st_mode as libc::mode_t) } != 0 {
            let err = uv_err(last_errno());
            if err != abi::uv_errno_t_UV_EPERM || unsafe { !is_cifs_or_smb(dstfd) } {
                return unsafe { finish_copyfile(srcfd, dstfd, err) };
            }
        }

        if unsafe { (*req).flags }
            & (abi::UV_FS_COPYFILE_FICLONE as c_int | abi::UV_FS_COPYFILE_FICLONE_FORCE as c_int)
            != 0
        {
            if unsafe { libc::ioctl(dstfd, libc::FICLONE, srcfd) } == 0 {
                return unsafe { finish_copyfile(srcfd, dstfd, 0) };
            }

            if unsafe { (*req).flags } & abi::UV_FS_COPYFILE_FICLONE_FORCE as c_int != 0 {
                return unsafe { finish_copyfile(srcfd, dstfd, uv_err(last_errno())) };
            }
        }

        let err = unsafe { copy_regular_data(srcfd, dstfd, src_stat.st_size as libc::off_t) };
        unsafe { finish_copyfile(srcfd, dstfd, err) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_read(req: *mut abi::uv_fs_t) -> isize {
    unsafe {
        let bufs = unsafe { slice::from_raw_parts((*req).bufs, (*req).nbufs as usize) };
        let nbufs = min(bufs.len(), get_iovmax());
        let off = unsafe { (*req).off as i64 };
        let file = unsafe { (*req).file };

        let rc = if off < 0 {
            if nbufs == 1 {
                let buf = bufs[0];
                unsafe { libc::read(file, buf.base.cast(), buf.len) }
            } else {
                unsafe { libc::readv(file, bufs.as_ptr().cast(), nbufs as c_int) }
            }
        } else if nbufs == 1 {
            let buf = bufs[0];
            unsafe { libc::pread(file, buf.base.cast(), buf.len, off as libc::off_t) }
        } else {
            unsafe {
                libc::preadv(
                    file,
                    bufs.as_ptr().cast(),
                    nbufs as c_int,
                    off as libc::off_t,
                )
            }
        };

        unsafe {
            finish_read_or_write(req);
        }

        if rc >= 0 {
            rc as isize
        } else {
            uv_err(last_errno()) as isize
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_write_all(req: *mut abi::uv_fs_t) -> isize {
    unsafe {
        let file = unsafe { (*req).file };
        let mut off = unsafe { (*req).off as i64 };
        let mut bufs =
            unsafe { slice::from_raw_parts((*req).bufs, (*req).nbufs as usize).to_vec() };
        let iovmax = get_iovmax();
        let mut total: isize = 0;

        while !bufs.is_empty() {
            let chunk = min(bufs.len(), iovmax);
            let rc = if off < 0 {
                if chunk == 1 {
                    let buf = bufs[0];
                    unsafe { libc::write(file, buf.base.cast(), buf.len) }
                } else {
                    unsafe { libc::writev(file, bufs.as_ptr().cast(), chunk as c_int) }
                }
            } else if chunk == 1 {
                let buf = bufs[0];
                unsafe { libc::pwrite(file, buf.base.cast(), buf.len, off as libc::off_t) }
            } else {
                unsafe {
                    libc::pwritev(
                        file,
                        bufs.as_ptr().cast(),
                        chunk as c_int,
                        off as libc::off_t,
                    )
                }
            };

            if rc < 0 && last_errno() == libc::EINTR {
                continue;
            }

            if rc <= 0 {
                unsafe {
                    finish_read_or_write(req);
                }
                if total == 0 {
                    return if rc == 0 {
                        0
                    } else {
                        uv_err(last_errno()) as isize
                    };
                }
                return total;
            }

            total += rc as isize;
            if off >= 0 {
                off += rc as i64;
            }

            let mut written = rc as usize;
            while written > 0 && !bufs.is_empty() {
                if bufs[0].len <= written {
                    written -= bufs[0].len;
                    bufs.remove(0);
                } else {
                    bufs[0].base = unsafe { bufs[0].base.add(written) };
                    bufs[0].len -= written;
                    written = 0;
                }
            }
        }

        unsafe {
            (*req).off = off as abi::off_t;
            finish_read_or_write(req);
        }
        total
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn execute_fs(req: *mut abi::uv_fs_t) {
    unsafe {
        let result = match unsafe { (*req).fs_type } {
            abi::uv_fs_type_UV_FS_OPEN => {
                let rc = unsafe {
                    libc::open(
                        (*req).path,
                        (*req).flags | libc::O_CLOEXEC,
                        (*req).mode as libc::mode_t,
                    )
                };
                if rc >= 0 {
                    unsafe {
                        (*req).file = rc;
                    }
                    rc as isize
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_CLOSE => {
                let rc = unsafe { libc::close((*req).file) };
                if rc == 0 {
                    0
                } else {
                    let err = last_errno();
                    if err == libc::EINTR || err == libc::EINPROGRESS {
                        0
                    } else {
                        uv_err(err) as isize
                    }
                }
            }
            abi::uv_fs_type_UV_FS_READ => unsafe { run_read(req) },
            abi::uv_fs_type_UV_FS_WRITE => unsafe { run_write_all(req) },
            abi::uv_fs_type_UV_FS_SENDFILE => unsafe { run_sendfile(req) },
            abi::uv_fs_type_UV_FS_COPYFILE => unsafe { run_copyfile(req) },
            abi::uv_fs_type_UV_FS_STAT => {
                let mut st: abi::stat = unsafe { zeroed() };
                let rc = unsafe { libc::stat((*req).path, (&mut st as *mut abi::stat).cast()) };
                if rc == 0 {
                    unsafe {
                        fill_uv_stat(ptr::addr_of_mut!((*req).statbuf), &st);
                        (*req).ptr = ptr::addr_of_mut!((*req).statbuf).cast();
                    }
                    0
                } else {
                    unsafe {
                        (*req).ptr = null_mut();
                    }
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_LSTAT => {
                let mut st: abi::stat = unsafe { zeroed() };
                let rc = unsafe { libc::lstat((*req).path, (&mut st as *mut abi::stat).cast()) };
                if rc == 0 {
                    unsafe {
                        fill_uv_stat(ptr::addr_of_mut!((*req).statbuf), &st);
                        (*req).ptr = ptr::addr_of_mut!((*req).statbuf).cast();
                    }
                    0
                } else {
                    unsafe {
                        (*req).ptr = null_mut();
                    }
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FSTAT => {
                let mut st: abi::stat = unsafe { zeroed() };
                let rc = unsafe { libc::fstat((*req).file, (&mut st as *mut abi::stat).cast()) };
                if rc == 0 {
                    unsafe {
                        fill_uv_stat(ptr::addr_of_mut!((*req).statbuf), &st);
                        (*req).ptr = ptr::addr_of_mut!((*req).statbuf).cast();
                    }
                    0
                } else {
                    unsafe {
                        (*req).ptr = null_mut();
                    }
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FTRUNCATE => {
                let rc = unsafe { libc::ftruncate((*req).file, (*req).off as i64 as libc::off_t) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_ACCESS => {
                let rc = unsafe { libc::access((*req).path, (*req).flags) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_CHMOD => {
                let rc = unsafe { libc::chmod((*req).path, (*req).mode as libc::mode_t) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FCHMOD => {
                let rc = unsafe { libc::fchmod((*req).file, (*req).mode as libc::mode_t) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FSYNC => {
                let rc = unsafe { libc::fsync((*req).file) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FDATASYNC => {
                let rc = unsafe { libc::fdatasync((*req).file) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_UNLINK => {
                let rc = unsafe { libc::unlink((*req).path) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_RMDIR => {
                let rc = unsafe { libc::rmdir((*req).path) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_MKDIR => {
                let rc = unsafe { libc::mkdir((*req).path, (*req).mode as libc::mode_t) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_MKDTEMP => {
                let rc = unsafe { libc::mkdtemp((*req).path.cast_mut()) };
                if rc.is_null() {
                    uv_err(last_errno()) as isize
                } else {
                    0
                }
            }
            abi::uv_fs_type_UV_FS_MKSTEMP => {
                let rc = unsafe { libc::mkstemp((*req).path.cast_mut()) };
                if rc >= 0 {
                    unsafe {
                        (*req).file = rc;
                    }
                    rc as isize
                } else {
                    unsafe {
                        *(*req).path.cast_mut() = 0;
                    }
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_RENAME => {
                let rc = unsafe { libc::rename((*req).path, (*req).new_path) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_SCANDIR => unsafe {
                let rc = read_directory_entries(req);
                if rc == 0 {
                    (*req).result
                } else {
                    (*req).ptr = null_mut();
                    rc as isize
                }
            },
            abi::uv_fs_type_UV_FS_OPENDIR => {
                let rc = unsafe { run_opendir(req) };
                if rc == 0 {
                    0
                } else {
                    rc as isize
                }
            }
            abi::uv_fs_type_UV_FS_READDIR => unsafe { run_readdir(req) as isize },
            abi::uv_fs_type_UV_FS_CLOSEDIR => {
                let rc = unsafe { run_closedir(req) };
                if rc == 0 {
                    0
                } else {
                    rc as isize
                }
            }
            abi::uv_fs_type_UV_FS_LINK => {
                let rc = unsafe { libc::link((*req).path, (*req).new_path) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_SYMLINK => {
                let rc = unsafe { libc::symlink((*req).path, (*req).new_path) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_READLINK => {
                let rc = unsafe { alloc_readlink_result(req) };
                if rc == 0 {
                    0
                } else {
                    rc as isize
                }
            }
            abi::uv_fs_type_UV_FS_CHOWN => {
                let rc = unsafe {
                    libc::chown(
                        (*req).path,
                        (*req).uid as libc::uid_t,
                        (*req).gid as libc::gid_t,
                    )
                };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FCHOWN => {
                let rc = unsafe {
                    libc::fchown(
                        (*req).file,
                        (*req).uid as libc::uid_t,
                        (*req).gid as libc::gid_t,
                    )
                };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_REALPATH => {
                let rc = unsafe { alloc_realpath_result(req) };
                if rc == 0 {
                    0
                } else {
                    rc as isize
                }
            }
            abi::uv_fs_type_UV_FS_LCHOWN => {
                let rc = unsafe {
                    libc::lchown(
                        (*req).path,
                        (*req).uid as libc::uid_t,
                        (*req).gid as libc::gid_t,
                    )
                };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_STATFS => {
                let rc = unsafe { alloc_statfs_result(req) };
                if rc == 0 {
                    0
                } else {
                    rc as isize
                }
            }
            abi::uv_fs_type_UV_FS_UTIME => {
                let ts = [
                    fs_to_timespec(unsafe { (*req).atime }),
                    fs_to_timespec(unsafe { (*req).mtime }),
                ];
                let rc = unsafe { libc::utimensat(libc::AT_FDCWD, (*req).path, ts.as_ptr(), 0) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_FUTIME => {
                let ts = [
                    fs_to_timespec(unsafe { (*req).atime }),
                    fs_to_timespec(unsafe { (*req).mtime }),
                ];
                let rc = unsafe { libc::futimens((*req).file, ts.as_ptr()) };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            abi::uv_fs_type_UV_FS_LUTIME => {
                let ts = [
                    fs_to_timespec(unsafe { (*req).atime }),
                    fs_to_timespec(unsafe { (*req).mtime }),
                ];
                let rc = unsafe {
                    libc::utimensat(
                        libc::AT_FDCWD,
                        (*req).path,
                        ts.as_ptr(),
                        libc::AT_SYMLINK_NOFOLLOW,
                    )
                };
                if rc == 0 {
                    0
                } else {
                    uv_err(last_errno()) as isize
                }
            }
            _ => abi::uv_errno_t_UV_ENOSYS as isize,
        };

        unsafe {
            (*req).result = result;
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn fs_work(work: *mut abi::uv__work) {
    unsafe {
        let req = unsafe {
            work.cast::<u8>()
                .sub(offset_of!(abi::uv_fs_t, work_req))
                .cast::<abi::uv_fs_t>()
        };
        unsafe {
            execute_fs(req);
        }
    }
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn fs_done(work: *mut abi::uv__work, status: c_int) {
    unsafe {
        let req = unsafe {
            work.cast::<u8>()
                .sub(offset_of!(abi::uv_fs_t, work_req))
                .cast::<abi::uv_fs_t>()
        };

        if status == abi::uv_errno_t_UV_ECANCELED {
            unsafe {
                (*req).result = status as isize;
            }
        }

        if let Some(cb) = unsafe { (*req).cb } {
            unsafe {
                cb(req);
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn run_or_submit(loop_: *mut abi::uv_loop_t, req: *mut abi::uv_fs_t) -> c_int {
    unsafe {
        if unsafe { (*req).cb }.is_none() {
            unsafe {
                fs_work(ptr::addr_of_mut!((*req).work_req));
                (*req).result as c_int
            }
        } else {
            unsafe {
                (*req).work_req.work = Some(fs_work);
                (*req).work_req.done = Some(fs_done);
                queue::init(ptr::addr_of_mut!((*req).work_req.wq));
                threadpool::submit(
                    loop_,
                    req.cast(),
                    ptr::addr_of_mut!((*req).work_req),
                    TaskClass::SlowIo,
                )
            }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn req_cleanup(req: *mut abi::uv_fs_t) {
    unsafe {
        if req.is_null() {
            return;
        }

        let statbuf_ptr = unsafe { ptr::addr_of_mut!((*req).statbuf).cast::<std::ffi::c_void>() };

        if unsafe { (*req).path != null() }
            && (unsafe { (*req).cb }.is_some()
                || matches!(
                    unsafe { (*req).fs_type },
                    abi::uv_fs_type_UV_FS_MKDTEMP | abi::uv_fs_type_UV_FS_MKSTEMP
                ))
        {
            unsafe {
                free_slot(req, SLOT_PATH);
            }
        }

        if unsafe { (*req).new_path != null() } && unsafe { (*req).cb }.is_some() {
            unsafe {
                free_slot(req, SLOT_NEW_PATH);
            }
        }

        unsafe {
            (*req).path = null();
            (*req).new_path = null();
        }

        if unsafe { (*req).fs_type } == abi::uv_fs_type_UV_FS_READDIR
            && unsafe { !(*req).ptr.is_null() }
        {
            unsafe {
                readdir_cleanup(req);
            }
        }

        unsafe {
            free_slot(req, SLOT_BUFS);
        }

        if unsafe { !(*req).reserved[SLOT_AUX].is_null() } {
            unsafe {
                free_slot(req, SLOT_AUX);
            }
        } else if unsafe { !(*req).ptr.is_null() }
            && unsafe { (*req).ptr } != statbuf_ptr
            && unsafe { (*req).fs_type } != abi::uv_fs_type_UV_FS_OPENDIR
            && unsafe { (*req).fs_type } != abi::uv_fs_type_UV_FS_READDIR
        {
            unsafe {
                allocator::free_bytes((*req).ptr.cast());
                (*req).ptr = null_mut();
            }
        }

        unsafe {
            free_slot(req, SLOT_PRIMARY);
            free_slot(req, SLOT_EXTRA);
            reset_request_pointers(req);
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn scandir_next(req: *mut abi::uv_fs_t, ent: *mut abi::uv_dirent_t) -> c_int {
    unsafe {
        if req.is_null() || ent.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        if unsafe { (*req).result } < 0 {
            return unsafe { (*req).result as c_int };
        }

        if unsafe { (*req).ptr.is_null() } {
            return abi::uv_errno_t_UV_EOF;
        }

        let index = unsafe { (*req).nbufs as usize };
        let total = unsafe { (*req).result as usize };
        if index >= total {
            unsafe {
                free_slot(req, SLOT_AUX);
                (*req).ptr = null_mut();
            }
            return abi::uv_errno_t_UV_EOF;
        }

        let entry = unsafe { *scandir_entries(req).add(index) };
        unsafe {
            *ent = entry;
            (*req).nbufs += 1;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn init_path_request(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    fs_type: abi::uv_fs_type,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() || path.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, fs_type, cb);
        }

        if cb.is_none() {
            unsafe {
                (*req).path = path;
            }
            0
        } else {
            unsafe { clone_path_into(req, path, SLOT_PATH) }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn init_two_path_request(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    fs_type: abi::uv_fs_type,
    path: *const c_char,
    new_path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() || path.is_null() || new_path.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, fs_type, cb);
        }

        if cb.is_none() {
            unsafe {
                (*req).path = path;
                (*req).new_path = new_path;
            }
            0
        } else {
            unsafe { clone_paths(req, path, new_path) }
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn open(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    flags: c_int,
    mode: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_OPEN, path, cb) };
        if rc != 0 {
            return rc;
        }

        unsafe {
            (*req).flags = flags;
            (*req).mode = mode as abi::mode_t;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn close(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_CLOSE, cb);
            (*req).file = file;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn read(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_READ, cb);
            (*req).file = file;
            (*req).off = offset as abi::off_t;
        }

        let rc = unsafe { copy_bufs(req, bufs, nbufs, cb.is_some()) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn write(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    bufs: *const abi::uv_buf_t,
    nbufs: c_uint,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_WRITE, cb);
            (*req).file = file;
            (*req).off = offset as abi::off_t;
        }

        let rc = unsafe { copy_bufs(req, bufs, nbufs, cb.is_some()) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn sendfile(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    out_fd: abi::uv_file,
    in_fd: abi::uv_file,
    offset: i64,
    length: usize,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_SENDFILE, cb);
            (*req).file = out_fd;
            (*req).flags = in_fd;
            (*req).off = offset as abi::off_t;
            (*req).bufsml[0].len = length;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn unlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_UNLINK, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mkdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    mode: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_MKDIR, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).mode = mode as abi::mode_t;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mkdtemp(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    tpl: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() || tpl.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_MKDTEMP, cb);
        }

        let cloned = clone_c_string(tpl);
        if cloned.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }
        unsafe {
            (*req).path = cloned;
            (*req).reserved[SLOT_PATH] = cloned.cast();
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn mkstemp(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    tpl: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() || tpl.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_MKSTEMP, cb);
        }

        let cloned = clone_c_string(tpl);
        if cloned.is_null() {
            return abi::uv_errno_t_UV_ENOMEM;
        }
        unsafe {
            (*req).path = cloned;
            (*req).reserved[SLOT_PATH] = cloned.cast();
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rmdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_RMDIR, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn scandir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    _flags: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_SCANDIR, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn link(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    new_path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe {
            init_two_path_request(loop_, req, abi::uv_fs_type_UV_FS_LINK, path, new_path, cb)
        };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn symlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    new_path: *const c_char,
    _flags: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe {
            init_two_path_request(
                loop_,
                req,
                abi::uv_fs_type_UV_FS_SYMLINK,
                path,
                new_path,
                cb,
            )
        };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn readlink(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_READLINK, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn realpath(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_REALPATH, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn chown(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    uid: abi::uv_uid_t,
    gid: abi::uv_gid_t,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_CHOWN, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).uid = uid;
            (*req).gid = gid;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fchown(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    uid: abi::uv_uid_t,
    gid: abi::uv_gid_t,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FCHOWN, cb);
            (*req).file = file;
            (*req).uid = uid;
            (*req).gid = gid;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn lchown(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    uid: abi::uv_uid_t,
    gid: abi::uv_gid_t,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_LCHOWN, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).uid = uid;
            (*req).gid = gid;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn stat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_STAT, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn lstat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_LSTAT, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fstat(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FSTAT, cb);
            (*req).file = file;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn rename(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    new_path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe {
            init_two_path_request(loop_, req, abi::uv_fs_type_UV_FS_RENAME, path, new_path, cb)
        };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fsync(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FSYNC, cb);
            (*req).file = file;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fdatasync(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FDATASYNC, cb);
            (*req).file = file;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn ftruncate(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    offset: i64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FTRUNCATE, cb);
            (*req).file = file;
            (*req).off = offset as abi::off_t;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn access(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    mode: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_ACCESS, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).flags = mode;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn chmod(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    mode: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_CHMOD, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).mode = mode as abi::mode_t;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn fchmod(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    mode: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FCHMOD, cb);
            (*req).file = file;
            (*req).mode = mode as abi::mode_t;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn utime(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    atime: f64,
    mtime: f64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_UTIME, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).atime = atime;
            (*req).mtime = mtime;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn futime(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    file: abi::uv_file,
    atime: f64,
    mtime: f64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_FUTIME, cb);
            (*req).file = file;
            (*req).atime = atime;
            (*req).mtime = mtime;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn lutime(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    atime: f64,
    mtime: f64,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_LUTIME, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe {
            (*req).atime = atime;
            (*req).mtime = mtime;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn statfs(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_STATFS, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn copyfile(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    new_path: *const c_char,
    flags: c_int,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_COPYFILE, cb);
        }

        let allowed_flags = abi::UV_FS_COPYFILE_EXCL as c_int
            | abi::UV_FS_COPYFILE_FICLONE as c_int
            | abi::UV_FS_COPYFILE_FICLONE_FORCE as c_int;
        if path.is_null() || new_path.is_null() || (flags & !allowed_flags) != 0 {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let rc = if cb.is_none() {
            unsafe {
                (*req).path = path;
                (*req).new_path = new_path;
            }
            0
        } else {
            unsafe { clone_paths(req, path, new_path) }
        };
        if rc != 0 {
            return rc;
        }

        unsafe {
            (*req).flags = flags;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn opendir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    path: *const c_char,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        let rc = unsafe { init_path_request(loop_, req, abi::uv_fs_type_UV_FS_OPENDIR, path, cb) };
        if rc != 0 {
            return rc;
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn readdir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    dir: *mut abi::uv_dir_t,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() || dir.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        if unsafe { (*dir).dir.is_null() || (*dir).dirents.is_null() } {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_READDIR, cb);
            (*req).ptr = dir.cast();
        }
        unsafe { run_or_submit(loop_, req) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn closedir(
    loop_: *mut abi::uv_loop_t,
    req: *mut abi::uv_fs_t,
    dir: *mut abi::uv_dir_t,
    cb: abi::uv_fs_cb,
) -> c_int {
    unsafe {
        if req.is_null() || dir.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        unsafe {
            fs_req_init(loop_, req, abi::uv_fs_type_UV_FS_CLOSEDIR, cb);
            (*req).ptr = dir.cast();
        }
        unsafe { run_or_submit(loop_, req) }
    }
}
