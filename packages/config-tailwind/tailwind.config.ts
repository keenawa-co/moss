import { Config } from "tailwindcss";
import tailwindAnimate from "tailwindcss-animate";

import tailwindTypography from "@tailwindcss/typography";

import breakpoints from "./extends/breakpoints";
import colors from "./extends/colors";
import fontSize from "./extends/fontSize";
import plugins from "./extends/plugins";
import typography from "./extends/typography";

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
  plugins: [tailwindAnimate, tailwindTypography, ...plugins],
};

export default config;
