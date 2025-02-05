import React from "react";

import type { Preview } from "@storybook/react";

import "@repo/moss-ui/src/styles.css";
import "@repo/moss-ui/src/fonts.css";

import * as themeFiles from "./themes";

interface Theme {
  "name": string;
  "slug": string;
  "type": string;
  "isDefault": boolean;
  "colors": Record<
    string,
    {
      "type": string;
      "value": string;
    }
  >;
}

const applyTheme = (theme: Theme) => {
  const root = document.documentElement;
  const colors = theme.colors;

  Object.entries(colors).forEach(([key, color]) => {
    // @ts-expect-error this shouldn't be an error and seems to be a problem with config, but tsconfig seems fine
    root.style.setProperty(`--moss-${key}`.replaceAll(".", "-"), color.value);
  });
};

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
        ],
        showName: true,
      },
    },
  },
  parameters: {
    layout: "fullscreen",
  },
  decorators: [
    (Story, context) => {
      const theme = context.args.theme ?? context.globals.theme;

      applyTheme(themeFiles[theme]);

      return (
        <div style={{ backgroundColor: "var(--moss-page-background)" }}>
          <Story />
        </div>
      );
    },
  ],
};

export default preview;
