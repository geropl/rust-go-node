#!/bin/bash

ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

pushd rust;
cargo build --release;
popd;

# move headers + lib rust -> go
mkdir go/lib
rm -rf go/lib/*
cp rust/target/release/librust.so go/lib/
cp rust/headers/* go/lib/

pushd go;
go build -ldflags="-r lib" main.go;
popd;