// ANCHOR: attribute_macro
use proc_macro::TokenStream;

// Attribute macros are defined by a public function with the
// `proc_macro_attribute` attribute.
#[proc_macro_attribute]
pub fn log_calls(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // When the attribute is written as a bare attribute name, like in this
    // example, the attribute `TokenStream` is empty. If the attribute had
    // arguments, `_attr` would contain anything following the attribute's name,
    // not including the outer delimiters.

    // The second `TokenStream` is the rest of the annotated item, including
    // other attributes on the item. Here, we parse the annotated function:
    let input_fn = syn::parse_macro_input!(item as syn::ItemFn);

    let fn_name = &input_fn.sig.ident; // The function's name.
    let block = &input_fn.block; // The function's body.
    let sig = &input_fn.sig; // The function's signature.
    let vis = &input_fn.vis; // The function's visibility.

    let expanded = quote::quote! {
        #vis #sig {
            println!("Calling function `{}`...", stringify!(#fn_name));
            let result = #block; // Execute the original function body.
            println!("Function `{}` finished.", stringify!(#fn_name));
            result // Return the result.
        }
    };

    // The returned `TokenStream` can replace the annotated item with an
    // arbitrary number of items.
    TokenStream::from(expanded)
}
// ANCHOR_END: attribute_macro
// ANCHOR: derive_macro
// The `proc_macro_derive` attribute marks the public function
// `debug_print_derive` as a custom `derive` macro for the `DebugPrint` trait.
// The input `TokenStream` is the token stream of the struct, enum, or union
// that has the `derive` attribute.
#[proc_macro_derive(DebugPrint)]
pub fn debug_print_derive(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    // `syn`'s `parse_macro_input!` macro converts the input `TokenStream` into
    // a structured `DeriveInput` enum.
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident; // The name of the struct/enum.
    let (impl_generics, ty_generics, where_clause) =
        ast.generics.split_for_impl();

    // Generate the implementation of the `DebugPrint` trait.
    // `quote!` allows us to write Rust code directly, and `#ident`,
    // `#ty_generics`, etc. are "splices" that insert the captured syntax
    // elements.
    let expanded = quote::quote! {
        impl #impl_generics DebugPrint for #name #ty_generics #where_clause {
            fn debug_print(&self) {
                // `stringify!(#name)` converts the identifier name into a string literal.
                println!("Debugging {}: {self:?}", stringify!(#name));
            }
        }
    };

    // Return the generated `TokenStream`.
    proc_macro::TokenStream::from(expanded)
}
// ANCHOR_END: derive_macro
// ANCHOR: function_macro

// #[proc_macro] marks `sql` as a function-like procedural macro.
// It must be a public function with signature of `(TokenStream) ->
// TokenStream`. The input `TokenStream` is what is inside the delimiters of the
// macro invocation and the output `TokenStream` replaces the entire macro
// invocation.
#[proc_macro]
pub fn sql(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Here, `input` contains all the tokens passed to the macro call (e.g.,
    // SELECT * FROM users WHERE id = 1).
    // For this simple example, we'll just treat the input as a string literal
    // and wrap it. For robust parsing of complex SQL, you'd use a dedicated
    // parser.
    let sql_query = input.to_string();

    // Generate Rust code that uses the SQL string:
    let expanded = quote::quote! {
        {
            let query = #sql_query;
            println!("Executing SQL query: {query}");
            // In a real scenario, you'd typically return some type
            // that represents the prepared statement or a query result.
            query
        }
    };

    proc_macro::TokenStream::from(expanded)
}
// ANCHOR_END: function_macro

// ANCHOR: procmacro2_example

// `proc-macro2` allows manipulating TokenStreams outside of a procedural macro
// context, making it easier to write unit tests for procedural macros.
// Here we define the core logic using `proc_macro2::TokenStream`.
fn process_tokens(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    input
        .into_iter()
        .map(|tt| match tt {
            proc_macro2::TokenTree::Ident(ident) if ident == "old_ident" => {
                let new_ident =
                    proc_macro2::Ident::new("new_ident", ident.span());
                proc_macro2::TokenTree::Ident(new_ident)
            }
            proc_macro2::TokenTree::Group(group) => {
                let new_stream = process_tokens(group.stream());
                let mut new_group =
                    proc_macro2::Group::new(group.delimiter(), new_stream);
                new_group.set_span(group.span());
                proc_macro2::TokenTree::Group(new_group)
            }
            other => other,
        })
        .collect()
}

// Then we expose the procedural macro interface which seamlessly converts
// between `proc_macro::TokenStream` and `proc_macro2::TokenStream`.
#[proc_macro]
pub fn replace_ident(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let output2 = process_tokens(input2);
    proc_macro::TokenStream::from(output2)
}

// And we can write unit tests for the core logic directly!
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_process_tokens() {
        let input =
            proc_macro2::TokenStream::from_str("let old_ident = 1;").unwrap();
        let expected =
            proc_macro2::TokenStream::from_str("let new_ident = 1;").unwrap();

        let output = process_tokens(input);

        // Converting to string for easy comparison of the TokenStreams
        assert_eq!(output.to_string(), expected.to_string());
    }
}
// ANCHOR_END: procmacro2_example
