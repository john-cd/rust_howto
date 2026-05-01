#![allow(dead_code)]
// ANCHOR: example
#![cfg(target_family = "unix")]
//! Simple Rust example using the `nix` crate to handle Unix-like operating
//! system tasks: create a file, write to it, read from it, and delete it.

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
use rustix::fd::AsFd;
use rustix::fd::BorrowedFd;

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

#[test]
fn test() -> nix::Result<()> {
    main()?;
    Ok(())
}
