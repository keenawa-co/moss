use anyhow::Result;
use homedir::my_home;
use platform_core::context_v2::async_context::AsyncContext;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use workbench_desktop::WorkbenchState;

use crate::utl::get_home_dir;
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
#[specta::specta]
pub async fn update_font_size(
    async_ctx: State<'_, AsyncContext>,
    state: State<'_, AppState>,
    input: i32,
) -> Result<(), String> {
    Ok(state
        .workbench
        .update_conf(async_ctx.inner(), input as usize)
        .map_err(|e| e.to_string())?)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn app_ready(app_handle: AppHandle) {
    let window = app_handle.get_webview_window("main").unwrap();
    window.show().unwrap();
}

// #[tauri::command(async)]
// #[specta::specta]
// pub async fn create_project(
//     state: State<'_, AppState>,
//     input: CreateProjectInput,
// ) -> Result<Option<ProjectDTO>, String> {
//     match state.project_service.create_project(&input).await {
//         Ok(Some(project)) => return Ok(Some(project.into())),
//         Ok(None) => return Ok(None),
//         Err(e) => {
//             let err = format!("An error occurred while creating the project: {e}");
//             error!(err);
//             return Err(err);
//         }
//     }
// }

#[tauri::command(async)]
#[specta::specta]
pub async fn workbench_get_state(state: State<'_, AppState>) -> Result<WorkbenchState, String> {
    Ok(WorkbenchState::Empty)
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
