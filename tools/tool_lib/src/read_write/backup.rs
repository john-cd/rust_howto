use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

/// Backs up the file at `filepath` (if it exists) and then writes the given
/// `contents` to the file.
pub fn backup_then_write_to<P: AsRef<Path>>(filepath: P, contents: String) -> Result<()> {
    backup(&filepath)?;

    let filepath = filepath.as_ref();
    let mut file = File::create(filepath)?; // Create a file if it does not exist, and truncate it if it does.
    file.write_all(contents.as_bytes())?;
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}

/// Create a backup copy of a file, if it exists.
/// This function will overwrite the contents of the destination.
pub(crate) fn backup<P: AsRef<Path>>(filepath: P) -> Result<()> {
    let filepath = filepath.as_ref();
    if fs::exists(filepath)? {
        let mut backup = PathBuf::from(filepath);
        backup.set_extension("bak");
        // This function will overwrite the contents of `to`.
        fs::copy(filepath, backup)?;
        // Alternatively:
        // That also replaces the backup file if it already exists.
        // fs::rename(filepath, backup)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use tempfile::tempdir;

    use super::*;

    // Helper function to read file content.
    fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut file = File::open(path.as_ref())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    // --- Tests for `backup` ---

    #[test]
    fn test_backup_file_exists() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("original.txt");
        let backup_path = dir.path().join("original.bak");
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

    #[test]
    fn test_backup_file_does_not_exist() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("non_existent.txt");
        let backup_path = dir.path().join("non_existent.bak");

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

    #[test]
    fn test_backup_overwrites_existing_backup() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("original_with_old_backup.txt");
        let backup_path = dir.path().join("original_with_old_backup.bak");
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

    // --- Tests for `backup_then_write_to` ---

    #[test]
    fn test_backup_then_write_to_file_exists() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_to_existing.txt");
        let backup_path = dir.path().join("write_to_existing.bak");
        let initial_content = "Initial content.";
        let new_content = "This is the new content written.";

        // Create the initial file
        fs::write(&file_path, initial_content)?;

        // Perform backup and write
        backup_then_write_to(&file_path, new_content.to_string())?;

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

    #[test]
    fn test_backup_then_write_to_file_does_not_exist() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_to_new.txt");
        let backup_path = dir.path().join("write_to_new.bak");
        let new_content = "Content for a new file.";

        // Perform backup and write on non-existent file
        backup_then_write_to(&file_path, new_content.to_string())?;

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

    #[test]
    fn test_backup_then_write_to_overwrites_existing_backup() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_to_existing_with_old_backup.txt");
        let backup_path = dir.path().join("write_to_existing_with_old_backup.bak");
        let initial_content = "Initial content for file.";
        let old_backup_content = "Old backup data.";
        let new_content = "Final content written to file.";

        // Create initial file and old backup
        fs::write(&file_path, initial_content)?;
        fs::write(&backup_path, old_backup_content)?;

        // Perform backup and write
        backup_then_write_to(&file_path, new_content.to_string())?;

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

    #[test]
    fn test_backup_then_write_to_empty_content() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("write_empty.txt");
        let backup_path = dir.path().join("write_empty.bak");
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
