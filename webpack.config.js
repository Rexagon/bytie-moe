const path = require('path');

const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = {
  entry: './index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'index.js'
  },
  module: {
    rules: [
      {
        test: /\.(html)$/,
        use: ['html-loader']
      },
      {
        test: /\.s[ac]ss$/i,
        use: [
          process.env.NODE_ENV !== 'production'
            ? 'style-loader'
            : MiniCssExtractPlugin.loader,
          'css-loader',
          'sass-loader'
        ]
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(path.join(__dirname, 'static'), 'template.html')
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'frontend')
    }),
    new MiniCssExtractPlugin({
      filename: 'style.css',
      chunkFilename: '[name].css',
      ignoreOrder: false
    })
  ]
};
