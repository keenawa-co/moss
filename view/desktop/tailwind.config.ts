import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";
import { mergeConfig } from "./src/components/tailwind/mergeConfig";

const config = mergeConfig({
  content: ["./src/**/*.{js,ts,jsx,tsx,css}", "../shared/ui/src/**/*.{js,ts,jsx,tsx,mdx,css}"],
  presets: [sharedConfig],
  theme: {
    extend: {
      //transitionProperty: { width: "width" },
    },
  },
  plugins: [tailwindTypography],
});

export default config;
