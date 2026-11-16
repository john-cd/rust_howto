//! Runs one of the `other_strings` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// The `bstr` crate provides types and traits for working with byte strings,
mod bstr;

/// This example demonstrates how to work with C-style strings (`CString` and `CStr`) in Rust, which
/// are essential for FFI (Foreign Function Interface) interactions.
mod cstring;
/// Working with `OsString` and `OsStr`, the platform-native strings.
mod osstring;
/// String interning example.
mod ustr;

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
    /// Runs the `bstr` example.
    #[command(name = "bstr")]
    Bstr,

    /// Runs the `cstring` example.
    #[command(name = "cstring")]
    Cstring,

    /// Runs the `osstring` example.
    #[command(name = "osstring")]
    Osstring,

    /// Runs the `ustr` example.
    #[command(name = "ustr")]
    Ustr,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::Bstr => {
                bstr::run();
            }
            Commands::Cstring => {
                cstring::run();
            }
            Commands::Osstring => {
                osstring::run();
            }
            Commands::Ustr => {
                ustr::run();
            }
        }
    }
}

/// Parses the command line and runs the selected example.
fn main() {
    dispatch(Cli::parse());
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    /// Every example is exposed as a subcommand.
    const SUBCOMMANDS: &[&str] = &["bstr", "cstring", "osstring", "ustr"];

    /// Confirms that the command and its arguments are valid.
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
}
