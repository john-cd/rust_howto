// TODO Reviewed ??-??-2026
use clap::Parser;
use clap::Subcommand;

mod blake3;
mod md5;
mod sha1;
mod sha2;
mod sha_digest;

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
    /// Runs the `blake3` example.
    #[command(name = "blake3")]
    Blake3,
    /// Runs the `md5` example.
    #[command(name = "md5")]
    Md5,
    /// Runs the `sha1` example.
    #[command(name = "sha1")]
    Sha1,
    /// Runs the `sha2` example.
    #[command(name = "sha2")]
    Sha2,
    /// Runs the `sha_digest` example.
    #[command(name = "sha_digest")]
    ShaDigest,
}

/// Runs the example selected on the command line.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Blake3 => {
                blake3::run();
            }
            Commands::Md5 => {
                md5::run();
            }
            Commands::Sha1 => {
                sha1::run();
            }
            Commands::Sha2 => {
                sha2::run()?;
            }
            Commands::ShaDigest => {
                sha_digest::run()?;
            }
        }
    }

    Ok(())
}
