use std::sync::Arc;

use tauri::State;
use workbench_desktop::contributions::resents::{RecentsViewContent, RecentsViewModel};
use workbench_desktop::menu::{BuiltInMenuNamespaces, MenuItem};
use workbench_desktop::parts::primary_activitybar::{
    DescribeActivityBarPartOutput, PrimaryActivityBarPart,
};
use workbench_desktop::parts::primary_sidebar::{DescribeSideBarPartOutput, PrimarySideBarPart};
use workbench_desktop::parts::{AnyPart, Parts};
use workbench_desktop::window::NativePlatformInfo;

use crate::AppState;

#[tauri::command]
pub fn native_platform_info(state: State<'_, AppState>) -> NativePlatformInfo {
    state.platform_info.clone()
}

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

// get_menu_items_by_many_namespaces
