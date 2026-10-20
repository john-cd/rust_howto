#![allow(dead_code)]
// ANCHOR: example
//! Watt is a procedural macro library that allows writing
//! procedural macros without the overhead of the `proc_macro` crate.
//!
//! It is a runtime for executing Rust procedural macros compiled as
//! WebAssembly.
//!
//! Using Watt involves two main parts:
//! 1. A "proc-macro" crate that acts as a shim, loading and executing the
//!    WASM-compiled macro.
//! 2. A "source" crate that contains the actual macro logic and is compiled to
//!    WASM.
//!
//! This approach provides isolation and can speed up compilation as the macro
//! logic is pre-compiled to WASM.

// use quote::quote;
// use syn::ItemFn;
// use syn::parse_macro_input;
// use watt::WasmMacro;

// /// It allows defining macros directly within Rust code using
// /// WebAssembly text format, offering a more streamlined approach compared to
// /// traditional `proc_macro` crate usage.
// static MY_MACRO: WasmMacro = watt::proc_macro!(
//     "
//     (func (export \"my_macro\")
//         (param i32) (result i32)
//         local.get 0
//         i32.const 1
//         i32.add
//     )
// "
// );

// /// Creates a procedural macro `my_macro` that takes a function and prints
// /// "Function called" when the function is called.
// #[proc_macro]
// pub fn my_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = parse_macro_input!(input as ItemFn);
//     let func_name = input.sig.ident;

//     let expanded = quote! {
//         fn #func_name() {
//             println!("Function called");
//         }
//     };

//     proc_macro::TokenStream::from(expanded)
// }

// // // Usage example:
// // use my_macro_crate::my_macro;

// // #[my_macro]
// // fn example_function() {
// //     println!("Hello from the macro-enhanced function!");
// // }

// // fn main() {
// //     example_function();
// // }

use development_tools_procedural_macro_helpers::watt_macro;

// Demonstrating the usage of a macro that could be powered by Watt.
// In this example, 'watt_macro' is a shim that would normally load a WASM file.
watt_macro!(some input);

fn main() {
    println!("Demonstrating Watt-like macro usage:");
    watt_demo();
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watt_demo() {
        main();
    }
}

// [finish](https://github.com/john-cd/rust_howto/issues/744)
