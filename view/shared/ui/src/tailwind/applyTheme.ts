import { Theme, ThemeTailwindVariables, mapThemeToTailwindVariables } from "@repo/theme";

//Applies the theme to the root element of the app.
export default function applyTheme(theme: Theme) {
  const themeObject: ThemeTailwindVariables = mapThemeToTailwindVariables(theme);
  const root = document.documentElement;

  Object.keys(themeObject).forEach((v) => {
    const propertyVal = themeObject[v as keyof ThemeTailwindVariables];
    root.style.setProperty(v, propertyVal);
  });
}
