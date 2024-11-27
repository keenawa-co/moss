import storybook from "eslint-plugin-storybook";
import reactLintConfig from "@repo/eslint-config/react.js";

export default [
  ...reactLintConfig,
  ...storybook.configs["flat/recommended"],
  {
    files: ["**/*.stories.ts", "**/*.stories.tsx"],
  },
];
