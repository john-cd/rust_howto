#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Serialize a Rust value to YAML and deserialize it with `serde-saphyr`.

use serde::Deserialize;
use serde::Serialize;

/// A person represented in a YAML document.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Person {
    /// The person's name.
    name: String,
    /// The person's age.
    age: u8,
    /// The person's email addresses.
    emails: Vec<String>,
}

/// Builds a sample person for the example and its test.
fn sample_person() -> Person {
    Person {
        name: "Ada Lovelace".to_owned(),
        age: 36,
        emails: vec!["ada@example.com".to_owned()],
    }
}

/// Serializes a person to YAML and parses the document back into a person.
fn yaml_round_trip(person: &Person) -> anyhow::Result<(String, Person)> {
    let yaml = serde_saphyr::to_string(person)?;
    let parsed = serde_saphyr::from_str(&yaml)?;
    Ok((yaml, parsed))
}

/// Prints a YAML document and the value parsed back from it.
fn main() -> anyhow::Result<()> {
    let person = sample_person();
    let (yaml, parsed) = yaml_round_trip(&person)?;
    println!("Serialized YAML:\n{yaml}");
    println!("Parsed person: {parsed:?}");
    anyhow::ensure!(person == parsed, "YAML round-trip changed the person");
    Ok(())
}

// ANCHOR_END: example

/// Runs the YAML serialization example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::sample_person;
    use super::yaml_round_trip;

    /// Verifies that serializing and parsing YAML preserves a typed value.
    #[test]
    fn yaml_round_trip_preserves_person() -> anyhow::Result<()> {
        let person = sample_person();
        let (yaml, parsed) = yaml_round_trip(&person)?;

        assert!(yaml.contains("name: Ada Lovelace"));
        assert_eq!(parsed, person);
        Ok(())
    }
}
