import { invokeIpc, IpcResult } from "@/lib/backend/tauri";

export * from "./types";

export const readThemeFile = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeIpc<string, string>("read_theme_file", {
    path: source,
  });
};
