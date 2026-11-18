// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Query extended metadata through Linux's safe statx wrapper.
//!
//! `statx` is the Linux system call that returns file metadata (size, owner,
//! times, and the creation time that older `stat` calls cannot give). It runs
//! on Linux only.

use std::io;
use std::io::Write;

use rustix::fs::AtFlags;
use rustix::fs::StatxFlags;
use rustix::fs::statx;
use tempfile::NamedTempFile;

/// Prints the size and, when available, birth time of a temporary file.
///
/// # Errors
///
/// Propagates temporary-file, write, and statx errors, including unsupported
/// system calls on older kernels or restricted hosts.
fn main() -> io::Result<()> {
    let mut file = NamedTempFile::new()?;
    file.write_all(b"Linux metadata example\n")?;
    // Query the open descriptor, avoiding another lookup of its temporary path.
    let metadata = statx(
        file.as_file(),
        "",
        AtFlags::EMPTY_PATH,
        StatxFlags::SIZE | StatxFlags::BTIME,
    )?;

    // A successful syscall may still omit individual requested fields.
    let available = StatxFlags::from_bits_retain(metadata.stx_mask);
    if available.contains(StatxFlags::SIZE) {
        println!("File size: {} bytes", metadata.stx_size);
    } else {
        println!("File size was not returned.");
    }
    if available.contains(StatxFlags::BTIME) {
        let birth = metadata.stx_btime;
        println!(
            "Birth time: {}.{:09} seconds since Unix epoch",
            birth.tv_sec, birth.tv_nsec
        );
    } else {
        println!("The kernel or filesystem did not report a birth time.");
    }
    // Report cleanup errors instead of leaving removal to the fallible Drop
    // path.
    file.close()?;
    Ok(())
}
// ANCHOR_END: example

/// Runs the Linux-only extended metadata demonstration.
/// Query extended metadata through Linux's safe statx wrapper.
///
/// # Errors
///
/// Propagates file creation, IO, metadata, and explicit cleanup errors.
/// Returns an error if a step of the example fails.
pub fn run() -> io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The native query completes for an owned temporary file.
    #[test]
    fn example_runs_for_the_current_target() {
        main().expect("Linux should allow querying the temporary file with statx");
    }

    /// Returned size metadata matches bytes written through the same open descriptor.
    #[test]
    fn size_mask_and_value_match_written_bytes() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        file.write_all(b"known payload")?;
        let metadata = statx(file.as_file(), "", AtFlags::EMPTY_PATH, StatxFlags::SIZE)?;
        assert!(StatxFlags::from_bits_retain(metadata.stx_mask).contains(StatxFlags::SIZE));
        assert_eq!(metadata.stx_size, 13);
        file.close()
    }

    /// A missing relative entry reports ENOENT rather than empty metadata.
    #[test]
    fn missing_entry_reports_noent() -> io::Result<()> {
        let directory = tempfile::tempdir()?;
        let descriptor = std::fs::File::open(directory.path())?;
        assert!(matches!(
            statx(&descriptor, "missing", AtFlags::empty(), StatxFlags::SIZE),
            Err(rustix::io::Errno::NOENT)
        ));
        drop(descriptor);
        directory.close()
    }
}
