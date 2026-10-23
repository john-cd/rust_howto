//! Command-line interface of the crate indices tool.
//!
//! TODO Reviewed ??-??-2026.

use std::ffi::OsString;
use std::io;
use std::io::BufRead as _;
use std::io::IsTerminal as _;
use std::path::PathBuf;

mod commands;

/// The command that the end user selected
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    /// No command was selected.
    #[default]
    None,
    /// Generate the page of a category.
    CategoryPage(CmdArgs),
    /// Generate the page of a category from a local metadata snapshot.
    CategoryPageFromMetadata(CategoryPageFromMetadataCmdArgs),
    /// Generate the alphabetical index of crates.
    AlphabeticalCratePage(AlphabeticalCratePageCmdArgs),
    /// Generate the page of a crate.
    CratePage(CmdArgs),
    /// List the crates used in a directory.
    ListCrates(DirectoryCmdArgs),
    /// Update the reference definitions of a file.
    UpdateRefDefs(CmdArgs2),
    /// Expand the crate blocks in a directory.
    ExpandCrateBlocks(ExpandCrateBlocksCmdArgs),
}

/// Command arguments with a list of crate names,
/// for `category_page` and `alphabetical_page`
#[derive(Debug)]
pub(crate) struct CmdArgs {
    /// Names of the crates.
    pub crate_names: Vec<String>,
}

/// The command arguments for `update_refdefs`
#[derive(Debug)]
pub(crate) struct CmdArgs2 {
    /// Names of the crates.
    pub crate_names: Vec<String>,
    /// File that contains the reference definitions.
    pub filepathbuf: PathBuf,
}

/// Arguments for generating a category page from a local metadata snapshot.
#[derive(Debug)]
pub(crate) struct CategoryPageFromMetadataCmdArgs {
    /// Names of the crates.
    pub crate_names: Vec<String>,
    /// Path of the metadata snapshot.
    pub metadata_path: PathBuf,
    /// Root folder of the book.
    pub book_root: PathBuf,
}

/// Arguments for generating an alphabetical index with published recipe links.
#[derive(Debug)]
pub(crate) struct AlphabeticalCratePageCmdArgs {
    /// Names of the crates.
    pub crate_names: Vec<String>,
    /// Root folder of the book.
    pub book_root: PathBuf,
}

/// Command arguments with a directory
#[derive(Debug)]
pub(crate) struct DirectoryCmdArgs {
    /// Directory to process.
    pub dirpathbuf: PathBuf,
}

/// Command arguments for `expand_crate_blocks`
#[derive(Debug)]
pub(crate) struct ExpandCrateBlocksCmdArgs {
    /// Directory to process.
    pub dirpathbuf: PathBuf,
    /// File that contains the reference definitions, if any.
    pub refdefs_filepathbuf: Option<PathBuf>,
}

/// Parses the command line and returns the selected command.
pub(super) fn get_cmd() -> anyhow::Result<Cmd> {
    // Parse the specified command-line arguments, exiting on failure.
    let matches = commands::cli().get_matches_from(capture_stdin()?);
    if let Some(m) = matches.subcommand_matches("category_page") {
        Ok(Cmd::CategoryPage(CmdArgs {
            crate_names: get_crate_names(m),
        }))
    } else if let Some(m) = matches.subcommand_matches("category_page_from_metadata") {
        Ok(Cmd::CategoryPageFromMetadata(
            CategoryPageFromMetadataCmdArgs {
                crate_names: get_crate_names(m),
                metadata_path: get_metadata_path(m),
                book_root: get_book_root(m),
            },
        ))
    } else if let Some(m) = matches.subcommand_matches("alphabetical_page") {
        Ok(Cmd::AlphabeticalCratePage(AlphabeticalCratePageCmdArgs {
            crate_names: get_crate_names(m),
            book_root: get_book_root(m),
        }))
    } else if let Some(m) = matches.subcommand_matches("crate_page") {
        Ok(Cmd::CratePage(CmdArgs {
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
    } else if let Some(m) = matches.subcommand_matches("expand_crate_blocks") {
        Ok(Cmd::ExpandCrateBlocks(ExpandCrateBlocksCmdArgs {
            dirpathbuf: get_dir_path(m),
            refdefs_filepathbuf: get_optional_file_path(m),
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
        .map(std::convert::Into::into)
        .collect::<Vec<String>>()
}

/// Get a file path from the command line arguments
fn get_file_path(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("file_path")
        .expect("file_path has a default value")
        .clone()
}

/// Get the required category metadata path.
fn get_metadata_path(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("metadata")
        .expect("metadata is required")
        .clone()
}

/// Get the book root used to find published recipe links.
fn get_book_root(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("book_root")
        .expect("book_root has a default value")
        .clone()
}

/// Get a dir path from the command line arguments
fn get_dir_path(m: &clap::ArgMatches) -> PathBuf {
    m.get_one::<PathBuf>("directory")
        .expect("directory has a default value")
        .clone()
}

/// Get an optional file path from the command line arguments.
/// Returns `None` if the argument was not provided.
fn get_optional_file_path(m: &clap::ArgMatches) -> Option<PathBuf> {
    m.get_one::<PathBuf>("file_path").cloned()
}
