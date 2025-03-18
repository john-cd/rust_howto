// ANCHOR: example
use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    emails: Vec<String>,
    address: Address,
    skills: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    country: String,
    postal_code: String,
}

fn main() -> anyhow::Result<()> {
    // Create a struct instance
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        emails: vec![
            "john.work@example.com".to_string(),
            "john.personal@example.com".to_string(),
        ],
        address: Address {
            street: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            country: "USA".to_string(),
            postal_code: "12345".to_string(),
        },
        skills: vec![
            "Rust".to_string(),
            "Python".to_string(),
            "DevOps".to_string(),
        ],
    };

    // Serialize to YAML
    let yaml = serde_yml::to_string(&person)?;
    println!("Serialized YAML:\n{}", yaml);

    // Write to file
    let mut file = File::create("temp/person.yaml")?;
    file.write_all(yaml.as_bytes())?;

    // Read from file
    let mut file = File::open("temp/person.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Deserialize from YAML
    let deserialized: Person = serde_yml::from_str(&contents)?;
    println!("\nDeserialized person: {:?}", deserialized);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
