# Counter App

Based on [near-examples/rust-counter](https://github.com/near-examples/rust-counter), this is a dummy project to be used during tests.

Has an internal counter that can be incremented.

## Build

Build the wasm binary:

```bash
cargo build --release --target wasm32-unknown-unknown
```

## Test

After having built the wasm binary, copy them to the `res/` directory:

```bash
find target/wasm32-unknown-unknown/release \
    -maxdepth 1 \
    -name \*.wasm \
    -prune \
    -exec cp {} res \;
```

Then run the simulation tests:

```bash
cargo test
```
