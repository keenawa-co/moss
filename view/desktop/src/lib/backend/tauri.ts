import { InvokeArgs, invoke as invokeTauri } from "@tauri-apps/api/core";
import { listen as listenTauri } from "@tauri-apps/api/event";
import type { EventCallback, EventName } from "@tauri-apps/api/event";

// Define all possible Tauri IPC commands as string literals
export type TauriIpcCommand =
  | "execute_command"
  | "get_translations"
  | "get_color_theme"
  | "create_new_window"
  | "sidebar_get_all_activities"
  | "get_menu_items_by_namespace"
  | "get_locales"
  | "get_state"
  | "get_themes";

export type IpcResult<T, E> = { status: "ok"; data: T } | { status: "error"; error: E };

export const handleTauriIpcError = (cmd: TauriIpcCommand, error: unknown) => {
  console.error(`Error in IPC command "${cmd}":`, error);

  // TODO: dispatch to a global error handler or show user notifications
};

export const invokeTauriIpc = async <T, E = unknown>(
  cmd: TauriIpcCommand,
  args?: InvokeArgs
): Promise<IpcResult<T, E>> => {
  try {
    const data = await invokeTauri<T>(cmd, args);
    return { status: "ok", data };
  } catch (err) {
    handleTauriIpcError(cmd, err);
    return { status: "error", error: err as E };
  }
};

export const listenTauriIpc = <T>(event: EventName, handle: EventCallback<T>) => {
  const unlisten = listenTauri(event, handle);
  return async () => await unlisten.then((unlistenFn) => unlistenFn());
};
