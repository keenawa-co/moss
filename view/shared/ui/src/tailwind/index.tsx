import React, { createContext, useContext, useEffect, useMemo } from "react";
import applyTheme from "./applyTheme";
import { Theme } from "@repo/moss-models";

type Props = {
  children: React.ReactNode;
  themeOverrides?: Theme;
  updateOnChange?: boolean;
};

type ThemeContextType = {
  theme: Theme;
};

const ThemeContext = createContext<ThemeContextType>({
  theme: {} as Theme,
});

export default function ThemeProvider(props: Props) {
  const theme: Theme = useMemo(() => {
    return {
      ...(props.themeOverrides ?? ({} as Theme)),
    };
  }, [props.themeOverrides]);

  useEffect(
    () => {
      applyTheme(theme);
    },
    // Must include `theme` in the dependencies array for the storybook theme toggle to work
    props.updateOnChange ? [theme] : []
  );

  const value = useMemo(() => {
    return { theme };
  }, [theme]);

  return <ThemeContext.Provider value={value}>{props.children}</ThemeContext.Provider>;
}
export function useThemeContext(): ThemeContextType {
  return useContext(ThemeContext);
}
