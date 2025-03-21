// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use flatbuffers::FlatBufferBuilder;

// // The `flatbuffers` crate provides runtime support
// // for the FlatBuffers format.

// // 'FlatBuffers' is a cross platform serialization library
// // architected for maximum memory efficiency.
// // It allows you to directly access serialized data
// // without parsing/unpacking it first, while
// // still having great forwards/backwards compatibility.
// // It was originally created at Google for game development
// // and other performance-critical applications.

// // Make sure to run flatc to generate the Rust code from the `.fbs` file:
// // flatc --rust -o . my_game.fbs

// // https://flatbuffers.dev/

// // Define the schema for our data
// flatbuffers_gen::r#gen::include_file!("my_game.fbs"); // Assuming your schema
// is in my_game.fbs

// fn main() {
//     // Create a FlatBufferBuilder
//     let mut builder = FlatBufferBuilder::new();

//     // Create some data to store
//     let name = builder.create_string("MyMonster");
//     let hp = 100;

//     // Create an object of our Monster struct
//     let monster = Monster::create(
//         &mut builder,
//         &MonsterArgs {
//             name: Some(name),
//             hp,
//             ..Default::default()
//         }
//     );

//     // Finish building the buffer
//     builder.finish(monster, None);

//     // Get the finished buffer as a byte slice
//     let finished_bytes = builder.finished_data();

//     // Now you can:
//     // 1. Serialize the finished_bytes to a file or send it over the network
//     // 2. Deserialize the finished_bytes to access the data in another
// program

//     println!("FlatBuffer size: {}", finished_bytes.len());
// }

// #[test]
// fn test() {
//     main();
// }
// [finish; https://flatbuffers.dev/languages/rust/](https://github.com/john-cd/rust_howto/issues/1043)
