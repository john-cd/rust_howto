//! Runs one of the `in_memory_cache` examples, chosen on the command line.
//!
//! Reviewed 10-27-2026.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// This example demonstrates two key uses of the `cached` crate: - Simple function memoization with
/// `#[cached]`,
mod cached;

/// Basic usage of `LruCache` from the `lru` crate.
mod lru;

/// Basic usage of the `moka` crate for high-performance concurrent caching.
mod moka;

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

    /// Runs the `cached` example.
    #[command(name = "cached")]
    Cached,

    /// Runs the `lru` example.
    #[command(name = "lru")]
    Lru,

    /// Runs the `moka` example.
    #[command(name = "moka")]
    Moka,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::Cached => {
                cached::run();
            }
            Commands::Lru => {
                lru::run();
            }
            Commands::Moka => {
                moka::run();
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
    const SUBCOMMANDS: &[&str] = &["cached", "lru", "moka"];

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
