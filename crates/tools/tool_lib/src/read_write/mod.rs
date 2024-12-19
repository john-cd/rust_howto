use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

mod diff;
mod ext;
mod merge;

pub use diff::*;
pub use ext::*;
pub use merge::*;

pub fn write_if_not_exists<P: AsRef<Path>>(
    filepath: P,
    contents: String,
) -> Result<()> {
    let filepath = filepath.as_ref();
    if !filepath.is_file() {
        anyhow::bail!("{:?} is not a regular file!", filepath);
    }

    let mut pathbuf: PathBuf = PathBuf::from(filepath);
    let mut c: u8 = 0;
    while fs::exists(pathbuf.as_path())? {
        pathbuf = ext::extend_extension(&pathbuf, "new");
        c += 1;
        if c > 3 {
            anyhow::bail!(
                "Could not create an entirely new file. {:?}",
                pathbuf
            );
        }
    }

    let mut file = File::create_new(filepath)?;
    file.write_all(contents.as_bytes())?;
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}

pub fn backup_then_write_to<P: AsRef<Path>>(
    filepath: P,
    contents: String,
) -> Result<()> {
    backup(&filepath)?;

    let filepath = filepath.as_ref();
    let mut file = File::create(filepath)?;
    file.write_all(contents.as_bytes())?;
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}

/// Create a backup copy of a file, if it exists.
/// This function will overwrite the contents of the destination.
fn backup<P: AsRef<Path>>(filepath: P) -> Result<()> {
    let filepath = filepath.as_ref();
    if fs::exists(filepath)? {
        let mut backup = PathBuf::from(filepath);
        backup.set_extension("bak");
        // This function will overwrite the contents of `to``.
        fs::copy(filepath, backup)?;
        // Alternatively:
        // That also replaces the backup file if it already exists.
        // fs::rename(filepath, backup)?;
    }
    Ok(())
}