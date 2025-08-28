#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This is a simple example of a procedural macro that derives the `Debug`
// //! trait for a struct.
// //!
// //! `proc-macro2` is a library for working with Rust's procedural macros.
// //! We create below a simple procedural macro that
// //! generates code to implement the `Debug` trait for a struct.
// //!
// //! In `Cargo.toml`, add:
// //! ```toml
// //! [dependencies]
// //! proc-macro2 = "1.0" # Or latest version
// //! quote = "1.0"
// //! syn = { version = "1.0", features = ["full"] }
// //!
// //! [lib]
// //! proc-macro = true
// //! ```
// use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
// use quote::quote;
// use syn::DeriveInput;
// use syn::parse_macro_input;

// /// The following defines a procedural macro `derive_debug` that
// automatically /// implements the `Debug` trait for a struct. It uses the
// `proc_macro`, /// `proc_macro2`, `quote`, and `syn` crates to parse the
// input, generate code, /// and return the result.
// #[proc_macro_derive(Debug)]
// pub fn derive_debug(input: TokenStream) -> TokenStream {
//     // Parse the input `TokenStream` into a `DeriveInput`:
//     let input = parse_macro_input!(input as DeriveInput);
//     // Generate the implementation:
//     let expanded = impl_debug(&input);
//     // Return the generated code as a TokenStream:
//     TokenStream::from(expanded)
// }

// /// The `impl_debug` function generates the actual implementation of the
// /// `Debug` trait.
// fn impl_debug(input: &DeriveInput) -> TokenStream2 {
//     let name = &input.ident;
//     let expanded = quote! {
//         impl std::fmt::Debug for #name {
//         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {{ ... }}", stringify!(#name))
//         }
//         }
//     };
//     expanded
// }

// #[test]
// fn test() {}
// // [finish](https://github.com/john-cd/rust_howto/issues/741)
