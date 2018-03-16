const path = require('path');

module.exports = {
  entry: './src/front/index.js',
  output: {
    filename: 'bundle.js',
    path: path.resolve(__dirname, 'src/front')
  }
};