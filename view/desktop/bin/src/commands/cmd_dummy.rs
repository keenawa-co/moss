use std::path::PathBuf;

use anyhow::Result;
use desktop_models::appearance::theming::{Theme, ThemeDescriptor};
use smol::fs;
use tauri::{AppHandle, Emitter, EventTarget, State, WebviewWindow};

use crate::utl::{get_themes_dir, read_theme_css, update_all_window_theme, update_window_theme};
use crate::AppState;

#[tauri::command(async)]
pub async fn fetch_all_themes() -> Result<Vec<String>, String> {
    let mut valid_themes: Vec<String> = vec![];
    let themes_dir = get_themes_dir()?;
    let dir_iter = std::fs::read_dir(themes_dir).map_err(|e| e.to_string())?;
    for entry in dir_iter {
        let entry = entry.map_err(|e| e.to_string())?;
        let file_name = entry.file_name().to_str().unwrap().to_owned();
        if !file_name.ends_with(".json") {
            continue;
        }
        valid_themes.push(file_name.strip_suffix(".json").unwrap().to_string());
    }

    Ok(valid_themes)
}

#[tauri::command(async)]
pub async fn fetch_themes() -> Result<Vec<ThemeDescriptor>, String> {
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

#[tauri::command(async)]
pub async fn read_theme(theme_name: String) -> Result<Theme, String> {
    let theme_file_path = get_themes_dir()?.join(format!("{theme_name}.json"));

    let content = std::fs::read_to_string(theme_file_path).map_err(|e| e.to_string())?;
    match serde_json::from_str::<Theme>(&content) {
        Ok(theme) => Ok(theme),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command(async)]
pub async fn get_selected_theme(
    current_window: WebviewWindow,
    state: State<'_, AppState>,
) -> Result<(), String> {
    dbg!("read");
    update_window_theme(&current_window).await;
    Ok(())
}

#[tauri::command(async)]
pub async fn set_selected_theme(
    app_handle: AppHandle,
    selected_theme: ThemeDescriptor,
    state: State<'_, AppState>,
) -> Result<(), String> {
    dbg!("write");
    {
        let mut stored_theme = state.selected_theme.lock();
        *stored_theme = selected_theme.clone();
    }
    update_all_window_theme(app_handle).await;
    Ok(())
}
