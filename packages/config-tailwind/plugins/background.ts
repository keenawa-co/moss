import createPlugin from "tailwindcss/plugin";

const backgroundPlugin = createPlugin(({ matchUtilities, theme }) => {
  matchUtilities(
    {
      background: (value: string) => ({ background: value }),
    },
    {
      values: theme("colors"), // Allow the use of arbitrary values
      type: ["color", "any"], // Permit any color values
    }
  );
});

export default backgroundPlugin;
