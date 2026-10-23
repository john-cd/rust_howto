#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates how to serialize and deserialize a struct to and
//! from YAML using the maintained `serde_norway` crate.
//!
//! First, we define two structs, `Person` and `Address`, and derive `Serialize`
//! and `Deserialize` for them. Then, in the `main` function, we create an
//! instance of `Person`, serialize it to a YAML string, write the YAML string
//! to a file, read the YAML string back from the file, and finally deserialize
//! it back into a `Person` struct.
//!
//! To run this example, we need to have the `serde`, `serde_norway`, and
//! `anyhow` crates in your `Cargo.toml` file.

use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;
use serde::Serialize;

/// A person, serialized to and deserialized from YAML.
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    /// The name of the person.
    name: String,
    /// The age of the person.
    age: u8,
    /// The email addresses of the person.
    emails: Vec<String>,
    /// The address of the person.
    address: Address,
    /// The skills of the person.
    skills: Vec<String>,
}

/// A postal address.
#[derive(Debug, Serialize, Deserialize)]
struct Address {
    /// The street.
    street: String,
    /// The city.
    city: String,
    /// The country.
    country: String,
    /// The postal code.
    postal_code: String,
}

/// Serializes a struct to YAML and deserializes it back.
fn main() -> anyhow::Result<()> {
    // Create a struct instance.
    let person = Person {
        name: "John Doe".to_owned(),
        age: 30,
        emails: vec![
            "john.work@example.com".to_owned(),
            "john.personal@example.com".to_owned(),
        ],
        address: Address {
            street: "123 Main St".to_owned(),
            city: "Anytown".to_owned(),
            country: "USA".to_owned(),
            postal_code: "12345".to_owned(),
        },
        skills: vec![
            "Rust".to_owned(),
            "Python".to_owned(),
            "DevOps".to_owned(),
        ],
    };

    // Serialize to YAML:
    let yaml = serde_norway::to_string(&person)?;
    println!("Serialized YAML:\n{yaml}");

    // Write to file:
    let mut file = File::create("temp/person.yaml")?;
    file.write_all(yaml.as_bytes())?;

    // Read from file:
    let mut file = File::open("temp/person.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize from YAML:
    let deserialized: Person = serde_norway::from_str(&contents)?;
    println!("\nDeserialized person: {deserialized:?}");

    Ok(())
}

// ANCHOR_END: example

/// Runs the example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the serde yml example's main path succeeds.
    #[test]
    fn test_main() -> anyhow::Result<()> {
        use std::fs;
        if !fs::exists("temp")? {
            fs::create_dir("temp")?;
        }
        main()?;
        Ok(())
    }
}
// TODO review
