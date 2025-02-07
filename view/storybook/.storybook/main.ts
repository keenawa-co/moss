import { dirname, join } from "path";
import * as tsconfigPaths from "vite-tsconfig-paths";

import type { StorybookConfig } from "@storybook/react-vite";

/**
 * This function is used to resolve the absolute path of a package.
 * It is needed in projects that use Yarn PnP or are set up within a monorepo.
 */
function getAbsolutePath(value: string): string {
  return dirname(require.resolve(join(value, "package.json")));
}

const config: StorybookConfig = {
  stories: ["../../desktop/src/components/**/*.stories.@(ts|tsx)", "../../web/src/**/*.stories.@(ts|tsx)"],
  staticDirs: ["../public"],
  addons: [
    getAbsolutePath("@storybook/addon-links"),
    getAbsolutePath("@storybook/addon-essentials"),
    getAbsolutePath("@storybook/addon-onboarding"),
    getAbsolutePath("@storybook/addon-interactions"),
    getAbsolutePath("@storybook/addon-designs"),
  ],
  framework: {
    name: getAbsolutePath("@storybook/react-vite"),
    options: {},
  },
  typescript: {
    reactDocgen: false, // Disable react-docgen for troubleshooting
  },
  async viteFinal(config) {
    return {
      ...config,
      plugins: [...(config.plugins || []), tsconfigPaths.default()],
    };
  },
};
export default config;
