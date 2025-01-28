// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// // Rhai is a lightweight scripting language.
// // It allows to embed scripting capabilities in Rust applications.

// use rhai::Engine;
// use rhai::RegisterFn;

// fn add(x: i64, y: i64) -> i64 {
//     x + y
// }

// fn main() {
//     // Create a new `Rhai` scripting engine
//     let mut engine = Engine::new();

//     // Register a Rust function to be used in the script
//     engine.register_fn("add", add);

//     // Define a simple script
//     let script = r#"
//         let a = 42;
//         let b = 58;
//         let result = add(a, b);
//         result
//     "#;

//     // Evaluate the script
//     match engine.eval::<i64>(script) {
//         Ok(result) => println!("The result of the script is: {}", result),
//         Err(e) => println!("Error evaluating the script: {:?}", e),
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/887)
