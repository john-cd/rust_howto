// // ANCHOR: example
// // Define a simple function to add two numbers
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(kani)]
// mod verification {
//     use kani::verifier;

//     use super::*;

//     // Define a verification function to verify the add function
//     fn verify_add() {
//         // Check that add function works for some example inputs
//         let result = add(2, 3);
//         assert!(result == 5);
//     }

//     verifier::verify!(verify_add);
// }

// fn main() {
//     println!("2 + 3 = {}", add(2, 3));
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/731)
