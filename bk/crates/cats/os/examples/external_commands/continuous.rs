#![cfg(target_os = "linux")]
#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to continuously read from the standard output
//! of an external command, filter the output, and print the filtered lines. It
//! uses the `journalctl` command to read system logs, filters lines containing
//! "usb", and prints them to the console.

use std::io::BufRead as _;
use std::io::BufReader;
use std::io::Error;
use std::process::Command;
use std::process::Stdio;

/// Runs `journalctl` and prints its output line by line as it is produced.
fn main() -> Result<(), Error> {
    // NOTE: `systemd` should be installed for this example to work.
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        // Spawn the command and capture its standard output.
        .spawn()?
        .stdout
        // Ensure that standard output was captured successfully.
        .ok_or_else(|| Error::other("Could not capture standard output."))?;

    // Create a buffered reader for efficient line-by-line reading.
    let reader = BufReader::new(stdout);

    // Iterate over the lines from the reader.
    reader
        .lines()
        // Convert each line to a String, discarding errors.
        .map_while(Result::ok)
        // Filter lines that contain the substring "usb".
        .filter(|line| line.contains("usb"))
        .for_each(|line| println!("{line}"));

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies journal output can be read on Linux systems with systemd.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
