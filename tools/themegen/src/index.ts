import { existsSync, mkdirSync, writeFileSync } from "fs";
import * as os from "os";

import { Theme } from "@repo/desktop-models";

// FIXME: temporary solution. Also should be fixed in packages/config-eslint/moss-lint-plugin/rules/validate-token-names.js
const homeDirectory = os.homedir();
const themesDirectory = `${homeDirectory}/.config/moss/themes`;

// Default
const defaultDarkTheme: Theme = {
  name: "Moss Dark Default",
  slug: "moss-dark",
  type: "dark",
  isDefault: false,
  colors: {
    "primary": { "type": "solid", "value": "rgba(255, 255, 255, 1)" }, // prettier-ignore
    "sideBar.background": { type: "solid", value: "rgba(39, 39, 42, 1)" },
    "toolBar.background": { type: "solid", value: "rgba(30, 32, 33, 1)" },
    "page.background": { type: "solid", value: "rgba(22, 24, 25, 1)" },
    "statusBar.background": { type: "solid", value: "#0F62FE" },
    "windowsCloseButton.background": { type: "solid", value: "rgba(196, 43, 28, 1)" },
    "windowControlsLinux.background": { type: "solid", value: "rgba(55, 55, 55, 1)" },
    "windowControlsLinux.text": { type: "solid", value: "rgba(255, 255, 255, 1)" },
    "windowControlsLinux.hoverBackground": { type: "solid", value: "rgba(66, 66, 66, 1)" },
    "windowControlsLinux.activeBackground": { type: "solid", value: "rgba(86, 86, 86, 1)" },
  },
};

const defaultLightTheme: Theme = {
  name: "Moss Light Default",
  slug: "moss-light",
  type: "light",
  isDefault: true,
  colors: {
    "primary": { "type": "solid", "value": "rgba(0, 0, 0, 1)" }, // prettier-ignore
    "sideBar.background": { type: "solid", value: "rgba(244, 244, 245, 1)" },
    "toolBar.background": { type: "solid", value: "rgba(224, 224, 224, 1)" },
    "page.background": { type: "solid", value: "rgba(255, 255, 255, 1)" },
    "statusBar.background": { type: "solid", value: "#0F62FE" },
    "windowsCloseButton.background": { type: "solid", value: "rgba(196, 43, 28, 1)" },
    "windowControlsLinux.background": { type: "solid", value: "rgba(218, 218, 218, 1)" },
    "windowControlsLinux.text": { type: "solid", value: "rgba(61, 61, 61, 1)" },
    "windowControlsLinux.hoverBackground": { type: "solid", value: "rgba(209, 209, 209, 1)" },
    "windowControlsLinux.activeBackground": { type: "solid", value: "rgba(191, 191, 191, 1)" },
  },
};

// Other

const pinkTheme: Theme = {
  name: "Moss Pink",
  slug: "moss-pink",
  type: "light",
  isDefault: false,
  colors: {
    "primary": { "type": "solid", "value": "rgba(0, 0, 0, 1)" }, // prettier-ignore
    "sideBar.background": { type: "solid", value: "rgba(234, 157, 242, 1)" },
    "toolBar.background": { type: "solid", value: "rgba(222, 125, 232, 1)" },
    "page.background": { type: "solid", value: "rgba(227, 54, 245, 1)" },
    "statusBar.background": {
      type: "gradient",
      value:
        "linear-gradient(90deg, rgba(255, 0, 0, 1) 0%, rgba(224, 0, 41, 1) 16%, rgba(190, 0, 87, 1) 34%, rgba(155, 0, 133, 1) 51%, rgba(63, 0, 255, 1) 100%)",
    },
    "windowsCloseButton.background": { type: "solid", value: "rgba(196, 43, 28, 1)" },
    "windowControlsLinux.background": { type: "solid", value: "rgba(218, 218, 218, 1)" },
    "windowControlsLinux.text": { type: "solid", value: "rgba(61, 61, 61, 1)" },
    "windowControlsLinux.hoverBackground": { type: "solid", value: "rgba(209, 209, 209, 1)" },
    "windowControlsLinux.activeBackground": { type: "solid", value: "rgba(191, 191, 191, 1)" },
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

  writeFileSync(fileName, JSON.stringify(theme, null, 2), {
    flag: "w",
  });
}

(async () => {
  try {
    ensureDirectoryExists(themesDirectory);
    await Promise.all(themes.map(writeThemeFile));
    console.log("Theme files generated successfully.");
  } catch (error) {
    console.error("Error generating theme files:", error);
    process.exit(1);
  }
})();
