# Build & Run

1. cd in to the root of the project
2. run `wasm-pack build`
3. cd in to the `www/` folder
4. run `npm install` and then `npm start`

You should now be able to open browser to `localhost:8080` and see pixi.js get initialized properly through
WebAssembly/Rust in the console. The Javascript executed in the browser window is in the file: `www/index.js`.