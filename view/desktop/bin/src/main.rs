// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

<<<<<<< HEAD
use app_lib::AppMain;
use std::process::ExitCode;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() -> ExitCode {
    let home_dir = std::env::var("HOME")
        .expect("Failed to retrieve the $HOME environment variable")
        .into();
=======
use app::{context_compact::AppContextCompact, AppCompact};
use app_lib::DesktopMain;
use tracing::{error, event, Level};
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    
    event!(Level::INFO, "inside foo");

    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        let home_dir = std::env::var("HOME")
            .expect("Failed to retrieve the $HOME environment variable")
            .into();
>>>>>>> e08df7a9 (feat: tracing-subscriber integration into the platform)

    AppMain::new(NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: NativePlatformInfo::new(),
    })
    .run(AppMain::open_main_window)
}
