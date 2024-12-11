use desktop_models::actions::MenuItem;
use tauri::State;
use workbench_desktop::contributions::resents::{RecentsViewContent, RecentsViewModel};

use crate::AppState;

#[tauri::command]
pub fn get_view_content(state: State<'_, AppState>) -> Result<RecentsViewContent, String> {
    let model = state
        .workbench
        .get_view::<RecentsViewModel>(
            "workbench.group.launchpad",
            "workbench.view.recentsView".to_string(),
        )
        .unwrap();

    model
        .content()
        .map_err(|err| format!("failed to get view content: {err}"))
}

#[tauri::command]
pub fn get_menu_items_by_namespace(
    state: State<'_, AppState>,
    namespace: String,
) -> Option<Vec<MenuItem>> {
    state.workbench.get_menu_items_by_namespace(namespace)
}
