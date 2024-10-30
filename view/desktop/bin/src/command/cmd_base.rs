use std::sync::Arc;

use tauri::State;
use workbench_desktop::contributions::resents::{
    RecentsViewContentProviderOutput, RecentsViewModel,
};
use workbench_desktop::menu::{MenuItem, Menus};
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
pub fn describe_primary_activitybar_part(
    state: State<'_, AppState>,
) -> Result<DescribeActivityBarPartOutput, String> {
    let part = state
        .workbench
        .get_part::<PrimaryActivityBarPart>(Parts::PrimaryActivityBar.as_str())
        .unwrap();

    part.describe(state.workbench.registry())
        .map_err(|err| format!("failed to describe primary activity bar: {err}"))
}

#[tauri::command]
pub fn describe_primary_sidebar_part(
    state: State<'_, AppState>,
) -> Result<DescribeSideBarPartOutput, String> {
    let part = state
        .workbench
        .get_part::<PrimarySideBarPart>(Parts::PrimarySideBar.as_str())
        .unwrap();

    part.describe(state.workbench.registry())
        .map_err(|err| format!("failed to describe primary sidebar: {err}"))
}

#[tauri::command]
pub fn get_view_content(
    state: State<'_, AppState>,
) -> Result<RecentsViewContentProviderOutput, String> {
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
pub fn get_menu_items(state: State<'_, AppState>) -> Option<Vec<MenuItem>> {
    if let Some(items) = state
        .workbench
        .get_menu_items(&Menus::ViewItemContext.into())
    {
        Some(items.clone())
    } else {
        None
    }
}
