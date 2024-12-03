import mossLintPlugin from "./moss-lint-plugin/index.js";

// Config for testing custom rules
/** @type {import('eslint').Linter.Config} */
export default [
  {
    files: ["*.test.js", "*.test.jsx"],
    plugins: {
      mossLint: mossLintPlugin,
    },
    rules: {
      "mossLint/tw-no-bg-with-arbitrary-value": "error",
    },
  },
];
