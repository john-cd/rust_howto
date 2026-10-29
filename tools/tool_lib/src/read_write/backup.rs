//! Backing up files before they are overwritten.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

use super::atomic;

/// Backs up the file at `filepath` (if it exists) and then replaces it with the
/// given `contents`.
///
/// The replacement is atomic (see `atomic::write_atomic`): if writing fails,
/// the original file is untouched.
///
/// # Errors
///
/// Returns an error if the backup cannot be made or the file cannot be replaced.
pub fn backup_then_write_to<P: AsRef<Path>>(filepath: P, contents: String) -> Result<()> {
    backup(&filepath)?;
    atomic::write_atomic(filepath, [contents])
}

/// Returns the backup path of `filepath`: the full file name plus `.bak`.
///
/// Appending (instead of replacing the extension) keeps `a.md` and `a.toml`
/// from sharing `a.bak`, and an input that already ends in `.bak` maps to a
/// different file (`a.bak.bak`) instead of itself.
pub(crate) fn backup_path(filepath: &Path) -> PathBuf {
    let mut name = filepath.file_name().unwrap_or_default().to_os_string();
    name.push(".bak");
    filepath.with_file_name(name)
}

/// Creates a backup copy of a file, if it exists.
///
/// Policy: each source file has exactly one backup, `<file name>.bak`, which is
/// replaced atomically by the latest pre-write contents. Older generations are
/// not kept.
///
/// # Errors
///
/// Returns an error if the file exists but cannot be copied.
pub(crate) fn backup<P: AsRef<Path>>(filepath: P) -> Result<()> {
    let filepath = filepath.as_ref();
    if fs::exists(filepath)? {
        atomic::copy_atomic(filepath, &backup_path(filepath))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Read as _;

    use tempfile::tempdir;

    use super::*;

    // Helper function to read file content.
    /// Reads UTF-8 text from `path`.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut file = File::open(path.as_ref())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    // --- Tests for `backup` ---

    /// Checks that the file is copied to a `.bak` file.
    #[test]
    fn test_backup_file_exists() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("original.txt");
        let backup_path = dir.path().join("original.txt.bak");
        let initial_content = "This is the original content.";

        // Create the original file.
        fs::write(&file_path, initial_content)?;

        // Perform the backup.
        backup(&file_path)?;

        // Assertions
        assert!(file_path.exists(), "Original file should still exist");
        assert!(backup_path.exists(), "Backup file should have been created");

        let original_content_after = read_file_content(&file_path)?;
        let backup_content = read_file_content(&backup_path)?;

        assert_eq!(
            original_content_after, initial_content,
            "Original file content should not change"
        );
        assert_eq!(
            backup_content, initial_content,
            "Backup file content should match original"
        );

        Ok(())
    }

    /// Checks that nothing happens when there is no file to back up.
    #[test]
    fn test_backup_file_does_not_exist() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("non_existent.txt");
        let backup_path = dir.path().join("non_existent.txt.bak");

        // Attempt to backup a non-existent file
        backup(&file_path)?;

        // Assertions
        assert!(
            !file_path.exists(),
            "Original file should not have been created"
        );
        assert!(
            !backup_path.exists(),
            "Backup file should not have been created"
        );

        Ok(())
    }

    /// Checks that an older backup is replaced.
    #[test]
    fn test_backup_overwrites_existing_backup() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("original_with_old_backup.txt");
        let backup_path = dir.path().join("original_with_old_backup.txt.bak");
        let original_content = "New original content.";
        let old_backup_content = "This is old backup content.";

        // Create the original file and an old backup
        fs::write(&file_path, original_content)?;
        fs::write(&backup_path, old_backup_content)?;

        // Perform the backup
        backup(&file_path)?;

        // Assertions
        assert!(file_path.exists(), "Original file should still exist");
        assert!(backup_path.exists(), "Backup file should still exist");

        let backup_content_after = read_file_content(&backup_path)?;
        assert_eq!(
            backup_content_after, original_content,
            "Backup file should be overwritten with new original content"
        );

        Ok(())
    }

    /// Checks that backups of different files and `.bak` inputs do not collide.
    #[test]
    fn test_backup_names_are_collision_free() -> Result<()> {
        let dir = tempdir()?;
        let md = dir.path().join("a.md");
        let toml = dir.path().join("a.toml");
        let bak = dir.path().join("a.bak");
        fs::write(&md, "md")?;
        fs::write(&toml, "toml")?;
        fs::write(&bak, "bak")?;

        backup(&md)?;
        backup(&toml)?;
        backup(&bak)?;

        assert_eq!(read_file_content(dir.path().join("a.md.bak"))?, "md");
        assert_eq!(read_file_content(dir.path().join("a.toml.bak"))?, "toml");
        assert_eq!(read_file_content(dir.path().join("a.bak.bak"))?, "bak");
        assert_eq!(read_file_content(&bak)?, "bak");
        Ok(())
    }

    /// Checks that a failed write leaves the destination untouched.
    #[test]
    fn test_backup_then_write_to_failure_preserves_source() -> Result<()> {
        let dir = tempdir()?;
        // A directory cannot be replaced by a file, so the call must fail.
        let target = dir.path().join("dir.txt");
        fs::create_dir(&target)?;
        fs::write(target.join("keep"), "keep")?;
        backup_then_write_to(&target, "new".to_owned()).expect_err("a directory is not writable");
        assert_eq!(read_file_content(target.join("keep"))?, "keep");
        Ok(())
    }

    // --- Tests for `backup_then_write_to` ---

    /// Checks that an existing file is backed up before it is overwritten.
    #[test]
    fn test_backup_then_write_to_file_exists() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_to_existing.txt");
        let backup_path = dir.path().join("write_to_existing.txt.bak");
        let initial_content = "Initial content.";
        let new_content = "This is the new content written.";

        // Create the initial file
        fs::write(&file_path, initial_content)?;

        // Perform backup and write
        backup_then_write_to(&file_path, new_content.to_owned())?;

        // Assertions
        assert!(file_path.exists(), "Original file should still exist");
        assert!(backup_path.exists(), "Backup file should have been created");

        let final_content = read_file_content(&file_path)?;
        let backup_content = read_file_content(&backup_path)?;

        assert_eq!(
            final_content, new_content,
            "Original file should contain the new content"
        );
        assert_eq!(
            backup_content, initial_content,
            "Backup file should contain the initial content"
        );

        Ok(())
    }

    /// Checks that a file is created when none exists.
    #[test]
    fn test_backup_then_write_to_file_does_not_exist() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_to_new.txt");
        let backup_path = dir.path().join("write_to_new.txt.bak");
        let new_content = "Content for a new file.";

        // Perform backup and write on non-existent file
        backup_then_write_to(&file_path, new_content.to_owned())?;

        // Assertions
        assert!(file_path.exists(), "New file should have been created");
        assert!(
            !backup_path.exists(),
            "Backup file should not be created for a new file"
        );

        let final_content = read_file_content(&file_path)?;
        assert_eq!(
            final_content, new_content,
            "New file should contain the specified content"
        );

        Ok(())
    }

    /// Checks that an older backup is replaced when writing.
    #[test]
    fn test_backup_then_write_to_overwrites_existing_backup() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_to_existing_with_old_backup.txt");
        let backup_path = dir.path().join("write_to_existing_with_old_backup.txt.bak");
        let initial_content = "Initial content for file.";
        let old_backup_content = "Old backup data.";
        let new_content = "Final content written to file.";

        // Create initial file and old backup
        fs::write(&file_path, initial_content)?;
        fs::write(&backup_path, old_backup_content)?;

        // Perform backup and write
        backup_then_write_to(&file_path, new_content.to_owned())?;

        // Assertions
        assert!(file_path.exists(), "Original file should still exist");
        assert!(backup_path.exists(), "Backup file should still exist");

        let final_content = read_file_content(&file_path)?;
        let backup_content_after = read_file_content(&backup_path)?;

        assert_eq!(
            final_content, new_content,
            "Original file should contain the new content"
        );
        assert_eq!(
            backup_content_after, initial_content,
            "Backup file should be overwritten with the initial content"
        );

        Ok(())
    }

    /// Checks that writing empty content empties the file after the backup.
    #[test]
    fn test_backup_then_write_to_empty_content() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_empty.txt");
        let backup_path = dir.path().join("write_empty.txt.bak");
        let initial_content = "Some content to be replaced by empty.";

        // Create initial file
        fs::write(&file_path, initial_content)?;

        // Perform backup and write with empty string
        backup_then_write_to(&file_path, String::new())?;

        // Assertions
        assert!(file_path.exists(), "Original file should still exist");
        assert!(backup_path.exists(), "Backup file should have been created");

        let final_content = read_file_content(&file_path)?;
        let backup_content = read_file_content(&backup_path)?;

        assert_eq!(final_content, "", "Original file should now be empty");
        assert_eq!(
            backup_content, initial_content,
            "Backup file should contain the initial content"
        );

        Ok(())
    }
}
