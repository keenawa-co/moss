import type { Config } from "tailwindcss";

import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";

const config: Pick<Config, "content" | "presets" | "theme" | "plugins"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx}", "../../packages/moss-ui/src/**/*.{js,ts,jsx,tsx,mdx}"],
  presets: [sharedConfig],
  theme: {
    extend: {
      fontSize: {
        "dv-tabs-font": "13px",
      },
      spacing: {
        "dv-tabs-height": "35px",
      },
      boxShadow: {
        "dv-floating": "8px 8px 8px 0px rgba(83, 89, 93, 0.5)",
      },
      zIndex: {
        "dv-overlay": "999",
      },
      borderWidth: {
        "dv-tab-divider": "1px",
      },
      opacity: {
        "dv-dragging": "0.5",
      },
      outline: {
        "dv-focus-outline": "1px solid var(--dv-paneview-active-outline-color)",
      },
    },
  },
  plugins: [tailwindTypography],
};

export default config;
