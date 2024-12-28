mod commands;
mod constants;
mod mem;
mod menu;
mod plugins;
mod utl;
mod window;

pub use constants::*;
use moss_desktop::app::lifecycle::{LifecycleManager, LifecyclePhase};
use moss_desktop::app::service::ServiceManager;

use crate::plugins::*;
use moss_desktop::app::state::AppState;
use plugins::app_formation;
use rand::random;
use smallvec::smallvec;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::plugin::TauriPlugin;
use tauri::{AppHandle, Listener, Manager, RunEvent, Runtime, WebviewWindow, WindowEvent, Wry};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use tauri_plugin_os;
use window::{create_window, CreateWindowInput};

use crate::commands::*;

#[macro_use]
extern crate serde;

pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(plugin_log::init())
        .plugin(plugin_window_state::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(plugin_app_formation::init());

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(mac_window::init());
    }

    builder
        .setup(|app| {
            // let lm = LifecycleManager::new();
            // let l = lm.observe(LifecyclePhase::Bootstrapping, |app_state| {
            //     println!("Hello!");
            // });
            // lm.set_phase(app.app_handle(), LifecyclePhase::Bootstrapping);

            // let service = LifecycleService::new();

            // let l = service.register_phase_listener(LifecyclePhase::Starting, |app_state| {
            //     println!("Hello!");
            // });

            // service.notify_phase(LifecyclePhase::Starting, app.app_handle().clone());

            // let service_manager = ServiceManager2::new();
            // service_manager.register();
            let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);

            // let r = app.handle().listen("event", |e| {});

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
                let _ = create_main_window(&app_handle, "/");
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
    let lifecycle_manager = app_handle.state::<Arc<LifecycleManager>>();
    lifecycle_manager.set_phase(app_handle, LifecyclePhase::Bootstrapping);

    // let state = AppState::new();
    // let theme_service = ThemeService::new(app_handle.clone(), Arc::clone(&state.cache));

    {
        // app_handle.manage(theme_service);
        // app_handle.manage(state);
    }

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
    let webview_window = create_window(app_handle, config);
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    webview_window
}

fn create_child_window(app_handle: &AppHandle, url: &str) -> WebviewWindow {
    let next_window_id = app_handle.state::<AppState>().inc_next_window_id() + 1;
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
    let webview_window = create_window(app_handle, config);
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    webview_window
}
