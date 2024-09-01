// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

<<<<<<< HEAD
<<<<<<< HEAD
use app_lib::AppMain;
use std::process::ExitCode;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() -> ExitCode {
    let home_dir = std::env::var("HOME")
        .expect("Failed to retrieve the $HOME environment variable")
        .into();
=======
=======
use std::{borrow::Borrow, io, sync::Arc};

>>>>>>> 894aa703 (feat: logging init & send them to frontend)
use app::{context_compact::AppContextCompact, AppCompact};
use app_lib::DesktopMain;
use tauri::{AppHandle, Emitter};
use tracing::{error, event, Level};
use tracing_subscriber::fmt::MakeWriter;
use workbench_tgui::window::{NativePlatformInfo, NativeWindowConfiguration};

fn main() {
    event!(Level::INFO, "before run");

    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        let home_dir = std::env::var("HOME")
            .expect("Failed to retrieve the $HOME environment variable")
            .into();
>>>>>>> e08df7a9 (feat: tracing-subscriber integration into the platform)

<<<<<<< HEAD
    AppMain::new(NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: NativePlatformInfo::new(),
    })
    .run(AppMain::open_main_window)
=======
        let configuration = NativeWindowConfiguration {
            home_dir,
            full_screen: false,
            platform_info: NativePlatformInfo::new(),
        };

        if let Err(err) = DesktopMain::new(configuration).open(ctx) {
            error!("{err:#?}")
        }
    });

    event!(Level::INFO, "after run");
>>>>>>> 894aa703 (feat: logging init & send them to frontend)
}

