#!/usr/bin/env bash

set -euo pipefail

echo "Running profiling with dhat-heap..."
cargo run --features dhat-heap

echo "Running profiling with dhat-ad-hoc..."
cargo run --features dhat-ad-hoc
