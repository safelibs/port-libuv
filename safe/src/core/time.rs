use crate::abi::linux_x86_64 as abi;
use crate::threading::thread;
use libc::{self, c_int, c_uint};
use std::sync::atomic::{AtomicI32, Ordering};

static FAST_CLOCK_ID: AtomicI32 = AtomicI32::new(-1);

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn monotonic_now_ms() -> u64 {
    let mut ts: libc::timespec = unsafe { std::mem::zeroed() };
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) } != 0 {
        return 0;
    }
    (ts.tv_sec as u64)
        .saturating_mul(1_000)
        .saturating_add((ts.tv_nsec as u64) / 1_000_000)
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
fn hrtime_raw(clock_id: libc::clockid_t) -> u64 {
    let mut ts: libc::timespec = unsafe { std::mem::zeroed() };
    if unsafe { libc::clock_gettime(clock_id, &mut ts) } != 0 {
        return 0;
    }
    (ts.tv_sec as u64)
        .saturating_mul(1_000_000_000)
        .saturating_add(ts.tv_nsec as u64)
}

pub(crate) fn hrtime_precise_ns() -> u64 {
    hrtime_raw(libc::CLOCK_MONOTONIC)
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
fn hrtime_fast_ns() -> u64 {
    let mut clock_id = FAST_CLOCK_ID.load(Ordering::Relaxed);
    if clock_id == -1 {
        clock_id = libc::CLOCK_MONOTONIC;
        let mut res: libc::timespec = unsafe { std::mem::zeroed() };
        if unsafe { libc::clock_getres(libc::CLOCK_MONOTONIC_COARSE, &mut res) } == 0
            && res.tv_nsec <= 1_000_000
        {
            clock_id = libc::CLOCK_MONOTONIC_COARSE;
        }
        FAST_CLOCK_ID.store(clock_id, Ordering::Relaxed);
    }
    hrtime_raw(clock_id)
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn clock_gettime_export(
    clock_id: abi::uv_clock_id,
    ts: *mut abi::uv_timespec64_t,
) -> c_int {
    unsafe {
        if ts.is_null() {
            return abi::uv_errno_t_UV_EFAULT;
        }

        let requested = match clock_id {
            abi::uv_clock_id_UV_CLOCK_MONOTONIC => libc::CLOCK_MONOTONIC,
            abi::uv_clock_id_UV_CLOCK_REALTIME => libc::CLOCK_REALTIME,
            _ => return abi::uv_errno_t_UV_EINVAL,
        };

        let mut value: libc::timespec = unsafe { std::mem::zeroed() };
        if unsafe { libc::clock_gettime(requested, &mut value) } != 0 {
            return -unsafe { *libc::__errno_location() };
        }

        unsafe {
            (*ts).tv_sec = value.tv_sec as i64;
            (*ts).tv_nsec = value.tv_nsec as i32;
        }

        0
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn hrtime_export() -> u64 {
    unsafe {
        let _ = hrtime_fast_ns();
        hrtime_precise_ns()
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn sleep_export(msec: c_uint) {
    unsafe {
        let mut timeout = libc::timespec {
            tv_sec: (msec / 1000) as libc::time_t,
            tv_nsec: ((msec % 1000) * 1_000_000) as libc::c_long,
        };

        loop {
            if unsafe { libc::nanosleep(&timeout, &mut timeout) } == 0 {
                return;
            }
            let err = unsafe { *libc::__errno_location() };
            if err != libc::EINTR {
                unsafe {
                    libc::abort();
                }
            }
        }
    }
}

pub(crate) fn available_parallelism_export() -> c_uint {
    thread::available_parallelism()
}
