import path, { dirname } from "node:path";
import { fileURLToPath } from "node:url";

import lintConfig from "@repo/eslint-config/eslint.config.js";

export const __dirname = dirname(fileURLToPath(import.meta.url));

export default [
  ...lintConfig,
  {
    files: ["**/*.ts", "**/*.tsx"],
    settings: {
      "import/resolver": {
        alias: {
          map: [
            ["@", path.resolve(__dirname, "./src")],
            ["@/packages", path.resolve(__dirname, "../../packages/moss-ui/src")],
          ],
          extensions: [".js", ".jsx", ".ts", ".d.ts", ".tsx"],
        },
      },
    },
  },
];
