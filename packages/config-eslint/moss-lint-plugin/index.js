import noBgRule from "./rules/tw-no-bg-with-arbitrary-value.js";

const plugin = {
  meta: {
    name: "moss-lint-plugin",
  },
  configs: {
    tests: {},
    recommended: {},
  },
  rules: {
    "tw-no-bg-with-arbitrary-value": noBgRule,
  },
  processors: {},
};

export default plugin;
