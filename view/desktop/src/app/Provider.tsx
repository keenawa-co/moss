import { ReactNode, useEffect } from "react";

import { useGetAppState } from "@/hooks/useGetAppState";
import { applyLanguagePack } from "@/utils/applyLanguagePack";
import { applyTheme } from "@/utils/applyTheme";

import LanguageProvider from "./LanguageProvider";
import ThemeProvider from "./ThemeProvider";

const Provider = ({ children }: { children: ReactNode }) => {
  useInitializeAppState();

  return (
    <LanguageProvider>
      <ThemeProvider>{children}</ThemeProvider>
    </LanguageProvider>
  );
};

const useInitializeAppState = () => {
  const { data } = useGetAppState();

  useEffect(() => {
    if (data) {
      const theme = data.preferences?.theme ?? data.defaults.theme;
      const languagePack = data.preferences?.locale ?? data.defaults.locale;

      applyTheme(theme);
      applyLanguagePack(languagePack);
    }
  }, [data]);
};

export default Provider;
