//! Runs one of the `directories` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cwd;
mod manipulate_dirs;
mod remove_dir_all;

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
    /// Runs the `cwd` example.
    #[command(name = "cwd")]
    Cwd,
    /// Runs the `manipulate_dirs` example.
    #[command(name = "manipulate_dirs")]
    ManipulateDirs,
    /// Runs the `remove_dir_all` example.
    #[command(name = "remove_dir_all")]
    RemoveDirAll,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Cwd => {
                cwd::run()?;
            }
            Commands::ManipulateDirs => {
                manipulate_dirs::run()?;
            }
            Commands::RemoveDirAll => {
                remove_dir_all::run()?;
            }
        }
    }
    Ok(())
}
