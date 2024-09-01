import { exists, BaseDirectory, readTextFile, readDir } from "@tauri-apps/plugin-fs";

type Theme = {
  name: string;
  type: string;
  colors: {
    primary: string;
    secondary: string;
    bgPrimary: string;
  };
};

let themes = await readThemeDirectories();
if (themes.length > 0) {
  console.dir(themes);
}

async function readThemeDirectories(): Promise<Theme[]> {
  let addonsDirectory: string = "./.moss/addons";
  const baseDirectory = BaseDirectory.Home;

  const themes = new Array<Theme>();

  // Checking if ./.moss/addons dir exists
  if (
    !(await exists(addonsDirectory, {
      baseDir: BaseDirectory.Home,
    }))
  ) {
    return themes;
  }

  // Reading theme provider directories
  const themeProviderDirectories: Array<string> = (await readDir(addonsDirectory, { baseDir: baseDirectory }))
    .filter((entry) => entry.isDirectory)
    .map((entry) => `${addonsDirectory}/${entry.name}`);
  if (themeProviderDirectories.length === 0) {
    return themes;
  }

  // Reading theme directories
  let themeDirectories = new Array<string>();
  for (const providerDirectory of themeProviderDirectories) {
    if (
      !(await exists(providerDirectory, {
        baseDir: baseDirectory,
      }))
    ) {
      continue;
    }
    const themeProviderEntries = await readDir(providerDirectory, { baseDir: baseDirectory });
    for (const entry of themeProviderEntries) {
      if (entry.isDirectory) {
        themeDirectories.push(`${providerDirectory}/${entry.name}`);
      }
    }
  }

  // Reading theme files
  const themeFilePaths = new Array<string>();
  for (const themeDirectory of themeDirectories) {
    if (
      !(await exists(themeDirectory, {
        baseDir: baseDirectory,
      }))
    ) {
      continue;
    }
    const themeEntries = await readDir(themeDirectory, { baseDir: baseDirectory });
    for (const entry of themeEntries) {
      if (entry.isFile) {
        themeFilePaths.push(`${themeDirectory}/${entry.name}`);
      }
    }
  }
  // Parsing theme files
  for (const filePath of themeFilePaths) {
    const themeString = await readTextFile(filePath, {
      baseDir: baseDirectory,
    });
    const theme: Theme | undefined = safeJsonParse<Theme>(themeString);

    if (theme) {
      themes.push(theme);
    } else {
      // FIXME: replace with logging
      console.error("Failed to parse theme string");
    }
  }
  return themes;
}
