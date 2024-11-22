use anyhow::Result;
use desktop_models::appearance::theming::Theme;
use tauri::State;

use crate::utl::get_themes_dir;
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
pub async fn read_theme(theme_name: String) -> Result<Theme, String> {
    let theme_file_path = get_themes_dir()?.join(format!("{theme_name}.json"));

    let content = std::fs::read_to_string(theme_file_path).map_err(|e| e.to_string())?;
    match serde_json::from_str::<Theme>(&content) {
        Ok(theme) => Ok(theme),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn get_stored_string(state: State<'_, AppState>) -> String {
    dbg!("read");
    let stored_string = state.react_query_string.lock();
    stored_string.clone()
}

#[tauri::command]
pub fn set_stored_string(new_string: String, state: State<'_, AppState>) {
    dbg!("write");
    let mut stored_string = state.react_query_string.lock();
    *stored_string = new_string;
}
