use tauri::State;
use workbench_tao::parts::activitybar::{ActivityBarPart, DescribeActivityBarPartOutput};
use workbench_tao::parts::{AnyPart, Parts};
use workbench_tao::window::NativePlatformInfo;

use crate::AppState;

#[tauri::command]
pub fn native_platform_info(state: State<'_, AppState>) -> NativePlatformInfo {
    state.platform_info.clone()
}

#[tauri::command]
pub fn describe_activity_bar_part(
    state: State<'_, AppState>,
) -> Result<DescribeActivityBarPartOutput, String> {
    let part = state
        .workbench
        .get_part::<ActivityBarPart>(Parts::ActivityBar.as_part_id())
        .unwrap();

    part.describe(state.workbench.registry())
        .map_err(|err| format!("failed to describe toolbar: {err}"))
}
