mod commands;
mod mem;
mod menu;
mod plugins;
mod utl;
mod window;

mod cli;
pub mod constants;

use commands::*;
use platform_core::context_v2::ContextCell;
use platform_core::platform::cross::client::CrossPlatformClient;
use platform_workspace::WorkspaceId;
use rand::random;
use std::env;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use tauri::{AppHandle, Manager, RunEvent, WebviewWindow, WindowEvent};
use tauri_plugin_cli::CliExt;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};
use window::{create_window, CreateWindowInput};
use workbench_desktop::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_desktop::Workbench;

use crate::constants::*;
use crate::plugins as moss_plugins;

#[macro_use]
extern crate serde;

pub struct AppState {
    pub workbench: Arc<Workbench>,
    pub platform_info: NativePlatformInfo,
    pub window_counter: AtomicUsize,
    pub react_query_string: Mutex<String>,
}

pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
                    Target::new(TargetKind::Webview),
                ])
                .level_for("tao", log::LevelFilter::Info)
                .level_for("plugin_runtime", log::LevelFilter::Info)
                .level_for("tracing", log::LevelFilter::Warn)
                .with_colors(ColoredLevelConfig::default())
                .level(if is_dev() {
                    log::LevelFilter::Trace
                } else {
                    log::LevelFilter::Info
                })
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_denylist(&["ignored"])
                .map_label(|label| {
                    if label.starts_with(OTHER_WINDOW_PREFIX) {
                        "ignored"
                    } else {
                        label
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(moss_plugins::tauri_mac_window::init());
    }

    builder
        .setup(|app| {
            let platform_info = NativePlatformInfo::new();
            let home_dir = crate::utl::get_home_dir()?;

            let service_group = utl::create_service_registry(NativeWindowConfiguration {
                home_dir,
                full_screen: false,
                platform_info: platform_info.clone(),
            })?;
            let platform_client = Rc::new(CrossPlatformClient::new());
            let ctx_cell = ContextCell::new(platform_client.clone());
            let mut ctx = ctx_cell.borrow().to_async();

            let mut workbench = Workbench::new(&mut ctx, service_group, WorkspaceId::Empty)?;
            workbench.initialize(&mut ctx)?;

            let app_state = AppState {
                workbench: Arc::new(workbench),
                platform_info,
                window_counter: AtomicUsize::new(0),
                react_query_string: Mutex::new(String::from("Hello, Tauri!")),
            };

            {
                app.handle().manage(ctx);
                app.handle().manage(app_state);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd_window::main_window_is_ready,
            cmd_window::create_new_window,
            cmd_dummy::get_stored_string,
            cmd_dummy::set_stored_string,
            cmd_dummy::workbench_get_state,
            cmd_dummy::app_ready,
            cmd_dummy::update_font_size,
            cmd_dummy::fetch_all_themes,
            cmd_dummy::read_theme,
            cmd_base::native_platform_info,
            cmd_base::get_view_content,
            cmd_base::get_menu_items_by_namespace,
        ])
        .on_window_event(|window, event| match event {
            #[cfg(target_os = "macos")]
            WindowEvent::CloseRequested { api, .. } => {
                if window.app_handle().webview_windows().len() == 1 {
                    window.app_handle().hide().ok();
                    api.prevent_close();
                }
            }
            WindowEvent::Focused(_) => { /* call updates, git fetch, etc. */ }

            _ => (),
        })
        .build(tauri::generate_context!())
        .expect("failed to run")
        .run(|app_handle, event| match event {
            RunEvent::Ready => {
                // Setting up CLI
                match app_handle.cli().matches() {
                    Ok(matches) => {
                        let subcommand = matches.subcommand;
                        if subcommand.is_none() {
                            let _ = create_main_window(app_handle, "/");

                            // let _ = create_main_window(app_handle, "/");
                        } else {
                            tauri::async_runtime::spawn(crate::cli::cli_handler(
                                subcommand.unwrap(),
                                app_handle.clone(),
                            ));
                        }
                    }
                    Err(_) => {}
                };
            }

            #[cfg(target_os = "macos")]
            RunEvent::ExitRequested { api, .. } => {
                app_handle.hide().ok();
                api.prevent_exit();
            }

            _ => {}
        });
}

fn create_main_window(handle: &AppHandle, url: &str) -> WebviewWindow {
    let window_number = handle
        .state::<AppState>()
        .window_counter
        .fetch_add(1, Ordering::SeqCst);
    let label = format!("{MAIN_WINDOW_PREFIX}{}", window_number);
    let config = CreateWindowInput {
        url,
        label: label.as_str(),
        title: "Moss Studio",
        inner_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        position: (
            100.0 + random::<f64>() * 20.0,
            100.0 + random::<f64>() * 20.0,
        ),
    };
    let webview_window = create_window(handle, config);
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));

    webview_window
}

fn is_dev() -> bool {
    #[cfg(dev)]
    {
        return true;
    }
    #[cfg(not(dev))]
    {
        return false;
    }
}
