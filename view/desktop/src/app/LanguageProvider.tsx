import { useEffect } from "react";

import { useLanguageStore } from "@/store/language";
import { LocaleDescriptor } from "@repo/moss-desktop";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const { setLanguageCode, initializeLanguages } = useLanguageStore();

  useEffect(() => {
    initializeLanguages();
  }, [initializeLanguages]);

  useEffect(() => {
    let unlisten: UnlistenFn;

    const handleLanguagePackChanged = (event: { payload: LocaleDescriptor }) => {
      const newLanguagePack: LocaleDescriptor = event.payload;
      setLanguageCode(newLanguagePack.code);
    };

    const setupListener = async () => {
      try {
        unlisten = await listen("core://language-pack-changed", handleLanguagePackChanged);
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
  }, [setLanguageCode]);

  return <>{children}</>;
};

export default LanguageProvider;
