import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { IThemeRGB, lightTheme, darkTheme, testTheme } from "@repo/ui";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function getTheme(theme: string): IThemeRGB {
  const themeMap: Record<string, IThemeRGB> = {
    light: lightTheme,
    dark: darkTheme,
    test: testTheme,
  };

  return themeMap[theme] || lightTheme;
}
