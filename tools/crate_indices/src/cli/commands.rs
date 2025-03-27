//! CLI commands
use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use clap_builder::builder::ValueHint;

/// Builds the CLI user interface
pub(super) fn cli() -> Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command's help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(subcommand_category_page())
        .subcommand(subcommand_alphabetical_page())
        .subcommand(subcommand_list_crates())
        .subcommand(subcommand_update_refdefs())
}

/// Builds the `category_page` subcommand of the CLI user interface
fn subcommand_category_page() -> Command {
    Command::new("category_page")
        .visible_alias("c")
        .about("Returns the markdown for the category page, given a list of crates")
        .arg(arg_crate_name())
}

/// Builds the `alphabetical_page` subcommand of the CLI user interface
fn subcommand_alphabetical_page() -> Command {
    Command::new("alphabetical_page")
        .visible_alias("a")
        .about("Returns the markdown for the alphabetical page, given a list of crates")
        .arg(arg_crate_name())
}

/// Builds the `list_crates` subcommand of the CLI user interface
fn subcommand_list_crates() -> Command {
    Command::new("list_crates")
        .visible_alias("l")
        .about("Returns the list of crates used in the book")
        .arg(arg_directory())
}

/// Builds the `update_refdefs` subcommand of the CLI user interface
fn subcommand_update_refdefs() -> Command {
    Command::new("update_refdefs")
        .visible_alias("u")
        .about("Update the book's master list of reference definitions, reading the list of crates to include from the file path passed in argument")
        .arg(arg_crate_name())
        .arg(arg_file_path())
}

// Arguments -------------------------------------------------

/// Builds the `crate_name` argument of the CLI user interface
fn arg_crate_name() -> clap::Arg {
    Arg::new("crate_name")
        .required(true)
        .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
        .action(clap::ArgAction::Append)
        .help("Crate name(s)")
}

/// Builds the `file_path` argument of the CLI user interface
fn arg_file_path() -> clap::Arg {
    Arg::new("file_path")
        .long("file")
        .short('f')
        .required(false)
        .action(clap::ArgAction::Set)
        .value_name("FILE") // placeholder for the argument's value in the help message / usage.
        .value_parser(clap::value_parser!(PathBuf))
        .value_hint(ValueHint::FilePath)
        .default_value("./src/refs/crate-refs.md")
        .help("File path")
}

/// Builds the `directory` argument of the CLI user interface
fn arg_directory() -> clap::Arg {
    Arg::new("directory")
        .long("directory")
        .short('d')
        .required(false)
        .action(clap::ArgAction::Set)
        .value_name("DIR_PATH") // placeholder for the argument's value in the help message / usage.
        .value_parser(clap::value_parser!(PathBuf))
        .value_hint(ValueHint::DirPath)
        .default_value(".")
        .help("Path of the directory to search for Cargo.toml files")
}
