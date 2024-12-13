_default:
  @just --list code --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CODE BUILDING -----------------------------------

# Format all code
fmtall:
  cargo +nightly fmt --all

# Check all code
checkall:
  cargo check --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build the code used by the book (`deps` crate only`)
build:
  cargo build --package dependencies --tests --locked

# Build code examples that are gated under features
build_with_features:
  cargo build --package dependencies --tests --locked --features postgres,redis,elasticsearch,meilisearch,mongodb,typesense

# Build all code
buildall:
  cargo build --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Scan all code for common mistakes
clippyall:
  cargo clippy --workspace --all-targets --locked -- -D warnings

# Test the code used by the book (`deps` crate only)
test: _clean_temp_dir && _clean_temp_dir
  cargo test --package dependencies --tests --locked -- --show-output || true
# Only the code in the `deps` package is tested.
# This used to rely on skeptic to build doctests - see `build.rs` - but skeptic is slow
# NOTE: `mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/` is not reliable
# when dealing with dependencies outside of the std library.
# See: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Test all code
testall: _clean_temp_dir && _clean_temp_dir
  cargo test --workspace --all-targets --locked || true
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Test all code using nextest
nextestall: _clean_temp_dir && _clean_temp_dir
  cargo nextest run --workspace --all-targets --locked --no-fail-fast || true
  cargo test --doc --workspace --quiet || true # nextest does not handle doctests

# Clean the `deps/temp` folder of most files prior / after testing
_clean_temp_dir:
  cargo run -p clean --quiet

# Run additional examples (under the `xmpl` folder)
[unix]
runall:
  ./runall.sh


## ---- HEAVY TESTS -----------------------------------

# Run tests using an external service: elasticsearch meilisearch mongodb postgres redis typesense
[group('heavy tests')]
heavy_test feature:
  ./heavy_test.sh {{ feature }}

# Run all tests that require external services
[group('heavy tests')]
heavytestall:
  #! /bin/bash
  set -euo pipefail
  for feature in redis postgres # "elasticsearch" "meilisearch" "mongodb" "postgres" "redis" "typesense"
  do
    echo ">> $feature"
    just heavy test ${feature}
    echo "------------------------"
  done

# Test the Redis connection
[group('heavy tests')]
test_redis_connection:
  #! /bin/bash
  set -euo pipefail
  d="{{justfile_directory()}}/.devcontainer/"
  docker compose -f ${d}compose.yaml \
    -f ${d}compose.override.yaml \
    -f ${d}compose-heavy-tests.yaml \
    up --wait redis
  docker run -it --rm --network rust_howto_dev_default --name redis-cli redis:alpine redis-cli -h rust_howto_dev-redis-1 -p 6379
