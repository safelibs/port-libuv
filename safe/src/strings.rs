use crate::allocator::UV_E2BIG;
use libc::c_char;
use std::arch::global_asm;
use std::ptr;

global_asm!(
    r#"
    .globl uv__strscpy
    .hidden uv__strscpy
    .set uv__strscpy, {target}
"#,
    target = sym safe_uv__strscpy_impl
);

global_asm!(
    r#"
    .globl uv__strtok
    .hidden uv__strtok
    .set uv__strtok, {target}
"#,
    target = sym safe_uv__strtok_impl
);

pub(crate) use safe_uv__strscpy_impl as uv__strscpy;
pub(crate) unsafe extern "C" fn safe_uv__strscpy_impl(
    d: *mut c_char,
    s: *const c_char,
    n: usize,
) -> isize {
    let mut i = 0usize;

    while i < n {
        let ch = *s.add(i);
        *d.add(i) = ch;
        if ch == 0 {
            return if i > isize::MAX as usize {
                UV_E2BIG as isize
            } else {
                i as isize
            };
        }
        i += 1;
    }

    if i == 0 {
        return 0;
    }

    *d.add(i - 1) = 0;
    UV_E2BIG as isize
}

pub(crate) unsafe extern "C" fn safe_uv__strtok_impl(
    str_: *mut c_char,
    sep: *const c_char,
    itr: *mut *mut c_char,
) -> *mut c_char {
    let start = if str_.is_null() { *itr } else { str_ };
    let mut tmp = start;

    if tmp.is_null() {
        return ptr::null_mut();
    }

    while *tmp != 0 {
        let mut sep_itr = sep;
        while *sep_itr != 0 {
            if *tmp == *sep_itr {
                *itr = tmp.add(1);
                *tmp = 0;
                return start;
            }
            sep_itr = sep_itr.add(1);
        }
        tmp = tmp.add(1);
    }

    *itr = ptr::null_mut();
    start
}
