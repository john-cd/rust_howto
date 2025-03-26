use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use tracing::info;
use walkdir::WalkDir;

/// Main entry point of the clean tool.
fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        //.with_max_level(tracing::Level::INFO)
        .init();
    // Look for "temp" subfolders
    // The root path is hard-coded to avoid any issues with the current working
    // directory
    for entry in WalkDir::new("/code/bk/crates")
        //.min_depth(2) // Look at the grandchildren, etc of the root folder
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir()) // Only look at directories
        .filter_map(|e| e.ok())
    // Silently skip folders with permission errors
    {
        if entry.path().ends_with("temp") {
            // println!("Clean {:?}", entry.path());
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
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

/// Remove all files (not starting with .) and subfolders in a given folder.
/// BEWARE: destructive.
fn clean_folder(dir: &Path) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // ignore e.g. .gitkeep in the base folder
            if entry.path().is_file() && !is_hidden(&entry) {
                fs::remove_file(path)?;
                info!("Removed {:?}", entry.path());
            }
            let path = entry.path();
            // remove subfolders
            if path.is_dir() {
                fs::remove_dir_all(path)?;
                info!("Removed {:?}", entry.path());
            }
        }
    }

    Ok(())
}
