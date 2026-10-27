#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parsing from raw bytes.

/// A message, created from raw bytes.
#[derive(Debug)]
struct Message(String);

// Implement `From` to convert from a `u8` slice to our custom struct.
impl From<&[u8]> for Message {
    fn from(bytes: &[u8]) -> Self {
        Self(String::from_utf8_lossy(bytes).into())
    }
}

/// Converts raw bytes into a `Message`.
fn main() {
    let raw: &[u8] = b"Hello world";
    let msg: Message = raw.into(); // Use `From<&[u8]>`.
    println!("{msg:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the from3 example through its entry point.
    #[test]
    fn test_from3_example_entry_point() {
        main();
    }
}
