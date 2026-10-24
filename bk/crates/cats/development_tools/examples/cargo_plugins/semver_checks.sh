# ANCHOR: example
# Install once with cargo-binstall or Cargo's built-in installer.
cargo binstall cargo-semver-checks
# Or install from source with a locked dependency set:
cargo install cargo-semver-checks --locked
# Compare the current crate's API with its compatibility baseline:
cargo semver-checks
# ANCHOR_END: example