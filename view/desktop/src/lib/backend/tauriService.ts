import { InvokeArgs } from "@tauri-apps/api/core";
import { EventName, EventCallback } from "@tauri-apps/api/event";
import { invokeIpc, listenIpc, ITauriIpcService, TauriIpcCommand, IpcResult } from "./tauri";

export class TauriIpcService implements ITauriIpcService {
  _serviceBrand: undefined;

  constructor() {}

  async invoke<T, E = unknown>(command: TauriIpcCommand, args?: InvokeArgs): Promise<IpcResult<T, E>> {
    return await invokeIpc<T, E>(command, args);
  }

  listen<T>(event: EventName, handle: EventCallback<T>): () => Promise<void> {
    return listenIpc<T>(event, handle);
  }
}
