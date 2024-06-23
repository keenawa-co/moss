#![cfg(target_os = "macos")]

use swift_rs::{swift, SRString};

pub type NSObject = *mut std::ffi::c_void;

swift!(pub fn set_app_name(name: &SRString));

pub fn set_application_name(name: &str) {
    let sr_name: SRString = name.into();
    unsafe {
        set_app_name(&sr_name);
    }
}
