import { RuleTester } from "@typescript-eslint/rule-tester";

import rule from "./only-valid-token-names";

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

ruleTester.run("only-valid-token-names", rule, {
  valid: [
    {
      name: "Valid selector in string",
      code: `<div className="bg-[--sidebar-background]"></div>`,
    },
  ],
  invalid: [
    {
      name: "Invalid selector in string",
      code: `<div className="bg-[--invalid-selector]"></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-selector",
          },
        },
      ],
    },
  ],
});
