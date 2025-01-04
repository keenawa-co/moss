import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { AppState, ThemeDescriptor } from "@repo/moss-desktop";
import { invoke } from "@tauri-apps/api/core";

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc<string, string>("get_color_theme", {
    path: source,
  });
};

export const getColorThemes = async (): Promise<ThemeDescriptor[]> => {
  return await invoke<ThemeDescriptor[]>("get_themes");
};

export const getState = async (): Promise<AppState> => {
  return await invoke<AppState>("get_state");
};
