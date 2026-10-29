// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Extract regular files from a gzipped tar archive after removing a shared
//! path prefix. The archive's file entries are written beneath the destination
//! directory, and paths containing parent components are rejected.

use std::fs;
use std::fs::File;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;
use anyhow::bail;
use flate2::read::GzDecoder;
use tar::Archive;

/// Extracts regular files from `archive_path` beneath `destination`.
///
/// # Errors
///
/// Returns an error if the archive cannot be read, an entry does not have the
/// configured prefix, or writing an extracted file fails.
fn extract_archive(
    archive_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<Vec<PathBuf>> {
    let file = File::open(archive_path.as_ref()).with_context(|| {
        format!("opening {}", archive_path.as_ref().display())
    })?;
    let mut archive = Archive::new(GzDecoder::new(file));
    let prefix = Path::new("bundle/logs");
    let destination = destination.as_ref();
    fs::create_dir_all(destination)?;
    let mut extracted_paths = Vec::new();

    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let archive_path = entry.path()?.into_owned();
        let relative_path =
            archive_path.strip_prefix(prefix).map_err(|_| {
                anyhow!(
                    "archive entry path does not start with prefix: {}",
                    archive_path.display()
                )
            })?;

        if relative_path.as_os_str().is_empty() {
            continue;
        }

        if relative_path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        {
            bail!(
                "archive entry has an unsafe path: {}",
                archive_path.display()
            );
        }

        if !entry.header().entry_type().is_file() {
            continue;
        }

        let output_path = destination.join(relative_path);
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut output = File::create(&output_path)?;
        std::io::copy(&mut entry, &mut output)?;
        extracted_paths.push(relative_path.to_owned());
    }

    Ok(extracted_paths)
}

/// Extracts the example archive into a local output directory.
fn main() -> Result<()> {
    let extracted_paths =
        extract_archive("temp/archive.tar.gz", "temp/extracted")?;
    println!("Extracted the following files:");
    for path in extracted_paths {
        println!("> {}", path.display());
    }
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    use flate2::Compression;
    use flate2::write::GzEncoder;

    use super::extract_archive;

    /// Creates a gzip-compressed tar archive containing one regular file.
    fn create_archive(
        archive_path: &Path,
        entry_path: &str,
    ) -> anyhow::Result<()> {
        let archive_file = File::create(archive_path)?;
        let encoder = GzEncoder::new(archive_file, Compression::default());
        let mut builder = tar::Builder::new(encoder);
        let contents = b"hello world";
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append_data(&mut header, entry_path, contents.as_slice())?;
        builder.into_inner()?.finish()?;
        Ok(())
    }

    /// Writes a raw path header so tests can represent a hostile archive entry.
    fn create_archive_with_raw_path(
        archive_path: &Path,
        entry_path: &str,
    ) -> anyhow::Result<()> {
        let archive_file = File::create(archive_path)?;
        let encoder = GzEncoder::new(archive_file, Compression::default());
        let mut builder = tar::Builder::new(encoder);
        let contents = b"hello world";
        let mut header = tar::Header::new_gnu();
        header.as_mut_bytes()[..entry_path.len()]
            .copy_from_slice(entry_path.as_bytes());
        header.set_size(contents.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append(&header, contents.as_slice())?;
        builder.into_inner()?.finish()?;
        Ok(())
    }

    /// Strips the shared archive prefix and writes the file beneath a temp dir.
    #[test]
    fn extraction_writes_the_prefix_relative_file() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let archive_path = temp_dir.path().join("archive.tar.gz");
        create_archive(&archive_path, "bundle/logs/example.txt")?;
        let contents = b"hello world";

        let destination = temp_dir.path().join("output");
        let extracted_paths = extract_archive(&archive_path, &destination)?;

        assert_eq!(extracted_paths, [std::path::PathBuf::from("example.txt")]);
        assert_eq!(fs::read(destination.join("example.txt"))?, contents);
        assert!(!destination.join("bundle").exists());
        Ok(())
    }

    /// Rejects an archive entry that does not start with the expected prefix.
    #[test]
    fn extraction_rejects_entries_without_the_prefix() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let archive_path = temp_dir.path().join("archive.tar.gz");
        create_archive(&archive_path, "other/example.txt")?;
        let destination = temp_dir.path().join("output");

        assert!(extract_archive(&archive_path, &destination).is_err());
        assert!(!destination.join("example.txt").exists());
        Ok(())
    }

    /// Rejects parent components rather than writing outside the destination.
    #[test]
    fn extraction_rejects_parent_components() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let archive_path = temp_dir.path().join("archive.tar.gz");
        create_archive_with_raw_path(
            &archive_path,
            "bundle/logs/../../outside.txt",
        )?;
        let destination = temp_dir.path().join("output");
        let outside_path = temp_dir.path().join("outside.txt");

        assert!(extract_archive(&archive_path, &destination).is_err());
        assert!(!outside_path.exists());
        Ok(())
    }

    #[test]
    fn main_extracts_the_sample_archive() -> anyhow::Result<()> {
        fs::create_dir_all("temp")?;
        create_archive(
            Path::new("temp/archive.tar.gz"),
            "bundle/logs/example.txt",
        )?;
        super::main()?;
        assert_eq!(fs::read("temp/extracted/example.txt")?, b"hello world");
        fs::remove_dir_all("temp/extracted")?;
        fs::remove_file("temp/archive.tar.gz")?;
        Ok(())
    }

    #[test]
    fn missing_archive_is_an_error() {
        let result = extract_archive(
            "temp/does_not_exist.tar.gz",
            "temp/does_not_exist_out",
        );
        assert!(result.is_err());
    }
}
