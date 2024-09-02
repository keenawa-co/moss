import { Settings, Content, TitleBar, Home, Menu, RootLayout, Sidebar } from "@/components";
import { Suspense, useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { twMerge } from "tailwind-merge";
import StatusBar from "@/components/StatusBar";
import { Icon, MenuItem, IconTitle, ThemeProvider } from "@repo/ui";
import { useTranslation } from "react-i18next";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { readThemesFromFiles } from "@/utils";
import { BaseDirectory } from "@tauri-apps/plugin-fs";
import { Theme } from "@repo/theme";

enum IconState {
  Default = "group-text-primary",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-primary",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-primary",
  Disabled = "text-primary bg-opacity-50",
}

function App() {
  const [sideBarVisible, setSideBarVisibility] = useState(true);

  const { i18n } = useTranslation();
  const [themes, setThemes] = useState<Theme[]>([]);
  const [themeNames, setThemeNames] = useState<string[]>([]);

  useEffect(() => {
    async function fetchThemes() {
      const fetchedThemes = await readThemesFromFiles(BaseDirectory.Home, "./.moss/themes");
      setThemes(fetchedThemes);
      setThemeNames(fetchedThemes.map((theme) => theme.name));
    }
    fetchThemes();
  }, []);

  const [selectedTheme, setSelectedTheme] = useState<string>(() => {
    const savedTheme = localStorage.getItem("theme");
    return savedTheme && themeNames.includes(savedTheme) ? savedTheme : themeNames[0];
  });

  useEffect(() => {
    const setLanguageFromLocalStorage = () => {
      const savedLanguage = localStorage.getItem("language");
      if (savedLanguage) {
        i18n.changeLanguage(savedLanguage);
      }
    };
    setLanguageFromLocalStorage();

    window.addEventListener("storage", () => {
      const storedTheme = localStorage.getItem("theme");
      if (storedTheme) {
        setSelectedTheme(storedTheme);
      }
    });
  }, []);

  return (
    <ThemeProvider themeOverrides={themes.find((theme) => theme.name === selectedTheme)} updateRGBOnChange>
      <RootLayout>
        <TitleBar />

        <Resizable proportionalLayout={false}>
          <ResizablePanel minSize={150} preferredSize={255} snap visible={sideBarVisible}>
            <Sidebar className="p-0 h-full ">
              <MenuItem className="group bg-zinc-200 mt-13 mb-3.5 overflow-hidden">
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
          </ResizablePanel>
          <ResizablePanel>
            <Content className="relative flex flex-col">
              <Suspense fallback="loading">
                <BrowserRouter>
                  <Menu />
                  <Routes>
                    <Route path="/" element={<Home />} />
                    <Route path="/settings" element={<Settings themes={themeNames} />} />
                  </Routes>
                </BrowserRouter>
              </Suspense>
            </Content>
          </ResizablePanel>
        </Resizable>

        <StatusBar
          className="absolute w-full bottom-0 h-5.5"
          branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri"
        />
      </RootLayout>
    </ThemeProvider>
  );
}
export default App;
