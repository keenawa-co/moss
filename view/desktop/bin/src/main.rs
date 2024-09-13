// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_lib::AppMain;
use std::process::ExitCode;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() -> ExitCode {
    // TODO: remove after the testing is done
    let mut pl_log_service = platform_log::log_service::create_service();
    pl_log_service.trace("Trace msg!");
    pl_log_service.debug("Debug msg!");
    pl_log_service.info("Info msg!");
    pl_log_service.warning("Warning msg!");
    pl_log_service.error("Error msg!");

    pl_log_service.flush_buffer_logger_to_cli();


    let home_dir = std::env::var("HOME")
        .expect("Failed to retrieve the $HOME environment variable")
        .into();

    return AppMain::new(NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: NativePlatformInfo::new(),
    })
    .run(AppMain::open_main_window);
}