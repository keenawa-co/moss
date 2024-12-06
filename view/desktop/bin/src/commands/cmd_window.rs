use tauri::{AppHandle, Emitter, Listener, Manager, WebviewWindow};

use crate::create_main_window;

#[derive(Clone, Serialize)]
struct EventAData {
    data: String,
}

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command]
pub async fn create_new_window(app_handle: AppHandle) {
    create_main_window(&app_handle, "/");
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
