mod commands;
mod mem;
mod menu;
mod plugins;
mod state;
mod utl;
mod window;

mod cli;
pub mod constants;

use cmd_window::set_color_theme;
use commands::*;
use dashmap::DashMap;
use desktop_models::appearance::theming::ThemeDescriptor;
use parking_lot::RwLock;
use platform_core::context_v2::ContextCell;
use platform_core::platform::cross::client::CrossPlatformClient;
use platform_workspace::WorkspaceId;
use rand::random;
use state::{AppState, Appearance, CommandHandler};
use std::env;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Manager, RunEvent, WebviewWindow, WindowEvent};
use tauri_plugin_cli::CliExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};
use window::{create_window, CreateWindowInput};
use workbench_desktop::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_desktop::Workbench;

use crate::constants::*;
use crate::plugins as moss_plugins;

#[macro_use]
extern crate serde;

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
        // .plugin(tauri_plugin_clipboard_manager::init())
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
            let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    tauri::async_runtime::spawn(cmd_window::create_new_window(
                                        app.clone(),
                                    ));
                                }
                                ShortcutState::Released => {}
                            }
                        }
                    })
                    .build(),
            )?;
            app.global_shortcut().register(ctrl_n_shortcut)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd_window::get_locales,
            cmd_window::get_translations,
            cmd_window::get_themes,
            cmd_window::main_window_is_ready,
            cmd_window::create_new_window,
            cmd_window::execute_command,
            cmd_base::get_view_content,
            cmd_base::get_menu_items_by_namespace,
            cmd_window::execute_command,
            cmd_window::get_color_theme,
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

fn create_main_window(app_handle: &AppHandle, url: &str) -> WebviewWindow {
    // ------------ Get rid of this mess as soon as possible ------------
    let platform_info = NativePlatformInfo::new();
    let home_dir = crate::utl::get_home_dir().expect("failed to get $HOME dir");

    let service_group = utl::create_service_registry(NativeWindowConfiguration {
        home_dir,
        full_screen: false,
        platform_info: platform_info.clone(),
    })
    .unwrap();
    let platform_client = Rc::new(CrossPlatformClient::new());
    let ctx_cell = ContextCell::new(platform_client.clone());
    let mut ctx = ctx_cell.borrow().to_async();

    let mut workbench = Workbench::new(&mut ctx, service_group, WorkspaceId::Empty).unwrap();
    workbench.initialize(&mut ctx).unwrap();

    // ----------------------------------------------------------------------

    let commands = DashMap::new();
    commands.insert(
        "workbench.changeColorTheme".into(),
        Arc::new(set_color_theme) as CommandHandler,
    );

    let window_number = 0;
    let app_state = AppState {
        commands,
        appearance: Appearance {
            theme_descriptor: RwLock::new(ThemeDescriptor {
                id: "theme-light".to_string(),
                name: "Theme Light".to_string(),
                source: "moss-light.css".to_string(),
            }),
        },
        workbench: Arc::new(workbench),
        platform_info,
        next_window_id: AtomicUsize::new(window_number),
    };

    {
        app_handle.manage(app_state);
    }

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
    let webview_window = create_window(app_handle, config);
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    webview_window
}

fn create_child_window(handle: &AppHandle, url: &str) -> WebviewWindow {
    let window_number = handle
        .state::<AppState>()
        .next_window_id
        .fetch_add(1, Ordering::SeqCst);

    let label = format!("{MAIN_WINDOW_PREFIX}{}", window_number + 1);
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
