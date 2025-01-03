import { ReactNode, useCallback, useEffect } from "react";

import { useGetColorThemes } from "@/hooks/useGetColorThemes";
import { useThemeStore } from "@/store/theme";
import { ThemeDescriptor } from "@repo/moss-desktop";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface ThemeProviderProps {
  children: ReactNode;
}

const ThemeProvider = ({ children }: ThemeProviderProps) => {
  const { data: newThemes } = useGetColorThemes();
  const { setThemes, setCurrentTheme } = useThemeStore();

  useEffect(() => {
    if (newThemes) {
      setThemes(newThemes);
    }
  }, [newThemes, setThemes]);

  const handleColorThemeChanged = useCallback(
    (event: { payload: ThemeDescriptor }) => {
      // console.log("handleColorThemeChanged");
      const { payload: newThemeDescriptor } = event;

      setCurrentTheme(newThemeDescriptor);
    },
    [setCurrentTheme]
  );

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    // console.log("useEffect handleColorThemeChanged");

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
  }, [handleColorThemeChanged]);

  return <>{children}</>;
};

export default ThemeProvider;
