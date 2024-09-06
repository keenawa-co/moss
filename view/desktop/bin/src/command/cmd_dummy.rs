use anyhow::Result;
use platform_core::common::context::async_context::AsyncContext;
use tauri::{AppHandle, Manager, State};
use workbench_tgui::WorkbenchState;

use crate::service::project_service::{CreateProjectInput, ProjectDTO};
use crate::service::session_service::SessionInfoDTO;
use crate::AppState;

#[tauri::command(async)]
#[specta::specta]
pub async fn update_font_size(
    async_ctx: State<'_, AsyncContext>,
    tctx: State<'_, platform_formation::context::Context>,
    state: State<'_, AppState<'_>>,
    input: i32,
) -> Result<(), String> {
    async_ctx.with_mut(|ctx| {
        state.workbench.update_conf(ctx, input as usize).unwrap();
        Ok(())
    })
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
    state: State<'_, AppState<'_>>,
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
pub async fn workbench_get_state(state: State<'_, AppState<'_>>) -> Result<WorkbenchState, String> {
    Ok(WorkbenchState::Empty)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn restore_session(
    state: State<'_, AppState<'_>>,
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
