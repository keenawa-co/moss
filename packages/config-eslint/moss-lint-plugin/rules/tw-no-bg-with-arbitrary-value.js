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

const getAllInvalidTokens = (str, loc) => {
  const invalidTokens = [];
  let arr;

  while ((arr = ANY_TW_BG_WITH_ARBITRARY_VALUE.exec(str)) !== null) {
    const className = arr[0];
    const name = arr[1] || arr[2] || arr[3] || arr[4];

    const startColumn = loc.start.column + str.indexOf(className) + 1;
    const endColumn = startColumn + className.length;

    invalidTokens.push({
      name,
      value: className,
      loc: {
        start: {
          line: loc.start.line,
          column: startColumn,
        },
        end: {
          line: loc.end.line,
          column: endColumn,
        },
      },
    });
  }

  return invalidTokens;
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
          const invalidTokens = getAllInvalidTokens(node.value, node.loc);

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
          const invalidTokens = getAllInvalidTokens(node.value.raw, node.loc);

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
