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

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "big_integers")]
    BigIntegers,
    #[command(name = "num_bigint")]
    NumBigint,
    #[command(name = "num_traits")]
    NumTraits,
    #[command(name = "ordered_float")]
    OrderedFloat,
    #[cfg(target_os = "linux")]
    #[command(name = "rug")]
    Rug,
    #[command(name = "rust_decimal")]
    RustDecimal,
    #[command(name = "typenum")]
    Typenum,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::BigIntegers => {
                let _ = big_integers::run();
            }
            Commands::NumBigint => {
                let _ = num_bigint::run();
            }
            Commands::NumTraits => {
                let _ = num_traits::run();
            }
            Commands::OrderedFloat => {
                let _ = ordered_float::run();
            }
            #[cfg(target_os = "linux")]
            Commands::Rug => {
                let _ = rug::run();
            }
            Commands::RustDecimal => {
                let _ = rust_decimal::run();
            }
            Commands::Typenum => {
                let _ = typenum::run();
            }
        }
    }
}
