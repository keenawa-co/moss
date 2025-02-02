import createPlugin from "tailwindcss/plugin";

const widgetPlugin = createPlugin(({ addUtilities }) => {
  addUtilities({
    ".html": {
      fontSize: "16px",
    },
    ".body": {
      fontSize: "13px",
    },
  });
});

const customBackgroundPlugin = createPlugin(({ matchUtilities, theme }) => {
  matchUtilities(
    {
      background: (value: string) => ({ background: value }),
    },
    {
      values: theme("colors"),
      type: ["color", "any"],
    }
  );
});

const plugins: Array<ReturnType<typeof createPlugin>> = [widgetPlugin, customBackgroundPlugin];

export default plugins;
