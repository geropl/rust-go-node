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
pushd go;
yarn install && yarn build
popd;