import { getAllInvalidTokens } from "../utils/getAllInvalidTokens.js";

const ANY_TW_BG_WITH_ARBITRARY_VALUE =
  /\b[\w\-:]*bg-(?:\[(?:var\((--[\w-]+)\)|(--[\w-]+))\]|\((?:var\((--[\w-]+)\)|(--[\w-]+))\))/g;

const fixArbitraryValue = (className) => {
  return className
    .replace("bg-(", "background-(")
    .replace("bg-[", "background-(")
    .replace("]", ")")
    .replace("var(", "")
    .replace("))", ")");
};

/** @type {import('eslint').Rule.RuleModule} **/
export default {
  meta: {
    name: "tw-no-bg-with-arbitrary-value",
    type: "problem",
    docs: {
      description: "Disallow bg- with arbitrary values in Tailwind.",
      category: "Invalid syntax",
      recommended: true,
    },
    fixable: "code",
    hasSuggestions: true,
    messages: {
      replaceBg:
        " Use 'background-' selector instead of 'bg-' for background arbitrary values.\nTailwind maps the 'bg-' prefix to css 'background-color', which is unsuitable for gradients or complex custom properties.",
    },
    defaultOptions: [],
  },
  create(context) {
    return {
      Literal(node) {
        if (typeof node.value === "string") {
          const invalidTokens = getAllInvalidTokens(node.value, node.loc, ANY_TW_BG_WITH_ARBITRARY_VALUE);

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "replaceBg",
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
          const invalidTokens = getAllInvalidTokens(node.value.raw, node.loc, ANY_TW_BG_WITH_ARBITRARY_VALUE);

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "replaceBg",
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
