import { promises as fs } from "fs";
import { ThemeOptions, getTheme } from "./theme";

const themes: ThemeOptions[] = [
  { theme: "light", name: "Moss Light Default" },
  { theme: "dark", name: "Moss Dark Default" },
  { theme: "pink", name: "Moss Pink Default" },
];

const generateThemes = themes.map((options) => ({
  ...options,
  theme: getTheme(options),
}));

fs.mkdir("./themes", { recursive: true })
  .then(() =>
    Promise.all(
      generateThemes.map((theme) =>
        fs.writeFile(`./themes/moss-${theme.theme}-default.json`, JSON.stringify(theme.theme, null, 2))
      )
    )
  )
  .catch(() => process.exit(1));
