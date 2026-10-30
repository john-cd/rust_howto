//! Runs one of the `build_time_tooling` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod cc_bundled_cpp1;
mod cc_bundled_static1;
mod cc_defines1;
mod out_dir_codegen;

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
    /// Runs the `cc_bundled_cpp1` example.
    #[command(name = "cc_bundled_cpp1")]
    CcBundledCpp1,
    /// Runs the `cc_bundled_static1` example.
    #[command(name = "cc_bundled_static1")]
    CcBundledStatic1,
    /// Runs the `cc_defines1` example.
    #[command(name = "cc_defines1")]
    CcDefines1,
    /// Runs the generated Rust source example.
    #[command(name = "out_dir_codegen")]
    OutDirCodegen,
}

/// Runs the example selected on the command line.
fn dispatch(cli: Cli) -> anyhow::Result<()> {
    if let Some(command) = cli.command {
        match command {
            Commands::CcBundledCpp1 => {
                cc_bundled_cpp1::run();
            }
            Commands::CcBundledStatic1 => {
                cc_bundled_static1::run()?;
            }
            Commands::CcDefines1 => {
                cc_defines1::run()?;
            }
            Commands::OutDirCodegen => {
                out_dir_codegen::run();
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
        "cc_bundled_cpp1",
        "cc_bundled_static1",
        "cc_defines1",
        "out_dir_codegen",
    ];

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
        let cli = Cli::try_parse_from(["example"])
            .expect("no subcommand is accepted");
        assert!(cli.command.is_none());
        dispatch(cli).expect("dispatching should succeed");
    }
}
