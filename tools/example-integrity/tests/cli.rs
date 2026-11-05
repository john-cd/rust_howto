// TODO Reviewed ??-??-2026.

//! CLI integration tests for `example-integrity`.

use std::fs;
use std::process::Command;

use example_integrity::Report;
use tempfile::tempdir;

/// Checks that the CLI produces a valid JSON report matching the Report schema.
#[test]
fn test_cli_json_schema_output() {
    let dir = tempdir().expect("temporary directory should be created");
    let root = dir.path();

    let output = Command::new(env!("CARGO_BIN_EXE_example-integrity"))
        .arg("--repo-root")
        .arg(root)
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));

    let report: Report = serde_json::from_slice(&output.stdout)
        .expect("stdout should be valid JSON deserializable into Report");

    assert!(report.chapters.is_empty());
    assert!(report.fragments.is_empty());
    assert!(report.includes.is_empty());
    assert!(report.cargo_targets.is_empty());
    assert!(report.modules.is_empty());
    assert!(report.standalone_snippets.is_empty());
    assert!(report.example_sources.is_empty());
    assert!(report.exceptions.is_empty());
    // Missing SUMMARY.md diagnostics are reported.
    assert!(!report.diagnostics.is_empty());
    assert!(report.has_errors());
}

/// Checks that `--fail` causes non-zero exit status on an intentional broken fixture book.
#[test]
fn test_cli_broken_fixture_with_fail_flag_fails() {
    let dir = tempdir().expect("temporary directory should be created");
    let root = dir.path();

    // Create broken fixture book: SUMMARY points to missing chapter file.
    let src = root.join("bk").join("src");
    fs::create_dir_all(&src).expect("src dir should be created");
    fs::write(
        src.join("SUMMARY.md"),
        "# Summary\n\n- [Missing Chapter](missing.md)\n",
    )
    .expect("SUMMARY.md should be written");

    // Also stub out bk/drafts/SUMMARY.md and later/src/SUMMARY.md so only the missing chapter is broken.
    let drafts = root.join("bk").join("drafts");
    fs::create_dir_all(&drafts).expect("drafts dir should be created");
    fs::write(drafts.join("SUMMARY.md"), "# Summary\n").expect("drafts SUMMARY.md should be written");

    let later = root.join("later").join("src");
    fs::create_dir_all(&later).expect("later dir should be created");
    fs::write(later.join("SUMMARY.md"), "# Summary\n").expect("later SUMMARY.md should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_example-integrity"))
        .arg("--fail")
        .arg("--repo-root")
        .arg(root)
        .output()
        .expect("binary should execute");

    assert!(!output.status.success(), "expected failure with --fail on broken fixture book");

    let report: Report = serde_json::from_slice(&output.stdout)
        .expect("stdout should be valid JSON deserializable into Report");
    assert!(report.has_errors());
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.code == "summary-target-missing"),
        "diagnostics: {:?}",
        report.diagnostics
    );
}

/// Checks that omitting `--fail` exits successfully even with diagnostics on a broken fixture book.
#[test]
fn test_cli_broken_fixture_without_fail_flag_succeeds() {
    let dir = tempdir().expect("temporary directory should be created");
    let root = dir.path();

    // Intentionally broken book: missing SUMMARY.md files.
    let output = Command::new(env!("CARGO_BIN_EXE_example-integrity"))
        .arg("--repo-root")
        .arg(root)
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "expected success without --fail even when diagnostics exist");

    let report: Report = serde_json::from_slice(&output.stdout)
        .expect("stdout should be valid JSON deserializable into Report");
    assert!(report.has_errors());
    assert!(
        report
            .diagnostics
            .iter()
            .any(|d| d.code == "summary-missing"),
        "diagnostics: {:?}",
        report.diagnostics
    );
}
