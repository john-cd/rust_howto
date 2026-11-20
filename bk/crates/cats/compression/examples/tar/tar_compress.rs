#![allow(clippy::incompatible_msrv)]
// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This function creates a gzip-compressed tar archive from a directory.
//!
//! A tar archive bundles many files into one, keeping paths and permissions;
//! gzip then compresses it, giving `.tar.gz`. Build the archive while streaming
//! to the output so that large directories need little memory.

use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

use flate2::Compression;
use flate2::write::GzEncoder;

/// Creates a sample input directory and archive in a new workspace.
///
/// # Errors
///
/// Returns an I/O error if the sample directory or archive cannot be written.
fn main(work_dir: &Path) -> io::Result<()> {
    fs::create_dir(work_dir)?;
    let source_dir = work_dir.join("source");
    fs::create_dir(&source_dir)?;
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
fn compress_directory(source_dir: &Path, archive_path: &Path) -> io::Result<()> {
    let archive_file = File::create_new(archive_path)?;
    let encoder = GzEncoder::new(archive_file, Compression::default());
    let mut builder = tar::Builder::new(encoder);
    builder.append_dir_all("backup/logs", source_dir)?;
    drop(builder.into_inner()?.finish()?);
    println!("Archive created at {}", archive_path.display());
    Ok(())
}

// ANCHOR_END: example

/// Creates a fresh workspace and writes the sample archive.
///
/// # Errors
///
/// Returns an I/O error if the workspace already exists or a write fails.
pub fn run(work_dir: &Path) -> io::Result<()> {
    main(work_dir)
}

/// Tests archive creation and workspace isolation.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io;

    use flate2::read::GzDecoder;
    use tar::Archive;

    use super::*;

    /// Archives a temporary directory and verifies the finished gzip tarball.
    #[test]
    fn archives_directory_and_finishes_gzip_stream() -> io::Result<()> {
        let temporary = tempfile::tempdir()?;
        let work_dir = temporary.path();
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
        let archived_contents = fs::read(destination.join("backup/logs/message.txt"))?;
        assert_eq!(archived_contents, expected);

        Ok(())
    }

    /// Independent workspaces archive only their own files and reject reuse.
    #[test]
    fn sample_runs_use_fresh_workspaces() -> io::Result<()> {
        let temporary = tempfile::tempdir()?;
        let first = temporary.path().join("first");
        let second = temporary.path().join("second");
        main(&first)?;
        fs::write(first.join("source/stale.txt"), "previous run")?;
        let original = fs::read(first.join("archive.tar.gz"))?;
        assert_eq!(
            main(&first).expect_err("workspace already exists").kind(),
            io::ErrorKind::AlreadyExists
        );
        main(&second)?;

        let archive_file = File::open(second.join("archive.tar.gz"))?;
        let destination = second.join("output");
        Archive::new(GzDecoder::new(archive_file)).unpack(&destination)?;
        assert!(destination.join("backup/logs/example.txt").is_file());
        assert!(!destination.join("backup/logs/stale.txt").exists());
        assert_eq!(fs::read(first.join("archive.tar.gz"))?, original);
        Ok(())
    }
}
