## WASM playground

This repository contains examples of how to call WASM modules from Rust.

Prepare environment:

```shell
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
```

## Examplds

### `wasmer-simple`

Prints "The answer is 42" to stdout.

```shell
make wasmer-simple
```

### `wasmer-json-exchange`

Shows how to pass/receive json data using functions.

```shell
make wasmer-json-exchange
```

## TODO

Some examples are planned but not implemented yet:

 - add `wasmer-wasi-json-exchange` example
 - add `wasmtime-json-exchange` example
 - add `wasmtime-wasi-json-exchange` example
 - add `host-function-http-request` example
 - add components model example

Feel free to contribute! :3
