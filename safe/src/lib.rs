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

mod abi;
mod accessors;
mod allocator;
mod r#async;
mod dl;
mod dns;
mod error;
mod ffi_exports;
mod fs_event;
mod fs_poll;
mod handle;
mod idna;
mod inet;
mod legacy;
mod r#loop;
mod os;
mod phase5_private;
mod pipe;
mod poll;
mod process_title;
mod request;
mod signal;
mod state;
mod stream;
mod strings;
mod tcp;
mod thread;
mod timer;
mod tty;
mod udp;
mod version;
mod watchers;

mod linux {
    pub(crate) mod epoll;
    pub(crate) mod inotify;
    pub(crate) mod socket;
}

pub const BASELINE_SCAFFOLD_PHASE: &str = "impl_01_baseline_capture";
