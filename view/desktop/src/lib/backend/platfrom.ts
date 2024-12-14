import { InvokeArgs } from "@tauri-apps/api/core";

import { invokeTauriIpc, IpcResult } from "./tauri";

export const invokeMossCommand = async <T, E>(cmd: string, args?: InvokeArgs): Promise<IpcResult<T, E>> => {
  return await invokeTauriIpc<T, E>("execute_command", {
    cmd,
    args,
  });
};
