import clsx, { ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { BaseDirectory, exists, readTextFile, readDir } from "@tauri-apps/plugin-fs";
import { Convert, Theme } from "@repo/theme";

export const cn = (...args: ClassValue[]) => {
  return twMerge(clsx(...args));
};

export async function readThemesFromFiles(baseDirectory: BaseDirectory, themePath: string): Promise<Array<Theme>> {
  const themes = new Array<Theme>();

  if (
    !(await exists(themePath, {
      baseDir: baseDirectory,
    }))
  ) {
    return themes;
  }

  const entries = await readDir(themePath, { baseDir: baseDirectory });
  console.warn(3333333333333333333333333);
  for (const entry of entries) {
    console.warn(231312312321312313);
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
