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
      name: "Valid token in selector",
      code: `<div className="background-[--moss-color-primary]"></div>`,
    },
    {
      name: "Valid token in selector",
      code: `<div className="text-[--moss-color-primary]"></div>`,
    },
    {
      name: "Valid token selector in template string ",
      code: `<div className={\`background-[--moss-color-primary]\`}></div>`,
    },
    {
      name: "Valid token in selector with var()",
      code: `<div className="background-[var(--moss-color-primary)]"></div>`,
    },
    {
      name: "Valid token in selector with pseudoclass",
      code: `<div className="hover:text-[--moss-color-primary]"></div>`,
    },
    {
      name: "Valid token with group selector",
      code: `<div className="group-text-[--moss-color-primary]"></div>`,
    },
    {
      name: "Valid tailwind arbitrary values",
      code: `<div className="before:content-['Festivus'] top-[117px]"></div>`,
    },
    {
      name: "Valid tailwind arbitrary properties",
      code: `<div className="[mask-type:luminance] lg:[--scroll-offset:44px]"></div>`,
    },
    {
      name: "Valid tailwind arbitrary variant",
      code: `<div className="lg:[&:nth-child(3)]:hover:underline"></div>`,
    },
  ],
  invalid: [
    {
      name: "Invalid selector in template string",
      code: `<div className={\`background-[--invalid-value]\`}></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-value",
          },
        },
      ],
    },
    {
      name: "Invalid selector in string",
      code: `<div className="background-[--invalid-value]"></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-value",
          },
        },
      ],
    },
    {
      name: "Invalid selector in string with var()",
      code: `<div className="background-[var(--invalid-value)]"></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-value",
          },
        },
      ],
    },
    {
      name: "Invalid token in selector with pseudoclass",
      code: `<div className="hover:text-[--invalid-value]"></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-value",
          },
        },
      ],
    },
    {
      name: "Invalid token with group selector",
      code: `<div className="group-text-[--invalid-value]"></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-value",
          },
        },
      ],
    },
    {
      name: "Invalid tokens",
      code: `<div className="text-[--very-long-invalid-value] text-[var(--invalid-value2)] hover:text-[--custom-val] group-bg-[--invalid-group-value]"></div>`,
      errors: [
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--very-long-invalid-value",
          },
        },
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-value2",
          },
        },
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--custom-val",
          },
        },
        {
          messageId: "invalidTokenName",
          data: {
            tokenName: "--invalid-group-value",
          },
        },
      ],
    },
  ],
});
