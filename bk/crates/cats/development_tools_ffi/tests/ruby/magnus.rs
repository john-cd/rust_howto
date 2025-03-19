// // ANCHOR: example
// use magnus::Error;
// use magnus::RString;
// use magnus::define_global_function;
// use magnus::eval;

// fn hello_rust() -> String {
//     String::from("Hello from Rust again!")
// }

// fn main() -> Result<(), Error> {
//     // Initialize the Ruby VM
//     let ruby = magnus::init()?;

//     // Create a new Ruby string object from the given Rust string
//     let string = RString::new(&ruby, "Hello from Rust!")?;

//     // Evaluate Ruby code
//     ruby.eval(&format!("puts '{}'", string.to_str()?))?;

//     // Define a global Ruby function that calls a Rust function
//     define_global_function("hello_rust", hello_rust);

//     // Evaluate Ruby code that calls the Rust function
//     eval::<()>(
//         r#"
//         puts hello_rust()
//     "#,
//     )?;

//     Ok(())
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [WIP finish](https://github.com/john-cd/rust_howto/issues/1035)
