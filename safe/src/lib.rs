#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
compile_error!("safe/libuv currently targets linux/x86_64 only");

pub mod abi;
pub mod core;
mod exports;
pub mod threading;
mod util {
    pub(crate) mod idna;
    pub(crate) mod inet;
}
pub mod unix;
#[path = "unix/async.rs"]
pub(crate) mod unix_async;
mod upstream_support;
pub mod version;
