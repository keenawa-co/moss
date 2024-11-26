import path from "node:path";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import webConfig from "@repo/eslint-config/web.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default [
  ...webConfig,
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
            ["@/shared", path.resolve(__dirname, "../shared/ui/src")],
          ],
          extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
        },
      },
    },
  },
];
