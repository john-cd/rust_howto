extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemFn};

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
    name: String,
}

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
