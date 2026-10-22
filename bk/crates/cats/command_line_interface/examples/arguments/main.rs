// TODO Reviewed ??-??-2026
use ::clap::Parser;
use ::clap::Subcommand;

mod clap_basic;
mod clap_complete;
#[path = "clap.rs"]
mod clap_example;
mod lexopt;
mod pico_args;

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
    /// Runs the `clap` example.
    #[command(name = "clap")]
    Clap,
    /// Runs the `clap_basic` example.
    #[command(name = "clap_basic")]
    ClapBasic,
    /// Runs the `clap_complete` example.
    #[command(name = "clap_complete")]
    ClapComplete,
    /// Runs the `lexopt` example.
    #[command(name = "lexopt")]
    Lexopt,
    /// Runs the `pico_args` example.
    #[command(name = "pico_args")]
    PicoArgs,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Clap => {
                clap_example::run()?;
            }
            Commands::ClapBasic => {
                clap_basic::run();
            }
            Commands::ClapComplete => {
                clap_complete::run()?;
            }
            Commands::Lexopt => {
                lexopt::run()?;
            }
            Commands::PicoArgs => {
                pico_args::run()?;
            }
        }
    }

    Ok(())
}
