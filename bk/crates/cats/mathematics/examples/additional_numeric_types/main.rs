// TODO Reviewed ??-??-2026
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

use clap::Parser;
use clap::Subcommand;

mod big_integers;
mod num_bigint;
mod num_traits;
mod ordered_float;
#[cfg(target_os = "linux")]
mod rug;
mod rust_decimal;
mod typenum;

/// Command-line arguments.
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[derive(Subcommand)]
enum Commands {
    /// Runs the `big_integers` example.
    #[command(name = "big_integers")]
    BigIntegers,
    /// Runs the `num_bigint` example.
    #[command(name = "num_bigint")]
    NumBigint,
    /// Runs the `num_traits` example.
    #[command(name = "num_traits")]
    NumTraits,
    /// Runs the `ordered_float` example.
    #[command(name = "ordered_float")]
    OrderedFloat,
    /// Runs the `rug` example.
    #[cfg(target_os = "linux")]
    #[command(name = "rug")]
    Rug,
    /// Runs the `rust_decimal` example.
    #[command(name = "rust_decimal")]
    RustDecimal,
    /// Runs the `typenum` example.
    #[command(name = "typenum")]
    Typenum,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BigIntegers => {
                big_integers::run();
            }
            Commands::NumBigint => {
                num_bigint::run();
            }
            Commands::NumTraits => {
                num_traits::run();
            }
            Commands::OrderedFloat => {
                ordered_float::run();
            }
            #[cfg(target_os = "linux")]
            Commands::Rug => {
                rug::run();
            }
            Commands::RustDecimal => {
                rust_decimal::run();
            }
            Commands::Typenum => {
                typenum::run();
            }
        }
    }
}
