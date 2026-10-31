// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generates Rust source with `quote` and formats it with `prettyplease`.
//!
//! Code generators build Rust code as token streams with `quote` and then
//! format it with `prettyplease`, so the output is readable. Generated bindings
//! and macros use this approach.

use quote::format_ident;
use quote::quote;

fn main() -> syn::Result<()> {
    // Splice values into the generated code with `#name`.
    let name = format_ident!("add_one");
    let tokens = quote! {
        pub fn #name(x: i32) -> i32 { x + 1 }
    };

    // A `TokenStream` prints on one line. Parse it into a `syn::File`, then
    // unparse with `prettyplease` for rustfmt-like output.
    let file: syn::File = syn::parse2(tokens)?;
    let text = prettyplease::unparse(&file);
    print!("{text}");

    assert_eq!(text, "pub fn add_one(x: i32) -> i32 {\n    x + 1\n}\n");
    Ok(())
}
// ANCHOR_END: example

pub fn run() -> syn::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the quote prettyplease example through its entry point.
    #[test]
    fn test_quote_prettyplease_example_entry_point() -> syn::Result<()> {
        main()
    }
}
