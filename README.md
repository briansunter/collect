# rouille-react
A starter crate for using Rust crate rouille as backend and ReactJs as frontend, built using webpack.

# Prerequisites

* Make sure you have the necessary [Rust toolchain](https://www.rust-lang.org/en-US/install.html) available on your computer.

* Make sure you have the latest version of [Node.js' package ecosystem installed `npm`](https://nodejs.org/en/)

# Step-by-step install

* Go to your `rouille-react` local directory
    > cd my/path/to/rouille-react/
* Install webpack locally. This step should create the `node_modules/` directory.
    > npm install webpack
* Install react components
    > npm install react react-dom
* Install babel loader
    > npm install babel-loader babel-core
* Install babel presets
    > npm install babel-preset-env babel-preset-react
* Install .css file loader
    > npm install css-loader
* Install webpack command line interface
    > npm install webpack-cli
* Create webpack bundle (in development mode) using
    > npx webpack
* Build rust server side using the `nightly` channel
    > cargo +nightly build
    
# Install using npm scripts

* Go to your `rouille-react` local directory
    > cd my/path/to/rouille-react/
* Install every dependencies locally by running
    > npm install
* Run the `npm`script (defined in the `package.json` file)
    > npm run build
    
The last command should run webpack to compile React.js file and build the Rust server side.

# Runing server

* Go to your `rouille-react` local directory
    > cd my/path/to/rouille-react/
* Run the `npm`script (defined in the `package.json` file)
    > npm run server