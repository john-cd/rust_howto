// ANCHOR: example
#![cfg(target_family = "unix")]

// `rustix` is a library that provides a safe and idiomatic Rust interface to
// low-level system calls.

// Add to your `Cargo.toml`:
// rustix = { version = "0.38.42", features = ["fs"] }

// C-compatible, nul-terminated string (for the file path)
use std::ffi::CStr;

// S_I* constants for use with openat, chmodat, and fchmod.
use rustix::fs::Mode;
// O_* constants for use with openat.
use rustix::fs::OFlags;
// Opens a file.
use rustix::fs::openat;
use rustix::fs::rename;
use rustix::fs::unlink;
use rustix::io::read;
use rustix::io::write;

fn main() -> anyhow::Result<()> {
    let data = b"Hello, rustix!";
    {
        // Open the file for writing
        let fd: rustix::fd::OwnedFd = openat(
            rustix::fs::CWD, // Current working directory
            CStr::from_bytes_with_nul(b"temp/example2.txt\0")?,
            OFlags::CREATE | OFlags::WRONLY | OFlags::TRUNC,
            Mode::RUSR | Mode::WUSR,
        )?;

        // Write to the file
        let bytes_written = write(fd, data)?;
        println!("Wrote {} bytes to the file.", bytes_written);

        // Close the file
    }
    {
        // Open the file for reading
        let fd2 = openat(
            rustix::fs::CWD,
            CStr::from_bytes_with_nul(b"temp/example2.txt\0")?,
            OFlags::RDONLY,
            Mode::empty(),
        )?;

        // Read from the file
        let mut buffer = vec![0; data.len()];
        read(fd2, &mut buffer)?;

        // Print the content of the file
        println!("{}", String::from_utf8_lossy(&buffer));

        // Close the file
    }
    // Rename the file
    let path = "temp/example2.txt";
    let new_file_path = "temp/my_renamed_file";
    rename(path, new_file_path)?;
    println!("File renamed to: {}", new_file_path);

    // Unlink (delete) the file
    unlink(new_file_path)?;
    println!("File deleted: {}", new_file_path);
    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
