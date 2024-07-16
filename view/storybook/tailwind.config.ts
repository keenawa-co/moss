import type { Config } from "tailwindcss";
import sharedConfig from "@repo/tailwind-config";

const config: Pick<Config, "prefix" | "presets" | "content"> = {
  content: ["../shared/ui/src/**/*.{js,ts,jsx,tsx,mdx}"],
  prefix: "ui-",
  presets: [sharedConfig],
};

export default config;
