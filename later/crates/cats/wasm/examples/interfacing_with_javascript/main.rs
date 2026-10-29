//! Runs one of the `interfacing_with_javascript` examples, chosen on the
//! command line.
//!
//! TODO Reviewed ??-??-2026.

use clap::Parser;
use clap::Subcommand;

mod js_import;
mod js_sys;
mod promises;
mod serde_wasm_bindgen;
mod web_sys;

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
    /// Runs the JavaScript import example.
    #[command(name = "js_import")]
    JsImport,
    /// Runs the `js_sys` example.
    #[command(name = "js_sys")]
    JsSys,
    /// Runs the Promise example.
    #[command(name = "promises")]
    Promises,
    /// Runs the Serde conversion example.
    #[command(name = "serde_wasm_bindgen")]
    SerdeWasmBindgen,
    /// Runs the `web_sys` example.
    #[command(name = "web_sys")]
    WebSys,
}

/// Runs the example selected on the command line.
fn main() {
    dispatch(Cli::parse());
}

/// Runs the example selected in the parsed command line.
fn dispatch(cli: Cli) {
    if let Some(command) = cli.command {
        match command {
            Commands::JsImport => js_import::run(),
            Commands::JsSys => js_sys::run(),
            Commands::Promises => promises::run(),
            Commands::SerdeWasmBindgen => serde_wasm_bindgen::run(),
            Commands::WebSys => web_sys::run(),
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory as _;

    use super::*;

    /// The command-line definition is internally consistent.
    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    /// Without a subcommand, nothing is run.
    #[test]
    fn no_subcommand_runs_nothing() {
        let cli =
            Cli::try_parse_from(["example"]).expect("no arguments are valid");
        assert!(cli.command.is_none());
        dispatch(cli);
    }

    /// Every declared subcommand name is accepted.
    #[test]
    fn declared_subcommands_parse() {
        let command = Cli::command();
        let names: Vec<_> = command
            .get_subcommands()
            .map(clap::Command::get_name)
            .collect();
        for name in names {
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
