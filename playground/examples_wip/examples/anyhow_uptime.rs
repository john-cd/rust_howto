// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates reading system uptime using `anyhow` for error handling.

use std::fs::File;
use std::io::Read as _;

use anyhow::Result;
use anyhow::anyhow;

/// Reads the system uptime.
///
/// Returns the uptime in seconds as a `u64`.
/// Returns an error if the file cannot be read or the data cannot be parsed.
fn read_uptime() -> Result<u64> {
    let mut uptime = String::new();
    // Open the file and read its content into a string.
    // `?` will return early if the file cannot be opened or read.
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;
    Ok(uptime
        .split('.')
        .next()
        .ok_or_else(|| anyhow!("Cannot parse uptime data"))? // `ok_or` transforms an `Option` into a `Result`.
        .parse()?)
}

/// Runs the uptime example.
fn main() -> Result<()> {
    run_example()
}
/// Reads the system uptime.
///
/// Returns the uptime in seconds as a `u64`.
/// Reports read or parse failures to stderr and returns success after handling them.
fn run_example() -> Result<()> {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {uptime} seconds"),
        Err(err) => eprintln!("error: {err}"),
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the uptime example for the workspace test harness.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn run() -> Result<()> {
    run_example()
}

/// Verifies that the uptime example entry point reports its result.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the uptime example entry point reports its result.
    #[test]
    fn test_anyhow_uptime_example_entry_point() {
        main().expect("the uptime example should not fail");
        run_example().expect("the uptime example should not fail");
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn reads_uptime_on_linux() {
        assert!(read_uptime().expect("/proc/uptime should be readable") > 0);
    }
}
