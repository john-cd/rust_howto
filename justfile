# list of the (last part of) folder names under the `xmpl` directory, space separated
xmpl := `find ./xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' '`

default:
  @just --list --unsorted
# or: @just --choose

# Clean Cargo's `target` and mdbook's `book` and `doctest_cache` directories
clean:
  cargo clean
  mdbook clean
  rm --recursive --force ./doctest_cache/

# # Format the code of all projects in the xmpl folder
# xfmt:
#   for d in {{xmpl}}; do ( echo $d; cargo fmt -v --package $d ); done

# # Scan the code of all projects in the xmpl folder for common mistakes
# xclippy:
#   for d in {{xmpl}}; do ( echo $d; cargo clippy --package $d ); done

# # Check all projects in the xmpl folder (and all of their dependencies) for errors
# xcheck:
#   for d in {{xmpl}}; do ( echo $d; cargo check --package $d ); done

# # Compile all projects in the xmpl folder
# xbuild:
#   for d in {{xmpl}}; do ( echo $d; cargo build --package $d ); done

# # Run all projects in the xmpl folder
# xrun:
#   for d in {{xmpl}}; do ( echo $d; cargo run --package $d ); done

# Format all examples
fmtall:
  cargo fmt --all

# Scan the code of all examples for common mistakes
clippyall:
  cargo clippy --workspace --all-targets --locked

# Check all examples
checkall:
  cargo check --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build all examples
buildall:
  cargo build --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Test all examples
testall:
  cargo test --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Run all examples
runall:
  #! /bin/bash
  set -o pipefail
  set -e
  # Run examples that are simple .rs files
  examples=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type f | xargs basename --suffix=.rs | tr '\n' ' ')
  for e in $examples; do ( echo $e; cargo run --example $e --locked || true); done
  # Run examples that are in a folder
  examples_in_dir=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type d | xargs basename --multiple | tr '\n' ' ')
  for e in $examples_in_dir; do ( echo $e; cargo run --example $e --locked || true ); done
  # Also run additional examples in the xmpl folder, if any
  for d in {{xmpl}}; do ( echo $d; cargo run --package $d --locked ); done

# Build the book from its markdown files
build:
  mdbook build
  # Add static assets
  cp static/*.* book/

# Test the examples embedded in the markdown
test:
  mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/
# see: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Serve the book (incl. testing of the examples embedded in the markdown)
serve: build
  mdbook serve --open
# to change the port: --port 3001

# Watch the book's markdown files and rebuilds it on changes
# watch:
#   mdbook watch --open

# Update Cargo.lock dependencies for all projects (incl. dependencies used by the book's examples and additional examples in the xmpl folder)
update:
  cargo update
