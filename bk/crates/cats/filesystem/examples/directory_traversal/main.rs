//! Runs one of the `directory_traversal` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod duplicate_name;
mod find_file;
mod globset;
mod ignore;
mod ignore_case;
#[cfg(target_os = "linux")]
mod loops;
mod modified;
mod png;
mod sizes;
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
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_cli_dispatcher() {
        let _ = Cli::try_parse_from(["example"]);
    }
}
