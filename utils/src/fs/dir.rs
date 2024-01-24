/// Directory handling
use std::ffi::OsStr;
use std::path::Path;

use anyhow::bail;
use anyhow::Error;

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
