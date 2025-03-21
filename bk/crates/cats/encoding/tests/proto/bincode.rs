// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use std::fs::File;
// use std::io::Cursor;

// use bincode::deserialize_from;
// use bincode::serialize_into;

// #[derive(Serialize, Deserialize, Debug)]
// struct MyData {
//     name: String,
//     age: u32,
//     is_active: bool,
// }

// fn main() -> anyhow::Result<()> {
//     // Create a sample data object
//     let data = MyData {
//         name: "Alice".to_string(),
//         age: 30,
//         is_active: true,
//     };

//     // Serialize to a file
//     let mut file = File::create("data.bin")?;
//     serialize_into(&mut file, &data)?;

//     // Deserialize from a file
//     let mut file = File::open("data.bin")?;
//     let deserialized_data: MyData = deserialize_from(&mut file)?;

//     // Serialize to a buffer
//     let mut buf = Vec::new();
//     serialize_into(&mut buf, &data)?;

//     // Deserialize from a buffer
//     let mut cursor = Cursor::new(&buf[..]);
//     let deserialized_data_from_buf: MyData = deserialize_from(&mut cursor)?;

//     // Print the deserialized data
//     println!("{:?}", deserialized_data);
//     println!("{:?}", deserialized_data_from_buf);

//     Ok(())
// }

// #[test]
// fn test() -> anyhow::Result<()> {
//     main()?;
//     Ok(())
// }
// [finish](https://github.com/john-cd/rust_howto/issues/1040)
