//! The `cli` module is the command line argument parser for the
//! application
//!
//! TODO Reviewed ??-??-2026.

use anyhow::Result;

/// Handle the `badge` CLI subcommand
mod badge_cmd;
/// Handle the `category_badge` CLI subcommand
mod category_badge_cmd;
/// Handle the `categories_badges_for_crate` CLI subcommand
mod category_badges_for_crate_cmd;
/// Loads and validates the settings that control cli behavior. Composition code uses this module
/// before starting services so invalid configuration is reported early.
mod config_flag;
/// Shared utilities functions
mod crate_name_arg;
/// The `info` subcommand, which gets information about a crate.
mod info_cmd;
/// Handle the `rbe` CLI subcommand
mod rbe;

use crate_name_arg::arg_crate_name;
use crate_name_arg::get_cmd_arg_crate_name;

use super::Cmd;
use super::Config;

/// `run` returns the configuration and command that the end user
/// selected.
pub(super) fn run() -> Result<(Config, Cmd)> {
    // Parse [env::args_os], exiting on failure.
    let matches = cli().get_matches();
    // Check for the existence of subcommands
    let conf = config_flag::get_config(&matches);
    let cmd = if let Some(b) = badge_cmd::get_cmd(&matches) {
        Cmd::Badges(b)
    } else if let Some(cc) = category_badges_for_crate_cmd::get_cmd(&matches) {
        Cmd::CategoriesForCrateBadge(cc)
    } else if let Some(c) = category_badge_cmd::get_cmd(&matches) {
        Cmd::CategoryBadges(c)
    } else if let Some(r) = rbe::get_cmd(&matches) {
        Cmd::Rbe(r)
    } else if let Some(i) = info_cmd::get_cmd(&matches) {
        Cmd::Info(i)
    } else {
        Cmd::None
    };
    Ok((conf, cmd))
}

/// Builds the CLI user interface
fn cli() -> clap::Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        //.about("")
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child
        // subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command's help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(badge_cmd::subcommand_badge())
        .subcommand(category_badges_for_crate_cmd::subcommand_category_badges_for_crate())
        .subcommand(category_badge_cmd::subcommand_category_badge())
        .subcommand(rbe::subcommand_rbe())
        .subcommand(info_cmd::subcommand_info())
        .arg(config_flag::arg_verbose())
}

/// Checks that the command-line application definition is internally consistent.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the command-line application definition is internally consistent.
    #[test]
    fn test_app() {
        cli().debug_assert();
    }
}
