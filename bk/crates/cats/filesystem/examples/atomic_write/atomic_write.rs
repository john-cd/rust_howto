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
fn replace_with_tempfile(target: &Path, contents: &[u8]) -> anyhow::Result<()> {
    // The temporary file must live on the same filesystem as the target,
    // otherwise the final rename is not atomic. The parent directory is the
    // safest choice.
    let dir = target
        .parent()
        .context("the target path has no parent directory")?;
    let mut temp = NamedTempFile::new_in(dir)?;
    temp.write_all(contents)?;
    // Flush the data to disk *before* the rename. Otherwise, after a crash the
    // new name could point to an empty or partially written file.
    temp.as_file().sync_all()?;
    // `persist` renames the temporary file over the target. It also replaces
    // an existing target on Windows. The temporary file is removed on error.
    temp.persist(target)?;
    Ok(())
}

/// Replaces `target` with `contents` using the `atomicwrites` crate, which
/// performs the same steps behind a closure-based API.
fn replace_with_atomicwrites(
    target: &Path,
    contents: &[u8],
) -> anyhow::Result<()> {
    AtomicFile::new(target, AllowOverwrite)
        .write(|file| file.write_all(contents))?;
    Ok(())
}

/// Writes the same file twice with each variant.
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

    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
