// #[allow(dead_code)]
// // ANCHOR: example
// // Declare an external C function.
// unsafe extern "C" {
//     /// This function is defined in C code and prints application
// information.     fn print_app_info();
// }

// fn main() {
//     // Call the external C function.
//     unsafe {
//         print_app_info();
//     }
//     println!("Printed app info.");
// }
// // ANCHOR_END: example

// #[ignore = "Needs review"]
// #[test]
// fn test() {
//     main();
// }
// // [finish; deal with extern](https://github.com/john-cd/rust_howto/issues/901)
