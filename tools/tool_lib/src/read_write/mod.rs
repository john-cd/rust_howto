use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;

mod backup;
mod diff;
mod ext;
mod merge;

pub use backup::*;
pub use diff::*;
pub use ext::*;
pub use merge::*;

/// Writes the given `contents` to a file at `filepath` if the file does not
/// already exist. If the file exists, it will attempt to create a new file
/// with a modified name.
///
/// If the file exists, it will append `.new` to the filename. If that file
/// also exists, it will append `.new` again, and so on, up to 3 times.
///
/// If it fails to create a new file after 3 attempts, it will return an
/// error.
///
pub fn write_if_not_exists<P: AsRef<Path>>(filepath: P, contents: String) -> Result<()> {
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
            anyhow::bail!("Could not create an entirely new file. {:?}", pathbuf);
        }
    }

    let mut file = File::create_new(filepath)?;
    file.write_all(contents.as_bytes())?;
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}
