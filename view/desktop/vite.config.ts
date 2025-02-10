import { resolve } from "path";
import { defineConfig } from "vite";

import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  server: {
    watch: {
      ignored: ["**/*.spec.ts", "**/*.test.ts", "storage/**"],
    },
  },
  plugins: [tailwindcss(), react()],
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
      "@/hooks": resolve(__dirname, "src/hooks"),
      "@/utils": resolve(__dirname, "src/utils"),
      "@/assets": resolve(__dirname, "src/assets"),
      "@/components": resolve(__dirname, "src/components"),
      "@/store": resolve(__dirname, "src/store"),
    },
  },
});
