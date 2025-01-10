import { mkdirSync, writeFileSync } from "fs";

import { defaultDarkTheme } from "./themes/dark";
import { defaultLightTheme } from "./themes/light";

const THEMES_DIR = "./themes";

(async () => {
  try {
    mkdirSync(THEMES_DIR, { recursive: true });

    await Promise.all([
      writeFileSync(`${THEMES_DIR}/light-default.json`, JSON.stringify(defaultLightTheme, null, 2), { flag: "w" }),
      writeFileSync(`${THEMES_DIR}/dark-default.json`, JSON.stringify(defaultDarkTheme, null, 2), { flag: "w" }),
    ]);
  } catch (err) {
    console.error("Error generating themes:", err);
    process.exit(1);
  }
})();
