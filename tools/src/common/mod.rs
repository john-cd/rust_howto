//! Common utilities methods for tests
use std::fs;
use std::path::Path;

use tracing::info;

/// Remove all files (not starting with .) and subfolders in a given folder.
/// BEWARE: destructive.
pub fn clean_folder(dir: &Path) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // ignore e.g. .gitkeep in the base folder
            if entry.path().is_file()
                && !entry.file_name().to_string_lossy().starts_with(".")
            {
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
