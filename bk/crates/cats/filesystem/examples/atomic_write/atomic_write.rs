// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Replace a file so that readers never see a half-written file.

use std::fs;
use std::io::Write as _;
use std::path::Path;

use anyhow::Context as _;
use atomicwrites::AllowOverwrite;
use atomicwrites::AtomicFile;
use tempfile::NamedTempFile;

/// Replaces `target` with `contents` using a temporary file in the same
/// directory.
///
/// On Unix, synchronizes the parent directory after replacement as well.
///
/// # Errors
///
/// Returns an error if temporary-file creation, writing, synchronization, or
/// replacement fails. A parent-directory sync error occurs after replacement,
/// so an error does not always mean that the old contents remain.
fn replace_with_tempfile(target: &Path, contents: &[u8]) -> anyhow::Result<()> {
    // The temporary file must live on the same filesystem as the target,
    // otherwise the final rename is not atomic. The parent directory is the
    // safest choice.
    let dir = target
        .parent()
        .context("the target path has no parent directory")?;
    #[cfg(unix)]
    let parent_directory = fs::File::open(dir)
        .context("open parent directory for synchronization")?;
    let mut temp = NamedTempFile::new_in(dir)?;
    temp.write_all(contents)?;
    // Flush the data to disk *before* the rename. Otherwise, after a crash the
    // new name could point to an empty or partially written file.
    temp.as_file().sync_all()?;
    // `persist` renames the temporary file over the target. It also replaces
    // an existing target on Windows. On failure, PersistError retains the
    // temporary file until the error is dropped.
    temp.persist(target)?;
    // Persisting the directory entry requires a separate sync on Unix.
    // The standard library has no equivalent portable directory sync on
    // Windows.
    #[cfg(unix)]
    parent_directory
        .sync_all()
        .context("file replaced, but syncing the parent directory failed")?;
    Ok(())
}

/// Replaces `target` with `contents` through `atomicwrites`' closure-based API.
///
/// # Errors
///
/// Returns an error if temporary-file creation, writing, synchronization, or
/// replacement fails.
fn replace_with_atomicwrites(
    target: &Path,
    contents: &[u8],
) -> anyhow::Result<()> {
    AtomicFile::new(target, AllowOverwrite)
        .write(|file| file.write_all(contents))?;
    Ok(())
}

/// Writes the same file twice with each variant.
///
/// # Errors
///
/// Returns an error if fixture creation, replacement, synchronization, or
/// reading the resulting contents fails.
pub fn run() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let target = dir.path().join("settings.txt");

    replace_with_tempfile(&target, b"first")?;
    replace_with_tempfile(&target, b"second")?;
    assert_eq!(fs::read_to_string(&target)?, "second");

    replace_with_atomicwrites(&target, b"third")?;
    replace_with_atomicwrites(&target, b"fourth")?;
    assert_eq!(fs::read_to_string(&target)?, "fourth");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Lists the names of the entries in `dir`.
    fn entry_names(dir: &Path) -> anyhow::Result<Vec<String>> {
        let mut names = fs::read_dir(dir)?
            .map(|entry| Ok(entry?.file_name().to_string_lossy().into_owned()))
            .collect::<anyhow::Result<Vec<_>>>()?;
        names.sort();
        Ok(names)
    }

    /// Replaces an existing file through tempfile and cleans up temporary
    /// names.
    #[test]
    fn tempfile_variant_replaces_and_leaves_no_stray_files()
    -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let target = dir.path().join("a.txt");
        replace_with_tempfile(&target, b"one")?;
        replace_with_tempfile(&target, b"two")?;
        assert_eq!(fs::read_to_string(&target)?, "two");
        assert_eq!(entry_names(dir.path())?, vec!["a.txt".to_owned()]);
        Ok(())
    }

    /// Replaces an existing file through atomicwrites without stray files.
    #[test]
    fn atomicwrites_variant_replaces_and_leaves_no_stray_files()
    -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let target = dir.path().join("b.txt");
        replace_with_atomicwrites(&target, b"one")?;
        replace_with_atomicwrites(&target, b"two")?;
        assert_eq!(fs::read_to_string(&target)?, "two");
        assert_eq!(entry_names(dir.path())?, vec!["b.txt".to_owned()]);
        Ok(())
    }

    /// Runs both replacement variants in an isolated temporary directory.
    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
