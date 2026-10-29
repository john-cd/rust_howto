// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of using the `zip` crate.
//!
//! The `zip` crate supports reading and writing ZIP files.
//!
//! This example demonstrates how to create a ZIP archive, add files and
//! directories to it, and set file options such as compression method and
//! permissions.

use std::fs::File;
use std::io::Write as _;

use zip::write::SimpleFileOptions;
use zip::write::ZipWriter;

/// Creates a zip archive with two files and an empty directory.
///
/// # Errors
///
/// Returns an error if temporary storage or archive writing fails.
fn main() -> anyhow::Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let path = temp_dir.path().join("my_archive.zip");
    let file = File::create(&path)?;
    let mut zip = ZipWriter::new(file);

    // Define file options for the files we'll add.
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        // Read, write, execute for owner and group, read and execute for
        // others.
        .unix_permissions(0o755);

    // Add a file to the archive with content.
    zip.start_file("hello.txt", options)?;
    zip.write_all(b"Test")?;

    // Add another file with different content, in a subfolder.
    zip.start_file("data/info.txt", options)?;
    zip.write_all(b"Some information.\n")?;

    // Add an empty directory.
    zip.add_directory("empty_folder", options)?;

    // Finish the archive.
    zip.finish()?;

    println!("Successfully created '{}'", path.display());
    Ok(())
}

// ANCHOR_END: example

/// # Errors
///
/// Returns an error if archive creation fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the zip example's main path succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error if archive creation fails.
    /// Runs the zip example through its entry point.
    #[test]
    fn test_zip_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn archive_roundtrip_preserves_names_and_contents() -> anyhow::Result<()> {
        use std::io::Read as _;

        let dir = tempfile::tempdir()?;
        let path = dir.path().join("a.zip");
        let mut zip = ZipWriter::new(File::create(&path)?);
        let options = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
        zip.start_file("hello.txt", options)?;
        zip.write_all(b"Test")?;
        zip.add_directory("empty_folder", options)?;
        zip.finish()?;

        let mut archive = zip::ZipArchive::new(File::open(&path)?)?;
        assert_eq!(archive.len(), 2);
        let mut contents = String::new();
        archive
            .by_name("hello.txt")?
            .read_to_string(&mut contents)?;
        assert_eq!(contents, "Test");
        assert!(archive.by_name("empty_folder/")?.is_dir());
        assert!(archive.by_name("missing.txt").is_err());
        Ok(())
    }
}
