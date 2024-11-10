//! The `cli` module is the command line argument parser for the
//! application

use std::env;

use anyhow::Result;

mod badge_cmd;
mod category_badge_cmd;
mod category_badges_for_crate_cmd;
mod config_flag;
mod crate_name_arg;
mod info_cmd;
mod rbe;

use crate_name_arg::*;

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
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command’s help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(badge_cmd::subcommand_badge())
        .subcommand(category_badges_for_crate_cmd::subcommand_category_badges_for_crate())
        .subcommand(category_badge_cmd::subcommand_category_badge())
        .subcommand(rbe::subcommand_rbe())
        .subcommand(info_cmd::subcommand_info())
        .arg(config_flag::arg_verbose())
}

#[test]
fn test_app() {
    cli().debug_assert();
}
