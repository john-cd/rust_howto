//! Runs one of the `certs` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod der;
mod parse_x509;
mod pem_rfc7468;
mod pkcs8;
mod rcgen;
mod read_pem;
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
    /// Parses a locally generated X.509 certificate.
    #[command(name = "parse_x509")]
    ParseX509,
    /// Runs the `pem_rfc7468` example.
    #[command(name = "pem_rfc7468")]
    PemRfc7468,
    /// Runs the `pkcs8` example.
    #[command(name = "pkcs8")]
    Pkcs8,
    /// Creates a self-signed development certificate.
    #[command(name = "rcgen")]
    Rcgen,
    /// Reads and decodes a PEM certificate file.
    #[command(name = "read_pem")]
    ReadPem,
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
            Commands::ParseX509 => {
                parse_x509::run()?;
            }
            Commands::PemRfc7468 => {
                pem_rfc7468::run()
                    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
            }
            Commands::Pkcs8 => {
                pkcs8::run()?;
            }
            Commands::Rcgen => {
                rcgen::run()?;
            }
            Commands::ReadPem => {
                read_pem::run()?;
            }
            Commands::X509Cert => {
                x509_cert::run()?;
            }
        }
    }

    Ok(())
}
