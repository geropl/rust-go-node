#!/bin/bash
set -Eeuo pipefail

cargo build --release;

# Build TS/node calling Rust/wasm
# generate wasm
pushd rust-lib-wasm;
yarn build;
popd;

# installs wasm and builds nodejs
yarn install;
pushd nodejs;
yarn build;
popd;

# Build go calling Rust
# move FFI headers + lib rust -> go
mkdir -p go/lib
rm -rf go/lib/*
cp target/release/librust_lib_ffi.so go/lib/
cp rust-lib-ffi/headers/* go/lib/

pushd go;
go build -ldflags="-r lib" main.go;
popd;