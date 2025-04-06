// ANCHOR: example
#![cfg(target_family = "unix")]
//! Simple Rust example using the `nix` crate to handle Unix-like operating
//! system tasks: create a file, write to it, read from it, and delete it.

// Raw file descriptor:
use std::os::fd::RawFd;

// Configuration options for opened files:
use nix::fcntl::OFlag;
// Open or create a file for reading, writing or executing:
use nix::fcntl::open;
// File mode / permissions flags:
use nix::sys::stat::Mode;
// Safe wrappers around functions found in libc “unistd.h” header:
// Close a raw file descriptor:
use nix::unistd::close;
// Read from a raw file descriptor:
use nix::unistd::read;
// Remove a directory entry:
use nix::unistd::unlink;
// Write to a raw file descriptor:
use nix::unistd::write;

fn main() -> nix::Result<()> {
    let path = "example.txt";
    let data = b"Hello, nix!";
    {
        // Create and open a file.
        let rfd: RawFd = open(
            path,
            // Create the file if it does not exist. Only allow writing:
            OFlag::O_CREAT | OFlag::O_WRONLY,
            // Read/write permissions for owner:
            Mode::S_IRUSR | Mode::S_IWUSR,
        )?;

        // Write to the file:
        unsafe {
            write(std::os::fd::BorrowedFd::borrow_raw(rfd), data)?;
        }

        // Close the file.
        close(rfd)?;
    }
    {
        // Open the file for reading:
        let rfd2: RawFd = open(path, OFlag::O_RDONLY, Mode::empty())?;

        // Read from the file:
        let mut buffer: Vec<u8> = vec![0; data.len()];
        read(rfd2, &mut buffer)?;

        // Print the content of the file:
        println!("{}", String::from_utf8_lossy(&buffer));

        // Close the file.
        close(rfd2)?;
    }
    // Delete the file.
    unlink(path)?;

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> nix::Result<()> {
    main()?;
    Ok(())
}
