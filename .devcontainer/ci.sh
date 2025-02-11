#!/usr/bin/env bash
set -eux
set -o pipefail

echo "----------"

## Spell checks
.devcontainer/spellcheck.sh list

cd ./bk

## Checks the Rust code formatting
## Fails if not formatted properly
cargo +nightly fmt --all --check

## TODO P2
## Check dependencies
# cargo deny check \
#     && cargo outdated --exit-code 1 \
#     && cargo udeps \
#     && rm -rf ~/.cargo/advisory-db \
#     && cargo audit \
#     && cargo pants

## Fetch the dependencies
cargo fetch

## Make sure all examples (and tools) compile
## - We prefer `cargo build ...` to `cargo check --workspace --all-targets --locked --profile ci`
## Some diagnostics and errors are only emitted during code generation, so they inherently won't be reported with cargo check.
## - `--all-targets` is equivalent to specifying `--lib --bins --tests --benches --examples`.
## - See .cargo/config.toml for the `ci` profile config. We removed optimizations, since we will run / test the examples just once.
## - Some examples require external services e.g. Redis, Mongodb... and are excluded from testing / hidden behind feature flags.
cargo build --workspace --all-targets --locked --profile ci --all-features

## Make sure that all examples are linted
## - Elevate clippy warnings to errors, which will in turn fail the build.
cargo clippy --workspace --all-targets --locked --profile ci --all-features -- --deny warnings

## Test all code examples (except heavy tests and "ignore_in_ci" tests)
cargo nextest run --workspace --all-targets --locked --cargo-profile ci --profile ci --hide-progress-bar --all-features

## `nextest` does not handle doctests.
cargo test --workspace --doc --locked --profile ci -- --show-output
## NOTE supersedes: mdbook test / skeptic tests

## Build the book (html)
mdbook build

## Add static assets
cp static/*.* book/html/

## `sitemap.xml` generator
##  We used https://lib.rs/crates/mdbook-sitemap-generator but there is now custom code in the mdbook-utils tool
## that is a companion to the book
# mdbook-sitemap-generator --domain john-cd.com/rust_howto/ --output book/sitemap.xml
# sed -i -e 's/<urls>/<url>/g' -e 's/<\/urls>/<\/url>/g' book/sitemap.xml

mdbook-utils sitemap

echo "----------"

## Do not remove.
## This is what will cause the dockerfile CMD to run.
exec "$@"
