import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { LocaleDescriptor, ThemeDescriptor } from "@repo/moss-desktop";
import { invoke } from "@tauri-apps/api/core";

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

export const getColorThemes = async (): Promise<ThemeDescriptor[]> => {
  return await invoke<ThemeDescriptor[]>("get_themes");
};

interface GetStateResult {
  preferences: {
    theme: ThemeDescriptor;
    locale: LocaleDescriptor;
  };
}

export const getState = async (): Promise<GetStateResult> => {
  return await invoke<GetStateResult>("get_state");
};
