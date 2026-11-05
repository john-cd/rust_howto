//! Reading, writing and backing up files.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

mod atomic;
mod backup;
mod diff;
mod ext;
mod merge;

pub use backup::*;
pub use diff::*;
pub use ext::*;
pub use merge::*;

/// Writes the given `contents` to a file at `filepath` if the file does not
/// already exist. If the file exists, it will attempt to create a new file
/// with a modified name.
///
/// If the file exists, it will append `.new` to the filename (before the
/// extension if present, via `extend_extension`). If that file also exists,
/// it will append `.new` again, and so on, up to 3 times.
///
/// If it fails to create a new file after 3 attempts, it will return an
/// error.
///
/// # Errors
///
/// Returns an error if the path exists but is not a regular file, if a new file
/// cannot be created after 3 attempts, or if an I/O error occurs while creating
/// or writing to the file.
pub fn write_if_not_exists<P: AsRef<Path>>(filepath: P, contents: String) -> Result<()> {
    let filepath = filepath.as_ref();
    if filepath.exists() && !filepath.is_file() {
        anyhow::bail!("{filepath:?} is not a regular file!");
    }

    let mut pathbuf: PathBuf = PathBuf::from(filepath);
    let mut c: u8 = 0;
    while fs::exists(pathbuf.as_path())? {
        pathbuf = ext::extend_extension(&pathbuf, "new");
        c += 1;
        if c > 3 {
            anyhow::bail!("Could not create an entirely new file. {pathbuf:?}");
        }
    }

    let mut file = File::create_new(&pathbuf)?;
    file.write_all(contents.as_bytes())?;
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    /// Tests writing to a file that does not yet exist.
    #[test]
    fn test_write_if_not_exists_file_does_not_exist() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("file.txt");
        let content = "Hello, world!".to_owned();

        write_if_not_exists(&target, content.clone())?;

        assert!(target.exists(), "Target file should have been created.");
        let read = fs::read_to_string(&target)?;
        assert_eq!(read, content, "Target content should match.");

        Ok(())
    }

    /// Tests that an existing file is preserved and `file.new.txt` is created.
    #[test]
    fn test_write_if_not_exists_first_collision_creates_new_suffix() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("file.txt");
        let new_target = dir.path().join("file.new.txt");
        let initial_content = "initial content";
        let updated_content = "updated content".to_owned();

        fs::write(&target, initial_content)?;

        write_if_not_exists(&target, updated_content.clone())?;

        assert!(target.exists(), "Original file should still exist.");
        assert!(
            new_target.exists(),
            "File with .new suffix should have been created."
        );

        let read_orig = fs::read_to_string(&target)?;
        let read_new = fs::read_to_string(&new_target)?;
        assert_eq!(
            read_orig, initial_content,
            "Original file content unchanged."
        );
        assert_eq!(read_new, updated_content, "New file has updated content.");

        Ok(())
    }

    /// Tests that when `file.txt` and `file.new.txt` exist, `file.new.new.txt` is created.
    #[test]
    fn test_write_if_not_exists_second_collision() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("file.txt");
        let first_new = dir.path().join("file.new.txt");
        let second_new = dir.path().join("file.new.new.txt");

        fs::write(&target, "v1")?;
        fs::write(&first_new, "v2")?;

        write_if_not_exists(&target, "v3".to_owned())?;

        assert!(target.exists());
        assert!(first_new.exists());
        assert!(
            second_new.exists(),
            "Second collision file should be created."
        );

        assert_eq!(fs::read_to_string(&target)?, "v1");
        assert_eq!(fs::read_to_string(&first_new)?, "v2");
        assert_eq!(fs::read_to_string(&second_new)?, "v3");

        Ok(())
    }

    /// Tests that when `file.txt`, `file.new.txt`, `file.new.new.txt`, and `file.new.new.new.txt`
    /// all exist, the function exceeds 3 attempts and returns an error.
    #[test]
    fn test_write_if_not_exists_exceeds_max_attempts() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("file.txt");
        let first_new = dir.path().join("file.new.txt");
        let second_new = dir.path().join("file.new.new.txt");
        let third_new = dir.path().join("file.new.new.new.txt");

        fs::write(&target, "v0")?;
        fs::write(&first_new, "v1")?;
        fs::write(&second_new, "v2")?;
        fs::write(&third_new, "v3")?;

        let result = write_if_not_exists(&target, "v4".to_owned());
        assert!(
            result.is_err(),
            "Should return an error when max attempts are exceeded."
        );

        let fourth_new = dir.path().join("file.new.new.new.new.txt");
        assert!(
            !fourth_new.exists(),
            "Fourth new file should not be created."
        );

        Ok(())
    }

    /// Tests writing to a file without an extension when a collision occurs (`file.new`).
    #[test]
    fn test_write_if_not_exists_without_extension() -> Result<()> {
        let dir = tempdir()?;
        let target = dir.path().join("file");
        let new_target = dir.path().join("file.new");

        fs::write(&target, "v0")?;
        write_if_not_exists(&target, "v1".to_owned())?;

        assert!(target.exists());
        assert!(new_target.exists());
        assert_eq!(fs::read_to_string(&target)?, "v0");
        assert_eq!(fs::read_to_string(&new_target)?, "v1");

        Ok(())
    }

    /// Tests that an error is returned when `filepath` is an existing directory.
    #[test]
    fn test_write_if_not_exists_on_directory_fails() -> Result<()> {
        let dir = tempdir()?;
        let subdir = dir.path().join("sub");
        fs::create_dir(&subdir)?;

        let result = write_if_not_exists(&subdir, "content".to_owned());
        assert!(
            result.is_err(),
            "Writing to an existing directory must fail."
        );

        Ok(())
    }
}
