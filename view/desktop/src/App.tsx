import { Settings, Content, TitleBar, Home, Menu, RootLayout, Sidebar } from "@/components";
import ThemeProvider from "@/components/tailwind/context";
import { Suspense, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { Icon, MenuItem, IconTitle } from "@repo/ui";
import { twMerge } from "tailwind-merge";
import StatusBar from "@/components/StatusBar";

import { IThemeRGB } from "@/components/tailwind/types";
import { tailwindColorTheme as lightTheme } from "@/components/tailwind/theme/light/colors";
import { tailwindColorTheme as darkTheme } from "@/components/tailwind/theme/dark/colors";
import { tailwindColorTheme as testTheme } from "@/components/tailwind/theme/test/colors";

enum IconState {
  Default = "group-text-primary",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-primary",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-primary",
  Disabled = "text-primary bg-opacity-50",
}

function App() {
  function getTheme(theme: string): IThemeRGB {
    switch (theme) {
      case "light":
        return lightTheme;
      case "dark":
        return darkTheme;
      case "test":
        return testTheme;
      default:
        return lightTheme;
    }
  }

  const themes = ["light", "dark", "test"];
  const [theme, setTheme] = useState<string>(themes[0]);

  const onChangeLTheme = (e: React.ChangeEvent<HTMLSelectElement>) => {
    const newTheme = e.target.value;
    setTheme(newTheme);
  };

  console.log("---------------->" + theme);

  return (
    <>
      <ThemeProvider themeRGBOverrides={getTheme(theme)} updateRGBOnChange>
        <RootLayout>
          <TitleBar />

          <Sidebar className="p-0">
            <MenuItem className="group bg-zinc-200 mt-13 mb-3.5">
              <Icon icon="Search" className={twMerge("h-4.5 w-4.5", IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-xs" title="Search..." />
              <Icon icon="SearchShortcut" className="fill-zinc-500 group-hover:fill-zinc-600 text-3xl ml-auto pr-2" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Home1" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Home" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Issues" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Issues" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Code" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Code" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Goals" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Goals" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Reports" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Reports" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Documentation" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Documentation" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="Settings" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Settings" />
            </MenuItem>

            <MenuItem className="group">
              <Icon icon="QuickSearch" className={twMerge(IconState.Default, IconState.Hover)} />
              <IconTitle className="text-primary text-sm" title="Quick Search" />
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
            <div>
              <select className="bg-pink-300 text-primary" defaultValue={themes[0]} onChange={onChangeLTheme}>
                {themes.map((t) => (
                  <option key={t} value={t}>
                    {t}
                  </option>
                ))}
              </select>
            </div>
          </Content>

          <StatusBar
            className="absolute w-full bottom-0 h-5.5"
            branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri"
          />
        </RootLayout>
      </ThemeProvider>
    </>
  );
}

export default App;
