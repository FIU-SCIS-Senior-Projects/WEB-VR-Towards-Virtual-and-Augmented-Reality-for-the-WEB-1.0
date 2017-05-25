const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

const PATHS = {
  app: "./app/index.js",
  build: path.resolve(__dirname, 'build'),
};

module.exports = {
  // Entries have to resolve to files! they rely on Node
  // convention by default so if a directory contains *index.js*,
  // it resolves to that
  entry: {
    app: PATHS.app,
  },
  output: {
    path: PATHS.build,
    filename: 'bundle.js',
    publicPath: '/build'
  },
  plugins: [
    new HtmlWebpackPlugin({
      title: 'webpack demo',
    }),
  ],
};