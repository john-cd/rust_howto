// // ANCHOR: example
// // ANCHOR_END: example

// // `serde_ignored` is a crate that helps you handle unknown fields when
// // deserializing with `serde`. It's useful when you want to ignore fields
// // that are present in the input but not defined in your struct.

// // In `Cargo.toml`:
// // [dependencies]
// // serde = { version = "1.0", features = ["derive"] }
// // serde_derive = "1.0"
// // serde_ignored = "0.1"
// // serde_json = "1.0"

// use serde::Deserialize;
// use serde_ignored::Path;
// use serde_json::Value;

// #[derive(Deserialize, Debug)]
// struct MyStruct {
//     field1: i32,
//     field2: String,
// }

// fn main() {
//     let json_data = r#"
//     {
//         "field1": 42,
//         "field2": "Hello, world!",
//         "unknown_field": "This will be ignored",
//         "another_unknown": 123
//     }
//     "#;

//     // Create a vector to store ignored fields
//     let mut ignored_fields = Vec::new();

//     // Deserialize the JSON data, collecting ignored fields
//     let result: Result<MyStruct, _> = serde_ignored::deserialize(
//         &mut serde_json::Deserializer::from_str(json_data),
//         |path: Path| {
//             println!("Ignored field: {}", path);
//             ignored_fields.push(path.to_string());
//         },
//     );

//     // Check the result of deserialization
//     match result {
//         Ok(my_struct) => {
//             println!("Deserialized struct: {:?}", my_struct);
//             println!("Ignored fields: {:?}", ignored_fields);
//         }
//         Err(e) => {
//             println!("Error deserializing: {}", e);
//         }
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [P0](https://github.com/john-cd/rust_howto/issues/756)
