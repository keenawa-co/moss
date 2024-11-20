import { Theme } from "@repo/desktop-models";
import { invokeIpc } from "@/lib/backend/tauri";

export const handleReadTheme = async (themeName: string) => {
  try {
    const response = await invokeIpc<Theme, string>("read_theme", { themeName });
    if (response.status === "error") throw new Error("Failed to read theme");
    const theme: Theme = response.data;
    return theme;
  } catch (error) {
    console.error(error);
  }
};
