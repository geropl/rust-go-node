FROM gitpod/workspace-full:latest

# FFI
RUN bash -lc "cargo install cbindgen"

# wasm
RUN bash -lc "cargo install wasm-pack"

# tools
RUN bash -lc "cargo install cargo-udeps && rustup toolchain install nightly-x86_64-unknown-linux-gnu"
RUN bash -lc "cargo install cargo-bloat --features regex-filter"
