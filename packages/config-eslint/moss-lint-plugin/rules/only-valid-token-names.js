// FIXME: temporary solution with test.json file
import themeInterface from "../test.json" with { type: "json" };

const colorKeysToCssValues = (colors) => {
  const keys = Object.keys(colors);
  return keys.map((color) => "--color-" + color.replaceAll(".", "-"));
};

const VALID_TOKENS = new Set(colorKeysToCssValues(themeInterface.colors));

const ANY_TW_SELECTOR_WITH_ARBITRARY_VALUE = /\b[\w|\-:]+\[(?:var\((--[\w-]+)\)|(--[\w-]+))\]/g;

const getAllInvalidTokens = (str, loc) => {
  const invalidTokens = [];
  let arr;

  while ((arr = ANY_TW_SELECTOR_WITH_ARBITRARY_VALUE.exec(str)) !== null) {
    const className = arr[0];
    const name = arr[1] || arr[2];

    const startColumn = loc.start.column + str.indexOf(className) + 1;
    const endColumn = startColumn + className.length;

    if (!VALID_TOKENS.has(name)) {
      invalidTokens.push({
        name,
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
  }

  return invalidTokens;
};

/**  @type {import('eslint').Rule.RuleModule} **/
export default {
  meta: {
    defaultOptions: [],
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
  create(context) {
    return {
      Literal(node) {
        if (typeof node.value === "string") {
          const invalidTokens = getAllInvalidTokens(node.value, node.loc);

          invalidTokens.forEach((token) => {
            context.report({
              node,
              messageId: "invalidTokenName",
              data: {
                tokenName: token.name,
              },
              loc: token.loc,
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
              messageId: "invalidTokenName",
              data: {
                tokenName: token.name,
              },
              loc: token.loc,
            });
          });
        }
      },
    };
  },
};
