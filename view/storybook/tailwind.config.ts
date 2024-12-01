import type { Config } from "tailwindcss";
import sharedConfig from "@repo/tailwind-config";

const config: Pick<Config, "presets" | "content"> = {
  presets: [sharedConfig],
  content: [
    "../../packages/moss-ui/src/**/*.{js,ts,jsx,tsx,mdx}",
    "../desktop/src/**/*.{js,jsx,mjs,ts,tsx}",
    "../web/src/**/*.{js,jsx,mjs,ts,tsx}",
    "../docs/src/**/*.{js,jsx,mjs,ts,tsx}",
  ],
};

export default config;
