use anyhow::Result;
use platform_core::context::async_context::AsyncContext;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager, State};
use workbench_tgui::WorkbenchState;

use crate::service::project_service::{CreateProjectInput, ProjectDTO};
use crate::service::session_service::SessionInfoDTO;
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
    let theme_file_path = PathBuf::from(std::env::var("HOME").unwrap())
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

#[tauri::command(async)]
#[specta::specta]
pub async fn create_project(
    state: State<'_, AppState>,
    input: CreateProjectInput,
) -> Result<Option<ProjectDTO>, String> {
    match state.project_service.create_project(&input).await {
        Ok(Some(project)) => return Ok(Some(project.into())),
        Ok(None) => return Ok(None),
        Err(e) => {
            let err = format!("An error occurred while creating the project: {e}");
            error!(err);
            return Err(err);
        }
    }
}

#[tauri::command(async)]
#[specta::specta]
pub async fn workbench_get_state(state: State<'_, AppState>) -> Result<WorkbenchState, String> {
    Ok(WorkbenchState::Empty)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn restore_session(
    state: State<'_, AppState>,
    project_source: Option<String>,
) -> Result<Option<SessionInfoDTO>, String> {
    match state.session_service.restore_session(project_source).await {
        Ok(Some(session_info)) => return Ok(Some(session_info.into())),
        Ok(None) => return Ok(None),
        Err(e) => {
            let err = format!("An error occurred while restoring the session: {e}");
            error!(err);
            return Err(err);
        }
    }
}

#[tauri::command]
#[specta::specta]
pub fn generate_log(app_handle: tauri::AppHandle) {
    // Generate a log message and emit it to the frontend
    let log_message = "Log message from backend".to_string();
    app_handle.emit("logs-stream", log_message).unwrap();
}

