#![allow(improper_ctypes)]

extern crate libc;
extern crate chrono_engine_sys;

mod ffi;
pub mod physics;
#[path = "core/mod.rs"]
mod core_private;

pub mod core {
    pub use core_private::{
        Shared,
        Shareable
    };
}
