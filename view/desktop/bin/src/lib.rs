mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod utl;
mod window;

use anyhow::Result;
use moss_extension_point::loader::Loader;
use moss_extension_point::registry;
use rand::random;
use tauri::{AppHandle, Manager, RunEvent, WebviewWindow, WindowEvent};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_os;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Layer;
use window::{create_window, CreateWindowInput};

use moss_addon::{BUILTIN_ADDONS_DIR, INSTALLED_ADDONS_DIR};
use moss_desktop::app::manager::AppManager;
use moss_desktop::app::state::AppStateManager;
use moss_desktop::services::addon_service::AddonService;
use moss_desktop::services::theme_service::ThemeService;
use moss_desktop::services::window_service::WindowService;
use moss_desktop::{
    app::instantiation::InstantiationType, services::lifecycle_service::LifecycleService,
};

use crate::commands::*;
use crate::plugins::*;
pub use constants::*;
use moss_desktop::services::locale_service::LocaleService;

#[macro_use]
extern crate tracing;

pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(plugin_log::init())
        .plugin(plugin_window_state::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(mac_window::init());
    }

    builder
        .setup(|app| {
            let log_format = tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true)
                .with_target(false)
                .compact();

            let log_level_filter = std::env::var("LOG_LEVEL")
                .unwrap_or("trace".to_string())
                .to_lowercase()
                .parse()
                .unwrap_or(LevelFilter::TRACE);

            let subscriber = tracing_subscriber::registry().with(
                tracing_subscriber::fmt::layer()
                    .event_format(log_format)
                    .with_ansi(true)
                    .with_span_events(FmtSpan::CLOSE)
                    .with_filter(log_level_filter),
            );

            tracing::subscriber::set_global_default(subscriber)
                .expect("failed to set tracing subscriber");

            let app_handle = app.app_handle();

            let app_state = AppStateManager::new();
            app_handle.manage(app_state);

            let mut loader = Loader::new();
            loader
                .load(
                    std::env::var("CARGO_WORKSPACE_ROOT_DIR").unwrap().into(),
                    registry::take(),
                )
                .unwrap();

            let app_manager = AppManager::new(app_handle.clone())
                .with_service(|_| LifecycleService::new(), InstantiationType::Instant)
                .with_service(
                    |app_handle| {
                        AddonService::new(
                            app_handle,
                            BUILTIN_ADDONS_DIR.to_path_buf(),
                            INSTALLED_ADDONS_DIR.to_path_buf(),
                        )
                    },
                    InstantiationType::Instant,
                )
                .with_service(|_| WindowService::new(), InstantiationType::Delayed)
                .with_service(
                    |app_handle| ThemeService::new(app_handle),
                    InstantiationType::Delayed,
                )
                .with_service(
                    |app_handle| LocaleService::new(app_handle),
                    InstantiationType::Delayed,
                );
            app_handle.manage(app_manager);

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
            cmd_window::create_new_window,
            cmd_window::execute_command,
            cmd_base::get_menu_items_by_namespace,
            cmd_window::execute_command,
            cmd_window::get_color_theme,
            cmd_window::get_state,
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
                let webview_window = create_main_window(&app_handle, "/");
                webview_window
                    .on_menu_event(move |window, event| menu::handle_event(window, &event));
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
    let label = format!("{MAIN_WINDOW_PREFIX}{}", 0);
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

    create_window(app_handle, config)
}

fn create_child_window(app_handle: &AppHandle, url: &str) -> Result<WebviewWindow> {
    let app_manager = app_handle.state::<AppManager>();
    let next_window_id = app_manager.service::<WindowService>()?.next_window_id() + 1;
    let config = CreateWindowInput {
        url,
        label: &format!("{MAIN_WINDOW_PREFIX}{}", next_window_id),
        title: "Moss Studio",
        inner_size: (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        position: (
            100.0 + random::<f64>() * 20.0,
            100.0 + random::<f64>() * 20.0,
        ),
    };

    Ok(create_window(app_handle, config))
}
