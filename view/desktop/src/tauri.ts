import { invoke } from "@tauri-apps/api/core";
import type { InvokeArgs } from "@tauri-apps/api/core";

// prettier-ignore
type TauriCommand = 
    | "sidebar_get_all_activities"
    | "describe_primary_activitybar_part"
    | "describe_primary_sidebar_part"
    | "get_view_content"
    | "get_menu_items";

export async function invokeCmd<T>(cmd: TauriCommand, args?: InvokeArgs): Promise<T> {
  return invoke(cmd, args);
}
