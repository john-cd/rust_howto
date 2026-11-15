//! Runs the build information example.
//!
//! TODO Reviewed ??-??-2026.

/// Embedding build details, such as the version, Git commit and build time, in the binary helps
/// with support: a bug report can state exactly which build produced it.
mod build_info;

/// Runs the build information example.
fn main() -> anyhow::Result<()> {
    build_info::run()
}
