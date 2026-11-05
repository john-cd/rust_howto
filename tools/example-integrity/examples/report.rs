//! Builds the integrity report for a repository and prints a summary.
//!
//! Run with `cargo run -p example-integrity --example report -- <repo-root>`.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use example_integrity::build_report;

/// Runs the audit on the repository root given as first argument (default `..`).
fn main() -> anyhow::Result<()> {
    let root = std::env::args_os()
        .nth(1)
        .map_or_else(|| PathBuf::from(".."), PathBuf::from);
    let report = build_report(&root)?;
    println!(
        "{} chapters, {} diagnostics, errors: {}",
        report.chapters.len(),
        report.diagnostics.len(),
        report.has_errors()
    );
    Ok(())
}

/// Runs the example on a missing path: it must fail rather than panic.
#[cfg(test)]
mod tests {
    use super::*;

    /// A nonexistent root is reported as an error.
    #[test]
    fn missing_root_is_an_error() {
        assert!(build_report(std::path::Path::new("/definitely/not/here")).is_err());
    }
}
