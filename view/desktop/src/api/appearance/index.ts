import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { AppState, LocaleDescriptor, ThemeDescriptor } from "@repo/moss-desktop";

// App state

export const getState = async (): Promise<IpcResult<AppState, string>> => {
  return await invokeTauriIpc<AppState, string>("get_state");
};

//Color themes

export const getColorThemes = async (): Promise<IpcResult<ThemeDescriptor[], string>> => {
  return await invokeTauriIpc<ThemeDescriptor[], string>("get_themes");
};

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc<string, string>("get_color_theme", {
    path: source,
  });
};

//Language packs

export const getLanguagePacks = async (): Promise<IpcResult<LocaleDescriptor[], string>> => {
  return await invokeTauriIpc<LocaleDescriptor[], string>("get_locales");
};
