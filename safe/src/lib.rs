#![allow(
    dead_code,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

mod bindings {
    #![allow(
        dead_code,
        improper_ctypes,
        non_camel_case_types,
        non_snake_case,
        non_upper_case_globals
    )]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use bindings::*;

mod accessors;
mod allocator;
mod dl;
mod error;
mod ffi_exports;
mod inet;
mod legacy;
mod os;
mod process_title;
mod strings;
mod thread;
mod version;

pub const BASELINE_SCAFFOLD_PHASE: &str = "impl_01_baseline_capture";
