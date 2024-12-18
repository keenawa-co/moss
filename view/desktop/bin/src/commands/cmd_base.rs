use moss_desktop::models::actions::MenuItem;
use moss_text::ReadOnlyStr;
use tauri::State;

use crate::AppState;

#[tauri::command]
pub fn get_menu_items_by_namespace(
    state: State<'_, AppState>,
    namespace: ReadOnlyStr,
) -> Result<Vec<MenuItem>, String> {
    if let Some(menu_items_ref) = state.menus.get(&namespace) {
        let menu_items = menu_items_ref.clone();
        Ok(menu_items)
    } else {
        Err(format!("Namespace '{}' not found", namespace))
    }
}
