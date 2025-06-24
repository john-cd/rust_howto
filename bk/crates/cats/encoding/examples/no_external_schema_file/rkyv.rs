#![allow(dead_code)]
// ANCHOR: example
//! `rkyv` (short for "archive") is a high-performance, zero-copy
//! (de)serialization framework that allows arbitrary field types
//! and safe zero-copy mutation.
//!
//! This example demonstrates how to serialize and deserialize a struct
//! using `rkyv` without an external schema file.
use rkyv::Archive;
use rkyv::Deserialize;
use rkyv::Serialize;
use rkyv::rancor::Error;

#[derive(Archive, Deserialize, Serialize, Debug)]
struct MyStruct {
    field1: String,
    field2: i32,
}

fn main() -> anyhow::Result<()> {
    // Create an instance of `MyStruct`.
    let data = MyStruct {
        field1: "Hello, world!".to_string(),
        field2: 42,
    };

    // Serialize the data to a byte array.
    let buffer = rkyv::to_bytes::<Error>(&data)?;

    // Print the serialized data.
    println!("Serialized data: {:?}", buffer);

    // Deserialize the data from the byte array.
    let deserialized_data = rkyv::from_bytes::<MyStruct, Error>(&buffer[..])?;

    println!("{:?}", deserialized_data);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
