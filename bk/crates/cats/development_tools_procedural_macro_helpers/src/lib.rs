// // ANCHOR: derive_macro
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::DeriveInput;
// use syn::parse_macro_input;

// #[proc_macro_derive(MyMacro)]
// pub fn my_macro(input: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let input = parse_macro_input!(input as DeriveInput);

//     // Build the output, possibly using quasi-quotation
//     let expanded = quote! {
//         // ...
//     };

//     // Hand the output tokens back to the compiler
//     TokenStream::from(expanded)
// }
// // ANCHOR_END: derive_macro

// TODO P1 write; review https://docs.rs/syn/latest/syn/index.html

// // ANCHOR: attribute_macro
// extern crate proc_macro;

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, ItemFn};

// #[proc_macro_attribute]
// pub fn log_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemFn);
//     let name = &input.sig.ident;

//     let gen = quote! {
//         fn #name() {
//             println!("Function {} is called", stringify!(#name));
//             #input
//         }
//     };

//     gen.into()
// }
// // ANCHOR_END: attribute_macro

// TODO P1 write
