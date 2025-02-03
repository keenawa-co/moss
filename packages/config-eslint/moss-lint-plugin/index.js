import onlyValidTokenNames from "./rules/only-valid-token-names.js";
import noBgWithArbitraryValue from "./rules/tw-no-bg-with-arbitrary-value.js";
import noOldSyntaxForArbitraryValues from "./rules/tw-no-old-syntax-for-arbitrary-values.js";

export default {
  meta: {
    name: "moss-lint-plugin",
  },
  rules: {
    "tw-no-bg-with-arbitrary-value": noBgWithArbitraryValue,
    "only-valid-token-names": onlyValidTokenNames,
    "tw-no-old-syntax-for-arbitrary-values": noOldSyntaxForArbitraryValues,
  },
};
