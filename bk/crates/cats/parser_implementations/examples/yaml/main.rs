//! Runs the YAML parser example.
//!
//! TODO Reviewed ??-??-2026.

mod serde_yml;

/// Runs the yaml example.
fn main() -> anyhow::Result<()> {
    serde_yml::run()
}
