use moss_ui::parts::toolbar::DescribeToolBarOutput;
use tauri::State;
use workbench_tao::parts::activitybar::DescribeActivityBarOutput;
use workbench_tao::parts::toolbar::describe_toolbar;
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
    // TODO: consider to use full import parts::toolbar::describe()
    // describe_toolbar(
    //     &state.workbench.frame,
    //     state.workbench.project_context_menu.get().unwrap(), // TODO: handle error
    // )
    state
        .workbench
        .describe_part()
        .map_err(|err| format!("failed to describe toolbar: {err}"))
}
