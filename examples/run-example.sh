#!/usr/bin/sh
set -e

# Run from this directory

# --- PATH ---

path="$1"
shift
if [[ "$path" == wasi* ]]; then
    target=wasm32-wasip2
    wasi=true
else
    target=wasm32-unknown-unknown
    wasi=false
fi

path="${path%/}" # remove trailing /
example="${path#*/}" # get the second part of path, which is the example name

guest="example_guest_${example}"
host="example_host_${example}"

wasm_path="$target/debug/$guest"

# --- OTHER ARGS ---

skip_build=""
manual_impl=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        -s|--skip-build)
            skip_build="--skip-build"
            shift
            ;;
        -m|--manual-impl)
            manual_impl="--manual-impl"
            shift
            ;;
        *)
            break
            ;;
    esac
done

# --- GUEST ---

cd ../guests

cargo expand -p "$guest" > "$path/src/expanded.rs"
cargo component build -p "$guest" --target "$target"

if $wasi; then
    echo "// This is a PARTIAL wasip2 wit file generated from the component example!" > "$path/example-wasi-p2-partial.wit"
    echo >> "$path/example-wasi-p2-partial.wit"
    wasm-tools component wit "target/$wasm_path.wasm" >> "$path/example-wasi-p2-partial.wit"
fi

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

if [[ "$manual_impl" == "--manual-impl" ]]; then
    cargo run --manifest-path ../Cargo.toml -p wasmi-component-bindgen -- --manual-impl "$path/example.wit" > "$path/src/bindings.rs"
else
    cargo run --manifest-path ../Cargo.toml -p wasmi-component-bindgen -- "$path/example.wit" > "$path/src/bindings.rs"
fi

if [[ "$skip_build" != "--skip-build" ]]; then
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

if [[ $status -eq 0 ]]; then
    echo "Example $path finished successfully"
else
    echo "Example $path has failed"
fi

exit $status
