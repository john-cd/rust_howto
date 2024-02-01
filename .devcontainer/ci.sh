#!/usr/bin/env bash
set -eux
set -o pipefail

echo "----------"

## Spell checks
.devcontainer/spellcheck.sh list

## Checks the Rust code formatting
## Fails if not formatted properly
cargo +nightly fmt --all --check

## Check dependencies
# cargo deny check \
#     && cargo outdated --exit-code 1 \
#     && cargo udeps \
#     && rm -rf ~/.cargo/advisory-db \
#     && cargo audit \
#     && cargo pants

## Fetch the dependencies
cargo fetch

## Build the book (html and fully processed markdown) into ./book/html and ./book/markdown
mdbook build

# NOTE: cargo build (specifically build.rs) requires that /code/book/markdown/ has been created

## Make sure all examples compile
## - We prefer `cargo build ...` to `cargo check --workspace --all-targets --locked --profile ci`
## Some diagnostics and errors are only emitted during code generation, so they inherently won’t be reported with cargo check.
## - `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
## - see .cargo/config.toml for `ci` profile config.
cargo build --workspace --all-targets --locked --profile ci

## Lint all examples
## - Elevate clippy warnings to errors, which will in turn fail the build.
cargo clippy --workspace --all-targets --locked --profile ci -- --deny warnings

## Test all examples (unit tests in /deps/examples, skeptic tests in /deps/tests and any tests in /xmpl)
cargo test --workspace --all-targets --locked --profile ci -- --show-output
## NOTE supersedes: mdbook test

## Add static assets
cp static/*.* book/html/

## `sitemap.xml` generator
##  We used https://lib.rs/crates/mdbook-sitemap-generator but there is now custom code in the mdbook-utils tool
## that is a companion to the book
# mdbook-sitemap-generator --domain john-cd.com/rust_howto/ --output book/sitemap.xml
# sed -i -e 's/<urls>/<url>/g' -e 's/<\/urls>/<\/url>/g' book/sitemap.xml

# TODO mdbook-utils sitemap

echo "----------"

## Do not remove.
## This is what will cause the dockerfile CMD to run.
exec "$@"
