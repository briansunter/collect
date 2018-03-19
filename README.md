# rouille-react
A starter crate for using Rust crate rouille as backend and ReactJs as frontend, built using webpack.

# Prerequisites

* Make sure you have the necessary [Rust toolchain](https://www.rust-lang.org/en-US/install.html) available on your computer.

* Make sure you have the latest version of [Node.js' package ecosystem installed `npm`](https://nodejs.org/en/)

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
