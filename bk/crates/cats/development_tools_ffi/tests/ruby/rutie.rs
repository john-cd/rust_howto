// // ANCHOR: example
// use rutie::*;

// // Compile the Rust code: `cargo build --release`
// // Run the Ruby script using a Ruby interpreter: `ruby script.rb`

// // Mark the RustModule function as a Ruby module.
// #[module]
// fn RustModule(_cls: Class) -> anyhow::Result<()> {
//     // Create a new Ruby class
//     Class::new("RustClass", None).define(|cls| {
//         // Define a class method
//         cls.def("hello", hello)?;
//         Ok(())
//     })?;

//     Ok(())
// }

// fn hello(_slf: Object) -> anyhow::Result<RString> {
//     // Create a new Ruby string
//     Ok(RString::new_utf8("Hello from Rust!"))
// }
// // ANCHOR_END: example

// [finish](https://github.com/john-cd/rust_howto/issues/1036)
