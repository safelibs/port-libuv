use crate::abi::linux_x86_64 as abi;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

fn utf8_decode1(bytes: &[u8], cursor: &mut usize) -> Result<u32, c_int> {
    debug_assert!(*cursor < bytes.len());

    let a = bytes[*cursor] as u32;
    *cursor += 1;
    if a < 128 {
        return Ok(a);
    }

    utf8_decode1_slow(bytes, cursor, a)
}

fn utf8_decode1_slow(bytes: &[u8], cursor: &mut usize, mut a: u32) -> Result<u32, c_int> {
    let (min, mut b, mut c, mut d);

    if a > 0xF7 {
        return Err(abi::uv_errno_t_UV_EINVAL);
    }

    match bytes.len().saturating_sub(*cursor) {
        _ if a > 0xEF => {
            min = 0x10000;
            a &= 7;
            b = bytes
                .get(*cursor)
                .copied()
                .ok_or(abi::uv_errno_t_UV_EINVAL)? as u32;
            *cursor += 1;
            c = bytes
                .get(*cursor)
                .copied()
                .ok_or(abi::uv_errno_t_UV_EINVAL)? as u32;
            *cursor += 1;
            d = bytes
                .get(*cursor)
                .copied()
                .ok_or(abi::uv_errno_t_UV_EINVAL)? as u32;
            *cursor += 1;
        }
        2.. if a > 0xDF => {
            min = 0x800;
            b = 0x80 | (a & 15);
            c = bytes
                .get(*cursor)
                .copied()
                .ok_or(abi::uv_errno_t_UV_EINVAL)? as u32;
            *cursor += 1;
            d = bytes
                .get(*cursor)
                .copied()
                .ok_or(abi::uv_errno_t_UV_EINVAL)? as u32;
            *cursor += 1;
            a = 0;
        }
        1.. if a > 0xBF => {
            min = 0x80;
            b = 0x80;
            c = 0x80 | (a & 31);
            d = bytes
                .get(*cursor)
                .copied()
                .ok_or(abi::uv_errno_t_UV_EINVAL)? as u32;
            *cursor += 1;
            a = 0;
        }
        _ => return Err(abi::uv_errno_t_UV_EINVAL),
    }

    if 0x80 != (0xC0 & (b ^ c ^ d)) {
        return Err(abi::uv_errno_t_UV_EINVAL);
    }

    b &= 63;
    c &= 63;
    d &= 63;
    a = (a << 18) | (b << 12) | (c << 6) | d;

    if a < min || a > 0x10FFFF || (0xD800..=0xDFFF).contains(&a) {
        return Err(abi::uv_errno_t_UV_EINVAL);
    }

    Ok(a)
}

fn idna_toascii_label(label: &[u8], out: &mut Vec<u8>) -> Result<(), c_int> {
    static ALPHABET: &[u8; 36] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let mut h = 0u32;
    let mut todo = 0u32;
    let mut cursor = 0usize;
    while cursor < label.len() {
        let code = utf8_decode1(label, &mut cursor)?;
        if code < 128 {
            h += 1;
        } else {
            todo += 1;
        }
    }

    if todo > 0 {
        out.extend_from_slice(b"xn--");
    }

    let mut written_ascii = 0u32;
    cursor = 0;
    while cursor < label.len() {
        let code = utf8_decode1(label, &mut cursor)?;
        if code > 127 {
            continue;
        }
        out.push(code as u8);
        written_ascii += 1;
        if written_ascii == h {
            break;
        }
    }

    if todo == 0 {
        return Ok(());
    }

    if h > 0 {
        out.push(b'-');
    }

    let mut n = 128u32;
    let mut bias = 72u32;
    let mut delta = 0u32;
    let mut first = true;

    while todo > 0 {
        let mut m = u32::MAX;
        cursor = 0;
        while cursor < label.len() {
            let code = utf8_decode1(label, &mut cursor)?;
            if code >= n && code < m {
                m = code;
            }
        }

        let x = m.checked_sub(n).ok_or(abi::uv_errno_t_UV_E2BIG)?;
        let y = h.checked_add(1).ok_or(abi::uv_errno_t_UV_E2BIG)?;
        if x > (!delta) / y {
            return Err(abi::uv_errno_t_UV_E2BIG);
        }

        delta = delta
            .checked_add(x.checked_mul(y).ok_or(abi::uv_errno_t_UV_E2BIG)?)
            .ok_or(abi::uv_errno_t_UV_E2BIG)?;
        n = m;

        cursor = 0;
        while cursor < label.len() {
            let code = utf8_decode1(label, &mut cursor)?;

            if code < n {
                delta = delta.checked_add(1).ok_or(abi::uv_errno_t_UV_E2BIG)?;
            }
            if code != n {
                continue;
            }

            let mut k = 36u32;
            let mut q = delta;
            loop {
                let mut t = 1u32;
                if k > bias {
                    t = k - bias;
                }
                if t > 26 {
                    t = 26;
                }
                if q < t {
                    break;
                }

                let x = q - t;
                let y = 36 - t;
                q = x / y;
                t += x % y;
                out.push(ALPHABET[t as usize]);
                k = k.checked_add(36).ok_or(abi::uv_errno_t_UV_E2BIG)?;
            }

            out.push(ALPHABET[q as usize]);

            delta /= 2;
            if first {
                delta /= 350;
                first = false;
            }

            h = h.checked_add(1).ok_or(abi::uv_errno_t_UV_E2BIG)?;
            delta = delta
                .checked_add(delta / h)
                .ok_or(abi::uv_errno_t_UV_E2BIG)?;

            bias = 0;
            while delta > (35 * 26) / 2 {
                bias = bias.checked_add(36).ok_or(abi::uv_errno_t_UV_E2BIG)?;
                delta /= 35;
            }

            bias = bias
                .checked_add((36 * delta) / (delta + 38))
                .ok_or(abi::uv_errno_t_UV_E2BIG)?;
            delta = 0;
            todo -= 1;
        }

        delta = delta.checked_add(1).ok_or(abi::uv_errno_t_UV_E2BIG)?;
        n = n.checked_add(1).ok_or(abi::uv_errno_t_UV_E2BIG)?;
    }

    Ok(())
}

pub(crate) fn toascii_bytes(input: &[u8]) -> Result<CString, c_int> {
    if input.is_empty() {
        return Err(abi::uv_errno_t_UV_EINVAL);
    }

    let mut out = Vec::with_capacity(input.len() + 16);
    let mut label_start = 0usize;
    let mut cursor = 0usize;

    while cursor < input.len() {
        let label_end = cursor;
        let code = utf8_decode1(input, &mut cursor)?;
        if code != b'.' as u32 && code != 0x3002 && code != 0xFF0E && code != 0xFF61 {
            continue;
        }

        idna_toascii_label(&input[label_start..label_end], &mut out)?;
        out.push(b'.');
        label_start = cursor;
    }

    if label_start < input.len() {
        idna_toascii_label(&input[label_start..], &mut out)?;
    }

    out.push(0);
    CString::from_vec_with_nul(out).map_err(|_| abi::uv_errno_t_UV_EINVAL)
}

pub(crate) fn toascii_host(input: &CStr) -> Result<CString, c_int> {
    toascii_bytes(input.to_bytes())
}
