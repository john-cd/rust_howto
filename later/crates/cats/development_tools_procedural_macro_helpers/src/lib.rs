// TODO Reviewed ??-??-2026

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;
use syn::ItemFn;
use syn::parse_macro_input;

// Retain the macro used by the existing `compile_macros` example.
/// A derive macro that adds a `hello_world` function to the type.
#[proc_macro_derive(MyMacro)]
pub fn my_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            fn hello_world() {
                println!("Hello, World! I am #name");
            }
        }
    };
    TokenStream::from(expanded)
}

// ANCHOR: attribute_macro
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
    let block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    let expanded = quote! {
        #vis #sig {
            println!("Function {} is called", stringify!(#name));
            #block
        }
    };

    expanded.into()
}
// ANCHOR_END: attribute_macro

// ANCHOR: darling_example
/// Options for the `MyDarlingMacro` derive macro.
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(my_macro))]
struct MyMacroOpts {
    /// The name given in the `my_macro` attribute.
    name: String,
}

/// A derive macro that uses `darling` to parse its options.
#[proc_macro_derive(MyDarlingMacro, attributes(my_macro))]
pub fn my_darling_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = MyMacroOpts::from_derive_input(&input).expect("Wrong options!");

    let name = &opts.name;
    let struct_name = &input.ident;

    let expanded = quote! {
        impl MyDarlingTrait for #struct_name {
            fn hello() {
                println!("Hello, {}!", #name);
            }
        }
    };

    TokenStream::from(expanded)
}
// ANCHOR_END: darling_example

// ANCHOR: proc_macro2_example
/// A derive macro, written with `proc-macro2`, that implements `Debug`.
#[proc_macro_derive(MyDebug)]
pub fn derive_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let expanded = quote! {
        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{ ... }}", stringify!(#name))
            }
        }
    };
    TokenStream::from(expanded)
}
// ANCHOR_END: proc_macro2_example

// Retain the shim imported by the existing `compile_macros` Watt example.
/// A function-like macro used by the `watt` example.
#[proc_macro]
pub fn watt_macro(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();
    let expanded = quote! {
        fn watt_demo() {
            println!("Watt macro received: {}", #input_str);
        }
    };
    TokenStream::from(expanded)
}
