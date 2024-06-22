// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{App, Manager};

mod mem;
mod menu;

#[macro_use]
extern crate tracing;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> tauri::Result<()> {
    tauri::Builder::default()
        .setup(move |app: &mut App| {
            app.get_webview_window("main").unwrap();
            Ok(())
        })
        .menu(menu::setup_window_menu)
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())?
        .run(|_, _| {});

    Ok(())
}
