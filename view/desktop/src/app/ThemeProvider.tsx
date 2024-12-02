import React, { useEffect } from "react";
import { useAtom } from "jotai";
import { useFetchThemes } from "@/hooks/useFetchThemes";
import { themeAtom } from "@/atoms/themeAtom";

import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { invoke } from "@tauri-apps/api/core";

interface ThemeProviderProps {
  children: React.ReactNode;
}

const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const { data: themes } = useFetchThemes();
  const [currentTheme, setCurrentTheme] = useAtom(themeAtom);

  const applyTheme = (cssContent: string) => {
    console.log("Applying Theme");
    let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;
    console.log(styleTag);
    if (styleTag) {
      styleTag.innerHTML = cssContent;
    } else {
      styleTag = document.createElement("style");
      styleTag.id = "theme-style";
      styleTag.innerHTML = cssContent;
      document.head.appendChild(styleTag);
    }
  };

  useEffect(() => {
    const appWebview = getCurrentWebviewWindow();
    appWebview.listen<string>("select_theme", (event) => {
      const selectedTheme = themes?.find((theme) => theme.id === event.payload) || null;
      console.log("select_theme event:", selectedTheme);
      setCurrentTheme(selectedTheme);
    });
    appWebview.listen<string>("apply_theme", (event) => {
      console.log("apply_theme event");
      console.log(event.payload);
      applyTheme(event.payload);
    });
  }, []);

  useEffect(() => {
    if (!currentTheme && themes && themes.length > 0) {
      invoke("get_selected_theme");
    }
  }, [themes]);

  return <>{children}</>;
};

export default ThemeProvider;
