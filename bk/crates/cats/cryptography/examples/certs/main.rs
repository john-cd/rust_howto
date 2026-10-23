//! Runs one of the `certs` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod der;
mod pem_rfc7468;
mod pkcs8;
mod x509_cert;

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
    /// Runs the `der` example.
    #[command(name = "der")]
    Der,
    /// Runs the `pem_rfc7468` example.
    #[command(name = "pem_rfc7468")]
    PemRfc7468,
    /// Runs the `pkcs8` example.
    #[command(name = "pkcs8")]
    Pkcs8,
    /// Runs the `x509_cert` example.
    #[command(name = "x509_cert")]
    X509Cert,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Der => {
                der::run();
            }
            Commands::PemRfc7468 => {
                pem_rfc7468::run()
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Pkcs8 => {
                pkcs8::run()?;
            }
            Commands::X509Cert => {
                x509_cert::run()?;
            }
        }
    }

    Ok(())
}
