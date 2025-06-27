#![allow(dead_code)]
// ANCHOR: example
//! Demonstrates basic serialization and deserialization using `serde`.
use serde::Deserialize;
use serde::Serialize;

// Define a struct and derive the `Serialize` and `Deserialize` traits.
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    // Rename the fields during serialization and deserialization.
    #[serde(rename = "first_name")]
    first: String,
    #[serde(rename = "last_name")]
    last: String,
    // Skip serializing the `age` field if it is None.
    #[serde(skip_serializing_if = "Option::is_none")]
    age: Option<u8>,
}

fn main() {
    // Create an instance of the struct.
    let person = Person {
        first: String::from("John"),
        last: String::from("Doe"),
        age: Some(30),
    };

    // Serialize the struct to a JSON string.
    let json = serde_json::to_string(&person).unwrap();
    println!("Serialized: {json}");

    // Deserialize the JSON string back to a struct.
    let deserialized_person: Person = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {deserialized_person:?}");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
