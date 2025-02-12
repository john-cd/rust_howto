set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CODE BUILDING -----------------------------------

alias f := fmt

# Format all bin and lib files
fmt:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Formatting {{root}}"
  cd {{root}}
  cargo +nightly fmt

alias fa := fmtall

# Format all packages and their local path-based dependencies
@fmtall:
  echo "Formatting all packages and their local path-based dependencies in {{root}}"
  cargo +nightly fmt --all

alias fc := fmtcurrent

# Format the current crate
[no-cd]
@fmtcurrent:
  echo "Formatting $(pwd)"
  cargo +nightly fmt

alias ckc := checkcurrent

# Check code in the current crate (or the default packages of the workspace if in the root) (all features)
[no-cd]
@checkcurrent:
  echo "Check code in $(pwd) (all features)"
  cargo check --all-targets --locked --all-features

alias cka := checkall

# Check all code (all packages, all features)
@checkall:
  echo "Check all code (all packages, all features) in $(pwd)"
  cargo check --workspace --all-targets --locked --all-features
# `--all-targets` is equivalent to specifying `--lib --bins --tests --benches --examples`.

alias bc := buildcurrent

# Build the package in the current crate (or the default packages of the workspace if in the root) (all features)
[no-cd]
@buildcurrent:
  echo "Building $(pwd) (all features)"
  cargo build --all-targets --locked --all-features

alias bp := buildpkg

# Build a specific package (all features)
@buildpkg pkg:
  echo "Building {{pkg}} (all features)"
  cargo build --all-targets --locked --all-features --package {{pkg}}

alias b := build

# Build just the code used by the book (default packages, default features)
build:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Build default packages (default features) in {{root}}"
  cd {{root}}
  cargo build --all-targets --locked --timings

alias bf := buildfeat

# Build just the code used by the book (default packages, all features)
buildfeat:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Build default packages (all features) in {{root}}"
  cd {{root}}
  cargo build --all-targets --locked --all-features --timings

alias ba := buildall

# Build all packages (all features)
@buildall:
  echo "Build all packages, all features in {{root}}"
  cargo build --workspace --all-targets --locked --all-features --timings

alias ca := clippyall

# Scan all code for common mistakes (all packages, all features)
@clippyall:
  echo "Scan all code for common mistakes (all packages, all features) in {{root}}"
  cargo clippy --workspace --all-targets --locked --all-features -- -D warnings

#alias t := test

## Test the code used by the book (default packages, default features)
#@test: _clean_temp_dir && _clean_temp_dir
##  echo "Test the code used by the book (default packages, default features) in $(pwd)"
##  -cargo test --all-targets --locked -- --show-output
## Only the code in default members of the workspace is tested.
## This used to rely on skeptic to build doctests - see `build.rs` - but skeptic is slow
## NOTE: `mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/` is not reliable
## when dealing with dependencies outside of the std library.
## See: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

#alias ta := testall

## Test all code (all packages, all features)
#@testall: _clean_temp_dir && _clean_temp_dir
#  echo "Test all code (all packages, all features) in {{root}}"
#  -cargo test --workspace --all-targets --locked --all-features

# Test any doctests in the workspace
@doctest:
  echo "Test any doctests in the workspace in {{root}}"
  cd {{root}}; cargo test --workspace --doc --locked

alias ntc := nextestcurrent

current_crate := file_name(invocation_directory())

# Test in the current directory (or the default packages of the workspace if in the root) (all features)
@nextestcurrent: _clean_temp_dir && _clean_temp_dir
  echo "Test {{current_crate}} (all features)"
  cargo nextest run --all-targets --locked --no-fail-fast --status-level all -E 'package({{current_crate}})' --all-features

alias nt := nextest

# Test the code used by the book (no doctests) (default packages, default features)
nextest: _clean_temp_dir && _clean_temp_dir
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Test the code under {{root}} (no doctests, default packages, default features)"
  cd {{root}}
  cargo nextest run --all-targets --locked --no-fail-fast --status-level all

alias nta := nextestall

# Test all code in the workspace, including doctests (all features)
nextestall: _clean_temp_dir && _clean_temp_dir
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Test all code in the workspace in {{root}} (incl. doctests) (all features)"
  cd {{root}}
  cargo test --doc --workspace --quiet # nextest does not handle doctests
  cargo nextest run --workspace --all-targets --locked --no-fail-fast --status-level all --all-features

# Clean the `crates/**/temp` folders of most files prior / after testing
@_clean_temp_dir:
  {{root}}/../bin/clean
#cargo run --manifest-path {{justfile_directory()}}/../tools/Cargo.toml -p clean --quiet

alias r := run

# Run an example
[no-cd]
@run exmpl:
  cargo run --locked --example {{exmpl}}

## ---- HEAVY TESTS -----------------------------------

# Run tests using an external service: amqp cassandra elasticsearch meilisearch mongodb mssql postgres redis
[group('heavy tests')]
[unix]
@heavy_test feature:
  echo "Run tests using {{feature}} in {{root}}"
  ./heavy_test.sh {{root}} {{feature}}

# Run all tests that require external services
[group('heavy tests')]
[unix]
heavytestall:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Run all tests that require external services"
  for feature in "amqp" "cassandra" "elasticsearch" "meilisearch" "mongodb" "mssql" "postgres" "redis"
  do
    echo ">> $feature"
    just code heavy_test ${feature}
    echo "------------------------"
  done

# # Test the Redis connection
#[group('heavy tests')]
#[unix]
#test_redis_conn:
#  #!/usr/bin/env bash
#  set -euo pipefail
#  echo "Test the Redis connection"
#  d="{{root}}/.devcontainer/"
#  docker compose -f ${d}compose.yaml \
#    -f ${d}compose.override.yaml \
#    -f ${d}compose-heavy-tests.yaml \
#    up --wait redis
#  docker run -it --rm --network rust_howto_dev_default --name redis-cli redis:alpine redis-cli -h rust_howto_dev-redis-1 -p 6379

# # Test the Mongo connection
# [group('heavy tests')]
# [unix]
# test_mongo_conn:
#   #!/usr/bin/env bash
#   set -euo pipefail
#   echo "Test the Mongo connection"
#   d="{{root}}/.devcontainer/"
#   docker compose -f ${d}compose.yaml \
#     -f ${d}compose.override.yaml \
#     -f ${d}compose-heavy-tests.yaml \
#     up --wait mongodb
#   docker run -it --rm --network rust_howto_dev_default --name mongo-cli mongo mongosh mongodb://mongoadmin:mysecretpassword@rust_howto_dev-mongodb-1:27017/
# --host rust_howto_dev-mongodb-1 --username mongoadmin --password mysecretpassword
