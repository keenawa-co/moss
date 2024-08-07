import {
  About,
  Content,
  DraggableTitleBar,
  Home,
  Menu,
  Properties,
  RootLayout,
  Sidebar,
  WindowControls,
} from "@/components";
import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./i18n";
import "@/shared/fonts.css";
import {
  HomeIcon,
  Icon,
  MenuItem,
  IconTitle,
  IssuesIcon,
  CodeIcon,
  GoalsIcon,
  ReportsIcon,
  DocumentationIcon,
  SearchIcon,
  SearchShortcutIcon,
  SettingsIcon,
  QuickSearchIcon,
} from "../../shared/ui/src";
import { twMerge } from "tailwind-merge";
import StatusBar from "./components/StatusBar";
import { useOperatingSystem } from "./hooks/useOperatingSystem";

enum IconState {
  Default = "text-zinc-500",
  DefaultStroke = "stroke-zinc-500",
  Hover = "hover:text-zinc-600",
  HoverStroke = "hover:stroke-zinc-600",
  Active = "text-olive-700",
  Disabled = "text-zinc-500 bg-opacity-50",
}

const os = "macOS"; //useOperatingSystem();
//const showControls = true; //window.location.search.includes("showControls");

let isMacOs = os == "macOS";

console.log("os------------------->" + os);
//console.log("showControls------------------->" + showControls);

function App() {
  return (
    <>
      <RootLayout>
        <DraggableTitleBar />
        <Sidebar className="p-0">
          <MenuItem className="group bg-zinc-200 mt-13 mb-3.5">
            <Icon className="h-4.5 w-4.5">
              <SearchIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-xs bg-zin" title="Search..." />
            <Icon className="h-4.5 w-5 ml-28">
              <SearchShortcutIcon className="text-zinc-500 hover:text-zinc-600" />
            </Icon>
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <HomeIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Home" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <IssuesIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Issues" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <CodeIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Code" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <GoalsIcon className={twMerge(IconState.DefaultStroke, IconState.HoverStroke)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Goals" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <ReportsIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Reports" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <DocumentationIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Documentation" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <SettingsIcon className={twMerge(IconState.Default, IconState.Hover)} />
            </Icon>
            <IconTitle className="text-zinc-900 text-sm" title="Settings" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <QuickSearchIcon className="stroke-zinc-500 hover:stroke-zinc-600" />
            </Icon>
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
          <WindowControls />
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
