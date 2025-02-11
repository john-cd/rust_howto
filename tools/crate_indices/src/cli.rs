use std::ffi::OsString;
use std::io;
use std::io::BufRead;
use std::io::IsTerminal;
use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use clap_builder::builder::ValueHint;

/// The command that the end user selected
#[allow(dead_code)]
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    #[default]
    None,
    CategoryPage(CmdArgs),
    AlphabeticalCratePage(CmdArgs),
    ListCrates,
    UpdateRefDefs(CmdArgs2),
}

// The command arguments
#[derive(Debug)]
pub(crate) struct CmdArgs {
    pub crate_names: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct CmdArgs2 {
    pub crate_names: Vec<String>,
    pub filepathbuf: PathBuf,
}

pub(super) fn get_cmd() -> anyhow::Result<Cmd> {
    // Parse the specified command-line arguments, exiting on failure.
    let matches = cli().get_matches_from(capture_stdin()?);
    if let Some(m) = matches.subcommand_matches("category_page") {
        Ok(Cmd::CategoryPage(CmdArgs {
            crate_names: get_crate_names(m),
        }))
    } else if let Some(m) = matches.subcommand_matches("alphabetical_page") {
        Ok(Cmd::AlphabeticalCratePage(CmdArgs {
            crate_names: get_crate_names(m),
        }))
    } else if let Some(m) = matches.subcommand_matches("update_refdefs") {
        Ok(Cmd::UpdateRefDefs(CmdArgs2 {
            crate_names: get_crate_names(m),
            filepathbuf: get_file_path(m),
        }))
    } else if let Some(_m) = matches.subcommand_matches("list_crates") {
        Ok(Cmd::ListCrates)
    } else {
        Ok(Cmd::None)
    }
}

/// Read from Stdin e.g. if called with `cat file.txt | my_app `
fn capture_stdin() -> anyhow::Result<Vec<OsString>> {
    let mut args: Vec<OsString> = std::env::args_os().collect();
    let stdin = io::stdin();
    // Are you or are you not a tty?
    if !stdin.is_terminal() {
        let handle = stdin.lock();
        for l in handle.lines() {
            args.push(l?.into());
        }
    }
    Ok(args)
}

fn get_crate_names(m: &clap::ArgMatches) -> Vec<String> {
    m.get_many::<String>("crate_name")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect::<Vec<String>>()
}

fn get_file_path(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("file_path")
        .expect("file_path has a default value")
        .to_path_buf()
}

/// Builds the CLI user interface
fn cli() -> Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        //.about("")
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

fn subcommand_alphabetical_page() -> Command {
    Command::new("alphabetical_page")
        .visible_alias("a")
        .about("Returns the markdown for the alphabetical page, given a list of crates")
        .arg(arg_crate_name())
}

fn subcommand_list_crates() -> Command {
    Command::new("list_crates")
        .visible_alias("l")
        .about("Returns the list of crates used in the book")
}

fn subcommand_update_refdefs() -> Command {
    Command::new("update_refdefs")
        .visible_alias("u")
        .about("Update the book's master list of reference definitions, reading the list of crates to include from the file path passed in argument")
        .arg(arg_crate_name())
        .arg(arg_file_path())
}

fn arg_crate_name() -> clap::Arg {
    Arg::new("crate_name")
        .required(true)
        .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
        .action(clap::ArgAction::Append)
        .help("Enter the crate name(s)")
}

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
        .help("Enter the file path")
}
