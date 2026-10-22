//! Runs one of the `password_hashing` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026

use clap::Parser;
use clap::Subcommand;

mod argon2;
mod bcrypt;
mod pbkdf2;
mod scrypt;

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
    /// Runs the `argon2` example.
    #[command(name = "argon2")]
    Argon2,
    /// Runs the `bcrypt` example.
    #[command(name = "bcrypt")]
    Bcrypt,
    /// Runs the `pbkdf2` example.
    #[command(name = "pbkdf2")]
    Pbkdf2,
    /// Runs the `scrypt` example.
    #[command(name = "scrypt")]
    Scrypt,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Argon2 => {
                argon2::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::Bcrypt => {
                bcrypt::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Pbkdf2 => {
                pbkdf2::run().map_err(|e| anyhow::anyhow!("{e:?}"))?;
            }
            Commands::Scrypt => {
                scrypt::run().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
        }
    }

    Ok(())
}
