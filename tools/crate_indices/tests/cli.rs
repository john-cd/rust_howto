//! Runs the `crate-indices` binary against temporary fixtures only.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;
use std::process::Stdio;

use tempfile::tempdir;

/// Runs the binary with `args`. Stdin is closed so no argument is read from a pipe.
fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_crate-indices"))
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("the binary should start")
}

/// Writes `contents` to `root/relative`, creating parent folders.
fn write(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    fs::create_dir_all(path.parent().expect("a parent exists")).expect("dirs should be created");
    fs::write(path, contents).expect("file should be written");
}

/// Running without a subcommand fails with a clear message.
#[test]
fn test_no_command_fails() {
    let output = run(&[]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("did not enter a command"),
        "stderr: {stderr}"
    );
}

/// An unknown subcommand is rejected by the argument parser.
#[test]
fn test_unknown_command_fails() {
    let output = run(&["bogus"]);
    assert!(!output.status.success());
}

/// `list_crates` prints the sorted, deduplicated dependencies of all manifests.
#[test]
fn test_list_crates_prints_sorted_unique_dependencies() {
    let dir = tempdir().expect("a temporary directory should be created");
    write(
        dir.path(),
        "one/Cargo.toml",
        "[package]\nname = \"one\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1\"\nanyhow = \"1\"\n",
    );
    write(
        dir.path(),
        "two/Cargo.toml",
        "[package]\nname = \"two\"\nversion = \"0.1.0\"\n\n[dependencies]\nserde = \"1\"\n\n[build-dependencies]\ncc = \"1\"\n",
    );
    let root = dir.path().to_str().expect("the path is UTF-8");
    let output = run(&["list_crates", "--directory", root]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .collect::<Vec<_>>(),
        ["anyhow", "cc", "serde"]
    );
}

/// `category_page_from_metadata` renders the page offline from a metadata snapshot.
#[test]
fn test_category_page_from_metadata_renders_categories() {
    let dir = tempdir().expect("a temporary directory should be created");
    write(
        dir.path(),
        "src/indices/examples_index.md",
        "{{#include refs.incl.md}}\n",
    );
    write(dir.path(), "src/indices/refs.incl.md", "");
    write(
        dir.path(),
        "metadata.toml",
        "[[categories]]\ncategory = \"Testing\"\nslug = \"testing\"\ndescription = \"Testing crates.\"\ncrates = [\"crate-a\"]\n",
    );
    let metadata = dir.path().join("metadata.toml");
    let output = run(&[
        "category_page_from_metadata",
        "--metadata",
        metadata.to_str().expect("the path is UTF-8"),
        "--book-root",
        dir.path().to_str().expect("the path is UTF-8"),
        "crate-a",
        "crate-b",
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Testing"), "stdout: {stdout}");
    assert!(stdout.contains("crate-a"), "stdout: {stdout}");
    // Crates absent from the snapshot are listed as uncategorized.
    assert!(stdout.contains("crate-b"), "stdout: {stdout}");
}

/// A missing metadata file is reported with its path.
#[test]
fn test_category_page_from_metadata_missing_file_fails() {
    let dir = tempdir().expect("a temporary directory should be created");
    let missing = dir.path().join("missing.toml");
    let output = run(&[
        "category_page_from_metadata",
        "--metadata",
        missing.to_str().expect("the path is UTF-8"),
        "crate-a",
    ]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing.toml"));
}

/// `expand_crate_blocks` leaves Markdown without crate directives untouched.
#[test]
fn test_expand_crate_blocks_without_directives_keeps_files() {
    let dir = tempdir().expect("a temporary directory should be created");
    write(dir.path(), "page.md", "# Title\n\nPlain text.\n");
    let output = run(&[
        "expand_crate_blocks",
        "--directory",
        dir.path().to_str().expect("the path is UTF-8"),
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(dir.path().join("page.md")).expect("file is readable"),
        "# Title\n\nPlain text.\n"
    );
}
