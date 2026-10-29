//! Removes build artifacts and other generated files from the repository.
//!
//! Deletion boundary: only the direct children of directories named `temp`
//! below the given root are removed. Symbolic links and reparse points are never
//! followed, so nothing outside the root tree is modified.
//!
//! TODO Reviewed ??-??-2026.

use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

use clap::Parser;
use tracing::info;
use walkdir::WalkDir;

/// Command-line arguments.
#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Root of the directory tree to search for `temp` folders.
    #[clap(help = "Path to the directory to clean.")]
    directory: PathBuf,
}

/// Cleans every `temp` folder under the directory given on the command line.
///
/// Traversal completes before anything is deleted, so an unreadable directory
/// or an invalid root aborts the run with a nonzero status and leaves the tree
/// untouched.
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let args = Args::parse();
    let root = args.directory;
    println!("Cleaning {}", root.display());

    // Symbolic links and reparse points are not followed during traversal.
    let walker = WalkDir::new(&root).follow_links(false).into_iter();
    for dir in find_temp_dirs(walker)? {
        clean_folder(&dir)?;
    }
    Ok(())
}

/// Collects the `temp` directories found by a directory walk.
///
/// Only real directories are selected; symbolic links are not entered.
///
/// # Errors
///
/// Returns the first traversal error (missing root, permission denied, ...)
/// with the offending path in the message.
fn find_temp_dirs(
    entries: impl Iterator<Item = walkdir::Result<walkdir::DirEntry>>,
) -> anyhow::Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| {
            let path = e
                .path()
                .map_or_else(|| "<unknown>".to_owned(), |p| p.display().to_string());
            anyhow::Error::new(e).context(format!("cannot traverse {path}"))
        })?;
        if entry.file_type().is_dir() && entry.path().ends_with("temp") {
            found.push(entry.path().to_path_buf());
        }
    }
    Ok(found)
}

/// Checks if a directory entry is hidden (starts with a dot).
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}

/// Removes all files (not starting with .) and subfolders in a given folder.
///
/// Deletion boundary: only direct children of `dir` are removed. A symbolic
/// link is removed as a link; its target is never followed or modified.
/// BEWARE: destructive.
///
/// # Errors
///
/// Returns an error if `dir` cannot be read or an entry cannot be removed.
fn clean_folder(dir: &Path) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                // Remove subfolders.
                fs::remove_dir_all(&path)?;
            } else if is_hidden(&entry) {
                // Ignore e.g. .gitkeep in the base folder.
                continue;
            } else {
                remove_non_dir(&path, file_type)?;
            }
            info!("Removed {:?}", path);
        }
    }

    Ok(())
}

/// Removes a file or a symbolic link without following the link.
fn remove_non_dir(path: &Path, file_type: fs::FileType) -> std::io::Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::FileTypeExt as _;
        if file_type.is_symlink_dir() {
            return fs::remove_dir(path);
        }
    }
    let _ = file_type;
    fs::remove_file(path)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use tempfile::tempdir;

    use super::*;

    /// Checks that only names starting with a dot are hidden.
    #[test]
    fn test_is_hidden() {
        let dir = tempdir().expect("input should parse");
        let hidden_file = dir.path().join(".hidden.txt");
        let visible_file = dir.path().join("visible.txt");

        fs::File::create(&hidden_file).expect("input should parse");
        fs::File::create(&visible_file).expect("input should parse");

        let hidden_entry = fs::read_dir(dir.path())
            .expect("input should parse")
            .find(|e| e.as_ref().expect("input should parse").path() == hidden_file)
            .expect("input should parse")
            .expect("input should parse");

        let visible_entry = fs::read_dir(dir.path())
            .expect("input should parse")
            .find(|e| e.as_ref().expect("input should parse").path() == visible_file)
            .expect("input should parse")
            .expect("input should parse");

        assert!(is_hidden(&hidden_entry));
        assert!(!is_hidden(&visible_entry));
    }

    // Helper function to create a dummy directory structure for testing
    /// Creates the dummy dir.
    fn create_dummy_dir(root: &Path) {
        fs::create_dir_all(root.join("temp")).expect("input should parse");
        fs::File::create(root.join("temp").join("file1.txt")).expect("input should parse");
        fs::File::create(root.join("temp").join(".hidden.txt")).expect("input should parse");
        fs::create_dir(root.join("temp").join("subdir")).expect("input should parse");
        fs::File::create(root.join("temp").join("subdir").join("file2.txt"))
            .expect("input should parse");
        fs::File::create(root.join("file3.txt")).expect("input should parse");
        fs::File::create(root.join(".gitkeep")).expect("input should parse");
    }

    /// Checks that files and folders are removed, except hidden ones.
    #[test]
    fn test_clean_folder() {
        let dir = tempdir().expect("input should parse");
        let temp_dir = dir.path().join("temp");
        create_dummy_dir(dir.path());

        // Check that the files and folders exist before cleaning
        assert!(temp_dir.join("file1.txt").exists());
        assert!(temp_dir.join(".hidden.txt").exists());
        assert!(temp_dir.join("subdir").exists());
        assert!(temp_dir.join("subdir").join("file2.txt").exists());
        assert!(dir.path().join("file3.txt").exists());
        assert!(dir.path().join(".gitkeep").exists());

        clean_folder(&temp_dir).expect("input should parse");

        // Check that the files and folders have been removed
        assert!(!temp_dir.join("file1.txt").exists());
        assert!(temp_dir.join(".hidden.txt").exists()); // should not be removed
        assert!(!temp_dir.join("subdir").exists());
        assert!(!temp_dir.join("subdir").join("file2.txt").exists());
        assert!(dir.path().join("file3.txt").exists()); // should not be removed
        assert!(dir.path().join(".gitkeep").exists()); // should not be removed
    }

    /// Checks that a failed traversal entry is reported with its path.
    #[test]
    fn test_find_temp_dirs_reports_injected_traversal_failure() {
        let dir = tempdir().expect("a temporary directory should be created");
        let missing = dir.path().join("missing");
        let failing = WalkDir::new(&missing).into_iter();
        let error = find_temp_dirs(failing).expect_err("a missing root should fail");
        assert!(format!("{error:#}").contains("missing"));
    }

    /// Checks that only `temp` directories are collected, not files named `temp`.
    #[test]
    fn test_find_temp_dirs_selects_directories_only() {
        let dir = tempdir().expect("a temporary directory should be created");
        fs::create_dir_all(dir.path().join("a").join("temp")).expect("dirs should be created");
        fs::create_dir_all(dir.path().join("b")).expect("dirs should be created");
        fs::File::create(dir.path().join("b").join("temp")).expect("file should be created");
        let found =
            find_temp_dirs(WalkDir::new(dir.path()).into_iter()).expect("traversal should succeed");
        assert_eq!(found, vec![dir.path().join("a").join("temp")]);
    }

    /// Checks that a symbolic link inside `temp` is removed without touching its target.
    #[cfg(unix)]
    #[test]
    fn test_clean_folder_does_not_follow_symlinks() {
        let outside = tempdir().expect("a temporary directory should be created");
        fs::File::create(outside.path().join("keep.txt")).expect("file should be created");
        let dir = tempdir().expect("a temporary directory should be created");
        let temp_dir = dir.path().join("temp");
        fs::create_dir(&temp_dir).expect("dir should be created");
        std::os::unix::fs::symlink(outside.path(), temp_dir.join("link"))
            .expect("symlink should be created");
        clean_folder(&temp_dir).expect("cleaning should succeed");
        assert!(!temp_dir.join("link").exists());
        assert!(outside.path().join("keep.txt").exists());
    }

    /// Checks that a missing folder is not an error.
    #[test]
    fn test_clean_folder_nonexistent() {
        let dir = tempdir().expect("input should parse");
        let non_existent_dir = dir.path().join("nonexistent");

        // Check that the folder does not exist
        assert!(!non_existent_dir.exists());

        // Should not panic
        let result = clean_folder(&non_existent_dir);
        result.expect("input should parse");
    }
}
