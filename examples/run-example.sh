#!/usr/bin/sh
set -e

# Run from this directory

path="$1"
if [[ "$path" == wasi* ]]; then
    target=wasm32-wasip2
    wasi=true
else
    target=wasm32-unknown-unknown
    wasi=false
fi
wasm_path="$target/debug/wasmi_component_example_guest"

cd "$path"

cd guest
cargo expand > src/expanded.rs
cargo component build --target "$target"
wasm-tools print "target/$wasm_path.wasm" > "target/$wasm_path.wat"
cd ..

if $wasi; then
    echo "// This is a PARTIAL wasip2 wit file generated from the component example!" > example-wasi-p2-partial.wit
    echo >> example-wasi-p2-partial.wit
    wasm-tools component wit "guest/target/$wasm_path.wasm" >> example-wasi-p2-partial.wit
fi

rm -rf modules component.wit
mkdir modules
wasm-tools component unbundle -t --module-dir modules --threshold 0 "guest/target/$wasm_path.wasm" > component.wat

cd modules
wasm-tools print unbundled-module0.wasm > unbundled-module0.wat
wasm-tools print unbundled-module1.wasm > unbundled-module1.wat 2> /dev/null || rm unbundled-module1.wat
wasm-tools print unbundled-module2.wasm > unbundled-module2.wat 2> /dev/null || rm unbundled-module2.wat
cd ..

cargo run --manifest-path ../../../Cargo.toml -p wasmi-component-bindgen -- example.wit > host/src/bindings.rs

if [[ "$2" != "--skip-build" ]]; then
    cd ../../..
    ./build.sh
    cd "examples/$path"
fi

cd host
cargo fmt
cargo run > output.log 2> error.log || true
cat output.log
cat error.log
cd ..
