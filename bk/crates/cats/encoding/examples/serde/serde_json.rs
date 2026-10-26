#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Serializing and deserializing a struct to and from JSON with `serde_json`.

use serde::Deserialize;
use serde::Serialize;

// Define a struct `Person`.
// The `Serialize` and `Deserialize` derive macros automatically implement the
// `Serialize` and `Deserialize` traits for `Person`.
/// A person, serialized to and from JSON.
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    /// Name of the person.
    name: String,
    /// Age of the person, in years.
    age: u8,
    /// Email address of the person.
    email: String,
}

/// Serializes a person to JSON and deserializes it back.
fn main() {
    // Create an instance of `Person`.
    let person = Person {
        name: "Alice".to_owned(),
        age: 30,
        email: "alice@example.com".to_owned(),
    };

    // Serialize the `Person` instance to a JSON string.
    let json_string = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {json_string}");

    // Deserialize the JSON string back into a `Person` instance.
    let deserialized_person: Person =
        serde_json::from_str(&json_string).unwrap();
    println!("Deserialized person: {deserialized_person:?}");
}

// ANCHOR_END: example

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
}
