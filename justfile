alias b := build
alias ba := buildall
alias bb := buildbook
alias ca := clippyall
alias f := fmtall
alias fa := fmtall
alias nta := nextestall
alias q := quick
alias s := serve
alias t := test
alias ta := testall

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
checkall:
  cargo check --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Build the code used by the book (`deps` crate only`)
build:
  cargo build --package deps --tests --locked

# Build all code
buildall:
  cargo build --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
# optional: --timings

# Scan all code for common mistakes
clippyall:
  cargo clippy --workspace --all-targets --locked

# Test the code used by the book (`deps` crate only)
test:
  cargo test --package deps --tests --locked -- --show-output
# Only the code in the `deps` package is tested.
# This used to rely on skeptic to build doctests - see `build.rs` - but skeptic is slow
# NOTE: `mdbook test --library-path /cargo-target-rust_howto/target/debug/deps/` is not reliable
# when dealing with dependencies outside of the std library.
# See: https://doc.rust-lang.org/rustdoc/command-line-arguments.html#-l--library-path-where-to-look-for-dependencies

# Test all code
testall:
  cargo test --workspace --all-targets --locked
# `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.

# Test all code using nextest
nextestall:
  cargo nextest run --workspace --all-targets --locked --no-fail-fast
  cargo test --doc --workspace # nextest does not handle doctests

# Run additional examples (under the `xmpl` folder)
[unix]
runall:
  #! /bin/bash
  set -o pipefail
  set -e
  # Create a list of the (last part of) folder names under the `xmpl` directory, space separated
  xmpl=$(find ./xmpl -mindepth 1 -maxdepth 1 -type d | awk -F'/' '{print $(NF)}' | tr '\n' ' ')
  # Run additional examples in the xmpl folder, if any
  for d in $xmpl; do ( echo $d; cargo run --package $d --locked ); done

## The examples under the deps folder have been replaced by tests.
## Should you want to add examples again, add the following to `runall`:
## Run examples that are simple .rs files in deps/examples
#examples=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type f | xargs basename --suffix=.rs | tr '\n' ' ')
#for e in $examples; do ( echo $e; cargo run --example $e --locked || true); done
## Run examples that are in a subfolder of deps/examples
#examples_in_dir=$(find ./deps/examples -mindepth 1 -maxdepth 1 -type d | xargs basename --multiple | tr '\n' ' ')
#for e in $examples_in_dir; do ( echo $e; cargo run --example $e --locked || true ); done

## The examples under the deps folder have been replaced by tests.
# # Run a specific example (among those in `deps/examples`)
# run example:
#   #cargo clean -p deps
#   cargo run -p deps --locked --example {{example}}

# Update Cargo.lock dependencies for all projects (incl. the book's examples, tools, and additional examples in `xmpl`)
[confirm]
update:
  cargo update

## ---- BOOK BUILDING -----------------------------------

# Build the book from its Markdown files (incl. refdefs, index, categories, sitemap, and static assets)
buildbook: _generate-refdefs _generate-index-category _mdbook-build-book && _sitemap _copystatic

# Generate the expanded markdown (input for skeptic) and the book's HTML / JS
[unix]
_mdbook-build-book:
  #! /bin/bash
  set -e
  if [ ! -f ./book.toml ]; then
    cp -f ./book.toml.bak ./book.toml
  fi
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

# Serve the book (incl. link checking)
serve: build
  mdbook serve -p 3000 -n 127.0.0.1 --open
  ## NOTE: conflicts with "port" / EXPOSE in the Docker / Docker compose configuration
  ## Or use: cd book/html ; python3 -m http.server 3000

# Serve the book from its Markdown files, skipping link checking and preprocessors for speed; rebuilds it on changes
[unix]
quick:
  #! /bin/bash
  set -o pipefail
  set -e
  # function called by trap
  cleanup() {
    cp -f ./book.toml.bak ./book.toml
    exit
  }
  trap cleanup 1 2 3 6
  if [ -f ./book.toml ]; then
    mv -f ./book.toml ./book.toml.bak
  fi
  MDBOOK_BOOK='{"title": "QUICK SERVE"}' mdbook serve -p 3001 -n 127.0.0.1 --open
# Note1: Using the env variable MDBOOK_* only seems to override existing values, not erase them.
# Examples:
#   MDBOOK_BOOK="$(toml2json ./book-dev.toml)" mdbook build
#   MDBOOK_OUTPUT__LINKCHECK='{"warning-policy": "ignore"}' MDBOOK_PREPROCESSOR__INDEXING='{"skip_renderer": "html,markdown,linkcheck"}'
# See the doc on overriding mdbook config: https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html
#
# Note2: mdbook watch --open --watcher=poll / native does not have -p -n options.

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

# Prepare for git push
prep: spell sortrefs fmtall clean clippyall testall buildbook serve

help := 'help'
empty := ''

# Call mdbook-utils to manage links, ref definitions, etc...
utils cmd=help *subcmd=empty:
  mdbook-utils {{cmd}} {{subcmd}}

# Run the (local) tools e.g to create badges and reference definitions
tools cmd=help *subcmd=empty:
  cargo run -p rust_howto_tools -- {{cmd}} {{subcmd}}

# Sort and deduplicate reference definitions in the central `*-refs.md` files
[unix]
sortrefs: _removelastslash
  sort -u ./src/refs/crate-refs.md -o /tmp/c.md
  mv -f /tmp/c.md ./src/refs/crate-refs.md
  rm -f /temp/c.md
  sort -u ./src/refs/other-refs.md -o /tmp/o.md
  mv -f /tmp/o.md ./src/refs/other-refs.md
  rm -f /temp/o.md
  sort -u ./src/refs/link-refs.md -o /tmp/l.md
  mv -f /tmp/l.md ./src/refs/link-refs.md
  rm -f /temp/l.md

# Remove the last / from URLs in the reference definition files
[unix]
_removelastslash:
   sed -i 's/[/]$//g' ./src/refs/crate-refs.md
   sed -i 's/[/]$//g' ./src/refs/other-refs.md
   sed -i 's/[/]$//g' ./src/refs/link-refs.md

# Check spelling in markdown
[unix]
spell:
  .devcontainer/spellcheck.sh
