_default:
  @just --list code --unsorted --justfile {{justfile()}}
#@just --choose

## ---- CODE BUILDING -----------------------------------

alias b := build
alias ba := buildall
alias ca := clippyall
alias f := fmtall
alias fa := fmtall
alias nta := nextestall
alias t := test
alias ta := testall

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
  cargo test --package deps --tests --locked -- --show-output || true
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
  {{justfile_directory()}}/scripts/runall.sh
