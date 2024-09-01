import { BaseDirectory, exists, readTextFile, readDir } from "@tauri-apps/plugin-fs";
import { Convert, Theme } from "@repo/theme";

export async function readAllFilesInDirectory(baseDirectory: BaseDirectory, themePath: string): Promise<Array<Theme>> {
  const themes = new Array<Theme>();

  if (
    !(await exists(themePath, {
      baseDir: baseDirectory,
    }))
  ) {
    return themes;
  }

  const entries = await readDir(themePath, { baseDir: baseDirectory });
  for (const entry of entries) {
    if (entry.isFile && entry.name.endsWith(".json")) {
      const themeString = await readTextFile(`${themePath}/${entry.name}`, {
        baseDir: baseDirectory,
      });
      const theme = Convert.toTheme(themeString);
      themes.push(theme);
    } else {
      continue;
    }
  }

  return themes;
}
