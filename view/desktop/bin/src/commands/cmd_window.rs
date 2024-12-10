use anyhow::anyhow;
use desktop_models::appearance::theming::ThemeDescriptor;
use tauri::{AppHandle, Emitter, EventTarget, Manager, State, WebviewWindow, Window};
use workbench_desktop::context::Context;

use crate::{create_child_window, AppState};

#[derive(Clone, Serialize)]
struct EventAData {
    data: String,
}

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command]
pub async fn create_new_window(app_handle: AppHandle) {
    create_child_window(&app_handle, "/");
}

#[tauri::command]
pub fn main_window_is_ready(current_window: WebviewWindow) {
    current_window.show().unwrap();

    current_window
        .emit(
            "channel1:eventA",
            EventAData {
                data: "Hello from Rust!".to_string(),
            },
        )
        .unwrap();
}

#[tauri::command]
pub fn handle_signal(ctx: State<'_, Context>) {
    let handler = ctx.signals.get("handle_change_theme").unwrap();

    handler(ctx.inner()).unwrap();
}

#[tauri::command(async)]
pub async fn get_color_theme(path: String) -> Result<String, String> {
    let path = crate::utl::get_themes_dir()
        .map_err(|err| err.to_string())?
        .join(path);

    if path.exists() && path.is_file() {
        Ok(smol::fs::read_to_string(path)
            .await
            .map_err(|err| err.to_string())?)
    } else {
        Err("File not found or inaccessible".to_string())
    }
}

#[tauri::command]
pub fn set_color_theme(
    window: Window,
    app_handle: AppHandle,
    state: State<'_, AppState>,
    theme_descriptor: ThemeDescriptor,
) {
    state
        .appearance
        .set_theme_descriptor(theme_descriptor.clone());

    for (label, _) in app_handle.webview_windows() {
        if window.label() == &label {
            continue;
        }

        app_handle
            .emit_to(
                EventTarget::webview_window(label),
                "core://color-theme-changed",
                theme_descriptor.clone(),
            )
            .unwrap();
    }
}
