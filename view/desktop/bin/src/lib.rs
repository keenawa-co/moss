mod commands;
mod config;
mod mem;
mod menu;
mod utl;

mod constants;
mod plugins;

use log::info;
use platform_core::context_v2::ContextCell;
use platform_core::platform::cross::client::CrossPlatformClient;
use platform_workspace::WorkspaceId;
use rand::random;
use std::env;
use std::rc::Rc;
use std::sync::Arc;
use tauri::{AppHandle, Manager, RunEvent, TitleBarStyle, WebviewUrl, WebviewWindow, WindowEvent};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};
use workbench_desktop::window::{NativePlatformInfo, NativeWindowConfiguration};
use workbench_desktop::Workbench;

use crate::commands::*;
use crate::constants::*;
use crate::plugins as moss_plugins;

#[macro_use]
extern crate serde;

pub struct AppState {
    pub workbench: Arc<Workbench>,
    pub platform_info: NativePlatformInfo,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
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

            let service_group = utl::create_service_registry(NativeWindowConfiguration {
                home_dir: std::env::var("HOME")
                    .expect("Failed to retrieve the $HOME environment variable")
                    .into(),
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
            };

            {
                app.handle().manage(ctx);
                app.handle().manage(app_state);
            }

            Ok(())
        })
        .menu(menu::setup_window_menu)
        .invoke_handler(tauri::generate_handler![
            cmd_initial::main_window_is_ready,
            cmd_dummy::workbench_get_state,
            cmd_dummy::app_ready,
            cmd_dummy::update_font_size,
            cmd_dummy::fetch_all_themes,
            cmd_dummy::read_theme,
            cmd_base::native_platform_info,
            cmd_base::describe_primary_activitybar_part,
            cmd_base::describe_primary_sidebar_part,
            cmd_base::get_view_content,
            cmd_base::get_menu_items,
        ])
        .build(tauri::generate_context!())
        .expect("failed to run")
        .run(|app_handle, event| {
            match event {
                RunEvent::Ready => {
                    let _ = create_main_window(app_handle, "/");
                }

                #[cfg(target_os = "macos")]
                RunEvent::ExitRequested { api, .. } => {
                    app_handle.hide().ok();
                    api.prevent_exit();
                }

                RunEvent::WindowEvent {
                    event: WindowEvent::Focused(true),
                    ..
                } => {

                    // TODO: call updates, git fetch, etc.
                }
                _ => {}
            }
        });
}

fn create_main_window(handle: &AppHandle, url: &str) -> WebviewWindow {
    let label = format!("{MAIN_WINDOW_PREFIX}{}", handle.webview_windows().len());
    let config = CreateWindowConfig {
        url,
        label: label.as_str(),
        title: "Moss Studio",
        inner_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        position: (
            100.0 + random::<f64>() * 20.0,
            100.0 + random::<f64>() * 20.0,
        ),
    };
    create_window(handle, config)
}

struct CreateWindowConfig<'s> {
    url: &'s str,
    label: &'s str,
    title: &'s str,
    inner_size: (f64, f64),
    position: (f64, f64),
}

fn create_window(handle: &AppHandle, config: CreateWindowConfig) -> WebviewWindow {
    info!("Create new window label={}", config.label);

    // TODO: set menu here

    let mut win_builder =
        tauri::WebviewWindowBuilder::new(handle, config.label, WebviewUrl::App(config.url.into()))
            .title(config.title)
            .center()
            .resizable(true)
            .visible(false)
            .fullscreen(false)
            .disable_drag_drop_handler()
            .inner_size(config.inner_size.0, config.inner_size.1)
            .position(config.position.0, config.position.1)
            .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);

    #[cfg(target_os = "macos")]
    {
        win_builder = win_builder
            .hidden_title(true)
            .title_bar_style(TitleBarStyle::Overlay);
    }

    if let Some(w) = handle.webview_windows().get(config.label) {
        info!(
            "Webview with label {} already exists. Focusing existing",
            config.label
        );
        w.set_focus().unwrap();
        return w.to_owned();
    }

    let win = win_builder.build().unwrap();

    win
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
