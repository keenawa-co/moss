const PATTERN = /(group-)?bg-\[(--[\w-]*|var\(.*?\))\]/;

const hasArbitraryValue = (str) => {
  return PATTERN.test(str);
};

const calcErrorLoc = (str, loc) => {
  const res = PATTERN.exec(str);

  const start = res.index;
  const len = res[0].length;

  return {
    start: { column: loc.start.column + start + 1, line: loc.start.line },
    end: { column: loc.start.column + start + len + 1, line: loc.end.line },
  };
};

const fixArbitraryValue = (str) => {
  return str.replace(PATTERN, (match) => match.replace("bg-[", "background-[").replace("var(", "").replace(")", ""));
};

/** @type {import('eslint').Rule.RuleModule} **/
export default {
  meta: {
    type: "problem",
    docs: {
      description: "Disallow bg- with arbitrary values in Tailwind CSS.",
      category: "Stylistic Issues",
    },
    fixable: "code",
    schema: [],
    messages: {
      replaceBg: `Use 'background-' selector instead of 'bg-' for background arbitrary values.\nTailwind maps the 'bg-' prefix to css 'background-color', which is unsuitable for gradients or complex custom properties.`,
    },
  },
  create(context) {
    return {
      Literal(node) {
        if (node.value && typeof node.value === "string" && hasArbitraryValue(node.value)) {
          context.report({
            node,
            messageId: "replaceBg",
            loc: calcErrorLoc(node.value, node.loc),
            fix(fixer) {
              const fixedValue = fixArbitraryValue(node.value);
              return fixer.replaceText(node, `"${fixedValue}"`);
            },
          });
        }
      },
      TemplateElement(node) {
        if (node.value && typeof node.value.raw === "string" && hasArbitraryValue(node.value.raw)) {
          fixArbitraryValue(node.value.raw);
          context.report({
            node,
            messageId: "replaceBg",
            loc: calcErrorLoc(node.value.raw, node.loc),
            fix(fixer) {
              const fixedValue = fixArbitraryValue(node.value.raw);
              return fixer.replaceText(node, `\`${fixedValue}\``);
            },
          });
        }
      },
    };
  },
};
