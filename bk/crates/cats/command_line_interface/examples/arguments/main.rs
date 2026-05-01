use clap::Parser;
use clap::Subcommand;

mod clap;
mod clap_basic;
mod clap_complete;
mod lexopt;
mod pico_args;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(name = "clap")]
    Clap,
    #[command(name = "clap_basic")]
    ClapBasic,
    #[command(name = "clap_complete")]
    ClapComplete,
    #[command(name = "lexopt")]
    Lexopt,
    #[command(name = "pico_args")]
    PicoArgs,
}

fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Clap => {
                clap::run();
            }
            Commands::ClapBasic => {
                clap_basic::run();
            }
            Commands::ClapComplete => {
                clap_complete::run();
            }
            Commands::Lexopt => {
                lexopt::run();
            }
            Commands::PicoArgs => {
                pico_args::run();
            }
        }
    }
}
