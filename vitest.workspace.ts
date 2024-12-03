import { defineWorkspace } from "vitest/config";

export default defineWorkspace([
  {
    test: {
      name: "packages",
      include: ["packages/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
    },
  },
  {
    test: {
      name: "desktop",
      include: ["view/desktop/**/*.{test,spec}.?(c|m)[jt]s?(x)"],
    },
  },
]);
