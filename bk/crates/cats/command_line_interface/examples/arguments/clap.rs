// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to use the `clap` crate to parse command-line
//! arguments.
//!
//! It showcases various features such as positional arguments, named arguments,
//! subcommands, default values, validation, and counting occurrences of flags.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use clap::Subcommand;

// The struct declaring the desired command-line arguments and commands:
// The `derive` feature flag is required (see `Cargo.toml`).
/// Command-line arguments.
#[derive(Parser, Debug)]
// Reads the following attributes from the package's `Cargo.toml`
// Alternatively, use `#[command(name = "MyApp")]`, etc...
#[command(author, version, about, long_about = None)]
// Displays Help if no arguments are provided:
/// Stores the command-line options accepted by the example.
#[command(arg_required_else_help = true)]
pub struct Cli {
    // Positional argument example.
    /// The pattern to look for (the doc comment appears in the help):
    pattern: Option<String>,

    /// Required argument example (with default value and validation):
    #[arg(default_value_t = 8080)]
    #[arg(value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    // Named argument example: the path to the file to look into:
    /// Named argument: the path to the file to look into.
    #[arg(short, long)]
    path: Option<PathBuf>,

    /// Count example - turn debugging information on:
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    // // Alternatively, use the
    // // `clap-verbosity-flag` crate:
    // // It adds the following flags through the entire program:
    // // -q silences output
    // // -v show warnings
    // // -vv show info
    // // -vvv show debug
    // // -vvvv show trace
    // //  By default, this will only report errors.
    // #[clap(flatten)]
    // verbose: clap_verbosity_flag::Verbosity,

    // Subcommands:
    /// The example to run.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// The subcommands that can be used with the CLI.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Read something:
    #[command(arg_required_else_help = true)]
    Read {
        /// A boolean flag:
        #[arg(short, long)]
        all: bool,

        /// The files to read.
        #[arg(required = true)]
        file: Vec<PathBuf>,
    },
    /// Say something:
    Tell,
}

/// Runs the example selected on the command line.
fn main() -> Result<()> {
    // `Clap` returns a Cli struct populated from `std::env::args_os()`...
    let cli = Cli::try_parse()?;
    // We also could use `parse()`.

    // The argument values we got...
    println!("Path: {:?}", cli.path);

    // Use `unwrap_or` to set defaults for optional arguments
    // (or use `default_value_t` as above).
    println!("Pattern: {:?}", cli.pattern.unwrap_or("".to_owned()));

    // We can also see how many times a particular flag or argument occurred.
    // Note, only flags can have multiple occurrences.
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // // Alternatively, use the `verbose` flag, if configured above
    // env_logger::Builder::new()
    //     .filter_level(cli.verbose.log_level_filter())
    //     .init();

    // Check for the existence of subcommands:
    match &cli.command {
        Some(Commands::Read { all, file }) => {
            if *all {
                println!("Read all...");
            } else {
                println!("Read just one...");
            }
            println!("{file:?}");
        }
        Some(Commands::Tell) => {
            println!("42");
        }
        None => {}
    }
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the clap example's main path succeeds.
    /// Runs the clap example through its entry point.
    #[test]
    fn test_clap_example_entry_point() {
        println!("{:?}", main());
    }

    #[test]
    fn defaults_apply_when_only_a_pattern_is_given()
    -> Result<(), Box<dyn std::error::Error>> {
        let cli = Cli::try_parse_from(["app", "needle"])?;
        assert_eq!(cli.pattern.as_deref(), Some("needle"));
        assert_eq!(cli.port, 8080);
        assert_eq!(cli.debug, 0);
        assert!(cli.path.is_none());
        assert!(cli.command.is_none());
        Ok(())
    }

    #[test]
    fn repeated_debug_flags_are_counted_and_options_parsed()
    -> Result<(), Box<dyn std::error::Error>> {
        let cli =
            Cli::try_parse_from(["app", "-dd", "-p", "a/b", "p", "9000"])?;
        assert_eq!(cli.debug, 2);
        assert_eq!(cli.path, Some(PathBuf::from("a/b")));
        assert_eq!(cli.port, 9000);
        Ok(())
    }

    #[test]
    fn port_zero_and_non_numbers_are_rejected() {
        assert!(Cli::try_parse_from(["app", "p", "0"]).is_err());
        assert!(Cli::try_parse_from(["app", "p", "http"]).is_err());
        assert!(Cli::try_parse_from(["app", "p", "70000"]).is_err());
    }

    #[test]
    fn read_subcommand_requires_files() -> Result<(), Box<dyn std::error::Error>>
    {
        let cli = Cli::try_parse_from(["app", "read", "-a", "f1", "f2"])?;
        match cli.command {
            Some(Commands::Read { all, file }) => {
                assert!(all);
                assert_eq!(file, [PathBuf::from("f1"), PathBuf::from("f2")]);
            }
            other => panic!("unexpected command: {other:?}"),
        }
        assert!(Cli::try_parse_from(["app", "read"]).is_err());
        assert!(matches!(
            Cli::try_parse_from(["app", "tell"])?.command,
            Some(Commands::Tell)
        ));
        Ok(())
    }

    #[test]
    fn no_arguments_shows_help_instead_of_running() {
        let Err(err) = Cli::try_parse_from(["app"]) else {
            panic!("parsing without arguments should fail");
        };
        assert_eq!(
            err.kind(),
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
}
