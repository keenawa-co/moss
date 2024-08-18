use std::{borrow::Cow, path::PathBuf};
use sysinfo::System;

#[derive(Debug)]
pub struct NativePlatformInfo<'a> {
    pub os: Cow<'a, str>,
    pub version: Cow<'a, str>,
    pub hostname: Cow<'a, str>,
}

impl<'a> NativePlatformInfo<'a> {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os: System::name()
                .map(Cow::Owned)
                .unwrap_or_else(|| Cow::Borrowed("unknown")),
            version: System::os_version()
                .map(Cow::Owned)
                .unwrap_or_else(|| Cow::Borrowed("unknown")),
            hostname: System::host_name()
                .map(Cow::Owned)
                .unwrap_or_else(|| Cow::Borrowed("unknown")),
        }
    }
}

#[derive(Debug)]
pub struct NativeWindowConfiguration<'a> {
    pub home_dir: PathBuf,
    pub full_screen: bool,
    pub platform_info: NativePlatformInfo<'a>,
}
