# ANCHOR: example
# Check each feature independently.
cargo hack check --each-feature
# Test every feature combination only when the feature count is manageable.
cargo hack test --feature-powerset
# ANCHOR_END: example