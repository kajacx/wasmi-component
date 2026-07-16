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

path="${path%/}" # remove trailing /
example="${path#*/}" # get the second part of path, which is the example name

guest="example_${example}_guest"
host="example_${example}_host"

wasm_path="$target/debug/$guest"

guest="${guest//_/-}" # replace _ with -
host="${host//_/-}" # replace _ with -

# --- GUEST ---

cd ../guests

cargo expand -p "$guest" > "$path/src/expanded.rs"
cargo component build -p "$guest" --target "$target"

if $wasi; then
    echo "// This is a PARTIAL wasip2 wit file generated from the component example!" > "$path/example-wasi-p2-partial.wit"
    echo >> "$path/example-wasi-p2-partial.wit"
    wasm-tools component wit "target/$wasm_path.wasm" >> "$path/example-wasi-p2-partial.wit"
fi

pwd
rm -rf "$path/modules" "$path/component.wat"
mkdir "$path/modules"
wasm-tools component unbundle -t --module-dir "$path/modules" --threshold 0 "target/$wasm_path.wasm" > "$path/component.wat"

cd "$path/modules"
wasm-tools print unbundled-module0.wasm > unbundled-module0.wat
wasm-tools print unbundled-module1.wasm > unbundled-module1.wat 2> /dev/null || rm unbundled-module1.wat
wasm-tools print unbundled-module2.wasm > unbundled-module2.wat 2> /dev/null || rm unbundled-module2.wat
cd ../../..

# --- HOST ---

cd ../examples

cargo run --manifest-path ../Cargo.toml -p wasmi-component-bindgen -- "$path/example.wit" > "$path/src/bindings.rs"

if [[ "$2" != "--skip-build" ]]; then
    cd ..
    ./build.sh
    cd examples
fi

cargo fmt -p "$host"
cargo expand -p "$host" bindings > "$path/src/expanded.rs"

status=0
cargo run -p "$host" > "$path/output.log" 2> "$path/error.log" || status=$?

cat "$path/output.log"
cat "$path/error.log"

if [[ $status -ne 0 ]]; then
    echo "Example $path has failed"
fi

exit $status
