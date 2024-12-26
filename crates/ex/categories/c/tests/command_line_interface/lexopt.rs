// // ANCHOR: example
// use lexopt::prelude::*;

// fn main() -> anyhow::Result<()> {
//     let mut parser = lexopt::Parser::from_env();
//     let mut name = String::new();
//     let mut age = 0;

//     while let Some(arg) = parser.next().unwrap() {
//         match arg {
//             lexopt::Arg::Long("name") => {
//                 name = parser.value()?.to_string_lossy().to_string();
//             }
//             lexopt::Arg::Long("age") => {
//                 age = parser.value()?.parse().expect("Invalid age");
//             }
//             lexopt::Arg::Value(val) => {
//                 eprintln!("Unexpected argument: {:?}", val);
//             }
//             _ => {
//                 eprintln!("Unexpected flag or option");
//             }
//         }
//     }

//     println!("Name: {}", name);
//     println!("Age: {}", age);
//     Ok(())
// }

// #[test]
// fn test() {
//     // Simulate command-line arguments
//     unsafe {
//         std::env::set_var("CARGO_BIN_EXE_lexopt_example", "lexopt_example");
//         std::env::set_var(
//             "CARGO_BIN_EXE_lexopt_example",
//             "--name John --age 30",
//         );
//     }
//     main().unwrap();
// }
// // ANCHOR_END: example
// // TODO finish
// // [P1](https://github.com/john-cd/rust_howto/issues/678)
