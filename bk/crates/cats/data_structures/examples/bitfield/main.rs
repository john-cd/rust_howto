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
                let _ = bitflags::run();
            }
            Commands::Bitvec => {
                let _ = bitvec::run();
            }
            Commands::Flagset => {
                let _ = flagset::run();
            }
            Commands::Roaring => {
                let _ = roaring::run();
            }
        }
    }
}
