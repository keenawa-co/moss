use anyhow::Result;
use hashbrown::HashMap;
use moss_desktop::{
    models::{appearance::theming::ThemeDescriptor, window::LocaleDescriptor},
    state::CommandContext,
};
use moss_text::{quote, ReadOnlyStr};
use serde_json::Value;
use std::path::PathBuf;

use crate::{create_child_window, AppState};
use tauri::{AppHandle, Emitter, State, WebviewWindow, Window};

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
pub fn execute_command(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    window: Window,
    cmd: ReadOnlyStr,
    args: HashMap<String, Value>,
) -> Result<Value, String> {
    if let Some(command_handler) = app_state.get_command(&cmd) {
        command_handler(CommandContext::new(app_handle, window, args), &app_state)
    } else {
        Err(format!("command with id {} is not found", quote!(cmd)))
    }
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

// FIXME: This is a temporary solution until we have a registry of installed
// plugins and the ability to check which theme packs are installed.
#[tauri::command(async)]
pub async fn get_themes() -> Result<Vec<ThemeDescriptor>, String> {
    Ok(vec![
        ThemeDescriptor {
            id: "theme-light".to_string(),
            name: "Theme Light".to_string(),
            source: PathBuf::from("moss-light.css")
                .to_string_lossy()
                .to_string(),
        },
        ThemeDescriptor {
            id: "theme-dark".to_string(),
            name: "Theme Dark".to_string(),
            source: PathBuf::from("moss-dark.css").to_string_lossy().to_string(),
        },
        ThemeDescriptor {
            id: "theme-pink".to_string(),
            name: "Theme Pink".to_string(),
            source: PathBuf::from("moss-pink.css").to_string_lossy().to_string(),
        },
    ])
}

// FIXME: This is a temporary solution until we have a registry of installed
// plugins and the ability to check which language packs are installed.
#[tauri::command]
pub fn get_locales() -> Vec<LocaleDescriptor> {
    vec![
        LocaleDescriptor {
            code: "en".to_string(),
            name: "English".to_string(),
            direction: Some("ltr".to_string()),
        },
        LocaleDescriptor {
            code: "de".to_string(),
            name: "Deutsche".to_string(),
            direction: Some("ltr".to_string()),
        },
        LocaleDescriptor {
            code: "ru".to_string(),
            name: "Русский".to_string(),
            direction: Some("ltr".to_string()),
        },
    ]
}

#[tauri::command]
pub fn get_translations(language: String, namespace: String) -> Result<serde_json::Value, String> {
    let path = crate::utl::get_home_dir()
        .map_err(|err| err.to_string())?
        .join(".config")
        .join("moss")
        .join("locales")
        .join(language)
        .join(format!("{namespace}.json"));

    match std::fs::read_to_string(path) {
        Ok(data) => {
            let translations: serde_json::Value =
                serde_json::from_str(&data).map_err(|err| err.to_string())?;

            Ok(translations)
        }
        Err(err) => Err(err.to_string()),
    }
}
