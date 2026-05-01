//! Demonstrates how to use `darling`, `proc-macro2`, `syn`, and `quote`
//! outside of a `proc-macro` crate.
//!
//! This example parses a simple struct syntax tree, generates a `Debug`
//! implementation, and prints the resulting token stream.

use clap::Parser;

mod darling;
mod paste;
mod proc_macro2;
mod quote;
mod syn;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    example: String,
}

fn main() {
    let cli = Cli::parse();
    match cli.example.as_str() {
        "darling" => darling::run(),
        "paste" => paste::run(),
        "proc_macro2" => proc_macro2::run(),
        "quote" => quote::run(),
        "syn" => syn::run(),
        _ => eprintln!("Unknown example. Try: darling, paste, proc_macro2, quote, syn"),
    }
}

// // [finish; review the following; decide what examples are needed](https://github.com/john-cd/rust_howto/issues/1158)
// // for proc macros. move proc macros to lib.rs
// // reorganize examples in folders
// // <https://doc.rust-lang.org/proc_macro/index.html>
// // <https://doc.rust-lang.org/rust-by-example/macros.html>
// // <https://doc.rust-lang.org/reference/macros.html>
// // <https://doc.rust-lang.org/reference/macros-by-example.html>
// // <https://doc.rust-lang.org/reference/procedural-macros.html>
// // <https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros>
// // <https://doc.rust-lang.org/proc_macro/>
// // <https://doc.rust-lang.org/proc_macro/struct.TokenStream.html>
// // <https://docs.rs/syn/latest/syn/>
// // <https://docs.rs/syn/latest/syn/visit/index.html>
// // <https://ferrous-systems.com/blog/testing-proc-macros/>
// // <https://medium.com/@alfred.weirich/the-rust-macro-system-part-1-an-introduction-to-attribute-macros-73c963fd63ea>
// // <https://www.shuttle.dev/blog/2022/12/23/procedural-macros>
// // <https://doc.rust-lang.org/core/macro.compile_error.html>
// // <https://github.com/dtolnay/watt>.
