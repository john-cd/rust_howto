#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Unpacking a gzip-compressed tar archive with `flate2` and `tar`.

use std::fs::File;
use std::io;
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

/// Unpacks the sample archive created by the compression example.
///
/// # Errors
///
/// Returns an I/O error if the archive cannot be opened or its entries cannot
/// be extracted.
fn main() -> io::Result<()> {
    let work_dir = std::env::temp_dir().join("rust_howto_tar_compress_example");
    decompress_archive(
        &work_dir.join("archive.tar.gz"),
        &work_dir.join("unpacked"),
    )
}

/// Extracts the gzip-compressed tar archive at `archive_path` into
/// `destination`.
///
/// # Errors
///
/// Returns an I/O error if the archive cannot be read, the destination cannot
/// be created, or extraction fails.
fn decompress_archive(
    archive_path: &Path,
    destination: &Path,
) -> io::Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    std::fs::create_dir_all(destination)?;
    archive.unpack(destination)?;
    println!("Archive extracted to {}", destination.display());
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), std::io::Error> {
    main()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;
    use std::path::PathBuf;
    use std::time::SystemTime;

    use flate2::Compression;
    use flate2::write::GzEncoder;
    use tar::Builder;
    use tar::Header;

    use super::*;

    /// Extracts an archive from an explicit path into a temporary destination.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if the fixture cannot be created, extracted, or
    /// read back.
    #[test]
    fn extracts_archive_from_given_path() -> io::Result<()> {
        let work_dir = create_test_directory()?;
        let archive_path = work_dir.join("input.tar.gz");
        let destination = work_dir.join("output");
        let expected = b"archive contents";

        let archive_file = File::create(&archive_path)?;
        let encoder = GzEncoder::new(archive_file, Compression::default());
        let mut builder = Builder::new(encoder);
        let mut header = Header::new_gnu();
        header.set_size(expected.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder.append_data(
            &mut header,
            "source/message.txt",
            expected.as_slice(),
        )?;
        drop(builder.into_inner()?.finish()?);

        decompress_archive(&archive_path, &destination)?;
        let extracted = fs::read(destination.join("source/message.txt"))?;
        assert_eq!(extracted, expected);

        fs::remove_dir_all(&work_dir)?;
        Ok(())
    }

    /// Creates a unique temporary directory for a tar decompression test.
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
            "rust_howto_tar_decompress_{}_{}",
            std::process::id(),
            timestamp
        ));
        fs::create_dir(&directory)?;
        Ok(directory)
    }
}

// [review; tar_decompress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/254)
