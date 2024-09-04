// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{context_compact::AppContextCompact, AppCompact};
use app_lib::DesktopMain;
use tracing::error;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() {
    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        let home_dir = std::env::var("HOME")
            .expect("Failed to retrieve the $HOME environment variable")
            .into();

        let configuration = NativeWindowConfiguration {
            home_dir,
            full_screen: false,
            platform_info: NativePlatformInfo::new(),
        };

        if let Err(err) = DesktopMain::new(configuration).open(ctx) {
            error!("{err:#?}")
        }
    });
}

