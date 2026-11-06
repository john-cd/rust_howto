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
    Ok(parse_matches(&matches))
}

/// Parses command-line argument matches into the corresponding [`Cmd`].
fn parse_matches(matches: &clap::ArgMatches) -> Cmd {
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

#[cfg(test)]
/// Parses the command-line arguments from an iterator and returns the selected [`Cmd`].
fn try_get_cmd_from<I, T>(args: I) -> Result<Cmd, clap::Error>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let matches = commands::cli().try_get_matches_from(args)?;
    Ok(parse_matches(&matches))
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
    use super::*;

    /// Verifies parsing when no subcommand is provided.
    #[test]
    fn test_try_get_cmd_none() {
        let cmd = try_get_cmd_from(["crate-indices"]).expect("empty args should parse");
        assert!(matches!(cmd, Cmd::None));
    }

    /// Verifies resolution of `category_page` and alias `c`.
    #[test]
    fn test_try_get_cmd_category_page() {
        let cmd = try_get_cmd_from(["crate-indices", "category_page", "serde", "tokio"])
            .expect("should parse");
        match cmd {
            Cmd::CategoryPage(args) => {
                assert_eq!(args.crate_names, vec!["serde", "tokio"]);
            }
            _ => panic!("expected CategoryPage, got {cmd:?}"),
        }

        let cmd_alias = try_get_cmd_from(["crate-indices", "c", "axum"]).expect("should parse");
        match cmd_alias {
            Cmd::CategoryPage(args) => {
                assert_eq!(args.crate_names, vec!["axum"]);
            }
            _ => panic!("expected CategoryPage, got {cmd_alias:?}"),
        }
    }

    /// Verifies resolution of `category_page_from_metadata` with default and custom book-root.
    #[test]
    fn test_try_get_cmd_category_page_from_metadata() {
        let cmd = try_get_cmd_from([
            "crate-indices",
            "category_page_from_metadata",
            "--metadata",
            "meta.toml",
            "serde",
        ])
        .expect("should parse");
        match cmd {
            Cmd::CategoryPageFromMetadata(args) => {
                assert_eq!(args.crate_names, vec!["serde"]);
                assert_eq!(args.metadata_path, PathBuf::from("meta.toml"));
                assert_eq!(args.book_root, PathBuf::from("."));
            }
            _ => panic!("expected CategoryPageFromMetadata, got {cmd:?}"),
        }

        let cmd_custom = try_get_cmd_from([
            "crate-indices",
            "category_page_from_metadata",
            "--metadata",
            "meta.toml",
            "--book-root",
            "my_root",
            "serde",
            "tokio",
        ])
        .expect("should parse");
        match cmd_custom {
            Cmd::CategoryPageFromMetadata(args) => {
                assert_eq!(args.crate_names, vec!["serde", "tokio"]);
                assert_eq!(args.metadata_path, PathBuf::from("meta.toml"));
                assert_eq!(args.book_root, PathBuf::from("my_root"));
            }
            _ => panic!("expected CategoryPageFromMetadata, got {cmd_custom:?}"),
        }
    }

    /// Verifies resolution of `alphabetical_page` and alias `a`.
    #[test]
    fn test_try_get_cmd_alphabetical_page() {
        let cmd = try_get_cmd_from(["crate-indices", "alphabetical_page", "serde"])
            .expect("should parse");
        match cmd {
            Cmd::AlphabeticalCratePage(args) => {
                assert_eq!(args.crate_names, vec!["serde"]);
                assert_eq!(args.book_root, PathBuf::from("."));
            }
            _ => panic!("expected AlphabeticalCratePage, got {cmd:?}"),
        }

        let cmd_alias =
            try_get_cmd_from(["crate-indices", "a", "--book-root", "custom_root", "tokio"])
                .expect("should parse");
        match cmd_alias {
            Cmd::AlphabeticalCratePage(args) => {
                assert_eq!(args.crate_names, vec!["tokio"]);
                assert_eq!(args.book_root, PathBuf::from("custom_root"));
            }
            _ => panic!("expected AlphabeticalCratePage, got {cmd_alias:?}"),
        }
    }

    /// Verifies resolution of `crate_page` and alias `p`.
    #[test]
    fn test_try_get_cmd_crate_page() {
        let cmd = try_get_cmd_from(["crate-indices", "crate_page", "serde", "clap"])
            .expect("should parse");
        match cmd {
            Cmd::CratePage(args) => {
                assert_eq!(args.crate_names, vec!["serde", "clap"]);
            }
            _ => panic!("expected CratePage, got {cmd:?}"),
        }

        let cmd_alias = try_get_cmd_from(["crate-indices", "p", "tracing"]).expect("should parse");
        match cmd_alias {
            Cmd::CratePage(args) => {
                assert_eq!(args.crate_names, vec!["tracing"]);
            }
            _ => panic!("expected CratePage, got {cmd_alias:?}"),
        }
    }

    /// Verifies resolution of `list_crates` and alias `l`.
    #[test]
    fn test_try_get_cmd_list_crates() {
        let cmd = try_get_cmd_from(["crate-indices", "list_crates"]).expect("should parse");
        match cmd {
            Cmd::ListCrates(args) => {
                assert_eq!(args.dirpathbuf, PathBuf::from("."));
            }
            _ => panic!("expected ListCrates, got {cmd:?}"),
        }

        let cmd_alias =
            try_get_cmd_from(["crate-indices", "l", "-d", "some/path"]).expect("should parse");
        match cmd_alias {
            Cmd::ListCrates(args) => {
                assert_eq!(args.dirpathbuf, PathBuf::from("some/path"));
            }
            _ => panic!("expected ListCrates, got {cmd_alias:?}"),
        }
    }

    /// Verifies resolution of `update_refdefs` and alias `u`.
    #[test]
    fn test_try_get_cmd_update_refdefs() {
        let cmd =
            try_get_cmd_from(["crate-indices", "update_refdefs", "anyhow"]).expect("should parse");
        match cmd {
            Cmd::UpdateRefDefs(args) => {
                assert_eq!(args.crate_names, vec!["anyhow"]);
                assert_eq!(args.filepathbuf, PathBuf::from("./src/refs/crate-refs.md"));
            }
            _ => panic!("expected UpdateRefDefs, got {cmd:?}"),
        }

        let cmd_alias = try_get_cmd_from([
            "crate-indices",
            "u",
            "--file",
            "my_refs.md",
            "serde",
            "tokio",
        ])
        .expect("should parse");
        match cmd_alias {
            Cmd::UpdateRefDefs(args) => {
                assert_eq!(args.crate_names, vec!["serde", "tokio"]);
                assert_eq!(args.filepathbuf, PathBuf::from("my_refs.md"));
            }
            _ => panic!("expected UpdateRefDefs, got {cmd_alias:?}"),
        }
    }

    /// Verifies resolution of `expand_crate_blocks` and alias `e`.
    #[test]
    fn test_try_get_cmd_expand_crate_blocks() {
        let cmd = try_get_cmd_from(["crate-indices", "expand_crate_blocks"]).expect("should parse");
        match cmd {
            Cmd::ExpandCrateBlocks(args) => {
                assert_eq!(args.dirpathbuf, PathBuf::from("."));
                assert_eq!(args.refdefs_filepathbuf, None);
            }
            _ => panic!("expected ExpandCrateBlocks, got {cmd:?}"),
        }

        let cmd_alias =
            try_get_cmd_from(["crate-indices", "e", "--directory", "docs", "-f", "refs.md"])
                .expect("should parse");
        match cmd_alias {
            Cmd::ExpandCrateBlocks(args) => {
                assert_eq!(args.dirpathbuf, PathBuf::from("docs"));
                assert_eq!(args.refdefs_filepathbuf, Some(PathBuf::from("refs.md")));
            }
            _ => panic!("expected ExpandCrateBlocks, got {cmd_alias:?}"),
        }
    }

    /// Verifies argument extraction helper functions directly against `ArgMatches`.
    #[test]
    fn test_argument_extraction_helpers() {
        let matches = commands::cli()
            .try_get_matches_from([
                "crate-indices",
                "expand_crate_blocks",
                "-d",
                "target_dir",
                "-f",
                "target_file.md",
            ])
            .expect("should parse");
        let sub_matches = matches
            .subcommand_matches("expand_crate_blocks")
            .expect("subcommand matches");

        assert_eq!(get_dir_path(sub_matches), PathBuf::from("target_dir"));
        assert_eq!(
            get_optional_file_path(sub_matches),
            Some(PathBuf::from("target_file.md"))
        );

        let matches_update = commands::cli()
            .try_get_matches_from([
                "crate-indices",
                "update_refdefs",
                "-f",
                "ref_file.md",
                "c1",
                "c2",
            ])
            .expect("should parse");
        let sub_update = matches_update
            .subcommand_matches("update_refdefs")
            .expect("subcommand matches");
        assert_eq!(get_file_path(sub_update), PathBuf::from("ref_file.md"));
        assert_eq!(get_crate_names(sub_update), vec!["c1", "c2"]);

        let matches_meta = commands::cli()
            .try_get_matches_from([
                "crate-indices",
                "category_page_from_metadata",
                "--metadata",
                "meta.toml",
                "--book-root",
                "root_dir",
                "c3",
            ])
            .expect("should parse");
        let sub_meta = matches_meta
            .subcommand_matches("category_page_from_metadata")
            .expect("subcommand matches");
        assert_eq!(get_metadata_path(sub_meta), PathBuf::from("meta.toml"));
        assert_eq!(get_book_root(sub_meta), PathBuf::from("root_dir"));
    }
}
