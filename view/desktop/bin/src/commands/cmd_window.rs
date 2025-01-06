use anyhow::Result;
use hashbrown::HashMap;
use moss_desktop::{
    app::manager::AppManager,
    command::CommandContext,
    models::application::{AppState, Defaults, LocaleDescriptor, Preferences, ThemeDescriptor},
    services::{locale_service::LocaleService, theme_service::ThemeService},
};
use moss_tauri::TauriResult;
use moss_text::{quote, ReadOnlyStr};
use serde_json::Value;
use tracing::instrument;

use crate::{create_child_window, menu, AppStateManager};
use tauri::{AppHandle, State, Window};

// According to https://docs.rs/tauri/2.1.1/tauri/webview/struct.WebviewWindowBuilder.html
// We should call WebviewWindowBuilder from async commands
#[tauri::command]
#[instrument(level = "trace", skip(app_handle))]
pub async fn create_new_window(app_handle: AppHandle) -> TauriResult<()> {
    let webview_window = create_child_window(&app_handle, "/")?;
    webview_window.on_menu_event(move |window, event| menu::handle_event(window, &event));
    Ok(())
}

#[tauri::command]
#[instrument(level = "trace", skip(app_handle, app_state), fields(window = window.label()))]
pub fn execute_command(
    app_handle: AppHandle,
    app_state: State<'_, AppStateManager>,
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
#[instrument(level = "trace", skip(app_manager))]
pub async fn get_color_theme(
    app_manager: State<'_, AppManager>,
    path: String,
) -> TauriResult<String> {
    let theme_service = app_manager.service::<ThemeService>()?;

    Ok(theme_service.get_color_theme(&path).await?)
}

#[tauri::command(async)]
pub async fn get_translations(
    app_manager: State<'_, AppManager>,
    language: String,
    namespace: String,
    opts: Option<GetTranslationsOptions>,
) -> TauriResult<Value> {
    let locale_service = app_manager.service::<LocaleService>()?;
    Ok(locale_service
        .get_translations(&language, &namespace, opts)
        .await?)
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(state_manager))]
pub fn get_state(state_manager: State<'_, AppStateManager>) -> Result<AppState, String> {
    Ok(AppState {
        preferences: Preferences {
            theme: state_manager.preferences.theme.read().clone(),
            locale: state_manager.preferences.locale.read().clone(),
        },
        defaults: Defaults {
            theme: state_manager.defaults.theme.clone(),
            locale: state_manager.defaults.locale.clone(),
        },
    })
}

#[tauri::command(async)]
#[instrument(level = "trace", skip(app_manager))]
pub async fn get_themes(app_manager: State<'_, AppManager>) -> TauriResult<Vec<ThemeDescriptor>> {
    let theme_service = app_manager.service::<ThemeService>()?;

    Ok(theme_service
        .get_color_themes()
        .clone()
        .into_iter()
        .collect())
}

#[tauri::command(async)]
pub async fn get_locales(app_manager: State<'_, AppManager>) -> TauriResult<Vec<LocaleDescriptor>> {
    let locale_service = app_manager.service::<LocaleService>()?;

    Ok(locale_service.get_locales().clone().into_iter().collect())
}

// // FIXME: This is a temporary solution until we have a registry of installed
// // plugins and the ability to check which language packs are installed.
// #[tauri::command]
// pub fn get_locales() -> Vec<LocaleDescriptor> {
//     vec![
//         LocaleDescriptor {
//             code: "en".to_string(),
//             name: "English".to_string(),
//             direction: Some("ltr".to_string()),
//         },
//         LocaleDescriptor {
//             code: "de".to_string(),
//             name: "Deutsche".to_string(),
//             direction: Some("ltr".to_string()),
//         },
//         LocaleDescriptor {
//             code: "ru".to_string(),
//             name: "Русский".to_string(),
//             direction: Some("ltr".to_string()),
//         },
//     ]
// }
