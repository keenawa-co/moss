import type { Config } from "tailwindcss";

import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";

const config: Pick<Config, "content" | "presets" | "theme" | "plugins"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx}", "../../packages/moss-ui/src/**/*.{js,ts,jsx,tsx,mdx}"],
  presets: [sharedConfig],
  theme: {
    extend: {
      backgroundColor: {
        "dv-enabled": "black",
        "dv-disabled": "orange",
        "dv-maximum": "green",
        "dv-minimum": "red",
      },
      cursor: {
        "ew-resize": "ew-resize",
        "ns-resize": "ns-resize",
        "w-resize": "w-resize",
        "e-resize": "e-resize",
        "n-resize": "n-resize",
        "s-resize": "s-resize",
      },
      transitionDuration: {
        "150": "0.15s",
      },
      transitionTimingFunction: {
        "ease-out": "ease-out",
      },
    },
  },
  plugins: [tailwindTypography],
};

export default config;
