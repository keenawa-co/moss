import React, { useCallback, useEffect } from "react";

import { getColorTheme } from "@/api/appearance";
import { useGetColorThemes } from "@/hooks/useGetColorThemes";
import { IpcResult } from "@/lib/backend/tauri";
import { useThemeStore } from "@/store/theme";
import { ThemeDescriptor } from "@repo/desktop-models";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface ThemeProviderProps {
  children: React.ReactNode;
}

const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const { data: themes, isLoading: themesLoading, error: themesError } = useGetColorThemes();

  const { currentTheme, setCurrentTheme } = useThemeStore();

  const applyThemeCSS = useCallback(async (theme: ThemeDescriptor | null) => {
    if (!theme) {
      console.warn("No theme to apply.");
      return;
    }

    try {
      const result: IpcResult<string, string> = await getColorTheme(theme.source);

      if (result.status === "ok") {
        const cssContent = result.data;
        let styleTag = document.getElementById("theme-style") as HTMLStyleElement | null;

        if (styleTag) {
          styleTag.innerHTML = cssContent;
        } else {
          styleTag = document.createElement("style");
          styleTag.id = "theme-style";
          styleTag.innerHTML = cssContent;
          document.head.appendChild(styleTag);
        }
      } else {
        console.error(`Error reading theme file for "${theme.id}":`, result.error);
      }
    } catch (error) {
      console.error(`Failed to apply theme "${theme.id}":`, error);
    }
  }, []);

  const setThemeWithoutSync = useCallback(
    (themeDescriptor: ThemeDescriptor) => {
      const selectedTheme = themes?.find((theme) => theme.id === themeDescriptor.id) || null;
      if (selectedTheme) {
        setCurrentTheme(selectedTheme);
      } else {
        console.error(`Theme with id "${themeDescriptor.id}" not found.`);
      }
    },
    [themes, setCurrentTheme]
  );

  useEffect(() => {
    applyThemeCSS(currentTheme);
  }, [currentTheme, applyThemeCSS]);

  useEffect(() => {
    if (!currentTheme && themes && themes.length > 0 && !themesLoading) {
      setThemeWithoutSync(themes[0]);
    }
  }, [currentTheme, themes, setThemeWithoutSync, themesLoading]);

  useEffect(() => {
    let unlisten: UnlistenFn;

    const handleColorThemeChanged = (event: { payload: ThemeDescriptor }) => {
      const newThemeDescriptor: ThemeDescriptor = event.payload;

      if (newThemeDescriptor.id !== currentTheme?.id) {
        setThemeWithoutSync(newThemeDescriptor);
      }
    };

    const setupListener = async () => {
      try {
        unlisten = await listen("core://color-theme-changed", handleColorThemeChanged);
      } catch (error) {
        console.error("Failed to set up theme change listener:", error);
      }
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, [currentTheme, setThemeWithoutSync]);

  useEffect(() => {
    if (themesError) {
      console.error("Error loading themes:", themesError);
    }
  }, [themesError]);

  return <>{children}</>;
};

export default ThemeProvider;
