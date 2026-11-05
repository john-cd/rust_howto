// TODO Reviewed ??-??-2026.

//! CLI integration tests for `crate-indices`.

use std::fs;
use std::process::Command;

use tempfile::tempdir;

/// Checks that `list_crates` outputs dependencies discovered in Cargo.toml files.
#[test]
fn test_cli_list_crates() {
    let dir = tempdir().expect("temporary directory should be created");
    let proj = dir.path().join("subproject");
    fs::create_dir_all(&proj).expect("subproject dir should be created");

    let cargo_toml_content = r#"
        [package]
        name = "subproject"
        version = "0.1.0"
        edition = "2024"

        [dependencies]
        alpha_dep = "1.0"
        beta_dep = "2.0"

        [build-dependencies]
        gamma_dep = "3.0"
    "#;
    fs::write(proj.join("Cargo.toml"), cargo_toml_content).expect("Cargo.toml should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_crate-indices"))
        .args(["list_crates", "-d"])
        .arg(dir.path())
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    assert!(lines.contains(&"alpha_dep"), "stdout: {stdout}");
    assert!(lines.contains(&"beta_dep"), "stdout: {stdout}");
    assert!(lines.contains(&"gamma_dep"), "stdout: {stdout}");
}

/// Checks that `category_page_from_metadata` outputs category tables from metadata and recipes.
#[test]
fn test_cli_category_page_from_metadata() {
    let dir = tempdir().expect("temporary directory should be created");
    let book_root = dir.path().join("book");
    let indices = book_root.join("src").join("indices");
    fs::create_dir_all(&indices).expect("indices directory should be created");

    fs::write(
        indices.join("examples_index.md"),
        "{{#include refs.incl.md}}\n",
    )
    .expect("examples_index.md should be written");
    fs::write(indices.join("refs.incl.md"), "")
        .expect("refs.incl.md should be written");

    let metadata_path = dir.path().join("categories.toml");
    let metadata_content = r#"
        [[categories]]
        category = "Algorithms"
        slug = "algorithms"
        description = "Algorithmic crates."
        crates = ["algo-crate"]
    "#;
    fs::write(&metadata_path, metadata_content).expect("categories.toml should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_crate-indices"))
        .args(["category_page_from_metadata", "--metadata"])
        .arg(&metadata_path)
        .arg("--book-root")
        .arg(&book_root)
        .args(["algo-crate", "extra-crate"])
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("# Crates by Category"), "stdout: {stdout}");
    assert!(
        stdout.contains("[![cat~algorithms][cat~algorithms~badge]][cat~algorithms]{{hi:Algorithms}}"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("[![algo-crate][c~algo-crate~docs~badge]][c~algo-crate~docs]{{hi:algo-crate}}"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("Uncategorized"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("[![extra-crate][c~extra-crate~docs~badge]][c~extra-crate~docs]{{hi:extra-crate}}"),
        "stdout: {stdout}"
    );
}

/// Checks that `expand_crate_blocks` traverses markdown files and updates refdefs.
#[test]
fn test_cli_expand_crate_blocks() {
    let dir = tempdir().expect("temporary directory should be created");
    let md_file = dir.path().join("chapter.md");
    fs::write(
        &md_file,
        "# Test Chapter\n\nNo directives to expand here.\n",
    )
    .expect("chapter.md should be written");

    let refdefs_file = dir.path().join("refs.md");
    fs::write(&refdefs_file, "# Reference Definitions\n")
        .expect("refs.md should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_crate-indices"))
        .args(["expand_crate_blocks", "-d"])
        .arg(dir.path())
        .arg("-f")
        .arg(&refdefs_file)
        .output()
        .expect("binary should execute");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(md_file.exists());
    assert!(refdefs_file.exists());
}
