// ANCHOR: example
use serde::Deserialize;
use serde::Serialize;

// Define a struct `Person`.
// The `Serialize` and `Deserialize` derive macros automatically implement the
// `Serialize` and `Deserialize` traits for `Person`.
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    // Create an instance of `Person`.
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // Serialize the `Person` instance to a JSON string.
    let json_string = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_string);

    // Deserialize the JSON string back into a `Person` instance.
    let deserialized_person: Person =
        serde_json::from_str(&json_string).unwrap();
    println!("Deserialized person: {:?}", deserialized_person);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
