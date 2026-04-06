use crate::allocator::{self, UV_EINVAL, UV_ENOBUFS};
use std::arch::global_asm;
use std::ptr;
use std::sync::Mutex;

#[derive(Clone, Copy)]
struct ProcessTitleState {
    title_ptr: *mut libc::c_char,
    len: usize,
    cap: usize,
    args_mem: *mut libc::c_void,
}

unsafe impl Send for ProcessTitleState {}

static PROCESS_TITLE_STATE: Mutex<ProcessTitleState> = Mutex::new(ProcessTitleState {
    title_ptr: ptr::null_mut(),
    len: 0,
    cap: 0,
    args_mem: ptr::null_mut(),
});

global_asm!(
    r#"
    .globl uv__process_title_cleanup
    .hidden uv__process_title_cleanup
    .set uv__process_title_cleanup, {target}
"#,
    target = sym safe_uv__process_title_cleanup_impl
);

unsafe fn apply_process_name(title: *const libc::c_char) {
    #[cfg(target_os = "linux")]
    {
        libc::prctl(libc::PR_SET_NAME, title);
    }
}

#[no_mangle]
pub unsafe extern "C" fn uv_setup_args(
    argc: libc::c_int,
    argv: *mut *mut libc::c_char,
) -> *mut *mut libc::c_char {
    if argc <= 0 || argv.is_null() {
        return argv;
    }

    let argc = argc as usize;
    let argv0 = *argv;
    let argv0_len = libc::strlen(argv0);
    let mut strings_size = argv0_len + 1;

    for i in 1..argc {
        strings_size += libc::strlen(*argv.add(i)) + 1;
    }

    let total_size = strings_size + (argc + 1) * std::mem::size_of::<*mut libc::c_char>();
    let new_argv = allocator::malloc(total_size).cast::<*mut libc::c_char>();
    if new_argv.is_null() {
        return argv;
    }

    let mut dst = new_argv.add(argc + 1).cast::<libc::c_char>();
    for i in 0..argc {
        let src = *argv.add(i);
        let len = libc::strlen(src) + 1;
        ptr::copy_nonoverlapping(src, dst, len);
        *new_argv.add(i) = dst;
        dst = dst.add(len);
    }
    *new_argv.add(argc) = ptr::null_mut();

    let last = *argv.add(argc - 1);
    let last_size = libc::strlen(last) + 1;
    let cap = last.add(last_size).offset_from(argv0) as usize;

    *PROCESS_TITLE_STATE
        .lock()
        .expect("process title mutex poisoned") = ProcessTitleState {
        title_ptr: argv0,
        len: argv0_len,
        cap,
        args_mem: new_argv.cast(),
    };

    new_argv
}

#[no_mangle]
pub unsafe extern "C" fn uv_set_process_title(title: *const libc::c_char) -> libc::c_int {
    if title.is_null() {
        return UV_EINVAL;
    }

    let mut state = PROCESS_TITLE_STATE
        .lock()
        .expect("process title mutex poisoned");
    if state.args_mem.is_null() {
        return UV_ENOBUFS;
    }

    let mut len = libc::strlen(title);
    if len >= state.cap {
        len = state.cap.saturating_sub(1);
    }

    ptr::copy_nonoverlapping(title, state.title_ptr, len);
    ptr::write_bytes(state.title_ptr.add(len), 0, state.cap.saturating_sub(len));
    state.len = len;
    apply_process_name(state.title_ptr);

    0
}

#[no_mangle]
pub unsafe extern "C" fn uv_get_process_title(
    buffer: *mut libc::c_char,
    size: usize,
) -> libc::c_int {
    if buffer.is_null() || size == 0 {
        return UV_EINVAL;
    }

    let state = PROCESS_TITLE_STATE
        .lock()
        .expect("process title mutex poisoned");
    if state.args_mem.is_null() {
        return UV_ENOBUFS;
    }
    if size <= state.len {
        return UV_ENOBUFS;
    }

    if state.len != 0 {
        ptr::copy_nonoverlapping(state.title_ptr, buffer, state.len + 1);
    }
    *buffer.add(state.len) = 0;

    0
}

unsafe extern "C" fn safe_uv__process_title_cleanup_impl() {
    let mut state = PROCESS_TITLE_STATE
        .lock()
        .expect("process title mutex poisoned");
    allocator::free(state.args_mem);
    *state = ProcessTitleState {
        title_ptr: ptr::null_mut(),
        len: 0,
        cap: 0,
        args_mem: ptr::null_mut(),
    };
}
