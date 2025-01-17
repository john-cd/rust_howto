// ANCHOR: example
use der::Decode;
use der::Encode;
use der::Sequence;

// This example demonstrates how to encode a struct to DER format, and decode it
// back using the `der` crate.

// Define a struct to be encoded/decoded
// Note that we derive the `Sequence` trait.
#[derive(Debug, PartialEq, Sequence)]
struct MyStruct {
    field1: u32,
    field2: String,
}
fn main() {
    // Create an instance of MyStruct
    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello, DER!".to_string(),
    };

    // Encode the struct to DER
    let der_encoded = my_struct.to_der().expect("Failed to encode");
    println!("DER Encoded: {:?}", der_encoded);

    // Decode the DER back to MyStruct
    let decoded_struct =
        MyStruct::from_der(&der_encoded).expect("Failed to decode");
    println!("Decoded Struct: {:?}", decoded_struct);

    // Verify that the decoded struct matches the original
    assert_eq!(my_struct, decoded_struct);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
