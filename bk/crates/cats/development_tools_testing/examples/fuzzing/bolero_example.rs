#![allow(dead_code)]
// ANCHOR: example
//! `bolero` is a fuzzing frontend for Rust that supports multiple engines
//! like libFuzzer, AFL, and Honggfuzz. It also allows running fuzz targets
//! as regular unit tests.

use bolero::check;

fn process_data(data: &[u8]) {
    if data.len() >= 3
        && data[0] == b'b'
        && data[1] == b'o'
        && data[2] == b'l'
        && data.len() >= 6
        && data[3] == b'e'
        && data[4] == b'r'
        && data[5] == b'o'
    {
        // Potential bug discovered by fuzzer
        // panic!("Bolero found a crash!");
    }
}

fn main() {
    // This will run the fuzzer when executed with `cargo bolero fuzz`
    // or as a regular test with `cargo test`.
    check!().for_each(|data| {
        process_data(data);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bolero() {
        // When running as a test, it uses a default number of iterations.
        main();
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}
