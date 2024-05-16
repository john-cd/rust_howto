alias b := build
alias ba := buildall
alias ca := clippyall
alias s := serve
alias t := test
alias ta := testall
alias nta := nextestall
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

# Test all code using nextest
nextestall: _build-book
  cargo nextest run --workspace --all-targets --locked --no-fail-fast
  cargo test --doc --workspace

# Run all examples (but not the tests)
[unix]
runall: _build-book
  #! /bin/bash
  set -o pipefail
  set -e
  ## Examples under the deps folder (removed)
  ## Run examples that are simple .rs files
  #examples=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type f | xargs basename --suffix=.rs | tr '\n' ' ')
  #for e in $examples; do ( echo $e; cargo run --example $e --locked || true); done
  ## Run examples that are in a folder
  #examples_in_dir=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type d | xargs basename --multiple | tr '\n' ' ')
  #for e in $examples_in_dir; do ( echo $e; cargo run --example $e --locked || true ); done
  # Create a list of the (last part of) folder names under the `xmpl` directory, space separated
  xmpl=$(find ./xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' ')
  # Also run additional examples in the xmpl folder, if any
  for d in $xmpl; do ( echo $d; cargo run --package $d --locked ); done
# TODO: this still repeatedly runs `build.rs` somehow.

# Run a specific example (among those in `deps/examples`)
run example: _build-book
  #cargo clean -p deps
  cargo run -p deps --locked --example {{example}}

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
build: _generate-refdefs _generate-index-category _build-book && _sitemap _copystatic

# Generate the expanded markdown (input for skeptic) and the book's HTML / JS
_build-book:
  mdbook build

# Generate new reference definitions for all crate the book's examples depend on...
_generate-refdefs:
# TODO mdbook-utils refdefs

# Generate the index and the category page.
_generate-index-category:
# TODO mdbook-utils

# Add static assets to book output
[unix]
_copystatic:
  cp static/*.* book/html/

[windows]
_copystatic:
  copy static\*.* book\html\

# Generate the sitemap.xml file
_sitemap:
  mdbook-utils sitemap

# Test the code used by the book
test: _build-book
  cargo test --package deps --tests --locked -- --show-output
# This relies on skeptic to build doctests - see `build.rs`
# NOTE: `mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/` is not reliable
# when dealing with dependencies outside of the std library.
# See: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Serve the book (incl. link checking)
serve: build
  cd book/html ; python3 -m http.server 3000
# TODO make work from Dev Containers: mdbook serve --open
# To change the port: --port 3001

# Watch the book's markdown files and rebuilds it on changes
# watch: _build-book
#   mdbook watch --open

# Prepare for git push
prep: spell fmtall clean build clippyall testall build serve

## Documentation --------------------------------------

# Build and display the `cargo doc` documentation for a specific package (e.g. deps)
[unix]
doc pkg:
  cargo clean --doc
  cargo doc --no-deps --document-private-items --locked --package {{pkg}}
  cd /cargo-target-rust_howto/target/doc/ ; python3 -m http.server 9000

# Build and display the `cargo doc` documentation for all packages
[unix]
docall:
  cargo clean --doc
  cargo doc --no-deps --workspace --locked
  # optional: --bins --examples
  # cargo doc --open does not seem to work when running from a Dev Container in VS Code;
  # the script that opens URLs into an external browser (see `$ echo $BROWSER`) does not handle raw HTML.
  cd /cargo-target-rust_howto/target/doc/ ; python3 -m http.server 9000
  # We could also use `live server` for dynamic reloading.
  # See README.md for other alternatives, such as:
  # xdg-open /cargo-target-rust_howto/target/doc/deps/index.html

## Utilities --------------------------------------

# Update Cargo.lock dependencies for all projects (incl. dependencies used by the book's examples and additional examples in the xmpl folder)
[confirm]
update:
  cargo update

help := 'help'
empty := ''

# Manage links, ref definitions, etc...
do cmd=help subcmd=empty:
  mdbook-utils {{cmd}} {{subcmd}}

[unix]
sortrefs:
  sort -u ./src/refs/crate-refs.md -o /tmp/c.md
  mv -f /tmp/c.md ./src/refs/crate-refs.md
  rm -f /temp/c.md
  sort -u ./src/refs/link-refs.md -o /tmp/l.md
  mv -f /tmp/l.md ./src/refs/link-refs.md
  rm -f /temp/l.md

spell:
  .devcontainer/spellcheck.sh
