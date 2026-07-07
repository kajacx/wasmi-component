#!/usr/bin/sh
set -e

# Run from this directory

wasm_path=wasm32-wasip2/debug/wasmi_component_example_guest

cd guest
cargo expand > src/expanded.rs
# cargo component build --target wasm32-wasip2
cargo build --target wasm32-wasip2
wasm-tools print "target/$wasm_path.wasm" > "target/$wasm_path.wat"
cd ..

echo "// This is a PARTIAL wasip2 wit file generated from the component example!" > example-wasi-p2-partial.wit
echo >> example-wasi-p2-partial.wit
wasm-tools component wit "guest/target/$wasm_path.wasm" >> example-wasi-p2-partial.wit

rm -rf modules component.wit
mkdir modules
wasm-tools component unbundle -t --module-dir modules --threshold 0 "guest/target/$wasm_path.wasm" > component.wat

cd modules
wasm-tools print unbundled-module0.wasm > unbundled-module0.wat
wasm-tools print unbundled-module1.wasm > unbundled-module1.wat 2> /dev/null || rm unbundled-module1.wat
wasm-tools print unbundled-module2.wasm > unbundled-module2.wat 2> /dev/null || rm unbundled-module2.wat
cd ..

cargo run --manifest-path ../../../Cargo.toml -p wasmi-component-bindgen -- example.wit > host/src/bindings.rs

cd ../../..
./build.sh
cd examples/wasi/stdio

cd host
cargo fmt
cargo run 2> error.log || true
cat error.log
cd ..
