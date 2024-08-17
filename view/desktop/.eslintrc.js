module.exports = {
  extends: ["@repo/eslint-config/web.js"],
  settings: {
    "import/resolver": {
      node: {
        paths: ["src"],
        extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
      },
      typescript: {
        project: "./tsconfig.json",
      },
      alias: {
        map: [
          ["@/hooks", path.resolve(__dirname, "./src/hooks")],
          ["@/assets", path.resolve(__dirname, "./src/assets")],
          ["@/components", path.resolve(__dirname, "./src/components")],
          ["@/shared", path.resolve(__dirname, "../shared/ui/src")],
        ],
        extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
      },
    },
  },
};
