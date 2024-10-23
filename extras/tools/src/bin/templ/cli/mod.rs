//! The `cli` module is the command line argument parser for the
//! application

use clap::Command;

mod badge;
mod config;
mod rbe;

use badge::*;
use config::*;
use rbe::*;

/// The command that the end user selected
#[derive(Default, Debug)]
pub(super) enum Cmd {
    Badges(BadgeCmdArgs),
    Rbe(RbeCmdArgs),
    #[default]
    None,
}

/// `run` returns the configuration and command that the end user
/// selected.
pub(super) fn run() -> (Config, Cmd) {
    let matches = cli().get_matches(); // Parse [env::args_os], exiting on failure.
                                       // Check for the existence of subcommands
    let conf = config::get_config(&matches);
    let cmd = if let Some(b) = badge::get_cmd(&matches) {
        Cmd::Badges(b)
    } else if let Some(r) = rbe::get_cmd(&matches) {
        Cmd::Rbe(r)
    } else {
        Cmd::None
    };
    (conf, cmd)
}

/// Builds the CLI user interface
fn cli() -> Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        //.about("")
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command’s help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(badge::subcommand_badge())
        .subcommand(rbe::subcommand_rbe())
        .arg(config::arg_verbose())
}

#[test]
fn test_app() {
    cli().debug_assert();
}
