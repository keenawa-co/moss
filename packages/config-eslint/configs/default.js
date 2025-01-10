import importPlugin from "eslint-plugin-import";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import reactRefreshPlugin from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";

import mossLintPlugin from "../moss-lint-plugin/index.js";

export default tseslint.config(...tseslint.configs.recommended, {
  ignores: [
    "node_modules/",
    "dist/",
    ".gitignore",
    ".prettierignore",
    "target/",
    ".turbo/",
    ".vscode/",
    "*.stories.*",
    "**/*.test.*",
    "**/*.spec.*",
  ],
  languageOptions: {},
  "settings": {
    "import/parsers": {
      "@typescript-eslint/parser": [".ts", ".tsx"],
    },
    "import/resolver": {
      "typescript": {
        "project": ["tsconfig.json", "packages/*/tsconfig.json", "view/*/tsconfig.json"], // ???
      },
      "node": {
        "extensions": [".js", ".jsx", ".ts", ".tsx", ".json"],
      },
    },
  },
  files: ["**/*.{ts,tsx,js,jsx}"],
  plugins: {
    "react-hooks": reactHooksPlugin,
    "react-refresh": reactRefreshPlugin,
    "@typescript-eslint": tseslint.plugin,
    mossLint: mossLintPlugin,
    import: importPlugin,
  },
  rules: {
    "import/no-unresolved": "error",
    "react-hooks/rules-of-hooks": "error",
    "react-hooks/exhaustive-deps": "warn",
    "react-refresh/only-export-components": ["warn", { allowConstantExport: true }],
    "@typescript-eslint/no-unused-vars": [
      "warn",
      {
        "argsIgnorePattern": "^_",
        "varsIgnorePattern": "^_",
        "caughtErrorsIgnorePattern": "^_",
      },
    ],
    "@typescript-eslint/no-explicit-any": "error",
    "prefer-const": "warn",
    "mossLint/tw-no-bg-with-arbitrary-value": "error",
    "mossLint/only-valid-token-names": "error",
  },
});
