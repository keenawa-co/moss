// view/desktop/vite.config.ts
import react from "file:///C:/Users/Enth/Desktop/projects/moss/node_modules/.pnpm/@vitejs+plugin-react@4.3.4_vite@6.0.3_@types+node@22.10.2_jiti@2.4.1_sass-embedded@1.83.0_terser@5.37.0_yaml@2.6.1_/node_modules/@vitejs/plugin-react/dist/index.mjs";
import { resolve } from "path";
import { defineConfig } from "file:///C:/Users/Enth/Desktop/projects/moss/node_modules/.pnpm/vite@6.0.3_@types+node@22.10.2_jiti@2.4.1_sass-embedded@1.83.0_terser@5.37.0_yaml@2.6.1/node_modules/vite/dist/node/index.js";
var vite_config_default = defineConfig({
  server: {
    watch: {
      ignored: ["**/*.spec.ts", "**/*.test.ts", "storage/**"]
    }
  },
  plugins: [react()],
  assetsInclude: "src/renderer/assets/**",
  resolve: {
    alias: {
      "@": resolve("src"),
      "@/hooks": resolve("src/hooks"),
      "@/assets": resolve("src/assets"),
      "@/components": resolve("src/components")
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidmlldy9kZXNrdG9wL3ZpdGUuY29uZmlnLnRzIl0sCiAgInNvdXJjZXNDb250ZW50IjogWyJjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZGlybmFtZSA9IFwiQzpcXFxcVXNlcnNcXFxcRW50aFxcXFxEZXNrdG9wXFxcXHByb2plY3RzXFxcXG1vc3NcXFxcdmlld1xcXFxkZXNrdG9wXCI7Y29uc3QgX192aXRlX2luamVjdGVkX29yaWdpbmFsX2ZpbGVuYW1lID0gXCJDOlxcXFxVc2Vyc1xcXFxFbnRoXFxcXERlc2t0b3BcXFxccHJvamVjdHNcXFxcbW9zc1xcXFx2aWV3XFxcXGRlc2t0b3BcXFxcdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL0M6L1VzZXJzL0VudGgvRGVza3RvcC9wcm9qZWN0cy9tb3NzL3ZpZXcvZGVza3RvcC92aXRlLmNvbmZpZy50c1wiO2ltcG9ydCByZWFjdCBmcm9tIFwiQHZpdGVqcy9wbHVnaW4tcmVhY3RcIjtcclxuaW1wb3J0IHsgcmVzb2x2ZSB9IGZyb20gXCJwYXRoXCI7XHJcbmltcG9ydCB7IGRlZmluZUNvbmZpZyB9IGZyb20gXCJ2aXRlXCI7XHJcblxyXG5leHBvcnQgZGVmYXVsdCBkZWZpbmVDb25maWcoe1xyXG4gIHNlcnZlcjoge1xyXG4gICAgd2F0Y2g6IHtcclxuICAgICAgaWdub3JlZDogW1wiKiovKi5zcGVjLnRzXCIsIFwiKiovKi50ZXN0LnRzXCIsIFwic3RvcmFnZS8qKlwiXSxcclxuICAgIH0sXHJcbiAgfSxcclxuICBwbHVnaW5zOiBbcmVhY3QoKV0sXHJcbiAgYXNzZXRzSW5jbHVkZTogXCJzcmMvcmVuZGVyZXIvYXNzZXRzLyoqXCIsXHJcbiAgcmVzb2x2ZToge1xyXG4gICAgYWxpYXM6IHtcclxuICAgICAgXCJAXCI6IHJlc29sdmUoXCJzcmNcIiksXHJcbiAgICAgIFwiQC9ob29rc1wiOiByZXNvbHZlKFwic3JjL2hvb2tzXCIpLFxyXG4gICAgICBcIkAvYXNzZXRzXCI6IHJlc29sdmUoXCJzcmMvYXNzZXRzXCIpLFxyXG4gICAgICBcIkAvY29tcG9uZW50c1wiOiByZXNvbHZlKFwic3JjL2NvbXBvbmVudHNcIiksXHJcbiAgICB9LFxyXG4gIH0sXHJcbn0pO1xyXG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQWtWLE9BQU8sV0FBVztBQUNwVyxTQUFTLGVBQWU7QUFDeEIsU0FBUyxvQkFBb0I7QUFFN0IsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDMUIsUUFBUTtBQUFBLElBQ04sT0FBTztBQUFBLE1BQ0wsU0FBUyxDQUFDLGdCQUFnQixnQkFBZ0IsWUFBWTtBQUFBLElBQ3hEO0FBQUEsRUFDRjtBQUFBLEVBQ0EsU0FBUyxDQUFDLE1BQU0sQ0FBQztBQUFBLEVBQ2pCLGVBQWU7QUFBQSxFQUNmLFNBQVM7QUFBQSxJQUNQLE9BQU87QUFBQSxNQUNMLEtBQUssUUFBUSxLQUFLO0FBQUEsTUFDbEIsV0FBVyxRQUFRLFdBQVc7QUFBQSxNQUM5QixZQUFZLFFBQVEsWUFBWTtBQUFBLE1BQ2hDLGdCQUFnQixRQUFRLGdCQUFnQjtBQUFBLElBQzFDO0FBQUEsRUFDRjtBQUNGLENBQUM7IiwKICAibmFtZXMiOiBbXQp9Cg==
