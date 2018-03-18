const path = require('path');
const webpack = require('webpack');

module.exports = {
  entry: './src/front/index.js',
  output: {
    path: path.resolve(__dirname, 'src/front'),
    filename: 'bundle.js'
  },
  module: {
    rules: [
        {
          test: /\.js$/,
          exclude: /node_modules/,
          use: {
            loader: 'babel-loader',
            options: { presets: ["env", "react"] }
          }
        },
        {
          test: /\.css$/,
          use: [ 'css-loader' ]
        }
      ]
  }
};