"use strict";

import * as vitest from "vitest";

import { RuleTester } from "@typescript-eslint/rule-tester";

import rule from "./no-bg-with-arbitrary-value";

RuleTester.afterAll = vitest.afterAll;
RuleTester.it = vitest.it;
RuleTester.itOnly = vitest.it.only;
RuleTester.describe = vitest.describe;

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

ruleTester.run("no-bg-with-arbitrary-value", rule, {
  valid: [
    // Literal
    `<div className="background-[--custom-bg]"></div>`,
    `<div className="background-[var(--custom-bg)]"></div>`,
    `<div className="group-background-[--custom-bg]"></div>`,
    `<div className="group-background-[var(--custom-bg)]"></div>`,
    `
      const styles = "background-[--custom-bg]"
      const Component = () => <div className={styles}></div>
    `,
    `
      const styles = "text-500 background-[--custom-bg] border text-[--custom-color] text-[var(--custom-color)]"
      const Component = () => <div className={styles}></div>
    `,
    //TemplateElement
    "<div className={`background-[--custom-bg]`}></div>",
    "<div className={`text-500 background-[--custom-bg] border text-[--custom-color] text-[var(--custom-color)]`}></div>",
  ],
  invalid: [
    //Literal
    {
      code: `<div className="bg-[--custom-bg]"></div>`,
      errors: [{ messageId: "replaceBg" }],
      output: `<div className="background-[--custom-bg]"></div>`,
    },
    {
      code: `<div className="bg-[var(--custom-bg)]"></div>`,
      errors: [{ messageId: "replaceBg" }],
      output: `<div className="background-[--custom-bg]"></div>`,
    },
    {
      code: `
        const ComponentStyles = "bg-[--custom-bg]";
        const Component = () => {
          return <div className={ComponentStyles}></div>;
        };
      `,
      errors: [{ messageId: "replaceBg" }],
      output: `
        const ComponentStyles = "background-[--custom-bg]";
        const Component = () => {
          return <div className={ComponentStyles}></div>;
        };
      `,
    },
    {
      code: `
        const styles = "text-500 bg-[--custom-bg] border text-[--custom-color] text-[var(--custom-color)]"
        const Component = () => <div className={styles}></div>
      `,
      errors: [{ messageId: "replaceBg" }],
      output: `
        const styles = "text-500 background-[--custom-bg] border text-[--custom-color] text-[var(--custom-color)]"
        const Component = () => <div className={styles}></div>
      `,
    },
    //TemplateElement
    {
      code: "<div className={`bg-[--custom-bg]`}></div>",
      errors: [{ messageId: "replaceBg" }],
      output: "<div className={`background-[--custom-bg]`}></div>",
    },
    {
      code: "<div className={`bg-[var(--custom-bg)]`}></div>",
      errors: [{ messageId: "replaceBg" }],
      output: "<div className={`background-[--custom-bg]`}></div>",
    },
  ],
});
