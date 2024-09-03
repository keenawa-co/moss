import merge from "deepmerge";
import { Config } from "tailwindcss";
import breakpoints from "./custom-config/breakpoints";
import colors from "./custom-config/colors";
import plugins from "./custom-config/plugins";
import typography from "./custom-config/typography";

const reactComponentsTailwindConfig: Config = {
  content: ["@repo/ui/**/*.{js,ts,jsx,tsx}"],
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
      backgroundImage: {
        "glow-conic": "conic-gradient(from 180deg at 50% 50%, #2a8af6 0deg, #a853ba 180deg, #e92a67 360deg)",
      },
      height: {
        "4.5": "1.125rem",
        "5.5": "1.375rem",
      },
      width: {
        "4.5": "1.125rem",
        "57": "14.313rem",
        "65": "16.25rem",
      },
      margin: {
        "13": "3.25rem",
        "5.5": "1.375rem",
      },
    },
  },
};

//Merge custom styles and Tailwind CSS configurations
export function mergeConfig(tailwindConfig: Config): Config {
  const merged = merge(reactComponentsTailwindConfig, { ...tailwindConfig });
  return merged;
}
