use std::path::PathBuf;
use std::vec;

use clap::arg;
use clap::value_parser;
use clap::Arg;
use clap::Command;

pub(crate) struct Config {
    pub badge: Option<BadgeConfig>,
    pub verbose: bool,
}

pub(crate) struct BadgeConfig {
    pub names: Vec<String>,
}

pub(crate) fn get_args() -> Config {
    let matches = cli().get_matches(); // Parse [env::args_os], exiting on failure.
                                       // Check for the existence of subcommands
    let mut badge = None;
    if let Some(matches) = matches.subcommand_matches("badge") {
        // "$ myapp badge" was run
        let names = matches
            .get_many::<String>("crate_name")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        badge = Some(BadgeConfig { names });
    }
    Config {
        verbose: matches.get_flag("verbose"),
        badge,
    }
}

fn cli() -> Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        //.about("")
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command’s help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(badge())
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Use verbose output"),
        )
}

fn badge() -> Command {
    Command::new("badge")
        .about("Create the markdown for a crate badge")
        .arg(
            Arg::new("crate_name")
                .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
                //.value_parser(value_parser!(...))
                .action(clap::ArgAction::Append)
                .help("Enter the crate name(s)"),
        )
}
