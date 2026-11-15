//! Runs one of the `paths` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// UTF-8 paths with `camino`.
mod camino;

/// Canonicalizing paths.
mod canonicalize;

/// Canonicalize a path without the Windows `\\?\` prefix.
mod dunce;

/// Building and inspecting paths with `Path` and `PathBuf`.
mod manipulate_paths;

/// Compute a relative path between two paths.
mod pathdiff;

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
    /// Runs the `camino` example.
    #[command(name = "camino")]
    Camino,

    /// Runs the `canonicalize` example.
    #[command(name = "canonicalize")]
    Canonicalize,

    /// Runs the `dunce` example.
    #[command(name = "dunce")]
    Dunce,

    /// Runs the `manipulate_paths` example.
    #[command(name = "manipulate_paths")]
    ManipulatePaths,

    /// Runs the `pathdiff` example.
    #[command(name = "pathdiff")]
    Pathdiff,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::Camino => {
                camino::run();
            }
            Commands::Canonicalize => {
                canonicalize::run()?;
            }
            Commands::Dunce => {
                dunce::run()?;
            }
            Commands::ManipulatePaths => {
                manipulate_paths::run();
            }
            Commands::Pathdiff => {
                pathdiff::run();
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
    const SUBCOMMANDS: &[&str] = &[
        "camino",
        "canonicalize",
        "dunce",
        "manipulate_paths",
        "pathdiff",
    ];

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
