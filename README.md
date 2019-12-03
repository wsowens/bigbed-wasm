# bigbed-wasm
Using the [BigBed crate](https://crates.io/crates/bigbed) to produce an online, serverless, `bedToBigBed` clone.

Try it here: [https://cise.ufl.edu/~wowens/bigbed](https://cise.ufl.edu/~wowens/bigbed)

## Getting Started

These instructions assume that you already have an up-to-date installation of `cargo` installed via `rustup`.
If this is not the case, refer to the [Rust website](https://www.rust-lang.org/tools/install).

First, add the WebAssembly target:
```
rustup target add wasm32-unknown-unknown --toolchain nightly
```
This target allows us to compile our Rust code to WebAssembly.


Next, install the add the `wasm-bindgen` CLI:
```
cargo +nightly install wasm-bindgen-cli
```
We will use this to optimize our WASM modules and wrap them with nice JavaScript.

With these requirements fulfilled, simply run `make` to generate the correct WASM module.
```
make
```

If you do not have `make`, you can simply execute the commands from the `makefile` manually:
```sh
# create the initial WASM module
cargo +nightly build --target wasm32-unknown-unknown
# use wasm-bindgen to optimize it and generate helper code
wasm-bindgen target/wasm32-unknown-unknown/debug/bigbed_wasm.wasm --out-dir optimized --target web
# move the WASM module + JavaScript into the web folder
cp optimized/bigbed_wasm.js optimized/bigbed_wasm_bg.wasm -t web
```

## Testing
Once the WASM module + JavaScript has been placed in `web/`, you simply serve these static files over any HTTP server.

For instance, people with python3 can test these files by first hosting an HTTP server:
```
python3 -m http.server
```
With the server running, go to your browser and navigate to the convert file: [http://localhost:8000/web/convert.html](http://localhost:8000/web/convert.html)

## License

This crate is licensed under GPL-3.0.
Please see the [LICENSE](./LICENSE) for details.
