import { ReactNode, useEffect } from "react";

import { applyTheme } from "@/utils/applyTheme";
import { ThemeDescriptor } from "@repo/moss-desktop";
import { useQueryClient } from "@tanstack/react-query";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const ThemeProvider = ({ children }: { children: ReactNode }) => {
  const queryClient = useQueryClient();

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleColorThemeChanged = (event: { payload: ThemeDescriptor }) => {
      applyTheme(event.payload);
      queryClient.invalidateQueries({ queryKey: ["getState"] });
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
  }, [queryClient]);

  return <>{children}</>;
};

export default ThemeProvider;
