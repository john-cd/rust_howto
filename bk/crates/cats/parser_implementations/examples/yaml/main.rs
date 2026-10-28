//! Runs the YAML parser example.
//!
//! TODO Reviewed ??-??-2026.

mod serde_saphyr;

/// Runs the yaml example.
fn main() -> anyhow::Result<()> {
    serde_saphyr::run()
}
