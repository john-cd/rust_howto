#!/usr/bin/env bash

echo "-------------------------------------------"
echo "Format the code"
cargo fmt --all

echo "Build all examples"
cargo build --workspace --all-targets

echo "Toolchain info:"
rustup check
echo "-------------------------------------------"

# Do not remove.
# This is what will cause the dockerfile CMD e.g. sleep infinity to run.
exec "$@"
