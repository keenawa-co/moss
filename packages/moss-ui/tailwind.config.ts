import type { Config } from "tailwindcss";
import tailwindAnimate from "tailwindcss-animate";

import sharedConfig from "@repo/tailwind-config";

import fontSize from "./src/tailwind/custom-config/fontSize";

const config: Pick<Config, "presets" | "content" | "extend" | "plugins" | "darkMode"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  presets: [sharedConfig],
  darkMode: "selector",
  extend: {
    fontSize,
  },
  plugins: [tailwindAnimate],
};

export default config;
