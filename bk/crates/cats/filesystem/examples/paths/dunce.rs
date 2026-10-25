#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Canonicalize a path without the Windows `\\?\` prefix.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Returns the canonical form of an existing path, without the `\\?\` prefix
/// that `std::fs::canonicalize` adds on Windows.
fn canonical_path(path: &Path) -> std::io::Result<PathBuf> {
    // `dunce::canonicalize` calls `std::fs::canonicalize`, then removes the
    // "verbatim" prefix when the shorter path is equivalent. On Unix it is
    // the same as `std::fs::canonicalize`.
    dunce::canonicalize(path)
}

/// Compares the standard library and `dunce`.
pub fn run() -> std::io::Result<()> {
    let dir = tempfile::tempdir()?;
    let std_path = fs::canonicalize(dir.path())?;
    let dunce_path = canonical_path(dir.path())?;
    println!("std:   {}", std_path.display());
    println!("dunce: {}", dunce_path.display());

    // On Windows, the standard library result starts with `\\?\`, which many
    // tools and `Command` arguments do not understand. `dunce::simplified`
    // strips the prefix from a path you already have.
    assert_eq!(dunce::simplified(&std_path), dunce_path);
    if !cfg!(windows) {
        assert_eq!(std_path, dunce_path);
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dunce_result_has_no_verbatim_prefix() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = canonical_path(dir.path())?;
        assert!(!path.to_string_lossy().starts_with(r"\\?\"));
        Ok(())
    }

    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()?;
        Ok(())
    }
}
