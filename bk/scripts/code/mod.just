set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../../..")
book_root := clean(root + "/bk")
code_root := clean(book_root + "/crates")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## ---- CODE BUILDING -----------------------------------

# Format the current crate
[no-cd]
@fmtcurrent:
  echo "Formatting $(pwd)"
  cargo +nightly fmt

# Format all packages and their local path-based dependencies
@fmtall:
  echo "Formatting all packages and their local path-based dependencies in {{code_root}}"
  cd {{code_root}}; cargo +nightly fmt --all

# Check code in the current crate (or the default packages of the workspace if in the root) (all features)
[no-cd]
@checkcurrent:
  echo "Check code in $(pwd) (all features)"
  cargo check --all-targets --locked --all-features

# Check all code (all features)
@checkall:
  echo "Check all code (all features) in {{code_root}}"
  cd {{code_root}}; cargo check --workspace --all-targets --locked --all-features
# `--all-targets` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build the package in the current crate (or the default packages of the workspace if in the root) (all features)
[no-cd]
@buildcurrent:
  echo "Building $(pwd) (all features)"
  cargo build --all-targets --locked --all-features

# Build a specific package (all features)
@buildpkg pkg:
  echo "Building {{pkg}} (all features)"
  cargo build --all-targets --locked --all-features --package {{pkg}}

# Build all code (default features)
@build:
  echo "Build all code (default features) in {{code_root}}"
  cd {{code_root}}; cargo build --workspace --all-targets --locked --timings

# Build all code (all features)
@buildall:
  echo "Build all code (all features) in {{code_root}}"
  cd {{code_root}}; cargo build --workspace --all-targets --locked --all-features --timings

# Scan all code for common mistakes (all features)
@clippyall:
  echo "Scan all code for common mistakes (all features) in {{code_root}}"
  cd {{code_root}}; cargo clippy --workspace --all-targets --locked --all-features -- -D warnings

# Test any doctests in the workspace
@doctest:
  echo "Test any doctests in the workspace in {{code_root}}"
  cd {{code_root}}; cargo test --workspace --doc --locked

# Test only tests that match a given keyword
@nextestthis test:
  cd {{code_root}} && cargo nextest run --locked --no-fail-fast --success-output final -E 'test(~{{test}}) or binary_id(~{{test}})'

current_crate := file_name(invocation_directory())

# Test in the current directory (or the default packages of the workspace if in the root) (all features)
[no-cd]
@nextestcurrent: _clean_temp_dir && _clean_temp_dir
  echo "Test {{current_crate}} (all features)"
  cargo nextest run --all-targets --locked --no-fail-fast --status-level all -E 'package({{current_crate}})' --all-features

# Test all code (no doctests) (default features)
@nextest: _clean_temp_dir && _clean_temp_dir
  echo "Test the code under {{code_root}} (no doctests, default features)"
  cd {{code_root}} && cargo nextest run --workspace --all-targets --locked --no-fail-fast --status-level all

# Test all code, including doctests (all features)
@nextestall: _clean_temp_dir && _clean_temp_dir
  echo "Test all code in {{code_root}} (incl. doctests) (all features)"
  cd {{code_root}} && cargo nextest run --workspace --all-targets --locked --no-fail-fast --status-level all --all-features && cargo test --doc --workspace

# Clean the `crates/**/temp` folders of most files prior / after testing
@_clean_temp_dir:
  echo "Clean temp folders"
  {{root}}/bin/clean {{code_root}}

# Run an example
[no-cd]
@run exmpl:
  cargo run --locked --example {{exmpl}}

## ---- HEAVY TESTS -----------------------------------

# Run tests using an external service: amqp cassandra elasticsearch meilisearch mongodb mssql postgres redis
[group('heavy tests')]
[unix]
@heavy_test feature:
  echo "Run heavy tests using {{feature}}"
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
