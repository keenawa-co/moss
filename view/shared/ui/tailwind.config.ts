import type { Config } from "tailwindcss";
import sharedConfig from "@repo/tailwind-config";

import tailwindAnimate from "tailwindcss-animate";

const config: Pick<Config, "presets" | "content" | "extend" | "plugins" | "darkMode"> = {
  content: ["./src/**/*.{js,ts,jsx,tsx,mdx}"],
  presets: [sharedConfig],
  darkMode: "selector",

  extend: {
    keyframes: {
      "accordion-down": {
        from: { height: "0" },
        to: { height: "var(--radix-accordion-content-height)" },
      },
      "accordion-up": {
        from: { height: "var(--radix-accordion-content-height)" },
        to: { height: "0" },
      },
    },
    animation: {
      "accordion-down": "accordion-down 0.2s ease-out",
      "accordion-up": "accordion-up 0.2s ease-out",
    },
  },
  plugins: [tailwindAnimate],
};

export default config;
