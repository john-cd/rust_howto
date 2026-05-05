// ANCHOR: example
//! Demonstrates reading system uptime using `anyhow` for error handling.

use std::fs::File;
use std::io::Read;

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
        .ok_or(anyhow!("Cannot parse uptime data"))? // `ok_or` transforms an `Option` into a `Result`.
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main().unwrap();
    }
}
