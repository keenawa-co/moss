import storybook from "eslint-plugin-storybook";
import webConfig from "@repo/eslint-config/web.js";

export default [
  ...storybook.configs["flat/recommended"],
  ...webConfig,
  {
    files: ["**/*.ts", "**/*.tsx"],
  },
];
