// TODO Reviewed ??-??-2026
//! Removes build artifacts and other generated files from the repository.
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

/// Cleans every `temp` folder under the given directory.
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::WARN)
        .init();

    let args = Args::parse();
    let crates_dir = args.directory;
    println!("Cleaning {}", crates_dir.display());

    // Look for "temp" subfolders
    // The root path is hard-coded to avoid any issues with the current working
    // directory
    for entry in WalkDir::new(crates_dir)
        //.min_depth(2) // Look at the grandchildren, etc of the root folder
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir()) // Only look at directories
        .filter_map(std::result::Result::ok)
    // Silently skip folders with permission errors
    {
        if entry.path().ends_with("temp") {
            clean_folder(entry.path())?;
        }
    }
    Ok(())
}

/// Checks if a directory entry is hidden (starts with a dot).
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .is_some_and(|s| s.starts_with('.'))
}

/// Remove all files (not starting with .) and subfolders in a given folder.
/// BEWARE: destructive.
fn clean_folder(dir: &Path) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // Ignore e.g. .gitkeep in the base folder.
            if entry.path().is_file() && !is_hidden(&entry) {
                fs::remove_file(path)?;
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
