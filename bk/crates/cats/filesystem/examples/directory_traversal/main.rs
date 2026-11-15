//! Runs one of the `directory_traversal` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

/// Find duplicate filenames in the current directory.
mod duplicate_name;
/// This example demonstrates how to traverse a directory and find files that
mod find_file;
/// This example demonstrates how to use the `globset` crate to match file paths
mod globset;
/// This example demonstrates how to use the `ignore` crate to traverse a directory recursively,
/// respecting `.gitignore` and similar files, and
mod ignore;
/// Demonstrates how to ignore case when matching file names.
mod ignore_case;
/// This example demonstrates the use of the `same_file` crate.
#[cfg(target_os = "linux")]
mod loops;
/// This example demonstrates how to traverse a directory and check the last
mod modified;
/// This example demonstrates how to use the `glob` crate
mod png;
/// Calculates the total size of all files within a directory tree
mod sizes;
/// This example demonstrates how to skip hidden files and directories during
mod skip_dot;

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
    /// Runs the `duplicate_name` example.
    #[command(name = "duplicate_name")]
    DuplicateName,
    /// Runs the `find_file` example.
    #[command(name = "find_file")]
    FindFile,
    /// Runs the `globset` example.
    #[command(name = "globset")]
    Globset,
    /// Runs the `ignore` example.
    #[command(name = "ignore")]
    Ignore,
    /// Runs the `ignore_case` example.
    #[command(name = "ignore_case")]
    IgnoreCase,
    /// Runs the `loops` example.
    #[cfg(target_os = "linux")]
    #[command(name = "loops")]
    Loops,
    /// Runs the `modified` example.
    #[command(name = "modified")]
    Modified,
    /// Runs the `png` example.
    #[command(name = "png")]
    Png,
    /// Runs the `sizes` example.
    #[command(name = "sizes")]
    Sizes,
    /// Runs the `skip_dot` example.
    #[command(name = "skip_dot")]
    SkipDot,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::DuplicateName => {
                duplicate_name::run()?;
            }
            Commands::FindFile => {
                find_file::run()?;
            }
            Commands::Globset => {
                globset::run()?;
            }
            Commands::Ignore => {
                ignore::run();
            }
            Commands::IgnoreCase => {
                ignore_case::run()?;
            }
            #[cfg(target_os = "linux")]
            Commands::Loops => {
                loops::run();
            }
            Commands::Modified => {
                modified::run()?;
            }
            Commands::Png => {
                png::run()?;
            }
            Commands::Sizes => {
                sizes::run()?;
            }
            Commands::SkipDot => {
                skip_dot::run()?;
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
        "duplicate_name",
        "find_file",
        "globset",
        "ignore",
        "ignore_case",
        #[cfg(target_os = "linux")]
        "loops",
        "modified",
        "png",
        "sizes",
        "skip_dot",
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
