#!/usr/bin/sh
set -e

# Run from this directory

cd crates/wasmi-component/src/wasi_p2
./generate.sh
cd ../../../..

cargo fmt
cargo build
