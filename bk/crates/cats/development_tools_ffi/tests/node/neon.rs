// // ANCHOR: example
// //! This example demonstrates how to create a simple Neon module that exports
// a function to JavaScript. //!
// //! `neon` is a library for embedding Rust code in 'Node.js' applications.
// //! It is slower than `napi`, but also widely used and well-maintained.
// //!
// //! # Steps
// //! - Install the `neon` crate first: `cargo install neon`
// //! - Create a new Neon project: `npm init neon my-neon-example``
// //! - Build the native module: `cargo build`
// //! - Run the JavaScript file: `node index.js`

// use neon::prelude::*;

// /// A simple function that returns a JavaScript string.
// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     // Create a JavaScript string
//     Ok(cx.string("hello from neon!"))
// }

// // Entry point of the Neon module.
// #[neon::main]
// fn main(mut cx: ModuleContext) -> NeonResult<()> {
//     // Expose the hello function to JavaScript
//     cx.export_function("hello", hello)?;
//     Ok(())
// }
// // ANCHOR_END: example

// // [finish how to test](https://github.com/john-cd/rust_howto/issues/1033)?
