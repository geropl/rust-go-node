#!/bin/bash

cargo build --release;

# generate wasm
pushd rust-lib-wasm;
wasm-pack build --target nodejs;
popd;

# move FFI headers + lib rust -> go
mkdir go/lib
rm -rf go/lib/*
cp rust-lib-ffi/target/release/librust.so go/lib/
cp rust-lib-ffi/headers/* go/lib/

pushd go;
go build -ldflags="-r lib" main.go;
popd;