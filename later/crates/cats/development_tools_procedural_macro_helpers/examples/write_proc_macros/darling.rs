// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parse derive attributes with a declarative `darling` input model.
//!
//! `FromDeriveInput` reads the `builder` attribute and can turn invalid keys
//! or values into `compile_error!` tokens with `write_errors()`.

use anyhow::Result;
use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

/// Parsed options for a named struct's `builder` attribute.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(builder))]
struct BuilderOptions {
    /// The name of the struct that carried the attribute.
    ident: syn::Ident,
    /// Optional prefix used to name generated builder methods.
    #[darling(default)]
    prefix: Option<String>,
}

/// Parses builder options from derive input.
///
/// # Errors
///
/// Returns a Darling error for malformed input or unknown attributes.
fn parse_options(input: TokenStream) -> darling::Result<BuilderOptions> {
    let input: DeriveInput =
        syn::parse2(input).map_err(darling::Error::from)?;
    BuilderOptions::from_derive_input(&input)
}

/// Prints parsed options and a compiler-style diagnostic for an invalid key.
///
/// # Errors
///
/// Returns an error if the valid example input cannot be parsed.
fn main() -> Result<()> {
    let valid = quote! { #[builder(prefix = "with_")] struct Foo; };
    let options = parse_options(valid)
        .map_err(|error| anyhow::anyhow!(error.write_errors().to_string()))?;
    println!("{} uses prefix {:?}", options.ident, options.prefix);

    let invalid = quote! { #[builder(unknown = "value")] struct Foo; };
    if let Err(error) = parse_options(invalid) {
        println!("Invalid option: {}", error.write_errors());
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the `darling` attribute-parsing example.
///
/// # Errors
///
/// Returns an error if the valid example input cannot be parsed.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses the configured prefix and supplies the default when it is absent.
    ///
    /// # Errors
    ///
    /// Returns an error if either attribute input cannot be parsed.
    #[test]
    fn parses_prefix_and_default() -> darling::Result<()> {
        let options = parse_options(quote! {
            #[builder(prefix = "with_")]
            struct Foo;
        })?;
        assert_eq!(options.ident.to_string(), "Foo");
        assert_eq!(options.prefix.as_deref(), Some("with_"));

        let defaults = parse_options(quote! { #[builder] struct Bar; })?;
        assert_eq!(defaults.prefix, None);
        Ok(())
    }

    /// Rejects an unknown option and renders it as a compiler diagnostic.
    #[test]
    fn rejects_unknown_attribute_keys() {
        let error = parse_options(quote! {
            #[builder(unknown = "value")]
            struct Foo;
        })
        .expect_err("unknown builder options must be rejected");
        let diagnostic = error.write_errors().to_string();
        assert!(diagnostic.contains("unknown"));
    }
}
