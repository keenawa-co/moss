use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

use crate::window::create_child_window;
use crate::{DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, OTHER_WINDOW_PREFIX};

#[derive(Clone, Serialize)]
struct EventAData {
    data: String,
}

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command]
pub async fn create_new_window(parent_window: WebviewWindow) {
    let app_handle = parent_window.app_handle().clone();
    create_child_window(
        parent_window.label(),
        "/",
        &format!(
            "{OTHER_WINDOW_PREFIX}{}",
            app_handle.webview_windows().len()
        ),
        "Moss Studio",
        (DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
        app_handle,
    )
    .expect("Failed to create new window");
}

#[tauri::command]
pub fn main_window_is_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main_0").unwrap();
    window.show().unwrap();

    window
        .emit(
            "channel1:eventA",
            EventAData {
                data: "Hello from Rust!".to_string(),
            },
        )
        .unwrap();
}
