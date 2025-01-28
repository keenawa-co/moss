import type { Config } from "tailwindcss";

import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";

const config: Pick<Config, "content" | "presets" | "theme" | "plugins"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx}"],
  presets: [sharedConfig],
  theme: {
    extend: {},
  },
  plugins: [tailwindTypography],
};

export default config;
