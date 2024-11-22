import { Theme, Colors } from "@repo/desktop-models";
import { existsSync, mkdirSync, writeFileSync } from "fs";
import * as os from "os";

// FIXME: temporary solution
const homeDirectory = os.homedir();
const themesDirectory = `${homeDirectory}/.config/moss/themes`;

// Default

const defaultDarkTheme: Theme = {
  name: "Moss Dark Default",
  slug: "moss-dark",
  type: "dark",
  isDefault: false,
  colors: {
    "primary": "rgba(255, 255, 255, 1)", // prettier-ignore
    "sideBar.background": "rgba(39, 39, 42, 1)",
    "toolBar.background": "rgba(30, 32, 33, 1)",
    "page.background": "rgba(22, 24, 25, 1)",
    "statusBar.background": "rgba(0, 122, 205, 1)",
    "windowsCloseButton.background": "rgba(196, 43, 28, 1)",
    "windowControlsLinux.background": "rgba(55, 55, 55, 1)",
    "windowControlsLinux.text": "rgba(255, 255, 255, 1)",
    "windowControlsLinux.hoverBackground": "rgba(66, 66, 66, 1)",
    "windowControlsLinux.activeBackground": "rgba(86, 86, 86, 1)",
  },
};

const defaultLightTheme: Theme = {
  name: "Moss Light Default",
  slug: "moss-light",
  type: "light",
  isDefault: true,
  colors: {
    "primary": "rgba(0, 0, 0, 1)", // prettier-ignore
    "sideBar.background": "rgba(244, 244, 245, 1)",
    "toolBar.background": "rgba(224, 224, 224, 1)",
    "page.background": "rgba(255, 255, 255, 1)",
    "statusBar.background": "rgba(0, 122, 205, 1)",
    "windowsCloseButton.background": "rgba(196, 43, 28, 1)",
    "windowControlsLinux.background": "rgba(218, 218, 218, 1)",
    "windowControlsLinux.text": "rgba(61, 61, 61, 1)",
    "windowControlsLinux.hoverBackground": "rgba(209, 209, 209, 1)",
    "windowControlsLinux.activeBackground": "rgba(191, 191, 191, 1)",
  },
};

// Other

const pinkTheme: Theme = {
  name: "Moss Pink",
  slug: "moss-pink",
  type: "pink",
  isDefault: false,
  colors: {
    "primary": "rgba(0, 0, 0, 1)", // prettier-ignore
    "sideBar.background": "rgba(234, 157, 242, 1)",
    "toolBar.background": "rgba(222, 125, 232, 1)",
    "page.background": "rgba(227, 54, 245, 1)",
    "statusBar.background": "rgba(63, 11, 69, 1)",
    "windowsCloseButton.background": "rgba(196, 43, 28, 1)",
    "windowControlsLinux.background": "rgba(218, 218, 218, 1)",
    "windowControlsLinux.text": "rgba(61, 61, 61, 1)",
    "windowControlsLinux.hoverBackground": "rgba(209, 209, 209, 1)",
    "windowControlsLinux.activeBackground": "rgba(191, 191, 191, 1)",
  },
};

const themes: Theme[] = [defaultDarkTheme, defaultLightTheme, pinkTheme];

function ensureDirectoryExists(directory: string): void {
  if (!existsSync(directory)) {
    mkdirSync(directory, { recursive: true });
  }
}

async function writeThemeFile(theme: Theme): Promise<void> {
  const fileName = `${themesDirectory}/${theme.slug}.json`;

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
