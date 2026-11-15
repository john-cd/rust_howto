//! Runs one of the `oracle` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(any(feature = "oracle", feature = "sibyl"))]
use clap::Parser;
#[cfg(any(feature = "oracle", feature = "sibyl"))]
use clap::Subcommand;

/// `diesel_oci` is a Diesel backend and connection implementation for Oracle
#[cfg(feature = "oracle")]
mod diesel_oci;

/// Connect to Oracle, query user tables with bind variables, and print rows.
#[cfg(feature = "oracle")]
mod oracle;

/// Connects to Oracle with Sibyl's blocking API and reads a value from `DUAL`.
#[cfg(feature = "sibyl")]
mod sibyl;

/// Command-line arguments.
#[cfg(any(feature = "oracle", feature = "sibyl"))]
#[derive(Parser)]
struct Cli {

    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(any(feature = "oracle", feature = "sibyl"))]
#[derive(Subcommand)]
enum Commands {

    /// Runs the `diesel_oci` example.
    #[cfg(feature = "oracle")]
    #[command(name = "diesel_oci")]
    DieselOci,

    /// Runs the `oracle` example.
    #[cfg(feature = "oracle")]
    #[command(name = "oracle")]
    Oracle,

    /// Runs the `sibyl` example.
    #[cfg(feature = "sibyl")]
    #[command(name = "sibyl")]
    Sibyl,
}

/// Runs the example selected on the command line.
fn main() -> anyhow::Result<()> {
    #[cfg(any(feature = "oracle", feature = "sibyl"))]
    {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                #[cfg(feature = "oracle")]
                Commands::DieselOci => {
                    diesel_oci::run()?;
                }
                #[cfg(feature = "oracle")]
                Commands::Oracle => {
                    oracle::run().map_err(anyhow::Error::msg)?;
                }
                #[cfg(feature = "sibyl")]
                Commands::Sibyl => {
                    sibyl::run()?;
                }
            }
        }
    }
    Ok(())
}

/// Verifies command-line selection and execution of the documented examples.
#[cfg(test)]
mod tests {
    use super::*;

    /// Parses the Diesel OCI subcommand when its feature is enabled.
    #[cfg(feature = "oracle")]
    #[test]
    fn test_diesel_oci_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "diesel_oci"]).is_ok());
    }

    /// Verifies oracle subcommand parses.
    #[cfg(feature = "oracle")]
    #[test]
    fn test_oracle_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "oracle"]).is_ok());
    }

    /// Verifies sibyl subcommand parses.
    #[cfg(feature = "sibyl")]
    #[test]
    fn test_sibyl_subcommand_parses() {
        assert!(Cli::try_parse_from(["example", "sibyl"]).is_ok());
    }

    /// Unknown subcommands are rejected.
    #[cfg(any(feature = "oracle", feature = "sibyl"))]
    #[test]
    fn test_unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
