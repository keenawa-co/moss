// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{context_compact::AppContextCompact, AppCompact};
use app_lib::DesktopMain;
use platform_core::common::runtime::AsyncRuntime;
use tracing::error;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() {
    // let platform_runtime = AsyncRuntime::new();
    // if let Err(e) = platform_runtime.run(|ctx| {
    //     let home_dir = std::env::var("HOME")
    //         .expect("Failed to retrieve the $HOME environment variable")
    //         .into();

    //     let configuration = NativeWindowConfiguration {
    //         home_dir,
    //         full_screen: false,
    //         platform_info: NativePlatformInfo::new(),
    //     };

    //     DesktopMain::new(configuration).run_internal(ctx)
    // }) {
    //     error!("{e:#?}")
    // }
    // AppCompact::new().run(|ctx: &mut AppContextCompact| {
    //     let home_dir = std::env::var("HOME")
    //         .expect("Failed to retrieve the $HOME environment variable")
    //         .into();

    //     let configuration = NativeWindowConfiguration {
    //         home_dir,
    //         full_screen: false,
    //         platform_info: NativePlatformInfo::new(),
    //     };

    //     if let Err(err) = DesktopMain::new(configuration).open(ctx) {
    //         error!("{err:#?}")
    //     }
    // })

    let home_dir = std::env::var("HOME")
        .expect("Failed to retrieve the $HOME environment variable")
        .into();

    let configuration = NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: NativePlatformInfo::new(),
    };

    DesktopMain::new(configuration)
        .run(DesktopMain::open_window)
        .unwrap();
}
