import { Settings, Content, TitleBar, Home, Menu, RootLayout, Sidebar, Logs } from "@/components";
import { Suspense, useEffect, useState } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "@/i18n";
import "@repo/ui/src/fonts.css";
import { twMerge } from "tailwind-merge";
import StatusBar from "@/components/StatusBar";
import { getTheme } from "@repo/ui";
import { Icon, MenuItem, IconTitle, ThemeProvider } from "@repo/ui";
import { THEMES } from "@/constants/index";
import { useTranslation } from "react-i18next";
import { Resizable, ResizablePanel } from "./components/Resizable";

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
  const [theme, setTheme] = useState<string>(() => {
    const savedTheme = localStorage.getItem("theme");
    return savedTheme && THEMES.includes(savedTheme) ? savedTheme : THEMES[0];
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
        setTheme(storedTheme);
      }
    });
  }, []);

  return (
    <ThemeProvider themeRGBOverrides={getTheme(theme)} updateRGBOnChange>
      <RootLayout>
        <Resizable proportionalLayout={false}>
          <ResizablePanel minSize={100} preferredSize={255} snap visible={sideBarVisible}>
            <Sidebar className="p-0 h-full w-full overflow-auto">
              <MenuItem className="group bg-zinc-200 mt-13 mb-3.5">
                <Icon icon="Search" className={twMerge("h-4.5 w-4.5 min-w-4", IconState.Default, IconState.Hover)} />
                <IconTitle className="text-primary text-xs" title="Search..." />
                <Icon
                  icon="SearchShortcut"
                  className="min-w-4  w-4.5 fill-zinc-500 group-hover:fill-zinc-600  ml-auto pr-2"
                />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Home1" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Home" />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Issues" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Issues" />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Code" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Code" />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Goals" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Goals" />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Reports" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Reports" />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Documentation" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle
                  className="text-primary text-sm"
                  title="Documentation with very long title to trigger overflow X"
                />
              </MenuItem>

              <MenuItem className="group">
                <Icon icon="Settings" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Settings" />
              </MenuItem>

<<<<<<< HEAD
              <MenuItem className="group">
                <Icon icon="QuickSearch" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                <IconTitle className="text-primary text-sm" title="Quick Search" />
              </MenuItem>
            </Sidebar>
          </ResizablePanel>
          <ResizablePanel>
            <Content className="content relative flex flex-col overflow-auto h-full">
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
          </ResizablePanel>
        </Resizable>
      </RootLayout>
    </ThemeProvider>
=======
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
                  <Route path="/logs" element={<Logs />} />
                </Routes>
              </BrowserRouter>
            </Suspense>
          </Content>

          <StatusBar
            className="absolute w-full bottom-0 h-5.5"
            branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri"
          />
        </RootLayout>
      </ThemeProvider>
    </>
>>>>>>> 0ef1d63c (feat: added temporary Logs page to the desktop app)
  );
}
export default App;
