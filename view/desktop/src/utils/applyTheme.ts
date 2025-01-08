import { getColorTheme } from "@/api/appearance";
import { IpcResult } from "@/lib/backend/tauri";
import { ThemeDescriptor } from "@repo/moss-desktop";

export const applyTheme = async (theme: ThemeDescriptor) => {
  try {
    const result: IpcResult<string, string> = await getColorTheme(theme.source);

    if (result.status === "ok") {
      const cssContent = result.data;
      let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

      if (styleTag) {
        styleTag.innerHTML = cssContent;
      } else {
        styleTag = document.createElement("style");
        styleTag.id = "theme-style";
        styleTag.innerHTML = cssContent;
        document.head.appendChild(styleTag);
      }
    } else {
      console.error(`Error reading theme file for "${theme.id}":`, result.error);
    }
  } catch (error) {
    console.error(`Failed to apply theme "${theme.id}":`, error);
  }
};
