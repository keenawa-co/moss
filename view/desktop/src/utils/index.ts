import clsx, { ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";
import { IThemeRGB, lightTheme, darkTheme, testTheme } from "@repo/ui";

/*
const dateFormatter = new Intl.DateTimeFormat(window.context.locale, {
  dateStyle: "short",
  timeStyle: "short",
  timeZone: "UTC",
});

export const formatDateFromMs = (ms: number) => dateFormatter.format(ms);
*/

export const cn = (...args: ClassValue[]) => {
  return twMerge(clsx(...args));
};

export const safeJsonParse = <T>(str: string): T | undefined => {
  try {
    const jsonValue: T = JSON.parse(str);
    return jsonValue;
  } catch {
    return undefined;
  }
};

export function getTheme(theme: string): IThemeRGB {
  switch (theme) {
    case "light":
      return lightTheme;
    case "dark":
      return darkTheme;
    case "test":
      return testTheme;
    default:
      return lightTheme;
  }
}
