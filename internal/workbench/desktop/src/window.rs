use once_cell::sync::OnceCell;
use std::{path::PathBuf, rc::Rc};
use sysinfo::System;
use tauri::AppHandle;

// pub struct Appearance {
//     pub theme_slug: String,
//     pub primary_color: Color,
//     pub bar_color: Color,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WorkbenchMode {
    #[default]
    Empty,
    Workspace,
}

pub enum WindowMode {
    Maximized,
    Normal,
    Fullscreen,
}

pub struct WindowState {
    // pub appearance: Appearance,
    pub mode: WindowMode,
    pub zoom_level: f32,
}

pub struct Window {
    pub raw: OnceCell<Rc<AppHandle>>,
    pub state: WindowState,
    pub mode: WindowMode,
    // pub platform:
}

#[derive(Debug, Clone, Serialize)]
pub struct NativePlatformInfo {
    pub os: String,
    pub version: String,
    pub hostname: String,
}

impl NativePlatformInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        Self {
            os: System::name().unwrap_or_else(|| "unknown".to_string()),
            version: System::os_version().unwrap_or_else(|| "unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct NativeWindowConfiguration {
    pub home_dir: PathBuf,
    pub full_screen: bool,
    pub platform_info: NativePlatformInfo,
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_test() {}
}
