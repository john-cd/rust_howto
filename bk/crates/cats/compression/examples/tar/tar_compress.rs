#![allow(dead_code)]
#![allow(clippy::incompatible_msrv)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This function creates a gzip-compressed tar archive from a directory.

use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use flate2::Compression;
use flate2::write::GzEncoder;

/// Creates a sample input directory and archive under the system temp
/// directory.
///
/// # Errors
///
/// Returns an I/O error if the sample directory or archive cannot be written.
fn main() -> io::Result<()> {
    let work_dir = std::env::temp_dir().join("rust_howto_tar_compress_example");
    let source_dir = work_dir.join("source");
    fs::create_dir_all(&source_dir)?;
    fs::write(
        source_dir.join("example.txt"),
        b"A file in the sample archive.\n",
    )?;
    compress_directory(&source_dir, &work_dir.join("archive.tar.gz"))
}

/// Compresses `source_dir` into a gzip-compressed tar archive at
/// `archive_path`.
///
/// Files are stored under the `backup/logs` path inside the archive.
///
/// # Errors
///
/// Returns an I/O error if the archive cannot be created, the directory cannot
/// be added, or the compressed stream cannot be finished.
fn compress_directory(
    source_dir: &Path,
    archive_path: &Path,
) -> io::Result<()> {
    let archive_file = File::create(archive_path)?;
    let encoder = GzEncoder::new(archive_file, Compression::default());
    let mut builder = tar::Builder::new(encoder);
    builder.append_dir_all("backup/logs", source_dir)?;
    drop(builder.into_inner()?.finish()?);
    println!("Archive created at {}", archive_path.display());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
///
/// Returns an I/O error if the sample source or archive cannot be written.
pub fn run() -> Result<(), std::io::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::time::SystemTime;

    use flate2::read::GzDecoder;
    use tar::Archive;

    use super::*;

    /// Archives a temporary directory and verifies the finished gzip tarball.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the fixture cannot be created, archived, or
    /// extracted for verification.
    #[test]
    fn archives_directory_and_finishes_gzip_stream() -> io::Result<()> {
        let work_dir = create_test_directory()?;
        let source_dir = work_dir.join("source");
        let archive_path = work_dir.join("archive.tar.gz");
        let destination = work_dir.join("extracted");
        let expected = b"compressed fixture";
        fs::create_dir(&source_dir)?;
        fs::write(source_dir.join("message.txt"), expected)?;

        compress_directory(&source_dir, &archive_path)?;

        let archive_file = File::open(&archive_path)?;
        let mut archive = Archive::new(GzDecoder::new(archive_file));
        archive.unpack(&destination)?;
        let archived_contents =
            fs::read(destination.join("backup/logs/message.txt"))?;
        assert_eq!(archived_contents, expected);

        fs::remove_dir_all(&work_dir)?;
        Ok(())
    }

    /// Creates a unique temporary directory for a tar compression test.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the directory cannot be created.
    fn create_test_directory() -> io::Result<PathBuf> {
        let timestamp = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(io::Error::other)?
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rust_howto_tar_compress_{}_{}",
            std::process::id(),
            timestamp
        ));
        fs::create_dir(&directory)?;
        Ok(directory)
    }
}
// [review; tar_compress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/255)
