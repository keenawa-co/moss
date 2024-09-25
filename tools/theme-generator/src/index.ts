import { Theme, Colors } from "@repo/moss-models";
import { existsSync, mkdirSync, writeFileSync } from "fs";
import * as os from "os";

// FIXME: temporary solution
const homeDirectory = os.homedir();
const themesDirectory = `${homeDirectory}/.config/moss/themes`;

const themes = [
  new Theme(
    "moss-dark",
    "dark",
    false,
    new Colors(
      "255, 255, 255, 1",
      "39, 39, 42, 1",
      "30, 32, 33, 1",
      "22, 24, 25, 1",
      "0, 122, 205, 1",
      "196, 43, 28, 1",
      "55, 55, 55, 1",
      "255, 255, 255, 1",
      "66, 66, 66, 1",
      "86, 86, 86, 1"
    )
  ),
  new Theme(
    "moss-light",
    "light",
    true,
    new Colors(
      "0, 0, 0, 1",
      "244, 244, 245, 1",
      "224, 224, 224, 1",
      "255,255,255, 1",
      "0, 122, 205, 1",
      "196, 43, 28, 1",
      "218, 218, 218, 1",
      "61, 61, 61, 1",
      "209, 209, 209, 1",
      "191, 191, 191, 1"
    )
  ),
  new Theme(
    "moss-pink",
    "pink",
    false,
    new Colors(
      "0, 0, 0, 1",
      "234, 157, 242, 1",
      "222, 125, 232, 1",
      "227, 54, 245, 1",
      "63, 11, 69, 1",
      "196, 43, 28, 1",
      "218, 218, 218, 1",
      "61, 61, 61, 1",
      "209, 209, 209, 1",
      "191, 191, 191, 1"
    )
  ),
];

function ensureDirectoryExists(directory: string): void {
  if (!existsSync(directory)) {
    mkdirSync(directory, { recursive: true });
  }
}

async function writeThemeFile(theme: Theme): Promise<void> {
  const fileName = `${themesDirectory}/${theme.name}.json`;

  const filteredColors = Object.fromEntries(
    Object.entries(theme.colors).filter(([key]) => key.includes(".") || !/[A-Z]/.test(key))
  );

  const filteredTheme = {
    ...theme,
    colors: filteredColors,
  };

  writeFileSync(fileName, JSON.stringify(filteredTheme, null, 2), {
    flag: "w",
  });
}

async function generateThemeFiles(): Promise<void> {
  try {
    await ensureDirectoryExists(themesDirectory);
    await Promise.all(themes.map(writeThemeFile));
    console.log("Theme files generated successfully.");
  } catch (error) {
    console.error("Error generating theme files:", error);
    process.exit(1);
  }
}

generateThemeFiles();
