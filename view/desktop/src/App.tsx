import { About, Content, TitleBar, Home, Menu, RootLayout, Sidebar } from "@/components";
import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import "./i18n";
import "../../shared/ui/src/fonts.css";
import { Icon, MenuItem, IconTitle } from "../../shared/ui/src";
import { twMerge } from "tailwind-merge";
import StatusBar from "./components/StatusBar";

enum IconState {
  Default = "group-text-zinc-500",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-zinc-600",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-olive-700",
  Disabled = "text-zinc-500 bg-opacity-50",
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
                <Route path="/about" element={<About />} />
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
