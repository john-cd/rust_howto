#! /bin/bash

echo "-------------------------------------------"
echo "Format the code"
cargo fmt --all

echo "Build all examples"
cargo build --workspace --all-targets

echo "Toolchain info:"
rustup check
echo "-------------------------------------------"
