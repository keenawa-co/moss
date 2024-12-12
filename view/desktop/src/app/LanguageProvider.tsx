import { useCallback, useEffect } from "react";

import { LanguagePack, useLanguageStore } from "@/store/language";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const LanguageProvider = ({ children }: { children: React.ReactNode }) => {
  const { currentLanguageCode, languagePacks, setLanguageCode, initializeLanguages } = useLanguageStore();

  useEffect(() => {
    initializeLanguages();
  }, [initializeLanguages]);

  const setLanguagePackWithoutSync = useCallback(
    (language: LanguagePack) => {
      const selectedLanguage = languagePacks?.find((languagePack) => language.code === languagePack.code) || null;
      if (selectedLanguage) {
        setLanguageCode(selectedLanguage.code);
      } else {
        console.error(`Language pack with code "${language.code}" not found.`);
      }
    },
    [languagePacks, setLanguageCode]
  );

  useEffect(() => {
    let unlisten: UnlistenFn;

    const handleLanguagePackChanged = (event: { payload: LanguagePack }) => {
      const newLanguagePack: LanguagePack = event.payload;

      if (newLanguagePack.code !== currentLanguageCode) {
        setLanguagePackWithoutSync(newLanguagePack);
      }
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
  }, [currentLanguageCode, setLanguagePackWithoutSync]);

  return <>{children}</>;
};

export default LanguageProvider;
