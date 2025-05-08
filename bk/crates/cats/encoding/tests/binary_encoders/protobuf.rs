// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `protobuf` crate to serialize
// //! and deserialize data using Protocol Buffers.
// //!
// //! In your `Cargo.toml`, add the `protobuf` crate as a dependency:
// //! ```toml
// //! [dependencies]
// //! protobuf = "3.7.2" # Or latest
// //! ```
// //!
// //! Compile the .proto file to generate the Rust code.
// //! You can use the `protoc` compiler with the Rust plugin:
// //! ```sh
// //! protoc --rust_out=. person.proto
// //! ```
// //! This will generate a Rust module with the compiled code for the `Person`
// //! message.

// /// Import the generated code for the Person message.
// /// The generated code is in the `person.rs` file.
// mod person;

// use person::Person;
// use protobuf::Message;

// fn main() {
//     // Create an instance of the Person message
//     let mut person = Person::new();
//     person.set_name(String::from("Alice"));
//     person.set_age(30);

//     // Serialize the Person message to a byte array
//     let serialized_person = person.write_to_bytes().unwrap();

//     // Print the serialized data
//     println!("Serialized data: {:?}", serialized_person);

//     // Deserialize the byte array back into a Person message
//     let deserialized_person =
// Person::parse_from_bytes(&serialized_person).unwrap();

//     // Print the deserialized Person message
//     println!("Deserialized person: {:?}", deserialized_person);
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/1045)
