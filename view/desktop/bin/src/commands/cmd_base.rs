use moss_desktop::{
    contributions::resents::{RecentsViewContent, RecentsViewModel},
    models::actions::MenuItem,
};
use moss_text::ReadOnlyStr;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub fn get_view_content(state: State<'_, AppState>) -> Result<RecentsViewContent, String> {
    let views_registry_lock = state.views.read();

    let model = views_registry_lock
        .get_view_model::<RecentsViewModel>(
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
    namespace: ReadOnlyStr,
) -> Option<Vec<MenuItem>> {
    let menu_registry_lock = state.menus.read();
    menu_registry_lock
        .get_menu_items_by_namespace(&namespace)
        .cloned()
}
