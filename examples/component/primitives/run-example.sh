#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo component build --target wasm32-unknown-unknown
cd ..

cargo run --manifest-path ../../../Cargo.toml -p wasmi-component-bindgen -- example.wit > host/src/bindings.rs

rm -rf modules
mkdir modules

cd host
cargo run || true
cd ..

cd modules
wasm-tools print module0.core.wasm > module0.core.wat 2> /dev/null || rm module0.core.wat
wasm-tools print module1.core.wasm > module1.core.wat 2> /dev/null || rm module1.core.wat
wasm-tools print module2.core.wasm > module2.core.wat 2> /dev/null || rm module2.core.wat
cd ..
