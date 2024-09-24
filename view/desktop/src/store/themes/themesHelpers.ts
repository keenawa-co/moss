import { commands } from "@/bindings";
import { Convert } from "@repo/moss_theme";

export const handleReadTheme = async (themeName: string) => {
  try {
    const response = await commands.readTheme(themeName);

    if (response.status === "error") throw new Error("Failed to read theme");

    return Convert.toTheme(response.data);
  } catch (error) {
    console.error(error);
  }
};
