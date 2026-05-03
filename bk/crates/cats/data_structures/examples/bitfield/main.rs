use clap::Parser;
use clap::Subcommand;

mod bitflags;
mod bitvec;
mod flagset;
mod roaring;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "bitflags")]
    Bitflags,
    #[command(name = "bitvec")]
    Bitvec,
    #[command(name = "flagset")]
    Flagset,
    #[command(name = "roaring")]
    Roaring,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bitflags => {
                bitflags::run();
            }
            Commands::Bitvec => {
                bitvec::run();
            }
            Commands::Flagset => {
                flagset::run();
            }
            Commands::Roaring => {
                roaring::run();
            }
        }
    }
}
