import { INITIAL_VIEWPORTS } from "@storybook/addon-viewport";
import type { Preview } from "@storybook/react";
import React from "react";
import "../../shared/ui/src/styles.css";
import { ThemeProvider, getTheme, staticColors } from "../../shared/ui/src";

const preview: Preview = {
  globalTypes: {
    theme: {
      name: "Theme",
      description: "Global theme for components",
      defaultValue: "light",
      toolbar: {
        icon: "circlehollow",
        items: [
          { value: "light", title: "Moss Light" },
          { value: "dark", title: "Moss Dark" },
          { value: "test", title: "Moss Test" },
        ],
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
      default: "blue",
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
      return (
        <ThemeProvider themeRGBOverrides={getTheme(theme)} updateRGBOnChange>
          <Story />
        </ThemeProvider>
      );
    },
  ],
};

export default preview;
