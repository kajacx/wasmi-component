#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

# wit-bindgen-wcl ./example.wit ./host/src/bindings.rs
rm -rf modules
mkdir modules

cd host
cargo run || true
cd ..

cd modules
wasm-tools print module0.core.wasm > module0.core.wat
wasm-tools print module1.core.wasm > module1.core.wat
wasm-tools print module2.core.wasm > module2.core.wat
cd ..
