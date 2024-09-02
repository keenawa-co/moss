import { IThemeRGB, IThemeVariables } from "../types";
import { readAllFilesInDirectory } from "../readThemes";
import { BaseDirectory } from "@tauri-apps/plugin-fs";

// Should be used to apply the theme to the root element of the app. Will use
// base colors in `theme/base/colors.ts` if no overrides are provided.
export default async function applyTheme(themeRGB: IThemeRGB) {
  let themes = readAllFilesInDirectory(BaseDirectory.Resource, "themes");

  for (var theme of await themes) {
    console.warn("-------------->>>>>");
    console.warn(theme);
  }

  const themeObject: IThemeVariables = mapTheme(themeRGB);
  const root = document.documentElement;

  Object.keys(themeObject).forEach((v) => {
    const propertyVal = themeObject[v as keyof IThemeVariables];
    const validation = validateRGB(propertyVal);
    if (!validation) {
      throw new Error(`Invalid RGB value for ${v}: ${propertyVal}`);
    }

    root.style.setProperty(v, propertyVal);
  });
}

function mapTheme(rgb: IThemeRGB): IThemeVariables {
  return {
    "--color-primary": rgb["rgb-primary"] ?? "",
    "--color-sidebar-background": rgb["rgb-sidebar-background"] ?? "",
    "--color-toolbar-background": rgb["rgb-toolbar-background"] ?? "",
    "--color-page-background": rgb["rgb-page-background"] ?? "",
    "--color-statusbar-background": rgb["rgb-statusbar-background"] ?? "",

    "--color-windows-close-button-background": rgb["rgb-windows-close-button-background"] ?? "",

    "--color-window-controls-linux-background": rgb["rgb-window-controls-linux-background"] ?? "",
    "--color-window-controls-linux-text": rgb["rgb-window-controls-linux-text"] ?? "",
    "--color-window-controls-linux-background-hover": rgb["rgb-window-controls-linux-background-hover"] ?? "",
    "--color-window-controls-linux-background-active": rgb["rgb-window-controls-linux-background-active"] ?? "",
  };
}

function validateRGB(rgb: string): boolean {
  if (!rgb) return true;
  const rgbRegex = /^(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})$/;
  return rgbRegex.test(rgb);
}
