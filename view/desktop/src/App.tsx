import { Settings, Content, TitleBar, Home, Menu, RootLayout, Sidebar } from "@/components";
import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { Icon, MenuItem, IconTitle } from "@repo/ui";
import { twMerge } from "tailwind-merge";
import StatusBar from "@/components/StatusBar";
import { exists, BaseDirectory, readTextFile, readDir } from "@tauri-apps/plugin-fs";
import { safeJsonParse } from "@/utils";

enum IconState {
  Default = "group-text-zinc-500",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-zinc-600",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-olive-700",
  Disabled = "text-zinc-500 bg-opacity-50",
}

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

function App() {
  return (
    <>
      <RootLayout>
        <TitleBar />
        <Sidebar className="p-0">
          <MenuItem className="group bg-zinc-200 mt-13 mb-3.5">
            <Icon icon="Search" className={twMerge("h-4.5 w-4.5", IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-xs bg-zin" title="Search..." />
            <Icon icon="SearchShortcut" className="fill-zinc-500  group-hover:fill-zinc-600 text-3xl ml-auto pr-2" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Home1" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Home" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Issues" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Issues" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Code" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Code" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Goals" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Goals" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Reports" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Reports" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Documentation" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Documentation" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="Settings" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Settings" />
          </MenuItem>

          <MenuItem className="group">
            <Icon icon="QuickSearch" className={twMerge(IconState.Default, IconState.Hover)} />
            <IconTitle className="text-zinc-900 text-sm" title="Quick Search" />
          </MenuItem>
        </Sidebar>

        <Content className="relative flex flex-col">
          <Suspense fallback="loading">
            <BrowserRouter>
              <Menu />
              <Routes>
                <Route path="/" element={<Home />} />
                <Route path="/settings" element={<Settings />} />
              </Routes>
            </BrowserRouter>
          </Suspense>
        </Content>
        <StatusBar
          className="absolute w-full bottom-0 h-5.5"
          branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri"
        />
      </RootLayout>
    </>
  );
}

export default App;
