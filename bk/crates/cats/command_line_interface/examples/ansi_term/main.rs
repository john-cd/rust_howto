// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod ansi_term_basic;
mod ansi_term_basic1;
mod ansi_term_basic2;
mod colored;

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
    /// Runs the `ansi_term_basic` example.
    #[command(name = "ansi_term_basic")]
    AnsiTermBasic,
    /// Runs the `ansi_term_basic1` example.
    #[command(name = "ansi_term_basic1")]
    AnsiTermBasic1,
    /// Runs the `ansi_term_basic2` example.
    #[command(name = "ansi_term_basic2")]
    AnsiTermBasic2,
    /// Runs the `colored` example.
    #[command(name = "colored")]
    Colored,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::AnsiTermBasic => {
                ansi_term_basic::run();
            }
            Commands::AnsiTermBasic1 => {
                ansi_term_basic1::run();
            }
            Commands::AnsiTermBasic2 => {
                ansi_term_basic2::run();
            }
            Commands::Colored => {
                colored::run();
            }
        }
    }
}
