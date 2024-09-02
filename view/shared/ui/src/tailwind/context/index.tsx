import React, { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";
import { tailwindColorTheme } from "../theme/light/colors";
import { IThemeColors, IThemeRGB } from "../types";
import applyTheme from "./applyTheme";
import { rgbToHex } from "./utils";
import { readThemesFromFiles } from "../readThemes";
import { BaseDirectory } from "@tauri-apps/plugin-fs";
import { Theme } from "@repo/theme";

type Props = {
  children: React.ReactNode;
  themeRGBOverrides?: IThemeRGB;
  updateRGBOnChange?: boolean;
};

type ThemeContextType = {
  themeRGB: IThemeRGB;
  convertThemeRGBToHex: () => IThemeColors;
};

const ThemeContext = createContext<ThemeContextType>({
  themeRGB: tailwindColorTheme,
  convertThemeRGBToHex: () => {
    return {};
  },
});

// ThemedProvider applies tailwind theme default, potentially with provided
// overrides. Must wrap your app root in this provider to use this library.
export default function ThemeProvider(props: Props) {
  const [themes, setThemes] = useState<Theme[]>([]);

  useEffect(() => {
    async function fetchThemes() {
      const fetchedThemes = await readThemesFromFiles(BaseDirectory.Resource, "themes");
      setThemes(fetchedThemes);
    }
    fetchThemes();
  }, []);

  useEffect(() => {
    for (const theme of themes) {
      console.warn("-------545454----->>>>>");
      console.warn(theme);
    }
  }, [themes]);

  const themeRGB: IThemeRGB = useMemo(() => {
    return {
      ...tailwindColorTheme,
      ...(props.themeRGBOverrides ?? {}),
    };
  }, [props.themeRGBOverrides]);

  useEffect(
    () => {
      applyTheme(themeRGB);
    },
    // Must include `themeRGB` in the dependencies array for the storybook theme
    // toggle to work
    props.updateRGBOnChange ? [themeRGB] : []
  );

  const convertThemeRGBToHex = useCallback((): IThemeColors => {
    const hexThemeRGB: IThemeColors = {};
    Object.keys(themeRGB).forEach((key) => {
      const rgb = themeRGB[key as keyof IThemeRGB];
      if (rgb) {
        const color = key.replace("rgb-", "");
        hexThemeRGB[color as keyof IThemeColors] = rgbToHex(rgb);
      }
    });
    return hexThemeRGB;
  }, [themeRGB]);

  const value = useMemo(() => {
    return { themeRGB, convertThemeRGBToHex };
  }, [themeRGB, convertThemeRGBToHex]);

  return <ThemeContext.Provider value={value}>{props.children}</ThemeContext.Provider>;
}
export function useThemeContext(): ThemeContextType {
  return useContext(ThemeContext);
}
