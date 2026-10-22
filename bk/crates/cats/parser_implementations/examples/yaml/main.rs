//! Runs the YAML parser example.
//!
//! TODO Reviewed ??-??-2026

mod serde_yml;

/// Runs the example from the `serde_yml` module.
fn main() -> anyhow::Result<()> {
    serde_yml::run()
}
