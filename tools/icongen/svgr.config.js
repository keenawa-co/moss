// https://react-svgr.com/docs/options/
module.exports = {
  outDir: "build",
  icon: true,
  expandProps: true,
  ref: false,
  typescript: true,
  replaceAttrValues: {
    "#808080": "currentColor",
  },

  // https://svgo.dev/docs/plugins/
  svgo: true,
  svgoConfig: {
    plugins: [
      {
        name: "preset-default",
        params: {
          overrides: {
            removeViewBox: false,
            convertColors: false,
          },
        },
      },
    ],
  },
};
