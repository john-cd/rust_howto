// ANCHOR: example
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use serde_json::Value;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// Deserialize a JSON string
fn main() -> Result<()> {
    let json_str = r#"
    {
        "name": "John Doe",
        "age": 42,
        "phones": [
            "+1 123456789",
            "+1 234567890"
        ]
    }
    "#;

    // Use the `Value` enum to represent any valid JSON value.
    let parsed: Value = serde_json::from_str(json_str)?;
    // There is also `from_slice` for parsing from a byte slice `&[u8]``
    // and `from_reader` for parsing from any `io::Read` like a `File` or a TCP
    // stream.

    // Then access parts of the data by indexing with square brackets.
    // The result of square bracket indexing is a borrow, so the type is
    // `&Value`.
    let name = parsed["name"].as_str().unwrap();
    let age = parsed["age"].as_i64().unwrap();
    let phones = parsed["phones"].as_array().unwrap();

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Phones: {:?}", phones);

    // Or use a custom `struct` that implements `Deserialize`
    let user: User = serde_json::from_str(json_str)?;
    println!("{:?}", user);

    // Serialize it back to a JSON string.
    let u = serde_json::to_string(&user)?;
    println!("{u}");
    // There is also `serde_json::to_vec` which serializes to a `Vec<u8>`
    // and `serde_json::to_writer`, which serializes to any `io::Write`,
    // such as a `File` or a TCP stream.

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [P1 add example](https://github.com/john-cd/rust_howto/issues/1105)
