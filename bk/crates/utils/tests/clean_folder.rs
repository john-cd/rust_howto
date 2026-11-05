//! Integration tests of the public `utils` API.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;

use tempfile::tempdir;
use utils::clean_folder;

/// Cleaning removes visible files and subfolders but keeps hidden files.
#[test]
fn clean_folder_keeps_hidden_files() -> anyhow::Result<()> {
    let dir = tempdir()?;
    fs::write(dir.path().join("a.txt"), "a")?;
    fs::write(dir.path().join(".gitkeep"), "")?;
    fs::create_dir_all(dir.path().join("sub/nested"))?;
    fs::write(dir.path().join("sub/nested/b.txt"), "b")?;

    clean_folder(dir.path())?;

    assert!(!dir.path().join("a.txt").exists());
    assert!(!dir.path().join("sub").exists());
    assert!(dir.path().join(".gitkeep").exists());
    Ok(())
}

/// A missing folder is not an error.
#[test]
fn clean_folder_ignores_missing_dir() -> anyhow::Result<()> {
    let dir = tempdir()?;
    clean_folder(&dir.path().join("missing"))?;
    Ok(())
}
