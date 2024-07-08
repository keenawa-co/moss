const HtmlWebpackPlugin = require('html-webpack-plugin');

process.env.NODE_ENV = 'development';
const host = process.env.HOST || 'localhost';

module.exports = {
  mode: 'development',
  devtool: 'inline-source-map',
  entry: './src/index.tsx',
  output: {
    filename: 'static/js/bundle.js',
  },
  devServer: {
    compress: true,
    hot: true,
    host,
    port: 3000,
  },
  plugins: [new HtmlWebpackPlugin()],
  module: {
    rules: [
      {
        test: /\.m?[jt]sx$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
      {
        test: /\.css$/,
        sideEffects: true,
        use: [
          require.resolve('style-loader'),
          {
            loader: require.resolve('css-loader'),
            options: {
              importLoaders: 1,
            },
          },
          {
            loader: require.resolve('postcss-loader'),
            options: {
              implementation: require.resolve('postcss'),
            },
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: ['.mjs', '.js', '.cjs', '.jsx', '.tsx', '.ts'],
    modules: ['node_modules'],
  },
};
