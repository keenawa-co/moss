import { About, Content, DraggableTopBar, Home, Menu, Properties, RootLayout, Sidebar } from "@/components";
import { Suspense } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./i18n";
import "@shared/fonts.css";
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
  TempSearchComponent,
  SearchIcon,
  SearchShortcutIcon,
} from "../../shared/ui/src";
import StatusBar from "./components/StatusBar";

enum State {
  Default = "text-stone-500",
  Hover = "text-stone-600",
  Active = "text-olive-700",
  Disabled = "text-stone-500 bg-opacity-50",
}

function App() {
  return (
    <>
      <RootLayout>
        <Sidebar className="p-0">
          <MenuItem className="group bg-stone-200 mt-13 mb-3.5">
            <Icon className="h-4.5 w-4.5">
              <SearchIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-xs" title="Search..." />
            <Icon className="h-4.5 w-5 ml-28">
              <SearchShortcutIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <HomeIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-sm" title="Home" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <IssuesIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-sm" title="Issues" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <CodeIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-sm" title="Code" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <GoalsIcon className="stroke-stone-500 hover:stroke-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-sm" title="Goals" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <ReportsIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-sm" title="Reports" />
          </MenuItem>
          <MenuItem className="group">
            <Icon className="h-4.5 w-4.5">
              <DocumentationIcon className="text-stone-500 hover:text-stone-600" />
            </Icon>
            <IconTitle className="text-stone-900 text-sm" title="Documentation" />
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

          <StatusBar
            className="sticky bottom-0 mt-auto"
            branch="MOSSMVP-37-Backend-Migrate-existing-backend-in-Tauri"
          />
        </Content>
      </RootLayout>
    </>
  );
}

export default App;
