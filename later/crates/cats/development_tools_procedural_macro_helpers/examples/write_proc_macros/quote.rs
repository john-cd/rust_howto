// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Generate a struct and typed getters from field syntax using `quote`.
//!
//! `#variable` interpolates a value implementing `ToTokens`; repetitions such
//! as `#(#declarations),*` generate comma-separated fields and
//! `#(#getters)*` generates methods. `format_ident!` builds getter names,
//! while `quote_spanned!` preserves each field's span on its generated method
//! so compiler diagnostics point back to the corresponding input field.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::ToTokens;
use quote::format_ident;
use quote::quote;
use quote::quote_spanned;
use syn::Ident;
use syn::parse_quote;

/// Generates a struct with one getter for each named field.
///
/// # Errors
///
/// Returns an error if a field is unnamed.
fn generate_struct(
    struct_name: &Ident,
    fields: &[syn::Field],
) -> Result<TokenStream> {
    let mut declarations = Vec::with_capacity(fields.len());
    let mut getters = Vec::with_capacity(fields.len());

    for field in fields {
        let ident = field
            .ident
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("getters require named fields"))?;
        let ty = &field.ty;
        declarations.push(quote! { pub #ident: #ty });

        let getter = format_ident!("get_{}", ident);
        getters.push(quote_spanned! { ident.span() =>
            pub fn #getter(&self) -> #ty {
                self.#ident
            }
        });
    }

    let mut tokens = TokenStream::new();
    quote! {
        pub struct #struct_name {
            #(#declarations),*
        }
    }
    .to_tokens(&mut tokens);
    tokens.extend(quote! {
        impl #struct_name {
            #(#getters)*
        }
    });
    Ok(tokens)
}

/// Formats the generated `Point` struct and its getter methods.
///
/// # Errors
///
/// Returns an error if generation or Rust syntax parsing fails.
fn generated_source() -> Result<String> {
    let struct_name = format_ident!("Point");
    let fields = vec![parse_quote!(x: i32), parse_quote!(y: i32)];
    let tokens = generate_struct(&struct_name, &fields)?;
    let syntax: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&syntax))
}

/// Prints a readable generated struct and its getters.
///
/// # Errors
///
/// Returns an error if code generation or parsing fails.
fn main() -> Result<()> {
    println!("{}", generated_source()?);
    Ok(())
}
// ANCHOR_END: example

/// Runs the `quote` generation example.
///
/// # Errors
///
/// Returns an error if code generation or parsing fails.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that generated code contains a typed getter for each field.
    ///
    /// # Errors
    ///
    /// Returns an error if generated Rust syntax cannot be parsed.
    #[test]
    fn generates_struct_getters() -> Result<()> {
        let source = generated_source()?;
        assert!(source.contains("fn get_x(&self) -> i32"));
        assert!(source.contains("fn get_y(&self) -> i32"));
        Ok(())
    }

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
