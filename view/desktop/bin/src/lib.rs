mod mem;
mod menu;

use tauri::{App, AppHandle, Manager};
use tauri_specta::{collect_commands, collect_events, ts};

#[macro_use]
extern crate tracing;

#[tauri::command]
#[specta::specta]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

pub fn run() -> tauri::Result<()> {
    let (invoke_handler, register_events) = {
        let builder = ts::builder()
            .events(collect_events![])
            .commands(collect_commands![app_ready, greet])
            .config(specta::ts::ExportConfig::new().formatter(specta::ts::formatter::prettier));

        #[cfg(debug_assertions)]
        let builder = builder.path("../src/bindings.ts");

        builder.build().unwrap()
    };

    tauri::Builder::default()
        .invoke_handler(invoke_handler)
        .setup(move |app: &mut App| {
            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async move { register_events(app) })
            });

            Ok(())
        })
        .menu(menu::setup_window_menu)
        .build(tauri::generate_context!())?
        .run(|_, _| {});

    Ok(())
}
