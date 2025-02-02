import { Config } from "tailwindcss";
import tailwindAnimate from "tailwindcss-animate";

import tailwindTypography from "@tailwindcss/typography";

import breakpoints from "./extends/breakpoints";
import colors from "./extends/colors";
import fontSize from "./extends/fontSize";
import typography from "./extends/typography";
import backgroundPlugin from "./plugins/background";

// We want each package to be responsible for its own content.
const config: Config = {
  theme: {
    gradientColorStops: colors,
    colors,
    extend: {
      fontSize: fontSize,
      fontFamily: typography,
      screens: breakpoints,
    },
  },
  plugins: [backgroundPlugin, tailwindAnimate, tailwindTypography],
};

export default config;
