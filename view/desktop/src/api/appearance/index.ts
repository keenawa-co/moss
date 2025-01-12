import { invokeTauriIpc, IpcResult } from "@/lib/backend/tauri";
import { AppState, LocaleDescriptor, MenuItem, ThemeDescriptor } from "@repo/moss-desktop";
import { invoke } from "@tauri-apps/api/core";

// App state

export const getState = async (): Promise<AppState> => {
  return await invoke<AppState>("get_state");
};

//Color themes

export const getColorThemes = async (): Promise<ThemeDescriptor[]> => {
  return await invoke<ThemeDescriptor[]>("get_themes");
};

export const getColorTheme = async (source: string): Promise<IpcResult<string, string>> => {
  return await invokeTauriIpc("get_color_theme", {
    path: source,
  });
};

//Language packs

export const getLanguagePacks = async (): Promise<LocaleDescriptor[]> => {
  return await invoke<LocaleDescriptor[]>("get_locales");
};

//Activities

export const getAllActivities = async (): Promise<IpcResult<MenuItem[], Error>> => {
  return await invokeTauriIpc("get_menu_items_by_namespace", { namespace: "headItem" }); // this here should be a type
};
