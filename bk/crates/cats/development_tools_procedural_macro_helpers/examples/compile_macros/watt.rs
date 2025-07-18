#![allow(dead_code)]
// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! Watt is a procedural macro library that allows you to write your
// //! procedural macros without the overhead of the `proc_macro` crate.
// //! Watt is a runtime for executing Rust procedural macros compiled as
// //! WebAssembly.
// use quote::quote;
// use syn::ItemFn;
// use syn::parse_macro_input;
// use watt::WasmMacro;

// /// It allows you to define macros directly within your Rust code using
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

// // #[test]
// // fn test() {
// //     main();
// // }
// // [finish](https://github.com/john-cd/rust_howto/issues/744)
