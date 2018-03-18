# rouille-react
A starter crate for using Rust crate rouille as backend and ReactJs as frontend, built using webpack.

# Prerequisites

* Make sure you have the necessary [Rust toolchain](https://www.rust-lang.org/en-US/install.html) available on your computer.

* Make sure you have the latest version of [Node.js' package ecosystem installed `npm`](https://nodejs.org/en/)

# Step-by-step install

* Go to your `rouille-react` local directory
    > cd my/path/to/rouille-react/
* Install webpack locally. This step should create the `node_modules/` directory.
    > npm install --save-dev webpack
* Install react components
    > npm install react react-dom --save
* Install babel presets
    > npm install --save-dev babel-preset-es2015 babel-preset-react
* Install .css file loader
    > npm install --save-dev css-loader
* Install webpack command line interface
    > npm install webpack-cli -D
* Build rust server side using the `nightly` channel
    > cargo +nightly build
* Create webpack bundle (in development mode) using
    > npx webpack --mode development
    
# Install using npm scripts



# Runing server

* Simply run, in your `rouille-react` local directory, the command line
    > cargo +nightly run