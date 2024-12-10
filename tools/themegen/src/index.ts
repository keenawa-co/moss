import { existsSync, mkdirSync, writeFileSync } from "fs";
import * as os from "os";

import { Theme } from "@repo/desktop-models";
import { defaultDarkTheme } from "./themes/moss-dark.ts";
import { defaultLightTheme } from "./themes/moss-light.ts";
import { pinkTheme } from "./themes/moss-pink.ts";

// FIXME: temporary solution. Also should be fixed in packages/config-eslint/moss-lint-plugin/rules/validate-token-names.js
const homeDirectory = os.homedir();
const themesDirectory = `${homeDirectory}/.config/moss/themes`;

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
