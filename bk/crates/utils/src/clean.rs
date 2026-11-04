//! Removing the content of a folder.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use tracing::info;

/// Checks if a directory entry is hidden (starts with a dot).
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}

/// Remove all files (not starting with .) and subfolders in a given folder.
/// BEWARE: destructive.
pub fn clean_folder(dir: &Path) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if entry.path().is_symlink() {
                fs::remove_file(&path)?;
                info!("Removed symlink {:?}", entry.path());
            }
            // Ignore e.g. .gitkeep in the base folder.
            if entry.path().is_file() && !is_hidden(&entry) {
                fs::remove_file(&path)?;
                info!("Removed {:?}", entry.path());
            }
            let path = entry.path();
            // Remove subfolders.
            if path.is_dir() {
                fs::remove_dir_all(path)?;
                info!("Removed {:?}", entry.path());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::path::Path;

    use tempfile::tempdir;

    use super::*;

    /// Checks that only filenames starting with a dot are considered hidden.
    #[test]
    fn test_is_hidden() {
        let dir = tempdir().expect("temporary directory should be created");
        let hidden_file = dir.path().join(".hidden.txt");
        let visible_file = dir.path().join("visible.txt");

        fs::File::create(&hidden_file).expect("hidden file should be created");
        fs::File::create(&visible_file).expect("visible file should be created");

        let hidden_entry = fs::read_dir(dir.path())
            .expect("read_dir should succeed")
            .find(|e| e.as_ref().expect("dir entry should be valid").path() == hidden_file)
            .expect("hidden file entry should be found")
            .expect("dir entry should be ok");

        let visible_entry = fs::read_dir(dir.path())
            .expect("read_dir should succeed")
            .find(|e| e.as_ref().expect("dir entry should be valid").path() == visible_file)
            .expect("visible file entry should be found")
            .expect("dir entry should be ok");

        assert!(is_hidden(&hidden_entry));
        assert!(!is_hidden(&visible_entry));
    }

    /// Helper to create a nested fixture directory for testing folder cleanup.
    fn create_test_fixtures(root: &Path) {
        fs::create_dir_all(root.join("subdir/nested")).expect("subdirectories should be created");
        fs::File::create(root.join("file1.txt")).expect("file1 should be created");
        fs::File::create(root.join(".hidden_file")).expect("hidden file should be created");
        fs::File::create(root.join(".gitkeep")).expect(".gitkeep should be created");
        fs::File::create(root.join("subdir").join("nested_file.txt"))
            .expect("nested file should be created");
        fs::File::create(root.join("subdir/nested").join("deep_file.txt"))
            .expect("deep file should be created");
    }

    /// Checks that files and subfolders are removed while top-level hidden files are preserved.
    #[test]
    fn test_clean_folder_basic() {
        let dir = tempdir().expect("temporary directory should be created");
        let target_dir = dir.path().join("target");
        fs::create_dir(&target_dir).expect("target directory should be created");
        create_test_fixtures(&target_dir);

        // Verify initial state.
        assert!(target_dir.join("file1.txt").exists());
        assert!(target_dir.join(".hidden_file").exists());
        assert!(target_dir.join(".gitkeep").exists());
        assert!(target_dir.join("subdir").exists());
        assert!(target_dir.join("subdir/nested_file.txt").exists());
        assert!(target_dir.join("subdir/nested/deep_file.txt").exists());

        clean_folder(&target_dir).expect("clean_folder should succeed");

        // Verify cleaned state.
        assert!(!target_dir.join("file1.txt").exists());
        assert!(target_dir.join(".hidden_file").exists());
        assert!(target_dir.join(".gitkeep").exists());
        assert!(!target_dir.join("subdir").exists());
    }

    /// Checks that cleaning an already empty folder succeeds without error.
    #[test]
    fn test_clean_folder_empty_dir() {
        let dir = tempdir().expect("temporary directory should be created");
        let target_dir = dir.path().join("empty");
        fs::create_dir(&target_dir).expect("empty directory should be created");

        clean_folder(&target_dir).expect("clean_folder on empty directory should succeed");
        assert!(target_dir.is_dir());
    }

    /// Checks that calling clean_folder on a nonexistent directory is a no-op and succeeds.
    #[test]
    fn test_clean_folder_nonexistent() {
        let dir = tempdir().expect("temporary directory should be created");
        let non_existent = dir.path().join("non_existent_folder");

        assert!(!non_existent.exists());
        clean_folder(&non_existent).expect("clean_folder on nonexistent path should succeed");
    }

    /// Checks that calling clean_folder on a regular file does not remove it or return an error.
    #[test]
    fn test_clean_folder_on_file() {
        let dir = tempdir().expect("temporary directory should be created");
        let file_path = dir.path().join("regular_file.txt");
        fs::File::create(&file_path).expect("file should be created");

        clean_folder(&file_path).expect("clean_folder on a file should return Ok");
        assert!(file_path.is_file());
    }

    /// Checks that a symbolic link inside the directory is removed without deleting the link target.
    #[cfg(unix)]
    #[test]
    fn test_clean_folder_symlinks() {
        let outside_dir = tempdir().expect("outside temp directory should be created");
        let target_file = outside_dir.path().join("target_file.txt");
        fs::write(&target_file, "important content").expect("target file should be written");

        let clean_dir = tempdir().expect("clean temp directory should be created");
        let symlink_path = clean_dir.path().join("symlink_to_file");
        symlink(&target_file, &symlink_path).expect("symlink should be created");

        assert!(symlink_path.exists());
        clean_folder(clean_dir.path()).expect("clean_folder should succeed");

        assert!(!symlink_path.exists());
        assert!(target_file.exists());
        assert_eq!(
            fs::read_to_string(&target_file).expect("target file content should be intact"),
            "important content"
        );
    }
}
