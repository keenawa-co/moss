// FIXME: temporary solution with test.json file
import themeInterface from "../test.json" with { type: "json" };

const colorKeysToCssValues = (colors) => {
  const keys = Object.keys(colors);
  return keys.map((color) => "--color-" + color.replaceAll(".", "-"));
};

const VALID_TOKENS = colorKeysToCssValues(themeInterface.colors);
const SELECTOR_WITH_ARBITRARY_VALUE = /(?:group-)?\w+-\[(?:var\((--[\w-]+)\)|(--[\w-]+))\]/g;
const ARBITRARY_VALUE = /\[(?:var\((--[\w-]+)\)|(--[\w-]+))\]/;

const getInvalidTokenName = (str) => {
  const selectors = str.match(SELECTOR_WITH_ARBITRARY_VALUE);
  if (selectors === null || selectors.length === 0) return false;

  const tokens = selectors.map((className) => {
    const selector = className.match(ARBITRARY_VALUE);
    const token = selector[1] || selector[2];
    return token;
  });

  return tokens.find((token) => !VALID_TOKENS.includes(token));
};

const hasInvalidTokens = (str) => {
  const selectors = str.match(SELECTOR_WITH_ARBITRARY_VALUE);

  if (selectors === null || selectors.length === 0) return false;

  return selectors.some((className) => {
    const selector = className.match(ARBITRARY_VALUE);
    const token = selector[1] || selector[2];

    return !VALID_TOKENS.includes(token);
  });
};

const getInvalidSelectorLoc = (str, loc) => {
  const selectors = str.match(SELECTOR_WITH_ARBITRARY_VALUE);
  if (selectors === null || selectors.length === 0) return false;

  const invalidSelector = selectors.find((className) => {
    const selector = className.match(ARBITRARY_VALUE);
    const token = selector[1] || selector[2];

    return !VALID_TOKENS.includes(token);
  });

  if (invalidSelector === undefined) return loc;

  const startColumn = loc.start.column + str.indexOf(invalidSelector) + 1;
  const endColumn = startColumn + invalidSelector.length;

  return {
    start: {
      line: loc.start.line,
      column: startColumn,
    },
    end: {
      line: loc.start.line,
      column: endColumn,
    },
  };
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
        if (node.value && typeof node.value === "string" && hasInvalidTokens(node.value)) {
          context.report({
            node,
            messageId: "invalidTokenName",
            data: {
              tokenName: getInvalidTokenName(node.value),
            },
            loc: getInvalidSelectorLoc(node.value, node.loc),
          });
        }
      },
      TemplateElement(node) {
        if (node.value && typeof node.value.raw === "string" && hasInvalidTokens(node.value.raw)) {
          context.report({
            node,
            messageId: "invalidTokenName",
            data: {
              tokenName: getInvalidTokenName(node.value),
            },
            loc: getInvalidSelectorLoc(node.value.raw, node.loc),
          });
        }
      },
    };
  },
};
