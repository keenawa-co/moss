import { env } from "process";
import { defineWorkspace } from "vitest/config";

import { storybookTest } from "@storybook/experimental-addon-test/vitest-plugin";

const storybookPluginConfig = {
  configDir: "./view/storybook/.storybook",
  tags: {
    include: ["stable"],
  },
};

const storybookTestConfig = {
  browser: {
    enabled: true,
    name: "chromium",
    provider: "playwright",
    headless: true,
  },
  isolate: false,
  setupFiles: ["./view/storybook/.storybook/vitest.setup.ts"],
};

export default defineWorkspace([
  {
    test: {
      name: "packages",
      include: ["packages/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
    },
  },
  {
    extends: "view/desktop/vite.config.ts",
    test: {
      name: "packages-stories",
      include: ["packages/**/*.stories.?(c|m)[jt]s?(x)"],
      ...storybookTestConfig,
    },
    plugins: [storybookTest(storybookPluginConfig)],
  },
  {
    test: {
      name: "desktop",
      include: ["view/desktop/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
    },
  },
  {
    extends: "view/desktop/vite.config.ts",
    test: {
      name: "desktop-stories",
      include: ["view/desktop/**/*.stories.?(c|m)[jt]s?(x)"],
      ...storybookTestConfig,
    },
    plugins: [storybookTest(storybookPluginConfig)],
  },
  {
    extends: "view/storybook/vite.config.ts",
    test: {
      name: "storybook",
      include: ["view/storybook/**/*.stories.?(c|m)[jt]s?(x)"],
      ...storybookTestConfig,
    },

    plugins: [storybookTest(storybookPluginConfig)],
  },
]);
