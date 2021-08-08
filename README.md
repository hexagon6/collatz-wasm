# collatz-wasm

## prequesites
1. you need to have Rust installed ([install instructions](https://www.rust-lang.org/tools/install))
2. to compile the rust code to WASM and link with your frontends javascript code, you need `wasm-pack` ([install instructions](https://rustwasm.github.io/wasm-pack/installer/))

## build
```bash
# build rust binaries
cargo build

# compile to wasm
wasm-pack build --target web
