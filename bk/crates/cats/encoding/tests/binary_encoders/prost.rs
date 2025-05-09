// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates how to use the `prost` crate to serialize and
// //! deserialize a protobuf message.
// //!
// //! It defines a simple `Person` message, serializes it to a byte buffer,
// //! and then deserializes it back into a `Person` struct.
// use person::Person;
// use prost::Message;

// mod person {
//     include!(concat!(env!("OUT_DIR"), "/tests/binary_encoders/person.rs"));
// }

// fn main() {
//     // Create a `Person` message.
//     let person = Person {
//         name: "Alice".to_string(),
//         age: 30,
//     };

//     // Serialize the message.
//     let mut buf = Vec::new();
//     person.encode(&mut buf).unwrap();

//     // Deserialize the message.
//     let decoded_person = Person::decode(&*buf).unwrap();

//     println!("Name: {}, Age: {}", decoded_person.name, decoded_person.age);
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/1044)
