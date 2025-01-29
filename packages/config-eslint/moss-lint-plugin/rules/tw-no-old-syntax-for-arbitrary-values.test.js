import { RuleTester } from "@typescript-eslint/rule-tester";

import rule from "./tw-no-old-syntax-for-arbitrary-values";

const ruleTester = new RuleTester({
  languageOptions: {
    parserOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      ecmaFeatures: {
        jsx: true,
      },
    },
  },
});

ruleTester.run("tw-no-old-syntax-for-arbitrary-values", rule, {
  valid: [
    {
      name: "Valid selector in string with new syntax",
      code: `<div className="background-(--custom-bg)"></div>`,
    },
    {
      name: "Valid selector in string with var",
      code: `<div className="background-[var(--custom-bg)]"></div>`,
    },
    {
      name: "Valid selector in string with var and pseudoclass",
      code: `<div className="hover:background-[var(--custom-bg)]"></div>`,
    },
  ],
  invalid: [
    {
      name: "Invalid selector in string",
      code: `<div className="bg-[--custom-bg]"></div>`,
      errors: [{ messageId: "replaceOldSyntax" }],
      output: `<div className="bg-(--custom-bg)"></div>`,
    },
    {
      name: "Invalid selector in string",
      code: `<div className="hover:bg-[--custom-bg]"></div>`,
      errors: [{ messageId: "replaceOldSyntax" }],
      output: `<div className="hover:bg-(--custom-bg)"></div>`,
    },
  ],
});
