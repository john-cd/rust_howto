#!/usr/bin/env bash
# Purpose: Run the continuous integration build inside the development container.
# Usage: Run with bash .devcontainer/ci.sh from the repository root and provide any required arguments.
#
# TODO Reviewed ??-??-2026.

set -eux
set -o pipefail

RUSTFMT_TOOLCHAIN="${RUSTFMT_TOOLCHAIN:-$(sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' rustfmt-toolchain.toml)}"

## @fn monitor_resources()
## @brief Periodically logs tmpfs and memory usage to stdout in the background.
monitor_resources() {
    while true; do
        echo "[$(date +%T)] --- Resource Snapshot ---"
        df -h /code/target/ 2>/dev/null || echo "tmpfs not yet mounted"
        free -h
        sleep 60
    done
}

## Start background monitoring
monitor_resources &
MONITOR_PID=$!

# Kill background monitor on script exit
trap 'kill $MONITOR_PID 2>/dev/null || true' EXIT

# Main script executed during the main CI workflow.
#
# See the `Dockerfile`.
# Spellchecks, then check the format of the code; build the mdbook-scrub preprocessor
# Build the book's code, lint it, test it, then build the book, copy the static files and build the sitemap.

echo "----------"

## Spell checks
bk/scripts/spelling/spellcheck.sh list

## Audit each independent Rust workspace with the shared policy.
## cargo-deny covers advisories, licenses, bans, and sources. cargo-audit uses the
## same RustSec advisory database; monthly Cargo updates are handled by Dependabot.
repo_root="$(pwd -P)"
just fmt-toml-check
for manifest in \
    "bk/crates/Cargo.toml" \
    "tools/Cargo.toml" \
    "xmpl/Cargo.toml" \
    "playground/Cargo.toml" \
    "publish/Cargo.toml"; do
    echo "Auditing ${manifest}"
    cargo deny \
        --manifest-path "${repo_root}/${manifest}" \
        --workspace \
        --config "${repo_root}/bk/crates/deny.toml" \
        check all --hide-inclusion-graph
done

## `later/crates` is staging content, not built or published by this repository.
## Its current graph has multiple advisories, an unapproved Git source, and a font
## license that need review before promotion; keep it out of the blocking gate.

cd bk/crates

## Checks the Rust code formatting
## Fails if not formatted properly
cargo +"$RUSTFMT_TOOLCHAIN" fmt --all --check

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

if [ "${SKIP_TESTS:-false}" != "true" ]; then
    ## Test all code examples (except heavy tests and "ignore_in_ci" tests)
    ## NEXTEST_ARGS can be used to pass --partition hash:K/N
    cargo nextest run --config-file nextest.toml --workspace --all-targets --locked --cargo-profile ci --profile ci --hide-progress-bar --all-features --status-level pass ${NEXTEST_ARGS:-}

    ## `nextest` does not handle doctests.
    cargo test --workspace --doc --locked --profile ci -- --show-output
fi
## NOTE supersedes: mdbook test / skeptic tests

if [ "${SKIP_BOOK_BUILD:-false}" = "true" ]; then
    echo "Skipping book build as requested."
    exit 0
fi

## Build the book through the supported recipe. PDF output is opt-in.
cd ..
python3 scripts/book/test_refdefs.py
python3 scripts/book/refdefs.py check --book-root .
PANDOC_BACKEND="$(command -v mdbook-pandoc || true)"
if [ "${BUILD_PDF:-false}" != "true" ] && [ -n "$PANDOC_BACKEND" ]; then
    mv "$PANDOC_BACKEND" "$PANDOC_BACKEND.ci-disabled"
    trap 'kill "$MONITOR_PID" 2>/dev/null || true; mv "$PANDOC_BACKEND.ci-disabled" "$PANDOC_BACKEND" 2>/dev/null || true' EXIT
fi
just bb
python3 scripts/book/check_search_index.py
if [ -n "$PANDOC_BACKEND" ] && [ -f "$PANDOC_BACKEND.ci-disabled" ]; then
    mv "$PANDOC_BACKEND.ci-disabled" "$PANDOC_BACKEND"
    trap 'kill "$MONITOR_PID" 2>/dev/null || true' EXIT
fi

echo "----------"

## Final report on resource consumption
echo "=== Final Memory usage ==="
free -h
echo "=== tmpfs Utilization ==="
df -h /code/target/ 2>/dev/null || true
echo "=== Cargo Cache Size ==="
du -sh /usr/local/cargo/ 2>/dev/null || true

echo "----------"

kill $MONITOR_PID 2>/dev/null || true

## Do not remove.
## This is what will cause the dockerfile CMD to run.
exec "$@"
