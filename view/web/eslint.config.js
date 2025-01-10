import nextPlugin from "@next/eslint-plugin-next";
import lintConfig from "@repo/eslint-config/eslint.config.js";

export default [
  ...lintConfig,
  {
    ignores: ["node_modules/", ".turbo/", ".next/"],
  },
  {
    files: ["**/*.ts", "**/*.tsx"],
    plugins: {
      "@next/next": nextPlugin,
    },
    rules: {
      ...nextPlugin.configs.recommended.rules,
    },
  },
];
