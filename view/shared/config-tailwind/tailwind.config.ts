import { mergeConfig } from "../ui/src/tailwind/mergeConfig";
import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

// We want each package to be responsible for its own content.
const config: Omit<Config, "content"> = mergeConfig({
  content: ["../ui/src/**/*.{js,ts,jsx,tsx,css}"],
  plugins: [
    plugin(function ({ matchUtilities, theme }) {
      matchUtilities(
        {
          bgc: (value: string) => ({
            background: value,
          }),
        },
        {
          // Allow the use of arbitrary values
          values: theme("colors"),
          type: ["color", "any"], // Permit any color values
        }
      );
    }),
  ],
});

export default config;
