#![cfg(target_os = "macos")]

use swift_rs::{swift, SRString};

pub type NSObject = *mut std::ffi::c_void;

swift!(pub fn set_app_name(name: &SRString));
