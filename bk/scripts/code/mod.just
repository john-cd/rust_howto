# Purpose: Format, build, lint, and test the Rust example workspaces.
# Usage: From bk/, run just code <recipe>.
#
# TODO Reviewed ??-??-2026.

set windows-shell := ["C:/Program Files/Git/bin/bash.exe", "-uc"]

exe := if os_family() == "windows" { ".exe" } else { "" }
root := replace(clean(source_directory() + "/../../.."), "\\", "/")
book_root := clean(root + "/bk")
code_root := clean(book_root + "/crates")
rustfmt_toolchain_default := trim(replace_regex(read(join(root, "rustfmt-toolchain.toml")), '(?s).*channel\s*=\s*"([^"]+)".*', "$1"))
rustfmt_toolchain_override := env("RUSTFMT_TOOLCHAIN", "")
rustfmt_toolchain := if rustfmt_toolchain_override == "" { rustfmt_toolchain_default } else { rustfmt_toolchain_override }

[no-exit-message]
_default:
    @just --list --unsorted --justfile {{ quote(source_file()) }}

## ---- CODE BUILDING -----------------------------------

# Format the current crate
[no-cd]
@fmtcurrent:
    echo "Formatting $(pwd)"
    cargo +{{ rustfmt_toolchain }} fmt

# Format all packages and their local path-based dependencies
@fmtall:
    echo "Formatting all packages and their local path-based dependencies in {{ quote(code_root) }}"
    cd {{ quote(code_root) }}; cargo +{{ rustfmt_toolchain }} fmt --all

# Format Cargo manifests in the book examples workspace
@fmt-toml:
    cargo fmt-toml --workspace-path {{ quote(code_root) }}

# Check Cargo manifest formatting in the book examples workspace
@fmt-toml-check:
    cargo fmt-toml --check --workspace-path {{ quote(code_root) }}

# Check code in the current crate (or the default packages of the workspace if in the root) (all features)
[no-cd]
@checkcurrent:
    echo "Check code in $(pwd) (all features)"
    cargo check --all-targets --locked --all-features

# Check all code (all features)
@checkall:
    echo "Check all code (all features) in {{ code_root }}"
    cd {{ quote(code_root) }}; cargo check --workspace --all-targets --locked --all-features

# `--all-targets` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build the package in the current crate (or the default packages of the workspace if in the root) (all features)
[no-cd]
@buildcurrent:
    echo "Building $(pwd) (all features)"
    cargo build --all-targets --locked --all-features

# Build a specific package (all features)
@buildpkg pkg:
    echo "Building {{ pkg }} (all features)"
    cargo build --all-targets --locked --all-features --package {{ pkg }}

# Build all code (default features)
@build:
    echo "Build all code (default features) in {{ quote(code_root) }}"
    cd {{ quote(code_root) }}; cargo build --workspace --all-targets --locked --timings

# Build all code (all features)
@buildall:
    echo "Build all code (all features) in {{ quote(code_root) }}"
    cd {{ quote(code_root) }}; cargo build --workspace --all-targets --locked --all-features --timings

# Scan all code for common mistakes (all features)
@clippyall:
    echo "Scan all code for common mistakes (all features) in {{ quote(code_root) }}"
    # sccache 0.18.0 expands the Leptos web-sys rustc arguments past Windows'
    # process command-line limit, so lint this package directly and separately.
    cd {{ quote(code_root) }}; cargo clippy --workspace --exclude web_programming_http_server --all-targets --locked --all-features -- -D warnings
    cd {{ quote(code_root) }}; RUSTC_WRAPPER= cargo clippy --package web_programming_http_server --all-targets --locked --all-features -- -D warnings

# Test any doctests in the workspace
@doctest:
    echo "Test any doctests in the workspace in {{ quote(code_root) }}"
    cd {{ quote(code_root) }}; cargo test --workspace --doc --locked

# Test only tests that match a given keyword
@nextestthis test:
    cd {{ quote(code_root) }} && cargo nextest run --config-file {{ quote(code_root + "/nextest.toml") }} --locked --no-fail-fast --success-output final -E 'test(~{{ test }})'

current_crate := file_name(invocation_directory())

# Test in the current directory (or the default packages of the workspace if in the root) (all features)
[no-cd]
@nextestcurrent: _clean_temp_dir && _clean_temp_dir
    echo "Test {{ current_crate }} (all features)"
    cargo nextest run --config-file {{ quote(code_root + "/nextest.toml") }} --all-targets --locked --no-fail-fast --status-level all -E 'package({{ current_crate }})' --all-features

# Test all code (no doctests) (default features)
@nextest: _clean_temp_dir && _clean_temp_dir
    echo "Test the code under {{ quote(code_root) }} (no doctests, default features)"
    cd {{ quote(code_root) }} && cargo nextest run --config-file {{ quote(code_root + "/nextest.toml") }} --workspace --all-targets --locked --no-fail-fast --status-level all

# Test all code, including doctests (all features)
@nextestall: _clean_temp_dir && _clean_temp_dir
    echo "Test all code in {{ quote(code_root) }} (incl. doctests) (all features)"
    # sccache 0.18.0 expands web-sys rustc arguments past Windows' process command-line limit.
    cd {{ quote(code_root) }}; cargo nextest run --config-file {{ quote(code_root + "/nextest.toml") }} --workspace --exclude web_programming_http_server --all-targets --locked --no-fail-fast --status-level all --all-features
    cd {{ quote(code_root) }}; RUSTC_WRAPPER= cargo nextest run --config-file {{ quote(code_root + "/nextest.toml") }} --package web_programming_http_server --all-targets --locked --no-fail-fast --status-level all --all-features
    cd {{ quote(code_root) }}; cargo test --doc --workspace

# Clean the `crates/**/temp` folders of most files prior / after testing
@_clean_temp_dir:
    echo "Clean temp folders"
    {{ quote(root + "/bin/clean" + exe) }} {{ quote(code_root) }}

# Run an example
[no-cd]
@run exmpl:
    cargo run --locked --example {{ exmpl }}

## ---- HEAVY TESTS -----------------------------------

# Run tests using an external service: amqp cassandra elasticsearch meilisearch mongodb mssql postgres redis
[group('heavy tests')]
@heavy_test feature:
    echo "Run heavy tests using {{ feature }}"
    ./heavy_test.sh {{ root }} {{ feature }}

# Run all tests that require external services
[group('heavy tests')]
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
