// TODO Reviewed ??-??-2026

// ANCHOR: fixture_support
//! Shared local files for the crate-index examples.

use std::fs;
use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use tempfile::TempDir;

/// Category metadata used by the offline page generators.
pub(crate) const CATEGORY_METADATA: &str = r#"
[[categories]]
category = "Serialization"
slug = "serialization"
description = "Crates for encoding and decoding data."
crates = ["serde", "serde_json"]
"#;

/// Creates a Cargo manifest and book-index fixture without network access.
pub(crate) fn create_fixture() -> Result<TempDir> {
    let fixture =
        tempfile::tempdir().context("create crate-index fixture directory")?;
    write_fixture_file(
        fixture.path(),
        "workspace/crates/sample/Cargo.toml",
        "[package]\nname = \"sample\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nserde = \"1\"\nserde_json = \"1\"\n",
    )?;
    write_fixture_file(
        fixture.path(),
        "book/src/indices/examples_index.md",
        "{{#include examples.incl.md}}\n",
    )?;
    write_fixture_file(
        fixture.path(),
        "book/src/indices/examples.incl.md",
        "| [Serialize JSON][ex~serialization~serde] | [![serde][c~serde~docs~badge]][c~serde~docs] |\n",
    )?;
    write_fixture_file(
        fixture.path(),
        "book/src/indices/refs.incl.md",
        "[ex~serialization~serde]: ../serialization.md#serde\n",
    )?;
    Ok(fixture)
}

/// Writes one fixture file and creates its parent directory when necessary.
fn write_fixture_file(
    root: &Path,
    relative_path: &str,
    contents: &str,
) -> Result<()> {
    let path = root.join(relative_path);
    let parent = path.parent().with_context(|| {
        format!("fixture path has no parent: {}", path.display())
    })?;
    fs::create_dir_all(parent)
        .with_context(|| format!("create {}", parent.display()))?;
    fs::write(&path, contents)
        .with_context(|| format!("write {}", path.display()))
}
// ANCHOR_END: fixture_support
