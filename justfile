alias b := build
alias ba := buildall
alias ca := clippyall
alias s := serve
alias t := test
alias ta := testall
alias f := fmtall
set windows-shell := ["cmd.exe", "/c"]

default:
  @just --list --unsorted
# or: @just --choose

# Clean Cargo's `target` and mdbook's `book` directories
clean: &&_clean
  cargo clean
  mdbook clean

[unix]
_clean:
  rm --recursive --force ./doctest_cache/

[windows]
_clean:
  if exist .doctest_cache rmdir /s /q .doctest_cache

# Format all code
fmtall:
  cargo +nightly fmt --all

# Check all code
checkall: _build-book
  cargo check --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build all code
buildall: _build-book
  cargo build --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Scan all code for common mistakes
clippyall: _build-book
  cargo clippy --workspace --all-targets --locked

# Test all code
testall: _build-book
  cargo test --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Run all examples
[unix]
runall: _build-book
  #! /bin/bash
  set -o pipefail
  set -e
  # Run examples that are simple .rs files
  examples=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type f | xargs basename --suffix=.rs | tr '\n' ' ')
  for e in $examples; do ( echo $e; cargo run --example $e --locked || true); done
  # Run examples that are in a folder
  examples_in_dir=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type d | xargs basename --multiple | tr '\n' ' ')
  for e in $examples_in_dir; do ( echo $e; cargo run --example $e --locked || true ); done
  # Create a list of the (last part of) folder names under the `xmpl` directory, space separated
  xmpl=$(find ./xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' ')
  # Also run additional examples in the xmpl folder, if any
  for d in $xmpl; do ( echo $d; cargo run --package $d --locked ); done
# TODO: this still repeatedly runs `build.rs` somehow.

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
build: _generate-refdefs _generate-index-category _build-book && _sitemap _copystatic

# Generate the expanded markdown (input for skeptic) and the book's HTML / JS
_build-book:
  mdbook build

# Generate new reference definitions for all crate the book's examples depend on...
_generate-refdefs:
  # TODO

# Generate the index and the category page.
_generate-index-category:
  # TODO

# Add static assets to book output
[unix]
_copystatic:
  cp static/*.* book/html/

[windows]
_copystatic:
  copy static\*.* book\html\

# Generate the sitemap.xml file
_sitemap:
  cargo run -p utils --bin sitemap

# Test all examples in the book's Markdown
test: _build-book
  cargo test --package deps --tests --examples --locked -- --show-output
# This relies on skeptic to build doctests - see `build.rs`
# NOTE: `mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/` is not reliable
# when dealing with dependencies outside of the std library
# see: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Serve the book (incl. link checking)
serve: _build-book
  mdbook serve --open
# To change the port: --port 3001

# Watch the book's markdown files and rebuilds it on changes
# watch: _build-book
#   mdbook watch --open

# Prepare for git push
prep: fmtall clean build clippyall testall build serve

## Utilities --------------------------------------

# Update Cargo.lock dependencies for all projects (incl. dependencies used by the book's examples and additional examples in the xmpl folder)
[confirm]
update:
  cargo update

default := 'help'
empty := ''

# Manage links, ref definitions, etc...
do cmd=default subcmd=empty:
  cargo run -p utils --bin do -- {{cmd}} {{subcmd}}

run xmpl:
  cargo run -p deps --example {{xmpl}}
