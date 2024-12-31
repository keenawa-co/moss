import { useEffect } from "react";

import { getState } from "@/api/appearance";
import { useLanguageStore } from "@/store/language";
import { useThemeStore } from "@/store/theme";

export const useInitializeAppState = () => {
  const { setCurrentTheme } = useThemeStore();
  const { setLanguageCode } = useLanguageStore();

  useEffect(() => {
    const fetchAndSetAppState = async () => {
      try {
        const { preferences } = await getState();
        setCurrentTheme(preferences.theme);
        setLanguageCode(preferences.locale.code);
      } catch (error) {
        console.error("Failed to fetch app state from backend:", error);
      }
    };

    fetchAndSetAppState();
  }, [setCurrentTheme, setLanguageCode]);
};
