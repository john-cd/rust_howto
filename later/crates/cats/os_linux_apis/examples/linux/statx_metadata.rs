// TODO Reviewed ??-??-2026.

//! Query extended metadata through Linux's safe statx wrapper.
//!
//! `statx` is the Linux system call that returns file metadata (size, owner,
//! times, and the creation time that older `stat` calls cannot give). It runs
//! on Linux only.

// ANCHOR: example
use std::io;
use std::io::Write;

use rustix::fs::AtFlags;
use rustix::fs::StatxFlags;
use rustix::fs::statx;
use tempfile::NamedTempFile;

/// Prints the size and, when available, birth time of a temporary file.
///
/// # Errors
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
///
/// # Errors
/// Propagates file creation, IO, metadata, and explicit cleanup errors.
pub fn run() -> io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example runs on Linux and has a harmless fallback
    /// elsewhere.
    #[test]
    fn example_runs_for_the_current_target() {
        main().expect(
            "Linux should allow querying the temporary file with statx",
        );
    }
}
