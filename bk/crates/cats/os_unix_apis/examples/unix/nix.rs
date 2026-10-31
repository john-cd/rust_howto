// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Simple Rust example using the `nix` crate to handle Unix-like operating
//! system tasks: create a file, write to it, read from it, and delete it.
//!
//! `nix` provides safe wrappers around Unix system calls (the requests a
//! program makes to the kernel), such as processes, signals and file
//! descriptors. It works on Linux, macOS and the BSDs, but not on Windows.

#![cfg(target_family = "unix")]

// File descriptor:
use std::os::fd::OwnedFd;

// Configuration options for opened files:
use nix::fcntl::OFlag;
// Open or create a file for reading, writing or executing:
use nix::fcntl::open;
// File mode / permissions flags:
use nix::sys::stat::Mode;
// Safe wrappers around functions found in libc "unistd.h" header:
// Close a raw file descriptor:
use nix::unistd::close;
// Read from a raw file descriptor:
use nix::unistd::read;
// Remove a directory entry:
use nix::unistd::unlink;
// Write to a raw file descriptor:
use nix::unistd::write;
use rustix::fd::AsFd as _;
use rustix::fd::BorrowedFd;

/// Creates a file, writes to it and reads it back using `nix`.
fn main() -> nix::Result<()> {
    let path = "example.txt";
    let data = b"Hello, nix!";
    {
        // Create and open a file.
        let rfd: OwnedFd = open(
            path,
            // Create the file if it does not exist. Only allow writing:
            OFlag::O_CREAT | OFlag::O_WRONLY,
            // Read/write permissions for owner:
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;

        let borrowed_fd: BorrowedFd<'_> = rfd.as_fd();
        // Write to the file:
        write(borrowed_fd, data)?;

        // Close the file.
        close(rfd)?;
    }
    {
        // Open the file for reading:
        let fd2: OwnedFd = open(path, OFlag::O_RDONLY, Mode::empty())?;

        // Read from the file:
        let mut buffer: Vec<u8> = vec![0; data.len()];
        read(fd2, &mut buffer)?;

        // Print the content of the file:
        println!("{}", String::from_utf8_lossy(&buffer));
    }
    // Delete the file.
    unlink(path)?;

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> nix::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the nix example through its entry point.
    #[test]
    fn test_nix_example_entry_point() -> nix::Result<()> {
        main()?;
        Ok(())
    }

    /// The data written to the file is read back.
    #[test]
    fn writes_then_reads_a_file() -> nix::Result<()> {
        let path = std::env::temp_dir().join("rust_howto_nix_test.txt");
        let data = b"Hello, nix!";
        let fd = open(
            &path,
            OFlag::O_CREAT | OFlag::O_WRONLY,
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;
        write(fd.as_fd(), data)?;
        close(fd)?;
        let fd = open(&path, OFlag::O_RDONLY, Mode::empty())?;
        let mut buffer = vec![0; data.len()];
        read(fd, &mut buffer)?;
        assert_eq!(buffer, data);
        unlink(&path)?;
        Ok(())
    }

    /// Opening a missing file fails with `ENOENT`.
    #[test]
    fn opening_a_missing_file_fails() {
        let result =
            open("/this/path/does/not/exist", OFlag::O_RDONLY, Mode::empty());
        assert_eq!(result.err(), Some(nix::errno::Errno::ENOENT));
    }
}
