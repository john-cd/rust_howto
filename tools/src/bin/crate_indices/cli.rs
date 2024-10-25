use std::ffi::OsString;
use std::io;
use std::io::BufRead;
use std::io::IsTerminal;

use clap::Arg;
use clap::Command;

/// The command that the end user selected
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    #[default]
    None,
    CategoryPage(CategoryPageCmdArgs),
}

// The command arguments
#[derive(Debug)]
pub(crate) struct CategoryPageCmdArgs {
    pub names: Vec<String>,
}

pub(super) fn get_cmd() -> anyhow::Result<Cmd> {
    // Parse the specified command-line arguments, exiting on failure.
    let matches = cli().get_matches_from(capture_stdin()?);
    if let Some(m) = matches.subcommand_matches("category_page") {
        let names = m
            .get_many::<String>("crate_name")
            .unwrap_or_default()
            .map(|v| v.into())
            .collect::<Vec<String>>();
        Ok(Cmd::CategoryPage(CategoryPageCmdArgs { names }))
    } else {
        Ok(Cmd::None)
    }
}

/// Builds the CLI user interface
fn cli() -> Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        //.about("")
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command’s help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(subcommand_category_page())
}

/// Builds the `category_page` subcommand of the CLI user interface
fn subcommand_category_page() -> Command {
    Command::new("category_page")
        .visible_alias("c")
        .about("Returns the markdown for the category page, given a list of crates")
        .arg(
            Arg::new("crate_name")
                .required(true)
                .value_name("CRATE_NAME") // placeholder for the argument's value in the help message / usage.
                .action(clap::ArgAction::Append)
                .help("Enter the crate name(s)"),
        )
}

/// Read from Stdin e.g. if called with `cat file.txt | my_app `
fn capture_stdin() -> anyhow::Result<Vec<OsString>> {
    let mut args: Vec<OsString> = std::env::args_os().map(|x| x).collect();
    let stdin = io::stdin();
    // Are you or are you not a tty?
    if !stdin.is_terminal() {
        let handle = stdin.lock();
        for l in handle.lines() {
            println!("{:?}", l);
            args.push(l?.into());
        }
    }
    println!("{:?}", args);
    Ok(args)
}
