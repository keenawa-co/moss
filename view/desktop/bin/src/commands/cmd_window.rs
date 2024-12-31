use anyhow::Result;
use hashbrown::HashMap;
use moss_desktop::{
    command::CommandContext,
    models::application::{AppStateInfo, LocaleDescriptor, PreferencesInfo, ThemeDescriptor},
    services::theme_service::{GetColorThemeOptions, ThemeService},
};
use moss_text::{quote, ReadOnlyStr};
use serde_json::Value;
use std::path::PathBuf;

use crate::{create_child_window, AppState};
use moss_desktop::services::locale_service::{GetTranslationsOptions, LocaleService};
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
pub async fn get_color_theme(
    app_state: State<'_, AppState>,
    path: String,
    opts: Option<GetColorThemeOptions>,
) -> Result<String, String> {
    let theme_service = app_state.services.get_unchecked::<ThemeService>();
    theme_service
        .get_color_theme(&path, opts)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn get_translations(
    locale_service: State<'_, LocaleService>,
    language: String,
    namespace: String,
    opts: Option<GetTranslationsOptions>,
) -> Result<serde_json::Value, String> {
    locale_service
        .get_translations(&language, &namespace, opts)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub fn get_state(app_state: State<'_, AppState>) -> Result<AppStateInfo, String> {
    Ok(AppStateInfo {
        preferences: PreferencesInfo {
            theme: app_state.preferences.theme.read().clone(),
            locale: app_state.preferences.locale.read().clone(),
        },
    })
}

// FIXME: This is a temporary solution until we have a registry of installed
// plugins and the ability to check which theme packs are installed.
#[tauri::command(async)]
pub async fn get_themes(app_state: State<'_, AppState>) -> Result<Vec<ThemeDescriptor>, String> {
    let theme_service = app_state.services.get_unchecked::<ThemeService>();
    let r: Vec<ThemeDescriptor> = theme_service
        .get_color_themes()
        .clone()
        .into_iter()
        .collect();

    Ok(theme_service
        .get_color_themes()
        .clone()
        .into_iter()
        .collect())
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
