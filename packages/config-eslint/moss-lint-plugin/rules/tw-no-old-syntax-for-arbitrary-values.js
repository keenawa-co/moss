import { getAllInvalidTokens } from "../utils/getAllInvalidTokens.js";

const ANY_TW_CLASS_WITH_ARBITRARY_VALUE_AND_SQUARE_BRACKETS = /\b[a-zA-Z-]+-\[--[a-zA-Z-]+\]/g;

const fixArbitraryValue = (className) => {
  return className.replace("-[", "-(").replace("]", ")");
};

/** @type {import('eslint').Rule.RuleModule} **/
export default {
  meta: {
    name: "tw-no-old-syntax-for-arbitrary-values",
    type: "problem",
    docs: {
      description: "Disallow '-[' with arbitrary values in Tailwind.",
      category: "Invalid syntax",
      recommended: true,
    },
    fixable: "code",
    hasSuggestions: true,
    messages: {
      replaceOldSyntax: "The selector '-[' is not supported by Tailwind 4. Use '-(' instead.",
    },
    defaultOptions: [],
  },
  create(context) {
    return {
      Literal(node) {
        if (typeof node.value === "string") {
          const invalidTokens = getAllInvalidTokens(
            node.value,
            node.loc,
            ANY_TW_CLASS_WITH_ARBITRARY_VALUE_AND_SQUARE_BRACKETS
          );

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "replaceOldSyntax",
              loc: token.loc,
              fix(fixer) {
                const startOffset = node.range[0] + token.loc.start.column - node.loc.start.column;
                const endOffset = startOffset + token.value.length;

                const fixedValue = fixArbitraryValue(token.value);
                return fixer.replaceTextRange([startOffset, endOffset], fixedValue);
              },
            });
          });
        }
      },
      TemplateElement(node) {
        if (typeof node.value.raw === "string") {
          const invalidTokens = getAllInvalidTokens(
            node.value.raw,
            node.loc,
            ANY_TW_CLASS_WITH_ARBITRARY_VALUE_AND_SQUARE_BRACKETS
          );

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "replaceOldSyntax",
              loc: token.loc,
              fix(fixer) {
                const startOffset = node.range[0] + token.loc.start.column - node.loc.start.column;
                const endOffset = startOffset + token.value.length;

                const fixedValue = fixArbitraryValue(token.value);
                return fixer.replaceTextRange([startOffset, endOffset], fixedValue);
              },
            });
          });
        }
      },
    };
  },
};
