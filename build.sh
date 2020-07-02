#!/bin/bash

cargo build --release;

# move headers + lib rust -> go
mkdir go/lib
rm -rf go/lib/*
cp rust-lib-ffi/target/release/librust.so go/lib/
cp rust-lib-ffi/headers/* go/lib/

pushd go;
go build -ldflags="-r lib" main.go;
popd;