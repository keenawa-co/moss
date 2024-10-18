use moss_ui::parts::toolbar::DescribeToolBarOutput;
use tauri::State;
use workbench_tao::parts::activitybar::{ActivityBarPart, DescribeActivityBarOutput};
use workbench_tao::parts::{AnyPart, ACTIVITY_BAR_PART};
use workbench_tao::window::NativePlatformInfo;

use crate::AppState;

#[tauri::command]
pub fn native_platform_info(state: State<'_, AppState>) -> NativePlatformInfo {
    state.platform_info.clone()
}

#[tauri::command]
pub fn describe_activity_bar_part(
    state: State<'_, AppState>,
) -> Result<DescribeActivityBarOutput, String> {
    let part = state
        .workbench
        .get_part::<ActivityBarPart>(ACTIVITY_BAR_PART)
        .unwrap();

    dbg!(&part.id());

    part.describe(state.workbench.layout())
        .map_err(|err| format!("failed to describe toolbar: {err}"))
}
