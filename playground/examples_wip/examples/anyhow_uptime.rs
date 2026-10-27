// TODO Reviewed ??-??-2026

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

/// Reads the system uptime.
///
/// Returns the uptime in seconds as a `u64`.
/// Returns an error if the file cannot be read or the data cannot be parsed.
fn main() -> Result<()> {
    match read_uptime() {
        Ok(uptime) => println!("uptime: {uptime} seconds"),
        Err(err) => eprintln!("error: {err}"),
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the uptime example for the workspace test harness.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anyhow_uptime_example_entry_point() {
        main().expect("the uptime example should read its source data");
    }
}
