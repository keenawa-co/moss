import type { Config } from "tailwindcss";
import sharedConfig from "@repo/tailwind-config";
import tailwindTypography from "@tailwindcss/typography";

const config: Pick<Config, "content" | "presets" | "darkMode" | "theme" | "plugins"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx}", "../shared/ui/src/**/*.{js,ts,jsx,tsx,mdx}"],
  presets: [sharedConfig],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: "var(--color-primary)",
        secondary: "var(--color-secondary)",
        bgPrimary: "var(--color-bg-primary)",
        tBase: "var(--color-text-base)",
      },
    },
  },
  plugins: [tailwindTypography],
};

export default config;
