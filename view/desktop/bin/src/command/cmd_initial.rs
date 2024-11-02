use tauri::{AppHandle, Emitter, Manager};

#[derive(Clone, Serialize)]
struct EventAData {
    data: String,
}

#[tauri::command]
pub fn main_window_is_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
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
