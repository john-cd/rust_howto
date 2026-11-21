//! Exercises the recipe table CLI against temporary book trees.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::tempdir;

/// Runs the actual binary against a temporary book root.
fn run(book_root: &Path) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_recipe-tables"))
        .arg(book_root)
        .output()
        .expect("the recipe-tables binary should start")
}

/// The CLI creates a missing fragment, reports it, then reports no edit on rerun.
#[test]
fn command_updates_a_book_and_is_idempotent() {
    let directory = tempdir().expect("a temporary directory should be created");
    let book_root = directory.path().join("book root with spaces");
    let source = book_root.join("src/chapter");
    fs::create_dir_all(&source).expect("the source directory should be created");
    fs::write(source.join("refs.incl.md"), "[ex~one]: chapter.md#one\n")
        .expect("references should be written");
    fs::write(
        source.join("chapter.md"),
        "{{#include chapter.incl.md}}\n## One {#one}\n",
    )
    .expect("the chapter should be written");

    let first = run(&book_root);
    assert!(
        first.status.success(),
        "{}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(String::from_utf8_lossy(&first.stdout).contains("Updated 1 recipe table fragment(s)."));
    assert!(source.join("chapter.incl.md").is_file());

    let second = run(&book_root);
    assert!(
        second.status.success(),
        "{}",
        String::from_utf8_lossy(&second.stderr)
    );
    assert!(
        String::from_utf8_lossy(&second.stdout).contains("Updated 0 recipe table fragment(s).")
    );
}
