import merge from "deepmerge";
import { Config } from "tailwindcss";
import breakpoints from "./theme/base/breakpoints";
import colors from "./theme/base/colors";
import plugins from "./theme/base/plugins";
import typography from "./theme/base/typography";

const reactComponentsTailwindConfig: Config = {
  content: ["@/**/*.{js,ts,jsx,tsx}", "@repo/ui/**/*.{js,ts,jsx,tsx}"],
  plugins,
  theme: {
    gradientColorStops: colors,
    colors,
    extend: {
      fontFamily: typography,
      screens: breakpoints,
      borderColor: {
        DEFAULT: colors.stone["100"],
      },
    },
  },
};

/**
 * Merge custom styles and Tailwind CSS configurations
 */
export function mergeConfig(tailwindConfig: Config): Config {
  const merged = merge(reactComponentsTailwindConfig, { ...tailwindConfig });
  return merged;
}
