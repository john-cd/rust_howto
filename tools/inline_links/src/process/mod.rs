use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

pub mod replace;

/// Walks through a directory and processes all markdown files.
///
/// # Arguments
///
/// * `root` - The directory to walk through.
/// * `f` - Function that processes one file at the time.
pub fn walk_directory<F>(root: &Path, f: F) -> Result<()>
where
    F: Fn(&Path) -> Result<()>,
{
    let files = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().is_some_and(|ext| ext == "md")
                && !e.path().ends_with("refs.incl.md")
                && !e.path().ends_with("SUMMARY.md")
            // && !e.path().to_string_lossy().contains("refs.md")
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    for file in files {
        println!("Processing {}.", file.display());
        f(&file)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    // fn test_walk_directory(root: &Path) -> Result<()> {
    //     walk_directory(root, replace::replace_in_file)?;
    //     Ok(())
    // }
}
