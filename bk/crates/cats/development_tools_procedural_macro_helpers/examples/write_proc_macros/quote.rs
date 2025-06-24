#![allow(dead_code)]
// ANCHOR: example
use quote::quote;
use syn::Ident;
use syn::parse_quote;

/// This example demonstrates the use of the `quote!` macro from the `quote`
/// crate to generate Rust code dynamically. It shows how to create an
/// identifier using `syn::parse_quote!` and then use it within a `quote!`
/// block to generate a `TokenStream` representing a piece of Rust code.
fn main() {
    // Create an identifier.
    let ident: Ident = parse_quote! { my_variable };

    // Use the `quote!` macro to generate a piece of code.
    // Write Rust syntax inside the `quote!` macro to get back a `TokenStream`.
    let tokens = quote! {
        let #ident = 42;
        println!("The value of {} is {}", stringify!(#ident), #ident);
    };

    // Print the generated code (for demonstration purposes).
    println!("{}", tokens);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [review / expand](https://github.com/john-cd/rust_howto/issues/742)
