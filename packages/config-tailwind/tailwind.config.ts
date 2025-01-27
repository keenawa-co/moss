import { Config } from "tailwindcss";

import breakpoints from "./extends/breakpoints";
import colors from "./extends/colors";
import fontSize from "./extends/fontSize";
import plugins from "./extends/plugins";
import typography from "./extends/typography";

// We want each package to be responsible for its own content.
const config: Config = {
  plugins,
  theme: {
    gradientColorStops: colors,
    colors,
    extend: {
      fontSize: fontSize,
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

export default config;
