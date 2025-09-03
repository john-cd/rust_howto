use std::fs::DirEntry;
use std::fs;
use std::path::Path;
use tracing::info;

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
