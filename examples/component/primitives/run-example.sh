#!/usr/bin/sh
set -e

# Run from this directory

wasm_path=wasm32-unknown-unknown/debug/wasmi_component_example_guest

cd guest
cargo expand > src/expanded.rs
cargo component build --target wasm32-unknown-unknown
wasm-tools print "target/$wasm_path.wasm" > "target/$wasm_path.wat"
cd ..

rm -rf modules
mkdir modules
chmod +777 modules
wasm-tools.exe component unbundle --module-dir modules --output modules "guest/target/$wasm_path.wasm" 2> /dev/null || true

cd modules
wasm-tools print unbundled-module0.wasm > unbundled-module0.wat
wasm-tools print unbundled-module1.wasm > unbundled-module1.wat 2> /dev/null || rm unbundled-module1.wat
wasm-tools print unbundled-module2.wasm > unbundled-module2.wat 2> /dev/null || rm unbundled-module2.wat
cd ..

cargo run --manifest-path ../../../Cargo.toml -p wasmi-component-bindgen -- example.wit > host/src/bindings.rs

cd host
cargo run || true
cd ..
