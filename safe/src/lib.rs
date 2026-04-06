#![allow(
    dead_code,
    improper_ctypes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const BASELINE_SCAFFOLD_PHASE: &str = "impl_01_baseline_capture";
