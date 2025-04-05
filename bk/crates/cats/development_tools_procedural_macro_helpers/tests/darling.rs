// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This module demonstrates the usage of the `darling` crate for simplifying
// the creation of custom derive macros. //!
// //! `darling` provides a convenient way to parse attributes on structs and
// enums, which is particularly //! useful when writing procedural macros.
// //!
// //! The example below shows how to define a custom derive macro `MyMacro`
// that extracts a `name` attribute.

// use proc_macro::TokenStream;
// use syn::{parse_macro_input, DeriveInput};
// use quote::quote;
// use darling::FromDeriveInput;

// /// Options for the `MyMacro` derive macro.
// ///
// /// This struct defines the structure of the attributes that `MyMacro`
// expects. #[derive(Debug, FromDeriveInput)]
// /// Specifies that the attributes for this struct are under the `my_macro`
// namespace. #[darling(attributes(my_macro))]
// struct MyMacroOpts {
//     name: String,
//     /// The name of the field to be used.
//     #[darling(default)]
//     field_name: Option<String>,
// }

// #[proc_macro_derive(MyMacro, attributes(my_macro))]
// pub fn my_macro_derive(input: TokenStream) -> TokenStream {
//     // Parse the input TokenStream of a macro, triggering a compile error if
// the tokens fail to parse.     let input = parse_macro_input!(input as
// DeriveInput);

//     // Parse the input and extract the options.
//     let opts = MyMacroOpts::from_derive_input(&input).expect("Wrong
// options!");

//     // Extract the name from the parsed options.
//     let name = &opts.name;
//     let field_name = opts.field_name.unwrap_or_else(||
// "default_field".to_string());

//     // Generate the expanded code.
//     let expanded = quote! {
//         /// This is an example struct that implements the `MyMacro` trait.
//         #[derive(Debug)]
//         struct MyStruct {
//             #field_name: String,
//         }

//         impl MyMacro for MyStruct {
//             pub fn hello() {
//                 println!("Hello, {}!", #name);
//             }
//         }
//     };

//     TokenStream::from(expanded)
// }

// /// A dummy trait for demonstration purposes.
// pub trait MyMacro {}

// /// A test function to demonstrate the usage of the macro.
// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/739)
