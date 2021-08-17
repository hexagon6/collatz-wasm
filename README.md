# collatz-wasm

1. pick a positive integer number
2. a) when odd: 3x + 1
2. b) when even: x/2
3. repeat until you find a number you had before

on positive integers this algorithm (the mathematical equivalent is called [Collatz Conjecture](https://en.wikipedia.org/wiki/Collatz_conjecture)) ends always at 1

![XKCD Collatz Conjecture](https://imgs.xkcd.com/comics/collatz_conjecture.png)

> it is an unsolved mathematical problem, but you could give it a try in the browser or another system which can run wasm with this module ;-)

> it accepts any javascript number (internally i32) and gives back an Array of numbers

## usage

In a browser after building locally:
```javascript
 import init, { collatz } from "./pkg/collatz_wasm.js"
 init()
   .then(() => {
      const list = Array.from(collatz(10))
      console.log(JSON.stringify(list))
   })
```

or by installing from [npm](https://www.npmjs.com/package/collatz-wasm) and using a bundler with:

```
import init, { collatz } from "./pkg/collatz_wasm.js"

...

collatz(1337)
```

## plans for the future

I plan on adding support for i64 (javascript BigInt), but have not yet reached a proper understanding of Rust Traits to do so. An alternative would be to copy code and replace i32 with i64, but who would do such a thing??

## uses

The Collatz Conjecture can be used to plot nice graphs and art, which was a motivation to create this module

## building from rust to wasm
### install dependencies
First install a rust environment (e.g. rustup)
and then install wasm-pack like-a-so: `$ cargo install wasm-pack`
### build wasm bundle
> based on this howto: https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

`$ wasm-pack build --target web`

This builds the rust lib.rs into a file bundle at `pkg/` which is ready to be imported into a javascript environment that has wasm support.

### test wasm bundle locally
Start a local webserver (e.g. `python3 -m http.server`) and navigate to index.html to see a test page importing the compiled wasm (open Developer Tools to see result logged in the Javascript console).
