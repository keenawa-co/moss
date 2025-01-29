import { RuleTester } from "@typescript-eslint/rule-tester";

import rule from "./tw-no-bg-with-arbitrary-value";

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

ruleTester.run("tw-no-bg-with-arbitrary-value", rule, {
  valid: [
    {
      name: "Valid selector in string",
      code: `<div className="background-(--custom-bg)"></div>`,
    },
    {
      name: "Valid selector in string with group-",
      code: `<div className="group-background-(--custom-bg)"></div>`,
    },
    {
      name: "Valid selector in string with hover:",
      code: `<div className="hover:background-(--custom-bg)"></div>`,
    },
    {
      name: "Valid selector in a string outside of className",
      code: `
        const styles = "background-(--custom-bg)"
        const Component = () => <div className={styles}></div>
      `,
    },
    {
      name: "Valid selector in a string outside of className next to other selectors",
      code: `
        const styles = "text-500 background-(--custom-bg) border text-(--custom-color)"
        const Component = () => <div className={styles}></div>
      `,
    },
    {
      name: "Valid selector in template string",
      code: "<div className={`background-(--custom-bg)`}></div>",
    },
    {
      name: "Valid selector in template string next to other selectors",
      code: "<div className={`text-500 background-(--custom-bg) border text-(--custom-color)`}></div>",
    },
  ],
  invalid: [
    {
      name: "Invalid selector in string",
      code: `<div className="bg-(--custom-bg)"></div>`,
      errors: [{ messageId: "replaceBg" }],
      output: `<div className="background-(--custom-bg)"></div>`,
    },
    {
      name: "Valid selector in string with hover:",
      code: `<div className="hover:bg-(--custom-bg)"></div>`,
      errors: [{ messageId: "replaceBg" }],
      output: `<div className="hover:background-(--custom-bg)"></div>`,
    },

    {
      name: "Invalid selector in a string outside of className",
      code: `
        const ComponentStyles = "bg-(--custom-bg)";
        const Component = () => {
          return <div className={ComponentStyles}></div>;
        };
      `,
      errors: [{ messageId: "replaceBg" }],
      output: `
        const ComponentStyles = "background-(--custom-bg)";
        const Component = () => {
          return <div className={ComponentStyles}></div>;
        };
      `,
    },
    {
      name: "Invalid selector in a string outside of className next to other selectors",
      code: `
        const styles = "text-500 bg-(--custom-bg) border text-(--custom-color)"
        const Component = () => <div className={styles}></div>
      `,
      errors: [{ messageId: "replaceBg" }],
      output: `
        const styles = "text-500 background-(--custom-bg) border text-(--custom-color)"
        const Component = () => <div className={styles}></div>
      `,
    },
    {
      name: "Invalid selector in template string",
      code: "<div className={`bg-(--custom-bg)`}></div>",
      errors: [{ messageId: "replaceBg" }],
      output: "<div className={`background-(--custom-bg)`}></div>",
    },
    {
      name: "Invalid selector in template string",
      code: "<div className={`bg-[var(--custom-bg)]`}></div>",
      errors: [{ messageId: "replaceBg" }],
      output: "<div className={`background-(--custom-bg)`}></div>",
    },
  ],
});
