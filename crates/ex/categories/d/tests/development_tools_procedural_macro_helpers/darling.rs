// // ANCHOR: example
// // darling is a crate in Rust that helps you write custom derive macros more
// easily. It provides a way to parse attributes on structs and enums, which can
// be useful when writing procedural macros. use proc_macro::TokenStream;
// use syn::{parse_macro_input, DeriveInput};
// use quote::quote;
// use darling::FromDeriveInput;

// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(my_macro))]
// struct MyMacroOpts {
//     name: String,
// }

// #[proc_macro_derive(MyMacro, attributes(my_macro))]
// pub fn my_macro_derive(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as DeriveInput);

//     let opts = MyMacroOpts::from_derive_input(&input).expect("Wrong
// options!");

//     let name = &opts.name;

//     let expanded = quote! {
//         impl MyMacro {
//             pub fn hello() {
//                 println!("Hello, {}!", #name);
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }
// // ANCHOR_END: example

// #[test]
// #[ignore = "not yet implemented"]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/739)
