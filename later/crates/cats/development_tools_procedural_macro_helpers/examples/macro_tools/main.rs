// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Snapshot-test the code that a derive helper generates.
//!
//! `cargo expand` shows the code produced by a macro, and `macrotest` compares
//! that expansion with a checked-in `*.expanded.rs` file. The same idea works
//! for the token-generating function behind a procedural macro: format its
//! output with `prettyplease`, then compare it with the expected source.
//! Keeping the generator in a plain function lets an ordinary unit test run
//! it, without a `proc-macro` crate or the compiler's expansion interface.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Generates `impl Named for <type>` returning the type's name.
///
/// # Errors
///
/// Returns an error if `input` is not a valid struct, enum or union.
fn derive_named(input: TokenStream) -> syn::Result<TokenStream> {
    let DeriveInput {
        ident, generics, ..
    } = syn::parse2(input)?;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let name = ident.to_string();
    Ok(quote! {
        impl #impl_generics Named for #ident #ty_generics #where_clause {
            fn name() -> &'static str {
                #name
            }
        }
    })
}

/// Formats generated tokens as readable Rust source, like `cargo expand`.
///
/// # Errors
///
/// Returns an error if the tokens are not a valid Rust file.
fn expand_to_string(tokens: TokenStream) -> Result<String> {
    let file: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&file))
}

/// The snapshot to compare against, like a `*.expanded.rs` file.
const EXPECTED: &str = "\
impl<T: Clone> Named for Wrapper<T> {
    fn name() -> &'static str {
        \"Wrapper\"
    }
}
";

/// Expands a derive input and checks it against the snapshot.
fn main() -> Result<()> {
    let input = quote! {
        struct Wrapper<T: Clone>(T);
    };
    let expanded = expand_to_string(derive_named(input)?)?;
    print!("{expanded}");
    assert_eq!(expanded, EXPECTED);
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_entry_point() {
        main().expect("the example should run successfully");
    }

    /// Invalid input is reported as an error, not a panic.
    #[test]
    fn rejects_invalid_input() {
        assert!(derive_named(quote! { fn not_a_type() {} }).is_err());
    }
}
