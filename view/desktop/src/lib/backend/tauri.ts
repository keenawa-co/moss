import { InvokeArgs, invoke as invokeTauri } from "@tauri-apps/api/core";
import { listen as listenTauri } from "@tauri-apps/api/event";
import type { EventCallback, EventName } from "@tauri-apps/api/event";
import { createDecorator } from "../instantiation/instantiation";

export type IpcResult<T, E> = { status: "ok"; data: T } | { status: "error"; error: E };

export type TauriIpcCommand =
  | "main_window_is_ready"
  | "create_new_window"
  | "sidebar_get_all_activities"
  | "get_view_content"
  | "get_menu_items_by_namespace";

export const handleIpcError = (cmd: TauriIpcCommand, error: unknown) => {
  console.error(`Error in IPC command "${cmd}":`, error);

  // TODO: dispatch to a global error handler or show user notifications
};

export const invokeIpc = async <T, E = unknown>(cmd: TauriIpcCommand, args?: InvokeArgs): Promise<IpcResult<T, E>> => {
  try {
    const data = await invokeTauri<T>(cmd, args);
    return { status: "ok", data };
  } catch (err) {
    handleIpcError(cmd, err);
    return { status: "error", error: err as E };
  }
};

export const listenIpc = <T>(event: EventName, handle: EventCallback<T>) => {
  const unlisten = listenTauri(event, handle);
  return async () => await unlisten.then((unlistenFn) => unlistenFn());
};

export interface ITauriIpcService {
  readonly _serviceBrand: undefined;

  invoke<T, E = unknown>(command: TauriIpcCommand, args?: InvokeArgs): Promise<IpcResult<T, E>>;
  listen<T>(event: EventName, handle: EventCallback<T>): () => Promise<void>;
}
export const ITauriIpcService = createDecorator<ITauriIpcService>("tauriIpcService");
