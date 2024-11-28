import path from "node:path";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import reactLintConfig from "@repo/eslint-config/react.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default [
  ...reactLintConfig,
  {
    files: ["**/*.ts", "**/*.tsx"],
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
            ["@/packages", path.resolve(__dirname, "../../packages/moss-ui/src")],
          ],
          extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
        },
      },
    },
  },
];
