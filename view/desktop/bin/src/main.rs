// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::AppMain;
use std::process::ExitCode;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() -> ExitCode {
    let home_dir = std::env::var("HOME")
        .expect("Failed to retrieve the $HOME environment variable")
        .into();

    AppMain::new(NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: NativePlatformInfo::new(),
    })
    .run(AppMain::open_main_window)
}