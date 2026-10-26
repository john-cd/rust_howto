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

use anyhow::bail;

/// Runs `journalctl` and prints its output line by line as it is produced.
fn main() -> anyhow::Result<()> {
    // NOTE: `systemd` should be installed for this example to work.
    let mut child = Command::new("journalctl")
        .stdout(Stdio::piped())
        // Spawn the command and capture its standard output.
        .spawn()?;
    let stdout = child
        .stdout
        .take()
        // Ensure that standard output was captured successfully.
        .ok_or_else(|| Error::other("Could not capture standard output."))?;

    // Create a buffered reader for efficient line-by-line reading.
    let reader = BufReader::new(stdout);

    // Read each line and propagate I/O errors rather than dropping them.
    for line in reader.lines() {
        let line = line?;
        if line.contains("usb") {
            println!("{line}");
        }
    }

    let status = child.wait()?;
    if !status.success() {
        bail!("`journalctl` exited with status {status}.");
    }

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies journal output can be read on Linux systems with systemd.
    #[test]
    #[ignore = "requires access to a Linux systemd journal"]
    fn test_main() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
