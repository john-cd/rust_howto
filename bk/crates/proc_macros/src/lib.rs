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
    // `quote!` allows you to write Rust code directly, and `#ident`, `#ty`,
    // etc., are "splices" that insert the captured syntax elements.
    let expanded = quote::quote! {
        impl #impl_generics DebugPrint for #name #ty_generics #where_clause {
            fn debug_print(&self) {
                // `stringify!(#name)` converts the identifier name into a string literal.
                println!("Debugging {}: {:?}", stringify!(#name), self);
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
            println!("Executing SQL query: {}", query);
            // In a real scenario, you'd typically return some type
            // that represents the prepared statement or a query result.
            query
        }
    };

    proc_macro::TokenStream::from(expanded)
}
// ANCHOR_END: function_macro
