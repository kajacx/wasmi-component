#!/usr/bin/sh
set -e

# Run from this directory

cargo run --manifest-path ../../../../Cargo.toml -p wasmi-component-bindgen ./wasi-p2-partial.wit > ./bindgen.rs

sed -i 's/^use wasmi_component::/use crate::/' ./bindgen.rs

cargo fmt --manifest-path ../../Cargo.toml
