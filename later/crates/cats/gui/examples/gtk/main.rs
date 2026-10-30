//! Runs one of the `gtk` examples, chosen on the command line.
//!
//! TODO Reviewed ??-??-2026.

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
use clap::Parser;
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
use clap::Subcommand;

#[cfg(all(feature = "gtk", not(target_os = "windows")))]
mod gtk4;
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
mod relm4;

/// Command-line arguments.
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
#[derive(Parser)]
struct Cli {
    /// The example to run.
    #[command(subcommand)]
    command: Option<Commands>,
}

/// The examples that can be run.
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
#[derive(Subcommand)]
enum Commands {
    /// Runs the `gtk4` example.
    #[command(name = "gtk4")]
    Gtk4,
    /// Runs the `relm4` example.
    #[command(name = "relm4")]
    Relm4,
}

/// Runs the example selected on the command line.
fn main() {
    run_with_args(std::env::args_os());
}

/// Runs the example selected by `args`, the first item being the program name.
#[cfg(all(feature = "gtk", not(target_os = "windows")))]
fn run_with_args<I, T>(args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let cli = Cli::parse_from(args);

    if let Some(command) = cli.command {
        match command {
            Commands::Gtk4 => {
                gtk4::run();
            }
            Commands::Relm4 => {
                relm4::run();
            }
        }
    }
}

/// Reports that the GTK examples are not available on Windows.
#[cfg(all(feature = "gtk", target_os = "windows"))]
fn run_with_args<I, T>(_args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    eprintln!("GTK examples are disabled on Windows.");
}

#[cfg(all(test, feature = "gtk", not(target_os = "windows")))]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// The command-line definition is internally consistent.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Running without a subcommand is accepted and runs nothing.
    #[test]
    fn missing_subcommand_runs_nothing() {
        run_with_args(["example"]);
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        for name in ["gtk4", "relm4"] {
            assert!(
                Cli::try_parse_from(["example", name]).is_ok(),
                "subcommand `{name}` should parse"
            );
        }
    }

    /// Unknown subcommands are rejected.
    #[test]
    fn unknown_subcommand_is_rejected() {
        assert!(Cli::try_parse_from(["example", "no-such-example"]).is_err());
    }
}
