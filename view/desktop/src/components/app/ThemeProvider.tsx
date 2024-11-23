import React, { useEffect } from "react";
import { useAtom } from "jotai";
import { useFetchThemes } from "@/hooks/useFetchThemes";
import { useChangeTheme } from "@/hooks/useChangeTheme";
import { themeAtom } from "@/atoms/themeAtom";

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
    if (currentTheme) {
      console.log(`Current theme: ${currentTheme.name}`);
      let themeLink = document.getElementById("theme-link") as HTMLLinkElement | null;

      if (themeLink) {
        console.log(`Updating theme to "${currentTheme.name}"`);
        themeLink.href = currentTheme.source;
      } else {
        console.log(`Loading theme "${currentTheme.name}"`);
        themeLink = document.createElement("link");
        themeLink.rel = "stylesheet";
        themeLink.id = "theme-link";
        themeLink.href = currentTheme.source;
        document.head.appendChild(themeLink);
      }
    }
  }, [currentTheme, themes, setTheme]);

  useEffect(() => {
    if (!currentTheme && themes && themes.length > 0) {
      setTheme(themes[0].id);
    }
  }, [currentTheme, themes, setTheme]);

  return <>{children}</>;
};

export default ThemeProvider;
