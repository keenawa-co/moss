mod mem;
mod menu;

use tauri::App;

#[macro_use]
extern crate tracing;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() -> tauri::Result<()> {
    tauri::Builder::default()
        .setup(move |_app: &mut App| Ok(()))
        .menu(menu::setup_window_menu)
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())?
        .run(|_, _| {});

    Ok(())
}
