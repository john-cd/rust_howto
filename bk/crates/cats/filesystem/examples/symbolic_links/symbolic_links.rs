#![allow(dead_code)]
// ANCHOR: example
use std::fs;
use std::path::Path;

use anyhow::Context;

// Creating symbolic links is platform-specific, because different operating
// systems handle them differently (e.g., file vs. directory links on Windows).
fn create_symlink(original: &Path, link: &Path) -> anyhow::Result<()> {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs;
        fs::symlink(original, link)
            .context("Failed to create a new symbolic link.")
    }
    #[cfg(target_family = "windows")]
    {
        use std::os::windows::fs;
        if original.is_dir() {
            fs::symlink_dir(original, link)
        } else {
            fs::symlink_file(original, link)
        }
    }
    // A fallback for unsupported platforms:
    #[cfg(not(any(unix, windows)))]
    {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Symbolic links not supported on this platform",
        ))
    }
}

fn inspect_path<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    // While `fs::metadata(path)` follows the symbolic link and returns metadata
    // for the target file or directory, `fs::symlink_metadata(path)` returns
    // metadata for the link itself:
    let metadata = fs::symlink_metadata(path)?;
    let file_type = metadata.file_type();

    // Check if a path is a symbolic link:
    if file_type.is_symlink() {
        // `read_link` reads a symbolic link and returns the `PathBuf` it points
        // to. If the given path is not a symlink, it will return an error.
        let target_path = fs::read_link(path)?;
        println!(
            "'{}' is a symbolic link pointing to '{}'",
            path.display(),
            target_path.display()
        );
    } else if file_type.is_dir() {
        println!("'{}' is a directory.", path.display());
    } else if file_type.is_file() {
        println!("'{}' is a file.", path.display());
    } else {
        println!("'{}' is something else.", path.display());
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let original = Path::new("temp/source.txt");
    fs::write(original, "Hello, world!")?;
    let link = Path::new("temp/my_link.txt");

    create_symlink(original, link)?;

    inspect_path(original)?;
    inspect_path(link)?;
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    let temp = Path::new("./temp");
    utils::clean_folder(&temp)
        .context("Failed to clean up the temp directory.")?;
    if !temp.exists() {
        fs::create_dir(temp).context("Failed to create the temp directory.")?;
    }
    main()?;
    Ok(())
}
