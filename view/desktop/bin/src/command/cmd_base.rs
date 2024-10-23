use tauri::State;
use workbench_tao::parts::primary_activitybar::{
    DescribeActivityBarPartOutput, PrimaryActivityBarPart,
};
use workbench_tao::parts::primary_sidebar::{DescribeSideBarPartOutput, PrimarySideBarPart};
use workbench_tao::parts::{AnyPart, Parts};
use workbench_tao::window::NativePlatformInfo;

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
        .get_part::<PrimaryActivityBarPart>(Parts::PrimaryActivityBar.as_part_id())
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
        .get_part::<PrimarySideBarPart>(Parts::PrimarySideBar.as_part_id())
        .unwrap();

    part.describe(state.workbench.registry())
        .map_err(|err| format!("failed to describe primary sidebar: {err}"))
}
