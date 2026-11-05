// TODO Reviewed ??-??-2026

//! Runs the `example-integrity` binary against temporary repositories only.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use tempfile::tempdir;

/// Runs the binary against `root`, optionally with `--fail`.
fn run(root: &Path, fail: bool) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_example-integrity"));
    command.arg("--repo-root").arg(root);
    if fail {
        command.arg("--fail");
    }
    command.output().expect("the binary should start")
}

/// Writes the three empty book summaries that a valid repository needs.
fn write_clean_fixture(root: &Path) {
    for book in ["bk/src", "bk/drafts", "later/src"] {
        let dir = root.join(book);
        fs::create_dir_all(&dir).expect("directory should be created");
        fs::write(
            dir.join("SUMMARY.md"),
            "# Summary
",
        )
        .expect("fixture should be written");
    }
}

/// Writes an exception entry that lacks a reason, which is an integrity error.
fn write_broken_fixture(root: &Path) {
    write_clean_fixture(root);
    let scripts = root.join("scripts");
    fs::create_dir_all(&scripts).expect("directory should be created");
    fs::write(
        scripts.join("example_chapter_exceptions.txt"),
        "bk/src/some_chapter.md\n",
    )
    .expect("fixture should be written");
}

/// An empty repository yields a JSON report with every section and exits successfully.
#[test]
fn test_clean_repository_reports_json_schema() {
    let dir = tempdir().expect("a temporary directory should be created");
    write_clean_fixture(dir.path());
    let output = run(dir.path(), true);
    assert!(output.status.success());
    let report: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("stdout should be valid JSON");
    for key in [
        "chapters",
        "fragments",
        "includes",
        "cargo_targets",
        "modules",
        "standalone_snippets",
        "example_sources",
        "exceptions",
        "diagnostics",
    ] {
        assert!(
            report[key].as_array().is_some_and(Vec::is_empty),
            "`{key}` should be an empty array: {report}"
        );
    }
}

/// Errors are reported in the JSON, but only `--fail` changes the exit status.
#[test]
fn test_fail_flag_controls_exit_status() {
    let dir = tempdir().expect("a temporary directory should be created");
    write_broken_fixture(dir.path());

    let lenient = run(dir.path(), false);
    assert!(lenient.status.success());
    let report: serde_json::Value =
        serde_json::from_slice(&lenient.stdout).expect("stdout should be valid JSON");
    let diagnostics = report["diagnostics"]
        .as_array()
        .expect("diagnostics should be an array");
    assert_eq!(diagnostics.len(), 1, "{report}");
    assert_eq!(diagnostics[0]["code"], "exception-no-reason");
    assert_eq!(diagnostics[0]["severity"], "error");

    let strict = run(dir.path(), true);
    assert!(!strict.status.success());
}

/// A missing repository root fails with a message that names the path.
#[test]
fn test_missing_root_fails_with_message() {
    let dir = tempdir().expect("a temporary directory should be created");
    let output = run(&dir.path().join("does-not-exist"), false);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does-not-exist"), "stderr: {stderr}");
}
