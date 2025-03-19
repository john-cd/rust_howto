// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use std::io::Cursor;
// // use std::io::Read;
// // use std::io::Write;

// use rmp_serde::Deserializer;
// use rmp_serde::Serializer;
// use serde::Deserialize;
// use serde::Serialize;

// // Provides serialization and deserialization to/from MessagePack.
// // Key features of rmp_serde:
// // Efficient: MessagePack is a compact binary format, leading to smaller
// // serialized data sizes and faster serialization/deserialization. Versatile:
// // Supports a wide range of data types and can be used with various
// input/output // sources. Interoperable: MessagePack is language-agnostic,
// enabling data // exchange between different programming languages.

// #[derive(Serialize, Deserialize, Debug)]
// struct MyData {
//     name: String,
//     age: u32,
//     is_active: bool,
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a sample data object
//     let data = MyData {
//         name: "Alice".to_string(),
//         age: 30,
//         is_active: true,
//     };

//     // Serialize to MessagePack
//     let mut buf = Vec::new();
//     // Creates a serializer that writes to the buffer,
//     // Serializes the data object into MessagePack format and writes it to
// the     // buffer.
//     data.serialize(&mut Serializer::new(&mut buf))?;

//     // Deserialize from MessagePack
//     let mut cursor = Cursor::new(&buf[..]);
//     let deserialized_data: MyData =
//         Deserializer::from_reader(&mut cursor)?.deserialize()?;

//     // Print the deserialized data
//     println!("{:?}", deserialized_data);

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// [WIP finish](https://github.com/john-cd/rust_howto/issues/1046)
