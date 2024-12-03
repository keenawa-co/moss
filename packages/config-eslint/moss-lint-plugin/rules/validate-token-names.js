import { defaultLightTheme } from "../../themegen/src/index.js";

console.log(defaultLightTheme);

/**  @type {import('eslint').Rule.RuleModule} **/
export default {
  meta: {
    type: "problem",
    docs: {
      description: "Validation of token names",
      category: "Invalid syntax",
      recommended: true,
    },
    messages: {
      invalidTokenName: "Invalid token name: {{tokenName}}",
    },
  },
};
