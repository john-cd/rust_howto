//! Demonstrates how to use `darling`, `proc-macro2`, `syn`, and `quote`
//! outside of a `proc-macro` crate.
//!
//! This example parses a simple struct syntax tree, generates a `Debug`
//! implementation, and prints the resulting token stream.

use clap::Parser;
use clap::Subcommand;

mod darling;
mod paste;
mod proc_macro2;
mod quote;
mod syn;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "darling")]
    Darling,
    #[command(name = "paste")]
    Paste,
    #[command(name = "proc_macro2")]
    ProcMacro2,
    #[command(name = "quote")]
    Quote,
    #[command(name = "syn")]
    Syn,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Darling => {
                darling::run();
            }
            Commands::Paste => {
                paste::run();
            }
            Commands::ProcMacro2 => {
                proc_macro2::run();
            }
            Commands::Quote => {
                quote::run();
            }
            Commands::Syn => {
                syn::run()?;
            }
        }
    }
    Ok(())
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
