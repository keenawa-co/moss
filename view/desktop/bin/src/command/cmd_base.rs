use tauri::State;
use workbench_tgui::window::NativePlatformInfo;

use crate::AppState;

#[tauri::command]
#[specta::specta]
pub fn native_platform_info(state: State<'_, AppState>) -> NativePlatformInfo {
    state.platform_info.clone()
}
