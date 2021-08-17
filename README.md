# collatz-wasm

## dependencies

`$ cargo install wasm-pack`

## build
> based on this howto: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

`$ wasm-pack build --target web`

This builds a file files into `pkg/` which are ready to be imported into a js environment

## run
Start a webserver `python3 -m http.server` and navigate to index.html to see a test page importing the compiled wasm.