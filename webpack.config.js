const path = require('path');

const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@broxus/wasm-pack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: '[name].[chunkhash].js'
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
  resolve: {
    extensions: ['.js', '.scss', '.css', '.wasm'],
    modules: [
      path.resolve(__dirname, 'src'),
      'node_modules',
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.resolve(path.join(__dirname, 'static'), 'template.html'),
      favicon: path.resolve(path.join(__dirname, 'static'), 'favicon.ico'),
    }),
    new WasmPackPlugin({
      outDir: path.resolve(__dirname, 'src/wasm/pkg'),
      crateDirectory: path.resolve(__dirname, 'src/wasm'),
      extraArgs: '--target web'
    }),
    new MiniCssExtractPlugin({
      filename: 'style.css',
      chunkFilename: '[name].[contenthash].css',
      ignoreOrder: false
    })
  ],
  experiments: {
    syncWebAssembly: true,
  }
};
