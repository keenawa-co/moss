import { useEffect } from "react";

import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { LocaleDescriptor } from "@repo/moss-desktop";
import { useQueryClient } from "@tanstack/react-query";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const queryClient = useQueryClient();

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    const handleLanguageChange = (event: { payload: LocaleDescriptor }) => {
      applyLanguagePack(event.payload);
      queryClient.invalidateQueries({ queryKey: ["getState"] });
    };

    const setupListener = async () => {
      try {
        unlisten = await listen("core://language-pack-changed", handleLanguageChange);
      } catch (error) {
        console.error("Failed to set up language pack change listener:", error);
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

export default LanguageProvider;
