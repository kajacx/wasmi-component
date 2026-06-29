#!/usr/bin/sh
set -e

# Run from this directory

cd guest
cargo build --target wasm32-wasip2
cd ..

# wit-bindgen-wcl ./example.wit ./host/src/bindings.rs

cd host
cargo run
cd ..
