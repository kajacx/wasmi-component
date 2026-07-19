#!/usr/bin/sh
set -e

# Run from this directory

cd crates/wasmi-component-wasi

cargo run --manifest-path ../../Cargo.toml -p wasmi-component-bindgen -- -m ./wasi-p2-partial.wit > ./src/bindgen.rs

cargo expand bindgen > ./src/expanded.rs

cd ../..

cargo fmt
cargo build
