import { mergeConfig } from "../ui/src/tailwind/mergeConfig";
import type { Config } from "tailwindcss";

// We want each package to be responsible for its own content.
const config: Omit<Config, "content"> = mergeConfig({
  content: ["../ui/src/**/*.{js,ts,jsx,tsx,css}"],
  theme: {
    extend: {},
  },
  plugins: [],
});

export default config;
