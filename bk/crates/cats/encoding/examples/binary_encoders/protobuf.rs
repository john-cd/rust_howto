#![allow(dead_code)]

// ANCHOR: example
//! Serialize and deserialize a Protocol Buffers message generated with `prost`.
//!
//! TODO Reviewed ??-??-2026

use person::Person;
use prost::Message;

mod person {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let mut encoded = Vec::new();
    person.encode(&mut encoded).unwrap();
    let decoded = Person::decode(encoded.as_slice()).unwrap();
    assert_eq!(person, decoded);

    println!("Encoded {} bytes", encoded.len());
    println!("Decoded person: {decoded:?}");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
    // [finish](https://github.com/john-cd/rust_howto/issues/1045)
}
