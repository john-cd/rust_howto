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
    Ok(cmd_from_matches(&matches))
}

/// Resolves parsed command-line matches into the selected command.
fn cmd_from_matches(matches: &clap::ArgMatches) -> Cmd {
    if let Some(m) = matches.subcommand_matches("category_page") {
        Cmd::CategoryPage(CmdArgs {
            crate_names: get_crate_names(m),
        })
    } else if let Some(m) = matches.subcommand_matches("category_page_from_metadata") {
        Cmd::CategoryPageFromMetadata(CategoryPageFromMetadataCmdArgs {
            crate_names: get_crate_names(m),
            metadata_path: get_metadata_path(m),
            book_root: get_book_root(m),
        })
    } else if let Some(m) = matches.subcommand_matches("alphabetical_page") {
        Cmd::AlphabeticalCratePage(AlphabeticalCratePageCmdArgs {
            crate_names: get_crate_names(m),
            book_root: get_book_root(m),
        })
    } else if let Some(m) = matches.subcommand_matches("crate_page") {
        Cmd::CratePage(CmdArgs {
            crate_names: get_crate_names(m),
        })
    } else if let Some(m) = matches.subcommand_matches("update_refdefs") {
        Cmd::UpdateRefDefs(CmdArgs2 {
            crate_names: get_crate_names(m),
            filepathbuf: get_file_path(m),
        })
    } else if let Some(m) = matches.subcommand_matches("list_crates") {
        Cmd::ListCrates(DirectoryCmdArgs {
            dirpathbuf: get_dir_path(m),
        })
    } else if let Some(m) = matches.subcommand_matches("expand_crate_blocks") {
        Cmd::ExpandCrateBlocks(ExpandCrateBlocksCmdArgs {
            dirpathbuf: get_dir_path(m),
            refdefs_filepathbuf: get_optional_file_path(m),
        })
    } else {
        Cmd::None
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Cmd;
    use super::cmd_from_matches;
    use super::commands;

    /// Parses `args` (without the program name) into a command.
    fn parse(args: &[&str]) -> Result<Cmd, clap::Error> {
        let argv = std::iter::once("crate-indices").chain(args.iter().copied());
        commands::cli()
            .try_get_matches_from(argv)
            .map(|m| cmd_from_matches(&m))
    }

    #[test]
    fn no_subcommand_selects_none() {
        assert!(matches!(parse(&[]).expect("parse succeeds"), Cmd::None));
    }

    #[test]
    fn category_page_collects_crate_names() {
        let Cmd::CategoryPage(a) =
            parse(&["category_page", "serde", "rand"]).expect("parse succeeds")
        else {
            panic!("wrong command");
        };
        assert_eq!(a.crate_names, ["serde", "rand"]);
    }

    #[test]
    fn aliases_resolve_to_same_commands() {
        assert!(matches!(
            parse(&["c", "serde"]).expect("parse succeeds"),
            Cmd::CategoryPage(_)
        ));
        assert!(matches!(
            parse(&["a", "serde"]).expect("parse succeeds"),
            Cmd::AlphabeticalCratePage(_)
        ));
        assert!(matches!(
            parse(&["p", "serde"]).expect("parse succeeds"),
            Cmd::CratePage(_)
        ));
        assert!(matches!(
            parse(&["l"]).expect("parse succeeds"),
            Cmd::ListCrates(_)
        ));
        assert!(matches!(
            parse(&["u", "serde"]).expect("parse succeeds"),
            Cmd::UpdateRefDefs(_)
        ));
        assert!(matches!(
            parse(&["e"]).expect("parse succeeds"),
            Cmd::ExpandCrateBlocks(_)
        ));
    }

    #[test]
    fn list_crates_defaults_to_current_directory() {
        let Cmd::ListCrates(a) = parse(&["list_crates"]).expect("parse succeeds") else {
            panic!("wrong command");
        };
        assert_eq!(a.dirpathbuf, Path::new("."));
        let Cmd::ListCrates(a) = parse(&["l", "-d", "x"]).expect("parse succeeds") else {
            panic!("wrong command");
        };
        assert_eq!(a.dirpathbuf, Path::new("x"));
    }

    #[test]
    fn update_refdefs_default_file() {
        let Cmd::UpdateRefDefs(a) = parse(&["update_refdefs", "serde"]).expect("parse succeeds")
        else {
            panic!("wrong command");
        };
        assert_eq!(a.filepathbuf, Path::new("./src/refs/crate-refs.md"));
        let Cmd::UpdateRefDefs(a) =
            parse(&["u", "serde", "--file", "f.md"]).expect("parse succeeds")
        else {
            panic!("wrong command");
        };
        assert_eq!(a.filepathbuf, Path::new("f.md"));
    }

    #[test]
    fn expand_crate_blocks_file_is_optional() {
        let Cmd::ExpandCrateBlocks(a) = parse(&["expand_crate_blocks"]).expect("parse succeeds")
        else {
            panic!("wrong command");
        };
        assert_eq!(a.dirpathbuf, Path::new("."));
        assert!(a.refdefs_filepathbuf.is_none());
        let Cmd::ExpandCrateBlocks(a) =
            parse(&["e", "-d", "d", "-f", "r.md"]).expect("parse succeeds")
        else {
            panic!("wrong command");
        };
        assert_eq!(a.refdefs_filepathbuf.as_deref(), Some(Path::new("r.md")));
    }

    #[test]
    fn metadata_and_alphabetical_use_book_root_default() {
        let Cmd::CategoryPageFromMetadata(a) = parse(&[
            "category_page_from_metadata",
            "--metadata",
            "m.toml",
            "serde",
        ])
        .expect("parse succeeds") else {
            panic!("wrong command");
        };
        assert_eq!(a.metadata_path, Path::new("m.toml"));
        assert_eq!(a.book_root, Path::new("."));
        let Cmd::AlphabeticalCratePage(a) =
            parse(&["alphabetical_page", "--book-root", "bk", "serde"]).expect("parse succeeds")
        else {
            panic!("wrong command");
        };
        assert_eq!(a.book_root, Path::new("bk"));
    }

    #[test]
    fn invalid_arguments_are_rejected() {
        // Missing required crate name.
        parse(&["category_page"]).expect_err("parse fails");
        // Missing required --metadata.
        parse(&["category_page_from_metadata", "serde"]).expect_err("parse fails");
        // Unknown subcommand and unknown flag.
        parse(&["bogus"]).expect_err("parse fails");
        parse(&["list_crates", "--nope"]).expect_err("parse fails");
        // Flag without a value.
        parse(&["list_crates", "-d"]).expect_err("parse fails");
    }
}
