#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to use the `prost` crate to serialize and
//! deserialize a protobuf message.
//!
//! It defines a simple `Person` message, serializes it to a byte buffer,
//! and then deserializes it back into a `Person` struct.

use person::Person;
use prost::Message;

/// The `Person` message, generated from the `.proto` file.
mod person {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

/// Encodes a `Person` message with `prost`, then decodes it.
///
/// # Errors
///
/// Returns an error if encoding or decoding the message fails.
fn main() -> anyhow::Result<()> {
    // Create a `Person` message.
    let person = Person {
        name: "Alice".to_owned(),
        age: 30,
    };

    // Serialize the message.
    let mut buf = Vec::new();
    person.encode(&mut buf)?;

    // Deserialize the message.
    let decoded_person = Person::decode(buf.as_slice())?;
    assert_eq!(person, decoded_person);

    println!("Name: {}, Age: {}", decoded_person.name, decoded_person.age);
    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
///
/// # Errors
///
/// Returns an error if encoding or decoding the message fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prost_round_trip_preserves_the_person_message() -> anyhow::Result<()> {
        main()
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/1044)
