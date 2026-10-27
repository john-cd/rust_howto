# Purpose: Check each feature independently.
# Usage: Run with bash bk/crates/cats/development_tools/examples/cargo_plugins/cargo_hack.sh from the example workspace, as described in the book.
#
# ANCHOR: example
# Check each feature independently.
cargo hack check --each-feature
# Test every feature combination only when the feature count is manageable.
cargo hack test --feature-powerset
# ANCHOR_END: example
