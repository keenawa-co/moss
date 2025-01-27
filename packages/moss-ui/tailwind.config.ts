import tailwindAnimate from "tailwindcss-animate";

import sharedConfig from "@repo/tailwind-config";

const config = {
  ...sharedConfig,
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  darkMode: "selector",
  plugins: [tailwindAnimate],
};

export default config;
