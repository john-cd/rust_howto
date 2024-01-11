#!/usr/bin/env bash
set -eux
set -o pipefail

echo "----------"

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

## Lint all examples
## - Elevate clippy warnings to errors, which will in turn fail the build.
## - `--all-targets`` is equivalent to specifying `--lib --bins --tests --benches --examples`.
## - see .cargo/config.toml for `ci` profile config.
cargo clippy --workspace --all-targets --locked --profile ci -- --deny warnings

## Make sure all examples compile
## We prefer `cargo build ...` to `cargo check --workspace --all-targets --locked --profile ci`
## Some diagnostics and errors are only emitted during code generation, so they inherently won’t be reported with cargo check.
cargo build --workspace --all-targets --locked --profile ci

## Build the book (html and fully processed markdown) into ./book/html and ./book/markdown
mdbook build

## Test all examples (unit tests in /deps/examples, skeptic tests in /deps/tests and anything in /xmpl)
cargo test --workspace --all-targets --locked --profile ci -- --show-output
## NOTE supersedes: mdbook test

## Add static assets
cp static/*.* book/html/

## `sitemap.xml` generator
##  We used https://lib.rs/crates/mdbook-sitemap-generator but there is now custom code in `deps/bin`
#mdbook-sitemap-generator --domain john-cd.com/rust_howto/ --output book/sitemap.xml
#sed -i -e 's/<urls>/<url>/g' -e 's/<\/urls>/<\/url>/g' book/sitemap.xml
cargo run -p tools --bin sitemap

echo "----------"

## Do not remove.
## This is what will cause the dockerfile CMD to run.
exec "$@"
