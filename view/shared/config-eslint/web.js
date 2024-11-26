import tseslint from "typescript-eslint";
import reactHooksPlugin from "eslint-plugin-react-hooks";
import reactRefreshPlugin from "eslint-plugin-react-refresh";

export default tseslint.config({
  extends: [tseslint.configs.recommended],
  files: ["**/*.ts", "**/*.tsx"],
  ignores: ["node_modules", "dist"],
  languageOptions: {},
  settings: {},
  plugins: {
    "react-hooks": reactHooksPlugin,
    "react-refresh": reactRefreshPlugin,
  },
  rules: {
    "react-hooks/rules-of-hooks": "error",
    "react-hooks/exhaustive-deps": "warn",
    "react-refresh/only-export-components": ["warn", { allowConstantExport: true }],
    semi: "warn",
  },
});
