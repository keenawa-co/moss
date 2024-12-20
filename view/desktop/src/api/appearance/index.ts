import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";

export interface GetColorThemeOptions {
  enableCache: boolean;
}

export const getColorTheme = async (
  source: string,
  opts?: GetColorThemeOptions
): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc<string, string>("get_color_theme", {
    path: source,
    opts,
  });
};
