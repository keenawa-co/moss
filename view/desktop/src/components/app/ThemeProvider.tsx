import React, { useEffect } from "react";
import { useAtom } from "jotai";
import { useFetchThemes } from "@/hooks/useFetchThemes";
import { useChangeTheme } from "@/hooks/useChangeTheme";
import { themeAtom } from "@/atoms/themeAtom";
import { readThemeFile } from "@/api/appearance";
import { IpcResult } from "@/lib/backend/tauri";

interface ThemeProviderProps {
  children: React.ReactNode;
}

const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const { data: themes } = useFetchThemes();
  const { mutate: mutateChangeTheme } = useChangeTheme();
  const [currentTheme, setCurrentTheme] = useAtom(themeAtom);

  const setTheme = (themeId: string) => {
    mutateChangeTheme(themeId, {
      onSuccess: () => {
        const selectedTheme = themes?.find((theme) => theme.id === themeId) || null;
        setCurrentTheme(selectedTheme);
      },
      onError: (error: Error) => {
        console.error("Error while changing theme:", error);
      },
    });
  };

  useEffect(() => {
    const applyTheme = async () => {
      if (currentTheme) {
        const result: IpcResult<string, string> = await readThemeFile(currentTheme.source);

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
          console.error(`Error reading theme file for "${currentTheme.id}":`, result.error);
        }
      }
    };

    applyTheme();
  }, [currentTheme]);

  useEffect(() => {
    if (!currentTheme && themes && themes.length > 0) {
      setTheme(themes[0].id);
    }
  }, [currentTheme, themes]);

  return <>{children}</>;
};

export default ThemeProvider;
