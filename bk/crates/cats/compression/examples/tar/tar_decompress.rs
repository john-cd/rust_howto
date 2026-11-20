// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Unpacking a gzip-compressed tar archive with `flate2` and `tar`.
//!
//! Unpacking an archive from an untrusted source can overwrite files outside
//! the target directory if an entry has a path such as `../../etc/passwd` (a
//! "zip slip" attack). Check paths before extracting.

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
fn main(work_dir: &Path) -> io::Result<()> {
    decompress_archive(&work_dir.join("archive.tar.gz"), &work_dir.join("unpacked"))
}

/// Extracts the gzip-compressed tar archive at `archive_path` into
/// `destination`.
///
/// # Errors
///
/// Returns an I/O error if the archive cannot be read, the destination cannot
/// be created, or extraction fails.
fn decompress_archive(archive_path: &Path, destination: &Path) -> io::Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    std::fs::create_dir(destination)?;
    archive.unpack(destination)?;
    println!("Archive extracted to {}", destination.display());
    Ok(())
}
// ANCHOR_END: example

/// Extracts the archive created by the compression example.
///
/// # Errors
///
/// Returns an I/O error if the archive cannot be read or extracted.
pub fn run(work_dir: &Path) -> io::Result<()> {
    main(work_dir)
}

/// Tests archive decompression, including missing and malformed input.
#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;

    use flate2::Compression;
    use flate2::write::GzEncoder;
    use tar::Builder;
    use tar::Header;

    use super::*;

    /// Extracts archive from given path.
    #[test]
    fn extracts_archive_from_given_path() -> io::Result<()> {
        let temporary = tempfile::tempdir()?;
        let work_dir = temporary.path();
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
        builder.append_data(&mut header, "source/message.txt", expected.as_slice())?;
        drop(builder.into_inner()?.finish()?);

        decompress_archive(&archive_path, &destination)?;
        let extracted = fs::read(destination.join("source/message.txt"))?;
        assert_eq!(extracted, expected);

        Ok(())
    }

    /// Runs the example through its entry point without error.
    #[test]
    fn test_main_entry_point() -> io::Result<()> {
        let temporary = tempfile::tempdir()?;
        let work_dir = temporary.path().join("sample");
        crate::tar_compress::run(&work_dir)?;
        main(&work_dir)?;
        let marker = work_dir.join("unpacked/keep.txt");
        fs::write(&marker, "user data")?;
        assert_eq!(
            main(&work_dir)
                .expect_err("destination already exists")
                .kind(),
            io::ErrorKind::AlreadyExists
        );
        assert_eq!(fs::read_to_string(marker)?, "user data");
        let unpacked = work_dir.join("unpacked/backup/logs/example.txt");
        assert_eq!(fs::read(unpacked)?, b"A file in the sample archive.\n");
        Ok(())
    }

    /// Confirms a missing archive returns `NotFound` without creating output.
    #[test]
    fn missing_archive_is_an_error() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let err = decompress_archive(&dir.path().join("missing.tar.gz"), &dir.path().join("out"))
            .expect_err("the archive does not exist");
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
        Ok(())
    }
}

// [review; tar_decompress.rs is noplayground - fix?](https://github.com/john-cd/rust_howto/issues/254)
