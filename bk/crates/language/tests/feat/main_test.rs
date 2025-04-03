// ANCHOR: example
#![allow(dead_code)]
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use anyhow::anyhow;

// Let's first define a function that returns a `Result`:

/// Reads the system uptime.
///
/// Returns the uptime in seconds as a `u64`.
/// Returns an error if the file cannot be read or the data cannot be parsed.
fn read_uptime() -> Result<u64> {
    // Open the file.
    // `?` will return early if the file cannot be opened.
    // Read the file content into a string.
    // `?` will return early if the file cannot be read.
    let mut uptime = String::new();
    File::open("/proc/uptime")?.read_to_string(&mut uptime)?;
    Ok(uptime
        .split('.')
        .next()
        .ok_or(anyhow!("Cannot parse uptime data"))? // `ok_or` transforms an `Option` into a `Result`.
        .parse()?)
}

// The first method to handle
// match read_uptime() {
//     Ok(uptime) => println!("uptime: {} seconds", uptime),
//     Err(err) => eprintln!("error: {}", err),
// };

/// The main function of the program.
///
/// This function is the entry point of the program.
/// It does not do anything in this example.
fn main() {}
// ANCHOR_END: example

#[test]
fn test_main() {
    main();
}
