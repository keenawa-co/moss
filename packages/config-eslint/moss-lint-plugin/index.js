import onlyValidTokenNames from "./rules/only-valid-token-names.js";
import noBgRule from "./rules/tw-no-bg-with-arbitrary-value.js";

export default {
  meta: {
    name: "moss-lint-plugin",
  },
  configs: {
    tests: {},
    recommended: {},
  },
  rules: {
    "tw-no-bg-with-arbitrary-value": noBgRule,
    "only-valid-token-names": onlyValidTokenNames,
  },
  processors: {},
};
