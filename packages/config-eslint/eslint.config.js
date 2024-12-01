// Config for testing custom rules

import mossLintPlugin from "./moss-lint-plugin/index.js";

/** @type {import('eslint').Linter.Config} */
export default [
  {
    files: ["*.js", "*.jsx"],
    plugins: {
      mossLint: mossLintPlugin,
    },
    rules: {
      "mossLint/no-bg-with-arbitrary-value": "error",
    },
  },
];
