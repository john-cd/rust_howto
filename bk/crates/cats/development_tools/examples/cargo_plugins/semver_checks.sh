# Purpose: Compare a crate's public API with its semver compatibility baseline.
# Usage: Run with bash bk/crates/cats/development_tools/examples/cargo_plugins/semver_checks.sh from the example workspace, as described in the book.
#
# ANCHOR: example
# Install once with cargo-binstall or Cargo's built-in installer.
cargo binstall cargo-semver-checks
# Or install from source with a locked dependency set:
cargo install cargo-semver-checks --locked
# Compare the current crate's API with its compatibility baseline:
cargo semver-checks
# ANCHOR_END: example
