// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// // `proc-macro2` is a library for working with Rust's procedural macros.
// // We create below a simple procedural macro that
// // generates code to implement the `Debug` trait for a struct.

// // In `Cargo.toml`
// // [dependencies]
// // proc-macro2 = "1.0"
// // quote = "1.0"
// // syn = { version = "1.0", features = ["full"] }

// // [lib]
// // proc-macro = true

// use proc_macro::TokenStream;
// use proc_macro2::TokenStream as TokenStream2;
// use quote::quote;
// use syn::DeriveInput;
// use syn::parse_macro_input;

// #[proc_macro_derive(Debug)]
// pub fn derive_debug(input: TokenStream) -> TokenStream {
//     // Parse the input TokenStream into a DeriveInput
//     let input = parse_macro_input!(input as DeriveInput);
//     // Generate the implementation
//     let expanded = impl_debug(&input);
//     // Return the generated code as a TokenStream
//     TokenStream::from(expanded)
// }

// fn impl_debug(input: &DeriveInput) -> TokenStream2 {
//     let name = &input.ident;
//     let expanded = quote! {
//         impl std::fmt::Debug for #name {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) ->
// std::fmt::Result {                 write!(f, "{} {{ ... }}",
// stringify!(#name))             }
//         }
//     };
//     expanded
// }

// #[test]
// fn test() {
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/741)
