import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc<string, string>("get_color_theme", {
    path: source,
  });
};
