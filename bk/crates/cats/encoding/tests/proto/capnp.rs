// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This example demonstrates reading and writing Cap'n Proto
// //! messages. It is intended to be used in conjunction with code
// //! generated by the `capnpc-rust` crate.
// //!
// //! Add the following dependencies to your `Cargo.toml`:
// //! ```toml
// //! [dependencies]
// //! capnp = "0.20.3" # Or latest version
// //! [build-dependencies]
// //! capnpc = "0.20.1
// //! ```
// //!
// //! Make sure the following lines are in your `build.rs` file:
// //! ```rust
// //! fn main() {
// //!     capnpc::CompilerCommand::new()
// //!         //.src_prefix("schema")
// //!         .file("tests/proto/foo.capnp")
// //!         .run()
// //!         .expect("schema compiler command");
// //! }
// //! ```
// //!
// //! Or compile the `.capnp` file manually using the `capnp` compiler:
// //! ```sh
// //! capnp compile -o . foo.capnp
// //! ```
// use capnp::serialize;
// use foo_capnp::my_struct::Builder;

// // Define the schema for our data
// mod foo_capnp {
//     include!(concat!(env!("OUT_DIR"), "/tests/proto/foo_capnp.rs"));
//     // Assuming your schema is in `foo_capnp.capnp`.
// }

// fn main() {
//     // Create a message builder.
//     let mut message = Builder::new_default();

//     // Create a root object of our `MyStruct` type.
//     let mut root = message.init_root::<foo_capnp::my_struct::MyStruct>();

//     // Sets the value of the name field.
//     root.set_name("Hello from Rust");
//     // Sets the value of the value field.
//     root.set_value(42);

//     // Serialize the message.
//     let mut write_buffer = Vec::new();
//     serialize::write_message(&mut write_buffer, &message).unwrap();

//     // Now you can:
//     // 1. Serialize the write_buffer to a file or send it over the network,
//     // 2. Deserialize the write_buffer to access the data in another program.

//     println!("Cap'n Proto message size: {}", write_buffer.len());
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish; see https://capnproto.org/ https://capnproto.org/rust.html https://capnproto.org/install.html](https://github.com/john-cd/rust_howto/issues/1041)
