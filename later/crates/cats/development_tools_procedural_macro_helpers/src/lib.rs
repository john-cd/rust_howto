// ANCHOR: derive_macro
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::parse_macro_input;

/// This is a derive macro example.
///
/// It will be used like this:
///
/// ```ignore
/// #[derive(MyMacro)]
/// ```
#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl #name {
            fn hello_world() {
                println!("Hello, World! I am #name");
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
// ANCHOR_END: derive_macro

// For more details, review the `syn` parser documentation:
// https://docs.rs/syn/latest/syn/index.html

// ANCHOR: attribute_macro
extern crate proc_macro;

use syn::ItemFn;

/// This is an attribute macro example.
///
/// It will be used like this:
///
/// ```ignore
/// #[log_fn]
/// ```
#[proc_macro_attribute]
pub fn log_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    let expanded = quote! {
        fn #name() {
            println!("Function {} is called", stringify!(#name));
            #input
        }
    };

    expanded.into()
}
// ANCHOR_END: attribute_macro

// ANCHOR: watt_macro
/// This is a demonstration of how a watt macro would be defined.
/// In a real scenario, the WASM file would be generated from a separate crate.
#[proc_macro]
pub fn watt_macro(input: TokenStream) -> TokenStream {
    // In a real watt implementation, we would load the WASM file:
    // static MACRO: watt::WasmMacro =
    // watt::proc_macro!(include_bytes!("my_macro.wasm"));
    // MACRO.proc_macro(input)

    // For this example, we'll just mock it:
    let input_str = input.to_string();
    let expanded = quote! {
        fn watt_demo() {
            println!("Watt macro received: {}", #input_str);
        }
    };
    TokenStream::from(expanded)
}
// ANCHOR_END: watt_macro

// [finish](https://github.com/john-cd/rust_howto/issues/1157)
