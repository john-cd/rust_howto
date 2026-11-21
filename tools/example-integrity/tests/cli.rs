//! Runs the `example-integrity` binary against temporary repositories only.
//!
//! TODO Reviewed ??-??-2026.

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

/// Runs an `example-integrity` subcommand against a temporary repository.
fn run_command(root: &Path, arguments: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_example-integrity"));
    command.arg("--repo-root").arg(root).args(arguments);
    command.output().expect("the binary should start")
}

/// Writes the three empty book summaries that a valid repository needs.
fn write_clean_fixture(root: &Path) {
    for book in ["bk/src", "later/src"] {
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

/// Fails on reported errors only when the strict flag is enabled.
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

/// The test survey reports noncompliant examples and stays informational by default.
#[test]
fn test_survey_command_is_informational_until_fail_is_requested() {
    let dir = tempdir().expect("a temporary directory should be created");
    let example = dir.path().join("bk/crates/demo/examples/example.rs");
    fs::create_dir_all(example.parent().expect("the example has a parent"))
        .expect("the example directory should be created");
    fs::write(&example, "fn main() {}").expect("the example should be written");

    let lenient = run_command(dir.path(), &["tests", "--summary", "bk/crates"]);
    assert!(lenient.status.success());
    assert!(String::from_utf8_lossy(&lenient.stdout).contains("0/1 compliant"));

    let strict = run_command(dir.path(), &["tests", "--summary", "bk/crates", "--fail"]);
    assert!(!strict.status.success());
}

/// The unused-example command compares native includes with Rust files under the selected book.
#[test]
fn unused_examples_preserves_legacy_root_and_include_scope() {
    let dir = tempdir().expect("a temporary directory should be created");
    let root = dir.path();
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"bk/crates/demo\"]\nresolver = \"3\"\n",
    )
    .expect("workspace manifest should be written");
    fs::create_dir_all(root.join("bk/crates/demo")).expect("crate directory should be created");
    fs::write(
        root.join("bk/crates/demo/Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .expect("crate manifest should be written");
    fs::create_dir_all(root.join("bk/src")).expect("book source should be created");
    fs::create_dir_all(root.join("later/src")).expect("future book source should be created");
    fs::write(root.join("bk/src/SUMMARY.md"), "# Summary\n").expect("summary should be written");
    fs::write(root.join("later/src/SUMMARY.md"), "# Summary\n").expect("summary should be written");
    fs::write(
        root.join("bk/src/example.md"),
        "# Example\n\n```rust\n{{#include ../crates/demo/examples/used.rs}}\n```\n",
    )
    .expect("chapter should be written");
    let examples = root.join("bk/crates/demo/examples");
    fs::create_dir_all(&examples).expect("examples directory should be created");
    fs::write(examples.join("used.rs"), "fn main() {}\n").expect("used example should be written");
    fs::write(examples.join("unused.rs"), "fn main() {}\n")
        .expect("unused example should be written");
    fs::write(examples.join("main.rs"), "fn main() {}\n").expect("entry point should be written");

    let output = run_command(root, &["unused-examples", "--summary"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "1 unused examples\n"
    );

    let details = run_command(root, &["unused-examples"]);
    assert!(details.status.success());
    assert_eq!(
        String::from_utf8_lossy(&details.stdout),
        "crates/demo/examples/unused.rs\nDONE\n"
    );
}
