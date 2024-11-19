use anyhow::Result;
use homedir::my_home;
use platform_core::context_v2::async_context::AsyncContext;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use workbench_desktop::WorkbenchState;

use crate::utl::get_home_dir;
use crate::AppState;

#[tauri::command(async)]
#[specta::specta]
pub async fn fetch_all_themes() -> Result<Vec<String>, String> {
    Ok(vec![
        "moss-dark".to_string(),
        "moss-light".to_string(),
        "moss-pink".to_string(),
    ])
}

#[tauri::command(async)]
#[specta::specta]
pub async fn read_theme(theme_name: String) -> Result<String, String> {
    let home_dir = get_home_dir()?;
    let theme_file_path = home_dir
        .join(".config")
        .join("moss")
        .join("themes")
        .join(format!("{theme_name}.json"));

    match std::fs::read_to_string(theme_file_path) {
        Ok(content) => Ok(content),
        Err(err) => {
            dbg!(&err);
            Err(format!("filed to read theme file: {err}"))
        }
    }
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
