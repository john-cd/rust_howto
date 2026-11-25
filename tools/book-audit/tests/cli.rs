//! Runs the book-audit executable against temporary repositories only.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use tempfile::tempdir;

/// Runs a book-audit command against a temporary repository.
fn run(root: &Path, arguments: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_book-audit"));
    command.arg("--repo-root").arg(root).args(arguments);
    command.output().expect("the binary should start")
}

/// The rendered link command rejects missing and non-directory artifact roots.
#[test]
fn rendered_link_command_rejects_invalid_artifact_directories() {
    let directory = tempdir().expect("a temporary directory should be created");
    let output = run(directory.path(), &["rendered-links", "missing/html"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("HTML directory does not exist"));

    let file = directory.path().join("book.html");
    fs::write(&file, "<html></html>").expect("the file should be written");
    let output = run(directory.path(), &["rendered-links", "book.html"]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("HTML directory does not exist"));
}

/// The visible TODO command returns a path and line with a failing status.
#[test]
fn visible_todos_command_reports_actionable_locations() {
    let directory = tempdir().expect("a temporary directory should be created");
    fs::create_dir_all(directory.path().join("bk/src"))
        .expect("the source directory should be created");
    fs::write(directory.path().join("LICENSE"), "License text\n")
        .expect("license should be written");
    fs::write(
        directory.path().join("bk/src/page.md"),
        "~~~~\n~~~\nTODO example\n~~~~text\nTODO still in example\n~~~~\nTODO remove this\n````\n```\nTODO example\n````\nTODO visible\n",
    )
    .expect("the page should be written");

    let output = run(directory.path(), &["visible-todos"]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("bk/src/page.md:7: visible TODO"));
    assert!(stderr.contains("bk/src/page.md:12: visible TODO"));
    assert_eq!(stderr.matches("visible TODO:").count(), 2);
}

/// The search-index command accepts mdBook's JSON.parse wrapper and prints its index count.
#[test]
fn search_index_command_accepts_a_valid_generated_index() {
    let directory = tempdir().expect("a temporary directory should be created");
    let output = directory.path().join("bk/book/html");
    fs::create_dir_all(&output).expect("the HTML directory should be created");
    fs::write(
        output.join("searchindex-1.js"),
        "window.search = Object.assign(window.search, JSON.parse('{\"doc_urls\":[\"chapter.html\"],\"body\":\"serde tokio\"}'));",
    )
    .expect("the index should be written");

    let result = run(directory.path(), &["search-index", "--book-root", "bk"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&result.stdout),
        "Checked 1 mdBook search index file(s).\n"
    );
}
