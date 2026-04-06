use crate::allocator::{UV_E2BIG, UV_EINVAL};
use std::ffi::{CStr, CString};

fn utf8_decode1(bytes: &[u8], index: &mut usize) -> Result<u32, libc::c_int> {
    let a = bytes[*index] as u32;
    *index += 1;

    if a < 0x80 {
        return Ok(a);
    }

    let remaining = bytes.len() - *index;
    let (mut code_point, min, needed) = if a > 0xF7 {
        return Err(UV_EINVAL);
    } else if a > 0xEF {
        (a & 7, 0x10000, 3usize)
    } else if a > 0xDF {
        (a & 15, 0x800, 2usize)
    } else if a > 0xBF {
        (a & 31, 0x80, 1usize)
    } else {
        return Err(UV_EINVAL);
    };

    if remaining < needed {
        return Err(UV_EINVAL);
    }

    for _ in 0..needed {
        let b = bytes[*index] as u32;
        *index += 1;
        if (b & 0xC0) != 0x80 {
            return Err(UV_EINVAL);
        }
        code_point = (code_point << 6) | (b & 0x3F);
    }

    if code_point < min || code_point > 0x10FFFF || (0xD800..=0xDFFF).contains(&code_point) {
        return Err(UV_EINVAL);
    }

    Ok(code_point)
}

fn idna_toascii_label(label: &[u8], out: &mut Vec<u8>) -> Result<(), libc::c_int> {
    static ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";

    let mut code_points = Vec::new();
    let mut index = 0usize;
    let mut ascii_count = 0u32;
    let mut non_ascii_count = 0u32;

    while index < label.len() {
        let c = utf8_decode1(label, &mut index)?;
        if c < 0x80 {
            ascii_count += 1;
        } else {
            non_ascii_count += 1;
        }
        code_points.push(c);
    }

    if non_ascii_count > 0 {
        out.extend_from_slice(b"xn--");
    }

    for &c in &code_points {
        if c < 0x80 {
            out.push(c as u8);
        }
    }

    if non_ascii_count == 0 {
        return Ok(());
    }

    if ascii_count > 0 {
        out.push(b'-');
    }

    let mut n = 128u32;
    let mut bias = 72u32;
    let mut delta = 0u32;
    let mut h = ascii_count;
    let mut todo = non_ascii_count;
    let mut first = true;

    while todo > 0 {
        let m = code_points
            .iter()
            .copied()
            .filter(|&c| c >= n)
            .min()
            .ok_or(UV_EINVAL)?;
        let x = m - n;
        let y = h + 1;
        delta = delta
            .checked_add(x.checked_mul(y).ok_or(UV_E2BIG)?)
            .ok_or(UV_E2BIG)?;
        n = m;

        for &c in &code_points {
            if c < n {
                delta = delta.checked_add(1).ok_or(UV_E2BIG)?;
            }
            if c != n {
                continue;
            }

            let mut k = 36u32;
            let mut q = delta;
            loop {
                let mut t = if k > bias { k - bias } else { 1 };
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
                k += 36;
            }

            out.push(ALPHABET[q as usize]);

            delta /= 2;
            if first {
                delta /= 350;
                first = false;
            }

            h += 1;
            delta += delta / h;

            bias = 0;
            while delta > 35 * 26 / 2 {
                delta /= 35;
                bias += 36;
            }
            bias += 36 * delta / (delta + 38);
            delta = 0;
            todo -= 1;
        }

        delta += 1;
        n += 1;
    }

    Ok(())
}

pub(crate) unsafe fn to_ascii_hostname(
    hostname: *const libc::c_char,
) -> Result<Option<CString>, libc::c_int> {
    if hostname.is_null() {
        return Ok(None);
    }

    let input = CStr::from_ptr(hostname).to_bytes();
    if input.is_empty() {
        return Err(UV_EINVAL);
    }

    let mut out = Vec::with_capacity(input.len() + 16);
    let mut start = 0usize;
    let mut index = 0usize;

    while index < input.len() {
        let label_start = index;
        let code_point = utf8_decode1(input, &mut index)?;
        if code_point == b'.' as u32
            || code_point == 0x3002
            || code_point == 0xFF0E
            || code_point == 0xFF61
        {
            idna_toascii_label(&input[start..label_start], &mut out)?;
            out.push(b'.');
            start = index;
        }
    }

    if start < input.len() {
        idna_toascii_label(&input[start..], &mut out)?;
    }

    CString::new(out).map(Some).map_err(|_| UV_EINVAL)
}
