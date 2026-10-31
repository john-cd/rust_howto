// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Creating and inspecting symbolic links.
//!
//! A symbolic link is a file that points to another path. Following it gives
//! the target; removing the link does not remove the target. On Windows
//! creating them may require special privileges.

use std::fs;
use std::path::Path;

// Creating symbolic links is platform-specific, because different operating
// systems handle them differently (e.g., file vs. directory links on Windows).
/// Creates a symbolic link `link` that points to `original`.
fn create_symlink(original: &Path, link: &Path) -> anyhow::Result<()> {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::fs;

        use anyhow::Context as _;
        fs::symlink(original, link)
            .context("Failed to create a new symbolic link.")
    }
    #[cfg(target_family = "windows")]
    {
        use std::os::windows::fs;
        if original.is_dir() {
            Ok(fs::symlink_dir(original, link)?)
        } else {
            Ok(fs::symlink_file(original, link)?)
        }
    }
    // A fallback for unsupported platforms:
    #[cfg(not(any(unix, windows)))]
    {
        Err(anyhow::Error::msg(
            "Symbolic links not supported on this platform",
        ))
    }
}

/// Prints whether `path` is a symbolic link and where it points.
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

/// Creates a file and a link to it, then inspects both.
fn main() -> anyhow::Result<()> {
    demo(Path::new("temp"))
}

/// Creates `source.txt` and `my_link.txt` in `dir`, then inspects both.
fn demo(dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(dir)?;
    let original = dir.join("source.txt");
    fs::write(&original, "Hello, world!")?;
    let link = dir.join("my_link.txt");
    let (original, link) = (original.as_path(), link.as_path());

    match fs::remove_file(link) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    create_symlink(&fs::canonicalize(original)?, link)?;

    inspect_path(original)?;
    inspect_path(link)?;
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example in a clean `temp` folder.
    /// Runs the symbolic links example through its entry point.
    #[test]
    fn test_symbolic_links_example_entry_point() -> anyhow::Result<()> {
        use anyhow::Context as _;
        let temp = Path::new("./temp");
        utils::clean_folder(temp)
            .context("Failed to clean up the temp directory.")?;
        if !temp.exists() {
            fs::create_dir(temp)
                .context("Failed to create the temp directory.")?;
        }
        main()?;
        Ok(())
    }

    /// The link points at the canonical source, reads through to its content,
    /// and rerunning replaces the old link.
    #[cfg(any(unix, windows))]
    #[test]
    fn creates_a_link_to_the_source() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        demo(dir.path())?;
        demo(dir.path())?;
        let link = dir.path().join("my_link.txt");
        assert!(fs::symlink_metadata(&link)?.file_type().is_symlink());
        assert_eq!(
            fs::canonicalize(fs::read_link(&link)?)?,
            fs::canonicalize(dir.path().join("source.txt"))?
        );
        assert_eq!(fs::read_to_string(&link)?, "Hello, world!");
        Ok(())
    }

    /// Linking to a path that already exists is an error.
    #[cfg(any(unix, windows))]
    #[test]
    fn existing_link_path_is_an_error() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let source = dir.path().join("s");
        fs::write(&source, "")?;
        let link = dir.path().join("l");
        create_symlink(&source, &link)?;
        assert!(create_symlink(&source, &link).is_err());
        assert!(inspect_path(dir.path().join("missing")).is_err());
        Ok(())
    }
}
