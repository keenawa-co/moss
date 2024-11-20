import { commands } from "@/bindings";

import { Theme } from "@repo/desktop-models";

export const handleReadTheme = async (themeName: string) => {
  try {
    const response = await commands.readTheme(themeName);

    if (response.status === "error") throw new Error("Failed to read theme");

    return response.data;
  } catch (error) {
    console.error(error);
  }
};
