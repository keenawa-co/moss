import { invokeIpc, IpcResult } from "@/lib/backend/tauri";

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeIpc<string, string>("get_color_theme", {
    path: source,
  });
};
