import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";

const config = {
  content: ["./src/**/*.{js,ts,jsx,tsx,css}"],
  presets: [sharedConfig],
  plugins: [tailwindTypography],
};

export default config;
