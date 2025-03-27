use std::ffi::OsString;
use std::io;
use std::io::BufRead;
use std::io::IsTerminal;
use std::path::PathBuf;

mod commands;

/// The command that the end user selected
#[allow(dead_code)]
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    #[default]
    None,
    CategoryPage(CmdArgs),
    AlphabeticalCratePage(CmdArgs),
    ListCrates(DirectoryCmdArgs),
    UpdateRefDefs(CmdArgs2),
}

/// Command arguments with a list of crate names,
/// for `category_page` and `alphabetical_page`
#[derive(Debug)]
pub(crate) struct CmdArgs {
    pub crate_names: Vec<String>,
}

/// The command arguments for `update_refdefs`
#[derive(Debug)]
pub(crate) struct CmdArgs2 {
    pub crate_names: Vec<String>,
    pub filepathbuf: PathBuf,
}

/// Command arguments with a directory
#[derive(Debug)]
pub(crate) struct DirectoryCmdArgs {
    pub dirpathbuf: PathBuf,
}

pub(super) fn get_cmd() -> anyhow::Result<Cmd> {
    // Parse the specified command-line arguments, exiting on failure.
    let matches = commands::cli().get_matches_from(capture_stdin()?);
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
    } else if let Some(m) = matches.subcommand_matches("list_crates") {
        Ok(Cmd::ListCrates(DirectoryCmdArgs {
            dirpathbuf: get_dir_path(m),
        }))
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

/// Get the list of crate names from the command line arguments
fn get_crate_names(m: &clap::ArgMatches) -> Vec<String> {
    m.get_many::<String>("crate_name")
        .unwrap_or_default()
        .map(|v| v.into())
        .collect::<Vec<String>>()
}

/// Get a file path from the command line arguments
fn get_file_path(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("file_path")
        .expect("file_path has a default value")
        .to_path_buf()
}

/// Get a dir path from the command line arguments
fn get_dir_path(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("directory")
        .expect("directory has a default value")
        .to_path_buf()
}
