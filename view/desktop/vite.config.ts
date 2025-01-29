import { resolve } from "path";
import { defineConfig } from "vite";

import react from "@vitejs/plugin-react";

export default defineConfig({
  server: {
    watch: {
      ignored: ["**/*.spec.ts", "**/*.test.ts", "storage/**"],
    },
  },
  plugins: [react()],
  assetsInclude: "src/renderer/assets/**",
  resolve: {
    alias: {
      "@": resolve("src"),
      "@/hooks": resolve("src/hooks"),
      "@/assets": resolve("src/assets"),
      "@/components": resolve("src/components"),
      "@/store": resolve("src/store"),
    },
  },
});
