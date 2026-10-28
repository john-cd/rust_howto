// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `prost` crate to serialize and
//! deserialize a protobuf message.
//!
//! It defines a simple `Person` message, serializes it to a byte buffer,
//! and then deserializes it back into a `Person` struct.
//!
//! The crate's build script compiles `person.proto` with `protox` and
//! `prost-build`, so building this example does not require an external
//! `protoc` executable.

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

/// # Errors
///
/// Returns an error if encoding or decoding the message fails.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the generated Protobuf type preserves its fields.
    #[test]
    fn prost_round_trip_preserves_the_person_message() -> anyhow::Result<()> {
        main()
    }

    /// Pins the wire format: field 1 (a string) then field 2 (a varint).
    #[test]
    fn encoding_matches_the_protobuf_wire_format() {
        let person = Person {
            name: "Alice".to_owned(),
            age: 30,
        };
        assert_eq!(
            person.encode_to_vec(),
            [0x0a, 5, b'A', b'l', b'i', b'c', b'e', 0x10, 30]
        );
    }

    /// Proto3 omits fields that hold their default value.
    #[test]
    fn default_values_are_not_encoded() {
        assert!(Person::default().encode_to_vec().is_empty());
        let decoded =
            Person::decode(&[][..]).expect("an empty buffer is valid");
        assert_eq!(decoded, Person::default());
    }

    #[test]
    fn truncated_input_is_an_error() {
        let bytes = Person {
            name: "Alice".to_owned(),
            age: 30,
        }
        .encode_to_vec();
        assert!(Person::decode(&bytes[..4]).is_err());
    }
}
