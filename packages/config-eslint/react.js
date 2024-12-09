import tseslint from "typescript-eslint";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import reactRefreshPlugin from "eslint-plugin-react-refresh";

export default tseslint.config({
  extends: [tseslint.configs.recommended],
  ignores: ["node_modules/", "dist/", ".gitignore", ".prettierignore", "target/", ".turbo/", ".vscode/", "*.stories.*"],
  languageOptions: {},
  settings: {},
  plugins: {
    "react-hooks": reactHooksPlugin,
    "react-refresh": reactRefreshPlugin,
    "@typescript-eslint": tseslint.plugin,
  },
  rules: {
    "react-hooks/rules-of-hooks": "error",
    "react-hooks/exhaustive-deps": "warn",
    "react-refresh/only-export-components": ["warn", { allowConstantExport: true }],
    "@typescript-eslint/no-unused-vars": "warn",
    "@typescript-eslint/no-explicit-any": "error",
    "prefer-const": "warn",
  },
});