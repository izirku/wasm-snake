# WASM Snake

An example implementation of a _Snake_ game in _Rust_ targeting _WASM_.

This implementation is a bit different from the original one (_see: Credits_), as it uses [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/) and _js-sys_ / _web-sys_ instead of _stdweb_.

_note_: **work in progerss** as some bugs are appearing while running it.

## Usage

Have _NodeJs_ and [wasm-pack](https://rustwasm.github.io/wasm-pack/) installed first, then do the following:

```shell
wasm-pack build
cd www
npm install
npm start
```

Game play - use the arrow keys...

## Credits

Original author's [repo](https://github.com/tensor-programming/wasm_snake_example)(code, and video tutorial, etc.)