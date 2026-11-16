// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Canonicalize a path without the Windows `\\?\` prefix.
//!
//! On Windows, `canonicalize` returns paths with a `\\?\` prefix that many
//! programs cannot handle. `dunce` removes it when it is safe to do so.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

/// Returns the canonical form of an existing path, without the `\\?\` prefix
///
/// that `std::fs::canonicalize` adds on Windows.
///
/// # Arguments
///
/// * `path` - Filesystem location the operation reads, updates, or uses as its scope.
///
/// # Errors
///
/// Returns an error if the path cannot be canonicalized.
fn canonical_path(path: &Path) -> std::io::Result<PathBuf> {
    // `dunce::canonicalize` calls `std::fs::canonicalize`, then removes the
    // "verbatim" prefix when the shorter path is equivalent. On Unix it is
    // the same as `std::fs::canonicalize`.
    dunce::canonicalize(path)
}

/// Compares the standard library and `dunce`.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn example_body() -> std::io::Result<()> {
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

/// Runs the example function and returns its result.
///
/// # Errors
///
/// Returns an error if the example operation fails.
pub fn run() -> std::io::Result<()> {
    example_body()
}

/// Tests canonical path handling, including removal of Windows verbatim prefixes.
#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that the dunce result has no verbatim prefix.
    #[test]
    fn dunce_result_has_no_verbatim_prefix() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let path = canonical_path(dir.path())?;
        assert!(!path.to_string_lossy().starts_with(r"\\?\"));
        Ok(())
    }

    /// Runs the path example using a temporary directory.
    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()?;
        Ok(())
    }
}
