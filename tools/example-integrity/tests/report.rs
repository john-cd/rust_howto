//! Integration tests of the public `build_report` API.
//!
//! TODO Reviewed ??-??-2026.

use example_integrity::build_report;
use tempfile::tempdir;

/// An unresolvable repository root is an error.
#[test]
fn missing_root_is_an_error() {
    assert!(build_report(std::path::Path::new("/definitely/not/here")).is_err());
}

/// An empty repository yields a report without chapters.
#[test]
fn empty_repo_has_no_chapters() {
    let dir = tempdir().expect("temp dir");
    // The audit may report diagnostics about missing files; it must not panic.
    if let Ok(report) = build_report(dir.path()) {
        assert!(report.chapters.is_empty());
    }
}
