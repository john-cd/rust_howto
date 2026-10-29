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
fn dispatch(cli: Cli) -> anyhow::Result<()> {
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

/// Parses the command line and runs the selected example.
fn main() -> anyhow::Result<()> {
    dispatch(Cli::parse())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["der", "parse_x509", "pem_rfc7468", "pkcs8", "rcgen", "read_pem", "x509_cert"];

    /// The command-line definition is internally consistent.
    #[test]
    fn test_cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Each example is reachable through its own subcommand.
    #[test]
    fn test_every_subcommand_parses() {
        for name in SUBCOMMANDS {
            let cli = Cli::try_parse_from(["example", name])
                .unwrap_or_else(|e| panic!("`{name}` should parse: {e}"));
            assert!(cli.command.is_some(), "`{name}` selected no command");
        }
    }

    /// Subcommand names are unique.
    #[test]
    fn test_subcommand_names_are_unique() {
        let mut sorted = SUBCOMMANDS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), SUBCOMMANDS.len());
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }

    /// Running without a subcommand selects nothing and succeeds.
    #[test]
    fn test_no_subcommand_runs_nothing() {
        let cli = Cli::try_parse_from(["example"]).expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
