// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod cell;
mod once_cell;
mod rc_refcell;
mod refcell;

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
    /// Runs the `cell` example.
    #[command(name = "cell")]
    Cell,
    /// Runs the `once_cell` example.
    #[command(name = "once_cell")]
    OnceCell,
    /// Runs the `rc_refcell` example.
    #[command(name = "rc_refcell")]
    RcRefcell,
    /// Runs the `refcell` example.
    #[command(name = "refcell")]
    Refcell,
}

/// Runs the example selected on the command line.
fn main() {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cell => {
                cell::run();
            }
            Commands::OnceCell => {
                once_cell::run();
            }
            Commands::RcRefcell => {
                rc_refcell::run();
            }
            Commands::Refcell => {
                refcell::run();
            }
        }
    }
}
