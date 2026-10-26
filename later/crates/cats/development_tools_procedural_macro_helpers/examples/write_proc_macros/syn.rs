// TODO Reviewed ??-??-2026

// ANCHOR: example
//! `syn` parses a token stream into Rust syntax-tree types such as
//! `DeriveInput`. A derive helper can inspect the input shape, reject
//! unsupported data with a spanned error, and return generated tokens for a
//! thin compiler entry point.

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::Ident;
use syn::Result as SynResult;

/// Generates a `describe` method for a struct with named fields.
///
/// # Errors
///
/// Returns a spanned error for invalid syntax or unsupported input.
fn derive_describe(input: TokenStream) -> SynResult<TokenStream> {
    let input: DeriveInput = syn::parse2(input)?;
    let ident = input.ident;
    let generics = input.generics;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields
                .named
                .into_iter()
                .map(|field| {
                    field.ident.ok_or_else(|| {
                        syn::Error::new_spanned(
                            &ident,
                            "Describe requires named fields",
                        )
                    })
                })
                .collect::<SynResult<Vec<Ident>>>()?,
            _ => {
                return Err(syn::Error::new_spanned(
                    &ident,
                    "Describe requires a struct with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &ident,
                "Describe can only be derived for structs",
            ));
        }
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// Returns a formatted description of the struct fields.
            pub fn describe(&self) -> String {
                let mut fields = Vec::new();
                #(
                    fields.push(format!(
                        concat!(stringify!(#fields), " = {:?}"),
                        &self.#fields
                    ));
                )*
                format!("{} {{ {} }}", stringify!(#ident), fields.join(", "))
            }
        }
    })
}

/// Generates and pretty-prints the method for a named struct.
///
/// # Errors
///
/// Returns an error if generation or Rust syntax parsing fails.
fn main() -> Result<()> {
    let input = quote! { struct Point { x: i32, y: i32 } };
    let generated = derive_describe(input)?;
    let syntax: syn::File = syn::parse2(generated)?;
    println!("{}", prettyplease::unparse(&syntax));
    Ok(())
}
// ANCHOR_END: example

/// Runs the `syn` derive-helper example.
///
/// # Errors
///
/// Returns an error if parsing or generation fails.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the generated method and a useful enum rejection.
    ///
    /// # Errors
    ///
    /// Returns an error if the generated code cannot be parsed.
    #[test]
    fn generates_for_structs_and_rejects_enums() -> SynResult<()> {
        let input = quote! { struct Point { x: i32, y: i32 } };
        let generated = derive_describe(input)?;
        let syntax: syn::File = syn::parse2(generated)?;
        let source = prettyplease::unparse(&syntax);
        assert!(source.contains("pub fn describe(&self) -> String"));

        let error = derive_describe(quote! { enum Direction { North } })
            .expect_err("derive_describe must reject enums");
        assert!(error.to_string().contains("only be derived for structs"));
        Ok(())
    }
}
