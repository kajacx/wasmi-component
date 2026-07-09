#!/usr/bin/sh
set -e

# Run from this directory

cd crates/wasmi-component/src/wasi_p2

cargo run --manifest-path ../../../../Cargo.toml -p wasmi-component-bindgen -- ./wasi-p2-partial.wit > ./bindgen.rs

sed -i 's/^use wasmi_component::/use crate::/' ./bindgen.rs

cd ../../../..

cargo fmt
cargo build
