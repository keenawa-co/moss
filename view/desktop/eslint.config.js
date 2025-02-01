import path, { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import lintConfig from "@repo/eslint-config/eslint.config.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

export default [
  ...lintConfig,
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
            ["@/store", path.resolve(__dirname, "./src/store")],
            ["@/components", path.resolve(__dirname, "./src/components")],
            ["@/packages", path.resolve(__dirname, "../../packages/moss-ui/src")],
          ],
          extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
        },
      },
    },
  },
];
