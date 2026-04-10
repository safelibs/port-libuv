use crate::abi::linux_x86_64 as abi;
use libc::{self, c_char, c_int, c_long, c_void};

const CPU_SET_WORDS: usize = 16;
const CPU_SETSIZE: c_int = (CPU_SET_WORDS * 64) as c_int;

#[repr(C)]
#[derive(Clone, Copy)]
struct CpuSet {
    bits: [u64; CPU_SET_WORDS],
}

struct ThreadStart {
    entry: abi::uv_thread_cb,
    arg: *mut c_void,
}

#[inline]
fn uv_err(err: c_int) -> c_int {
    if err == 0 {
        0
    } else {
        -err
    }
}

#[inline]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn last_errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

#[cold]
// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn abort_now() -> ! {
    unsafe {
        unsafe {
            libc::abort();
        }
    }
}

#[inline]
fn min_stack_size() -> usize {
    const MIN_STACK: usize = 8192;
    MIN_STACK.max(libc::PTHREAD_STACK_MIN as usize)
}

fn default_stack_size() -> usize {
    2 << 20
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn thread_stack_size() -> usize {
    let mut lim: libc::rlimit = unsafe { std::mem::zeroed() };
    if unsafe { libc::getrlimit(libc::RLIMIT_STACK, &mut lim) } != 0 {
        return default_stack_size();
    }

    if lim.rlim_cur == libc::RLIM_INFINITY {
        return default_stack_size();
    }

    let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
    if pagesize <= 0 {
        return default_stack_size();
    }

    let pagesize = pagesize as usize;
    let aligned = (lim.rlim_cur as usize) - ((lim.rlim_cur as usize) % pagesize);
    if aligned >= min_stack_size() {
        aligned
    } else {
        default_stack_size()
    }
}

fn cpu_zero(set: &mut CpuSet) {
    set.bits = [0; CPU_SET_WORDS];
}

fn cpu_set(cpu: usize, set: &mut CpuSet) {
    set.bits[cpu / 64] |= 1u64 << (cpu % 64);
}

fn cpu_isset(cpu: usize, set: &CpuSet) -> bool {
    (set.bits[cpu / 64] & (1u64 << (cpu % 64))) != 0
}

fn cpu_count(set: &CpuSet) -> usize {
    set.bits.iter().map(|bits| bits.count_ones() as usize).sum()
}

// SAFETY(syscall_ffi): these affinity entry points are raw libc interfaces for thread scheduling.
unsafe extern "C" {
    // SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
    fn pthread_setaffinity_np(
        thread: libc::pthread_t,
        cpusetsize: usize,
        cpuset: *const CpuSet,
    ) -> c_int;
    // SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
    fn pthread_getaffinity_np(
        thread: libc::pthread_t,
        cpusetsize: usize,
        cpuset: *mut CpuSet,
    ) -> c_int;
    // SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
    fn sched_getaffinity(pid: libc::pid_t, cpusetsize: usize, cpuset: *mut CpuSet) -> c_int;
    // SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
    fn sched_getcpu() -> c_int;
}

// SAFETY(ffi_callback): bridges the libuv C ABI through raw pointers and callback types.
extern "C" fn thread_start(arg: *mut c_void) -> *mut c_void {
    let start = unsafe { Box::from_raw(arg.cast::<ThreadStart>()) };
    if let Some(entry) = start.entry {
        unsafe {
            entry(start.arg);
        }
    }
    std::ptr::null_mut()
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn current_tid() -> libc::id_t {
    unsafe { libc::syscall(libc::SYS_gettid as c_long) as libc::id_t }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn create(
    tid: *mut abi::uv_thread_t,
    entry: abi::uv_thread_cb,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        let params = abi::uv_thread_options_t {
            flags: abi::uv_thread_create_flags_UV_THREAD_NO_FLAGS,
            stack_size: 0,
        };
        unsafe { create_ex(tid, &params, entry, arg) }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn create_ex(
    tid: *mut abi::uv_thread_t,
    params: *const abi::uv_thread_options_t,
    entry: abi::uv_thread_cb,
    arg: *mut c_void,
) -> c_int {
    unsafe {
        if tid.is_null() || entry.is_none() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut stack_size = if !params.is_null()
            && unsafe { (*params).flags & abi::uv_thread_create_flags_UV_THREAD_HAS_STACK_SIZE }
                != 0
        {
            unsafe { (*params).stack_size }
        } else {
            0
        };

        if stack_size == 0 {
            stack_size = thread_stack_size();
        } else {
            let pagesize = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
            if pagesize > 0 {
                let pagesize = pagesize as usize;
                stack_size = (stack_size + pagesize - 1) & !(pagesize - 1);
            }
            stack_size = stack_size.max(min_stack_size());
        }

        let mut attr: libc::pthread_attr_t = unsafe { std::mem::zeroed() };
        let mut attr_ptr: *mut libc::pthread_attr_t = std::ptr::null_mut();
        if stack_size > 0 {
            let err = unsafe { libc::pthread_attr_init(&mut attr) };
            if err != 0 {
                unsafe { abort_now() };
            }
            let err = unsafe { libc::pthread_attr_setstacksize(&mut attr, stack_size) };
            if err != 0 {
                unsafe { abort_now() };
            }
            attr_ptr = &mut attr;
        }

        let boxed = Box::new(ThreadStart { entry, arg });
        let raw = Box::into_raw(boxed).cast::<c_void>();
        let err =
            unsafe { libc::pthread_create(tid.cast(), attr_ptr.cast_const(), thread_start, raw) };

        if !attr_ptr.is_null() {
            let destroy_err = unsafe { libc::pthread_attr_destroy(attr_ptr) };
            if destroy_err != 0 {
                unsafe { abort_now() };
            }
        }

        if err != 0 {
            unsafe {
                drop(Box::from_raw(raw.cast::<ThreadStart>()));
            }
        }

        uv_err(err)
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn join(tid: *mut abi::uv_thread_t) -> c_int {
    unsafe {
        if tid.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }
        uv_err(unsafe { libc::pthread_join(*tid, std::ptr::null_mut()) })
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn self_thread() -> abi::uv_thread_t {
    unsafe { unsafe { libc::pthread_self() } }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn equal(t1: *const abi::uv_thread_t, t2: *const abi::uv_thread_t) -> c_int {
    unsafe {
        if t1.is_null() || t2.is_null() {
            return 0;
        }
        unsafe { libc::pthread_equal(*t1, *t2) }
    }
}

pub(crate) fn cpumask_size() -> c_int {
    CPU_SETSIZE
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn setaffinity(
    tid: *mut abi::uv_thread_t,
    cpumask: *mut c_char,
    oldmask: *mut c_char,
    mask_size: usize,
) -> c_int {
    unsafe {
        if tid.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let cpumasksize = cpumask_size();
        if mask_size < cpumasksize as usize {
            return abi::uv_errno_t_UV_EINVAL;
        }

        if !oldmask.is_null() {
            let rc = unsafe { getaffinity(tid, oldmask, mask_size) };
            if rc < 0 {
                return rc;
            }
        }

        let mut set = CpuSet {
            bits: [0; CPU_SET_WORDS],
        };
        cpu_zero(&mut set);
        for i in 0..cpumasksize as usize {
            if unsafe { *cpumask.add(i) } != 0 {
                cpu_set(i, &mut set);
            }
        }

        uv_err(unsafe { pthread_setaffinity_np(*tid, std::mem::size_of::<CpuSet>(), &set) })
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn getaffinity(
    tid: *mut abi::uv_thread_t,
    cpumask: *mut c_char,
    mask_size: usize,
) -> c_int {
    unsafe {
        if tid.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let cpumasksize = cpumask_size();
        if mask_size < cpumasksize as usize {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut set = CpuSet {
            bits: [0; CPU_SET_WORDS],
        };
        cpu_zero(&mut set);
        let err = unsafe { pthread_getaffinity_np(*tid, std::mem::size_of::<CpuSet>(), &mut set) };
        if err != 0 {
            return uv_err(err);
        }

        for i in 0..cpumasksize as usize {
            unsafe {
                *cpumask.add(i) = cpu_isset(i, &set) as c_char;
            }
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn getcpu() -> c_int {
    unsafe {
        let cpu = unsafe { sched_getcpu() };
        if cpu >= 0 {
            cpu
        } else {
            -last_errno()
        }
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn getpriority(tid: abi::uv_thread_t, priority: *mut c_int) -> c_int {
    unsafe {
        if priority.is_null() {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut policy = 0;
        let mut param: libc::sched_param = unsafe { std::mem::zeroed() };
        let err = unsafe { libc::pthread_getschedparam(tid, &mut policy, &mut param) };
        if err != 0 {
            return uv_err(err);
        }

        if policy == libc::SCHED_OTHER
            && unsafe { libc::pthread_equal(tid, libc::pthread_self()) } != 0
        {
            unsafe {
                *libc::__errno_location() = 0;
            }
            let nice = unsafe { libc::getpriority(libc::PRIO_PROCESS, current_tid()) };
            if nice == -1 && last_errno() != 0 {
                return -last_errno();
            }
            unsafe {
                *priority = nice;
            }
            return 0;
        }

        unsafe {
            *priority = param.sched_priority;
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
fn set_nice_for_calling_thread(priority: c_int) -> c_int {
    unsafe {
        if !(abi::UV_THREAD_PRIORITY_LOWEST..=abi::UV_THREAD_PRIORITY_HIGHEST).contains(&priority) {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let nice = 0 - priority * 2;
        if unsafe { libc::setpriority(libc::PRIO_PROCESS, current_tid(), nice) } != 0 {
            return -last_errno();
        }
        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn setpriority(tid: abi::uv_thread_t, priority: c_int) -> c_int {
    unsafe {
        if !(abi::UV_THREAD_PRIORITY_LOWEST..=abi::UV_THREAD_PRIORITY_HIGHEST).contains(&priority) {
            return abi::uv_errno_t_UV_EINVAL;
        }

        let mut policy = 0;
        let mut param: libc::sched_param = unsafe { std::mem::zeroed() };
        let err = unsafe { libc::pthread_getschedparam(tid, &mut policy, &mut param) };
        if err != 0 {
            return uv_err(err);
        }

        if policy == libc::SCHED_OTHER
            && unsafe { libc::pthread_equal(tid, libc::pthread_self()) } != 0
        {
            return unsafe { set_nice_for_calling_thread(priority) };
        }

        let min = unsafe { libc::sched_get_priority_min(policy) };
        let max = unsafe { libc::sched_get_priority_max(policy) };
        if min == -1 || max == -1 {
            return -last_errno();
        }

        let range = max - min;
        let mapped = match priority {
            abi::UV_THREAD_PRIORITY_HIGHEST => max,
            abi::UV_THREAD_PRIORITY_ABOVE_NORMAL => min + range * 3 / 4,
            abi::UV_THREAD_PRIORITY_NORMAL => min + range / 2,
            abi::UV_THREAD_PRIORITY_BELOW_NORMAL => min + range / 4,
            abi::UV_THREAD_PRIORITY_LOWEST => min,
            _ => return 0,
        };

        if param.sched_priority != mapped {
            param.sched_priority = mapped;
            let err = unsafe { libc::pthread_setschedparam(tid, policy, &param) };
            if err != 0 {
                return uv_err(err);
            }
        }

        0
    }
}

// SAFETY(syscall_ffi): crosses raw libc, kernel, or translated upstream FFI boundaries that Rust cannot model safely.
pub(crate) fn available_parallelism() -> libc::c_uint {
    let mut set = CpuSet {
        bits: [0; CPU_SET_WORDS],
    };
    cpu_zero(&mut set);

    let mut rc = if unsafe { sched_getaffinity(0, std::mem::size_of::<CpuSet>(), &mut set) } == 0 {
        cpu_count(&set) as c_long
    } else {
        unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) }
    };

    if rc < 1 {
        rc = 1;
    }

    rc as libc::c_uint
}
