//! Runs one of the `string_encoding` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod base64;
mod form_urlencoded;
mod hex;
mod hex_crate;
mod percent_encode;
mod percent_encoding;
mod url_encode;

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
    /// Runs the `base64` example.
    #[command(name = "base64")]
    Base64,
    /// Runs the `form_urlencoded` example.
    #[command(name = "form_urlencoded")]
    FormUrlencoded,
    /// Runs the `hex` example.
    #[command(name = "hex")]
    Hex,
    /// Runs the `hex_crate` example.
    #[command(name = "hex_crate")]
    HexCrate,
    /// Runs the `percent_encode` example.
    #[command(name = "percent_encode")]
    PercentEncode,
    /// Runs the `percent_encoding` example.
    #[command(name = "percent_encoding")]
    PercentEncoding,
    /// Runs the `url_encode` example.
    #[command(name = "url_encode")]
    UrlEncode,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Base64 => {
                base64::run()?;
            }
            Commands::FormUrlencoded => {
                form_urlencoded::run()?;
            }
            Commands::HexCrate => {
                hex_crate::run()?;
            }
            Commands::Hex => {
                hex::run()?;
            }
            Commands::PercentEncode => {
                percent_encode::run()?;
            }
            Commands::PercentEncoding => {
                percent_encoding::run()?;
            }
            Commands::UrlEncode => {
                url_encode::run();
            }
        }
    }
    Ok(())
}
