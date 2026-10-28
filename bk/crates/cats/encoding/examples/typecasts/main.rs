//! Runs one of the `typecasts` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod bytemuck;
mod primitive;
mod zerocopy;

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
    /// Runs the `bytemuck` example.
    #[command(name = "bytemuck")]
    Bytemuck,
    /// Runs the primitive conversion example.
    #[command(name = "primitive")]
    Primitive,
    /// Runs the `zerocopy` example.
    #[command(name = "zerocopy")]
    Zerocopy,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Bytemuck => {
                bytemuck::run();
            }
            Commands::Primitive => {
                primitive::run()?;
            }
            Commands::Zerocopy => {
                zerocopy::run()?;
            }
        }
    }
    Ok(())
}
