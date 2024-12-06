import storybook from "eslint-plugin-storybook";

import lintConfig from "@repo/eslint-config/eslint.config.js";

export default [
  ...lintConfig,
  ...storybook.configs["flat/recommended"],
  {
    files: ["**/*.stories.ts", "**/*.stories.tsx"],
  },
];
