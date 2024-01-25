/// Directory handling
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::Error;
use tracing::info;

/// Check if a path is a directory
/// Return a PathBuf if it is.
pub(crate) fn check_is_dir<S>(dir: S) -> Result<PathBuf, Error>
where
    S: AsRef<OsStr>,
{
    let dir_path = PathBuf::from(dir.as_ref());
    if !dir_path.is_dir() {
        bail!("{:?} should be a folder!", dir_path);
    }
    Ok(dir_path)
}

/// Create the parent directory(ies) for a given file (that will be
/// created later), if they don't exist
pub(crate) fn create_parent_dirs_for<P>(file_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let file_path: &Path = file_path.as_ref();
    if !file_path.is_file() {
        bail!("{:?} shoud be a file!", file_path);
    }
    match file_path.parent() {
        Some(dir) if dir != Path::new("") => {
            create_dir(dir)?;
        }
        _ => {}
    }
    Ok(())
}

/// Create a directory (including parent dierctories as needed)
pub fn create_dir<S>(dir: &S) -> Result<(), Error>
where
    S: AsRef<OsStr> + ?Sized + Debug,
{
    let dir_path = Path::new(dir);
    if !dir_path.is_dir() {
        bail!("{:?} is not a directory", dir);
    }
    match dir_path.try_exists() {
        Ok(false) => {
            std::fs::create_dir_all(dir_path)?;
            info!("{} created", dir_path.display());
        }
        Ok(true) => {
            // debug: println!("{} already exists", dest_dir);
        }
        Err(_) => {
            bail!("{:?}'s existence can neither be confirmed nor denied.", dir);
        }
    }
    Ok(())
}
