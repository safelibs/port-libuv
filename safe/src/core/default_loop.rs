use crate::abi::linux_x86_64 as abi;
use crate::core::loop_;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::Mutex;

struct DefaultLoopSlot {
    initialized: Mutex<bool>,
    storage: UnsafeCell<MaybeUninit<abi::uv_loop_t>>,
}

// SAFETY(abi_layout): the slot is process-global loop storage and access is synchronized by the mutex.
unsafe impl Sync for DefaultLoopSlot {}

static DEFAULT_LOOP: DefaultLoopSlot = DefaultLoopSlot {
    initialized: Mutex::new(false),
    storage: UnsafeCell::new(MaybeUninit::zeroed()),
};

#[inline]
// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
fn storage_ptr() -> *mut abi::uv_loop_t {
    unsafe { (*DEFAULT_LOOP.storage.get()).as_mut_ptr() }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn default_loop() -> *mut abi::uv_loop_t {
    unsafe {
        let mut initialized = DEFAULT_LOOP.initialized.lock().unwrap();
        let ptr = storage_ptr();
        if *initialized {
            return ptr;
        }

        unsafe {
            std::ptr::write_bytes(ptr, 0, 1);
        }
        if unsafe { loop_::loop_init(ptr) } != 0 {
            return std::ptr::null_mut();
        }

        *initialized = true;
        ptr
    }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn is_default_loop(loop_: *mut abi::uv_loop_t) -> bool {
    unsafe { std::ptr::eq(loop_, storage_ptr()) }
}

// SAFETY(abi_layout): accesses libuv's C ABI layout through raw pointers and stable field offsets.
pub(crate) fn mark_closed() {
    unsafe {
        let mut initialized = DEFAULT_LOOP.initialized.lock().unwrap();
        *initialized = false;
    }
}
