import noBgRule from "./rules/no-bg-with-arbitrary-value.js";

const plugin = {
  meta: {
    name: "moss-lint-plugin",
  },
  configs: {},
  rules: {
    "no-bg-with-arbitrary-value": noBgRule,
  },
  processors: {},
};

export default plugin;
