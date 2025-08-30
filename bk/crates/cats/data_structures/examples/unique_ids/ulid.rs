#![allow(dead_code)]
// ANCHOR: example
use std::str::FromStr;

use ulid::Ulid;

fn main() {
    // Generate a new ULID.
    // It automatically captures the current system time for the timestamp
    // portion and fills the rest with cryptographically secure random data:
    let new_ulid = Ulid::new();
    println!("Generated ULID: {}", new_ulid);

    // You can also get its components:
    println!("Timestamp: {}", new_ulid.timestamp_ms());
    println!("Randomness: {:x}", new_ulid.random()); // Print as hexadecimal.

    println!("--------------------");

    // Parse a ULID from a string:
    let ulid_str = "01K3W709C2RK498J5C5XBTXP1Y";
    match Ulid::from_str(ulid_str) {
        // Or use `parse()`.
        Ok(parsed_ulid) => {
            println!("Successfully parsed ULID: {}", parsed_ulid);
            println!("Parsed timestamp: {:?} ms", parsed_ulid.datetime());
        }
        Err(e) => {
            println!("Failed to parse ULID: {}", e);
        }
    }
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
