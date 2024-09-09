import { commands } from "@/bindings";
import { Content, RootLayout, Sidebar } from "@/components";
import "@/i18n";
import { Convert, Theme } from "@repo/theme";
import { Icon, IconTitle, MenuItem, ThemeProvider } from "@repo/ui";
import "@repo/ui/src/fonts.css";
import {
  DockviewApi,
  DockviewReact,
  DockviewReadyEvent,
  IDockviewPanelHeaderProps,
  IDockviewPanelProps,
} from "dockview";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { twMerge } from "tailwind-merge";
import { Resizable, ResizablePanel } from "./components/Resizable";
import { usePanelApi } from "./hooks/usePanelApi";
import * as PagesComponents from "./pages/index";
import { cn } from "./utils";

const handleFetchAllThemes = async () => {
  try {
    let response = await commands.fetchAllThemes();
    if (response.status === "ok") {
      return response.data;
    }
    throw new Error("Failed to fetch themes: Invalid response status");
  } catch (error) {
    console.error("Failed to fetch themes:", error);
    throw error;
  }
};

const handleReadTheme = async (themeName: string): Promise<Theme> => {
  try {
    let response = await commands.readTheme(themeName);
    if (response.status === "ok") {
      return Convert.toTheme(response.data);
    }
    throw new Error("Failed to read theme: Invalid response status");
  } catch (error) {
    console.error("Failed to read theme:", error);
    throw error;
  }
};

enum IconState {
  Default = "group-text-primary",
  DefaultStroke = "group-stroke-zinc-500",
  Hover = "group-hover:text-primary",
  HoverStroke = "group-hover:stroke-zinc-600",
  Active = "text-primary",
  Disabled = "text-primary bg-opacity-50",
}

// DOCKVIEW
// default
const DefaultPanel = (props: IDockviewPanelProps) => (
  <div className={cn(`h-full grid place-items-center text-3xl`)}>{props.api.title}</div>
);

const WatermarkPanel = () => (
  <div className="h-full w-full grid place-items-center bg-white">
    <div>
      <div className="text-center text-5xl font-bold">No content chosen</div>
      <img className="w-80 mx-auto" src="https://media.tenor.com/OA8KFcZxPjsAAAAi/sad-emoji.gif" alt="" />
    </div>
  </div>
);

// custom
const CustomTab = (props: IDockviewPanelHeaderProps) => {
  // const metadata = usePanelApi(props.api);

  // useEffect(() => {
  //   // console.log("custom tab  metadata", metadata);
  // }, [metadata]);

  return (
    <div
      className={cn(`flex items-center justify-between px-4 h-full`, {})}
      onClick={(event) => {
        // on mouse middle click
        if (event.button === 1) props.api.close();
      }}
    >
      <div className="text-primary">{props.api.title}</div>

      <div className="ml-4 px-1 hover:bg-red-500 rounded-full" onClick={() => props.api.close()}>
        X
      </div>
    </div>
  );
};

const CustomPanel = (props: IDockviewPanelProps) => {
  const metadata = usePanelApi(props.api);

  useEffect(() => {
    // console.log("custom panel metadata", metadata);
  }, [metadata]);

  return (
    <div
      className={cn(`h-full flex flex-col justify-center items-center`, {
        "bg-green-300": props.api.isActive,
        "bg-red-300": !props.api.isActive,
      })}
    >
      <div className="text-3xl font-bold mb-12">{props.api.isActive ? "Active now" : "Inactive"}</div>
    </div>
  );
};

function App() {
  const [sideBarVisible] = useState(true);
  const { i18n } = useTranslation();
  const [themes, setThemes] = useState<string[]>([]);
  const [selectedTheme, setSelectedTheme] = useState<Theme | undefined>(undefined);

  // Initialize theme
  useEffect(() => {
    const initializeThemes = async () => {
      try {
        const allThemes = await handleFetchAllThemes();
        if (allThemes) setThemes(allThemes);

        const savedThemeName = localStorage.getItem("theme");
        let themeToUse: Theme | undefined;

        if (savedThemeName) themeToUse = await handleReadTheme(savedThemeName);

        if (themeToUse) {
          setSelectedTheme(themeToUse);
        } else {
          localStorage.setItem("theme", themes[0]);
          setSelectedTheme(await handleReadTheme(themes[0]));
        }
      } catch (error) {
        console.error("Failed to initialize themes:", error);
      }
    };
    initializeThemes();
  }, []);

  // Initialize language
  useEffect(() => {
    const setLanguageFromLocalStorage = () => {
      const savedLanguage = localStorage.getItem("language");
      if (savedLanguage) i18n.changeLanguage(savedLanguage);
    };
    setLanguageFromLocalStorage();
  }, [i18n]);

  // Handle theme change
  useEffect(() => {
    const handleStorageChange = async () => {
      const storedTheme = localStorage.getItem("theme");
      if (storedTheme) setSelectedTheme(await handleReadTheme(storedTheme));
    };

    window.addEventListener("storage", handleStorageChange);
    return () => {
      window.removeEventListener("storage", handleStorageChange);
    };
  }, [themes]);

  useEffect(() => {
    if (!selectedTheme) console.error("Failed to initialize theme");
  }, [selectedTheme]);

  // DOCKVIEW
  const [dockviewApi, setDockviewApi] = useState<DockviewApi | null>(null);

  const onReady = (event: DockviewReadyEvent) => {
    setDockviewApi(event.api);
  };

  useEffect(() => {
    dockviewApi?.clear();
    addCustomTab("HomePage");
    addCustomTab("SettingsPage");
    addCustomTab("LogsPage");
  }, [dockviewApi]);

  // registration
  const panels = {
    default: DefaultPanel,
    watermark: WatermarkPanel,
    custom: CustomPanel,
    ...PagesComponents,
  };

  type PanelNames = keyof typeof panels;

  const tabs = {
    custom: CustomTab,
  };

  // actions
  const addDefaultTab = (panel?: PanelNames) => {
    dockviewApi?.addPanel({
      id: `id_${Math.random()}-default-${panel}`,
      title: panel || "Default",
      component: panel || "default",
    });
  };

  const addCustomTab = (panel?: PanelNames) => {
    dockviewApi?.addPanel({
      id: `id_${Math.random()}-custom-${panel}`,
      title: panel || "Custom",
      component: panel || "custom",
      tabComponent: "custom",
    });
  };

  return (
    <>
      {!selectedTheme ? (
        <div className="relative min-h-screen flex bg-storm-800">
          <div className="container max-w-screen-xl mx-auto flex justify-center items-center text-4xl text-white">
            Loading...
          </div>
        </div>
      ) : (
        <ThemeProvider themeOverrides={selectedTheme} updateOnChange>
          <RootLayout>
            <Resizable proportionalLayout={false}>
              <ResizablePanel minSize={100} preferredSize={255} snap visible={sideBarVisible}>
                <Sidebar className="p-0 h-full w-full overflow-auto">
                  <MenuItem className="group bg-zinc-200 mt-13 mb-3.5">
                    <Icon
                      icon="Search"
                      className={twMerge("h-4.5 w-4.5 min-w-4", IconState.Default, IconState.Hover)}
                    />
                    <IconTitle className="text-primary text-xs" title="Search..." />
                    <Icon
                      icon="SearchShortcut"
                      className="min-w-4  w-4.5 fill-zinc-500 group-hover:fill-zinc-600  ml-auto pr-2"
                    />
                  </MenuItem>

                  <MenuItem className="group" onClick={() => addDefaultTab("HomePage")}>
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

                  <MenuItem className="group" onClick={() => addDefaultTab("LogsPage")}>
                    <Icon icon="Reports" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                    <IconTitle className="text-primary text-sm" title="Logs" />
                  </MenuItem>

                  <MenuItem className="group">
                    <Icon icon="Documentation" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                    <IconTitle
                      className="text-primary text-sm"
                      title="Documentation with very long title to trigger overflow X"
                    />
                  </MenuItem>

                  <MenuItem className="group" onClick={() => addDefaultTab("SettingsPage")}>
                    <Icon icon="Settings" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                    <IconTitle className="text-primary text-sm" title="Settings" />
                  </MenuItem>

                  <MenuItem className="group">
                    <Icon icon="QuickSearch" className={twMerge(IconState.Default, IconState.Hover, "min-w-4")} />
                    <IconTitle className="text-primary text-sm" title="Quick Search" />
                  </MenuItem>
                </Sidebar>
              </ResizablePanel>
              <ResizablePanel>
                <Content className="content relative flex flex-col overflow-auto h-full">
                  <DockviewReact
                    onReady={onReady}
                    components={panels}
                    tabComponents={tabs}
                    watermarkComponent={panels.watermark}
                  />
                </Content>
              </ResizablePanel>
            </Resizable>
          </RootLayout>
        </ThemeProvider>
      )}
    </>
  );
}
export default App;
