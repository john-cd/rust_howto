use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::Error;
use walkdir::DirEntry;
use walkdir::WalkDir;

/// True if the directory entry is hidden (starts with a `.`)
fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

/// Locate Markdown files within a directory
pub fn find_markdown_paths<P: AsRef<Path>>(
    root_directory: P,
) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    let walker = WalkDir::new(root_directory)
        .sort_by_file_name()
        .into_iter()
        // Skip hidden files and directories efficiently on unix systems
        .filter_entry(|e| !is_hidden(e));

    for entry in walker
        // Yields only the values for which the supplied closure returns Some(value)
        // Ignores WalkDir errors
        .filter_map(|res| res.ok())
        .filter(|de| de.file_type().is_file())
    {
        match entry.path().extension() {
            Some(extension) => {
                if extension == "md" {
                    // debug: println!("{}", entry.path().display());
                    paths.push(entry.into_path());
                } else {
                    println!("Not a Markdown file: {}", entry.path().display());
                }
            }
            None => {
                println!(
                    "Could not extract extension for {}",
                    entry.path().display()
                );
            }
        }
    }
    Ok(paths)
}

/// Create a directory
pub fn create_dir<S>(dest_dir: &S) -> Result<(), Error>
where
    S: AsRef<OsStr> + ?Sized + std::fmt::Debug,
{
    let p = Path::new(dest_dir);
    match p.try_exists() {
        Ok(false) => {
            std::fs::create_dir_all(p)?;
            println!("{} created", p.display());
        }
        Ok(true) => {
            // debug: println!("{} already exists", dest_dir);
        }
        Err(_) => {
            bail!(
                "{:?}'s existence can neither be confirmed nor denied.",
                dest_dir
            );
        }
    }
    Ok(())
}
