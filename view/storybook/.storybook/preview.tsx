import { INITIAL_VIEWPORTS } from "@storybook/addon-viewport";
import type { Preview } from "@storybook/react";
import React from "react";
import "@repo/ui/src/styles.css";
import { ThemeProvider, staticColors } from "@repo/ui";
import * as themeFiles from "./themes";
import { Convert, Theme } from "@repo/theme";

const themes: Map<string, Theme> = new Map();
for (const themeName in themeFiles) {
  const theme = themeFiles[themeName];
  themes.set(theme.name, Convert.toTheme(JSON.stringify(theme)));
}

const preview: Preview = {
  globalTypes: {
    theme: {
      name: "Theme",
      description: "Global theme for components",
      defaultValue: Array.from(themes.entries()).find(([_, theme]) => theme.default === true)?.[0] || "moss-light",
      toolbar: {
        icon: "circlehollow",
        items: Array.from(themes.keys()).map((themeName) => ({
          value: themeName,
          title: themeName,
        })),
        dynamicTitle: true,
      },
    },
  },
  parameters: {
    actions: { argTypesRegex: "^on[A-Z].*" },
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },

    backgrounds: {
      default: "lightish",
      values: [
        { name: "light", value: "white" },
        { name: "lightish", value: staticColors.stone["50"] },
        { name: "dark", value: staticColors.space["700"] },
        { name: "blue", value: staticColors.ocean["400"] },
      ],
    },
    viewport: {
      viewports: INITIAL_VIEWPORTS,
    },
  },
  decorators: [
    (Story, context) => {
      const theme = context.args.theme ?? context.globals.theme;
      console.warn("-------------------->", theme);
      return (
        <ThemeProvider themeOverrides={themes.get(theme)} updateOnChange>
          <Story />
        </ThemeProvider>
      );
    },
  ],
};

export default preview;
