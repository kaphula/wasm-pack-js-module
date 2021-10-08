const CopyPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack')
const path = require('path');

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),
    // new CopyWebpackPlugin(['index.html']),
    new CopyPlugin(
        {
          patterns: [
            {from: 'index.html', to: "dest" },
          ]
        }
    ),

    // new webpack.ProvidePlugin({
    //   TextDecoder: ['text-encoding', 'TextDecoder'],
    //   TextEncoder: ['text-encoding', 'TextEncoder']
    // })
  ],
  mode: 'development',
  experiments: {
    asyncWebAssembly: true
  }
};
