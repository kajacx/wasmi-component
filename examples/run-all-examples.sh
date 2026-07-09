#!/usr/bin/sh
set -e

# Run from this directory

cd ..
./build.sh
cd examples

./run-example.sh component/primitives --skip-build
./run-example.sh component/lists --skip-build
./run-example.sh wasi/stdio --skip-build

echo "All examples finished successfully"
