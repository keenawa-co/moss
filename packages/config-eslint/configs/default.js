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
  settings: {},
  files: ["**/*.{ts,tsx,js,jsx}"],

  plugins: {
    "react-hooks": reactHooksPlugin,
    "react-refresh": reactRefreshPlugin,
    "@typescript-eslint": tseslint.plugin,
    mossLint: mossLintPlugin,
  },
  rules: {
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
    //FIXME: enable when tabs are completely implemented
    "mossLint/only-valid-token-names": "off",
    "mossLint/tw-no-old-syntax-for-arbitrary-values": "error",
  },
});
