//! Replacing files atomically through a temporary sibling file.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::io::Write as _;
use std::path::Path;

use anyhow::Context as _;
use anyhow::Result;
use tempfile::NamedTempFile;

/// Creates a temporary file in the directory that will hold `path`.
///
/// The temporary file lives next to the destination so that the final rename
/// stays on one filesystem, and it is deleted if it is dropped before being
/// persisted.
fn temp_sibling(path: &Path) -> Result<NamedTempFile> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    NamedTempFile::new_in(parent)
        .with_context(|| format!("cannot create a temporary file next to {}", path.display()))
}

/// Moves a completed temporary file over `path`, replacing the destination.
fn persist(tmp: NamedTempFile, path: &Path) -> Result<()> {
    tmp.persist(path)
        .map(drop)
        .map_err(|e| e.error)
        .with_context(|| format!("cannot replace {}", path.display()))
}

/// Writes every item to `path` by replacing it atomically.
///
/// The data is written to a temporary sibling file and flushed to storage
/// before the file is renamed over `path`. If any step fails, `path` is left
/// exactly as it was and the temporary file is removed, so a partially written
/// file is never presented as complete. The permissions of an existing
/// destination are preserved.
///
/// Limitations: the rename is atomic on POSIX filesystems. On Windows it
/// replaces the destination with `MoveFileEx`, which fails (without changing
/// the destination) if another process holds the destination open without
/// share-delete access. A symbolic link at `path` is replaced by a regular file.
///
/// # Errors
///
/// Returns an error naming the path if the temporary file cannot be created,
/// written, synchronized, or renamed over `path`.
pub(crate) fn write_atomic(
    path: impl AsRef<Path>,
    items: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> Result<()> {
    let path = path.as_ref();
    let mut tmp = temp_sibling(path)?;
    for item in items {
        tmp.write_all(item.as_ref())
            .with_context(|| format!("cannot write the replacement for {}", path.display()))?;
    }
    tmp.as_file()
        .sync_all()
        .with_context(|| format!("cannot sync the replacement for {}", path.display()))?;
    if let Ok(metadata) = fs::metadata(path) {
        fs::set_permissions(tmp.path(), metadata.permissions())
            .with_context(|| format!("cannot copy the permissions of {}", path.display()))?;
    }
    persist(tmp, path)
}

/// Copies `from` to `to`, replacing `to` atomically.
///
/// # Errors
///
/// Returns an error naming the path if `from` cannot be copied or `to` cannot be replaced.
pub(crate) fn copy_atomic(from: &Path, to: &Path) -> Result<()> {
    let tmp = temp_sibling(to)?;
    fs::copy(from, tmp.path())
        .with_context(|| format!("cannot copy {} to {}", from.display(), to.display()))?;
    tmp.as_file()
        .sync_all()
        .with_context(|| format!("cannot sync the copy at {}", to.display()))?;
    persist(tmp, to)
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    /// Verifies that a new file is created with the concatenated items.
    #[test]
    fn test_write_atomic_creates_file() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("new.txt");
        write_atomic(&path, ["a\n", "b\n"])?;
        assert_eq!(fs::read_to_string(&path)?, "a\nb\n");
        Ok(())
    }

    /// Verifies that an existing file is replaced and no temporary file remains.
    #[test]
    fn test_write_atomic_replaces_file_without_leftovers() -> Result<()> {
        let dir = tempdir()?;
        let path = dir.path().join("old.txt");
        fs::write(&path, "old")?;
        write_atomic(&path, ["new"])?;
        assert_eq!(fs::read_to_string(&path)?, "new");
        assert_eq!(fs::read_dir(dir.path())?.count(), 1);
        Ok(())
    }

    /// Verifies that a failed replacement leaves the source intact and no temporary file behind.
    #[test]
    fn test_write_atomic_failure_preserves_source() -> Result<()> {
        let dir = tempdir()?;
        // The destination is a non-empty directory, so the final rename must fail.
        let path = dir.path().join("target");
        fs::create_dir(&path)?;
        fs::write(path.join("keep.txt"), "keep")?;
        let error = write_atomic(&path, ["data"]).expect_err("replacing a directory should fail");
        assert!(format!("{error:#}").contains("target"));
        assert_eq!(fs::read_to_string(path.join("keep.txt"))?, "keep");
        assert_eq!(fs::read_dir(dir.path())?.count(), 1);
        Ok(())
    }

    /// Verifies that a copy replaces an existing destination.
    #[test]
    fn test_copy_atomic_replaces_destination() -> Result<()> {
        let dir = tempdir()?;
        let from = dir.path().join("from.txt");
        let to = dir.path().join("to.txt");
        fs::write(&from, "source")?;
        fs::write(&to, "stale")?;
        copy_atomic(&from, &to)?;
        assert_eq!(fs::read_to_string(&to)?, "source");
        Ok(())
    }
}
