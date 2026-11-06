//! CLI commands
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use clap_builder::builder::ValueHint;

/// Builds the CLI user interface
pub(super) fn cli() -> Command {
    clap::command!() // reads name, version, author, and description from `Cargo.toml`
        .help_expected(true) // Panic if help descriptions are omitted. This choice is propagated to all child
        // subcommands.
        .flatten_help(true) // Flatten subcommand help into the current command's help
        .version(clap::crate_version!()) // Sets the version for the short version (-V) and help messages.
        .subcommand(subcommand_category_page())
        .subcommand(subcommand_category_page_from_metadata())
        .subcommand(subcommand_alphabetical_page())
        .subcommand(subcommand_crate_page())
        .subcommand(subcommand_list_crates())
        .subcommand(subcommand_update_refdefs())
        .subcommand(subcommand_expand_crate_blocks())
}

/// Builds the `category_page` subcommand of the CLI user interface
fn subcommand_category_page() -> Command {
    Command::new("category_page")
        .visible_alias("c")
        .about("Returns the markdown for the category page, given a list of crates")
        .arg(arg_crate_name())
}

/// Builds the offline `category_page_from_metadata` subcommand.
fn subcommand_category_page_from_metadata() -> Command {
    Command::new("category_page_from_metadata")
        .about("Returns the category page from metadata and published recipe links")
        .arg(arg_metadata_file())
        .arg(arg_book_root())
        .arg(arg_crate_name())
}

/// Builds the `alphabetical_page` subcommand of the CLI user interface
fn subcommand_alphabetical_page() -> Command {
    Command::new("alphabetical_page")
        .visible_alias("a")
        .about("Returns the alphabetical page with links to published recipes")
        .arg(arg_book_root())
        .arg(arg_crate_name())
}

/// Builds the `crate_page` subcommand of the CLI user interface
fn subcommand_crate_page() -> Command {
    Command::new("crate_page")
        .visible_alias("p")
        .about("Returns the markdown for individual crate sections in crates_and_examples.md, given a list of crates")
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
        .arg(arg_file_path_with_default())
}

/// Builds the `expand_crate_blocks` subcommand of the CLI user interface
fn subcommand_expand_crate_blocks() -> Command {
    Command::new("expand_crate_blocks")
        .visible_alias("e")
        .about(
            "Walk a directory of Markdown files, expand {{#crate <name>}} directives into full \
             crate badge blocks, and optionally merge the collected reference definitions into \
             a refdefs file",
        )
        .arg(arg_directory())
        .arg(arg_optional_file_path())
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

/// Builds the `file_path` argument with a default value (used by `update_refdefs`).
fn arg_file_path_with_default() -> clap::Arg {
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

/// Builds the required metadata file argument.
fn arg_metadata_file() -> clap::Arg {
    Arg::new("metadata")
        .long("metadata")
        .required(true)
        .action(clap::ArgAction::Set)
        .value_name("FILE")
        .value_parser(clap::value_parser!(PathBuf))
        .value_hint(ValueHint::FilePath)
        .help("TOML category metadata snapshot")
}

/// Builds the book root argument used to discover published recipe links.
fn arg_book_root() -> clap::Arg {
    Arg::new("book_root")
        .long("book-root")
        .required(false)
        .action(clap::ArgAction::Set)
        .value_name("DIR_PATH")
        .value_parser(clap::value_parser!(PathBuf))
        .value_hint(ValueHint::DirPath)
        .default_value(".")
        .help("Book root containing src/indices/examples_index.md")
}

/// Builds the `file_path` argument without a default value (used by `expand_crate_blocks`).
/// The refdefs file is optional: when not provided, reference definitions are not merged.
fn arg_optional_file_path() -> clap::Arg {
    Arg::new("file_path")
        .long("file")
        .short('f')
        .required(false)
        .action(clap::ArgAction::Set)
        .value_name("FILE")
        .value_parser(clap::value_parser!(PathBuf))
        .value_hint(ValueHint::FilePath)
        .help("Optional path to a refdefs file where reference definitions will be merged")
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the clap command configuration contains no internal errors.
    #[test]
    fn verify_cli() {
        cli().debug_assert();
    }

    /// Verifies that `--book-root` defaults to `.` when omitted in subcommands.
    #[test]
    fn test_subcommand_defaults() {
        let matches = cli()
            .try_get_matches_from(["crate-indices", "alphabetical_page", "tokio"])
            .expect("arguments should parse");
        let sub_matches = matches
            .subcommand_matches("alphabetical_page")
            .expect("subcommand should match");
        assert_eq!(
            sub_matches.get_one::<PathBuf>("book_root"),
            Some(&PathBuf::from("."))
        );

        let matches_list = cli()
            .try_get_matches_from(["crate-indices", "list_crates"])
            .expect("arguments should parse");
        let sub_list = matches_list
            .subcommand_matches("list_crates")
            .expect("subcommand should match");
        assert_eq!(
            sub_list.get_one::<PathBuf>("directory"),
            Some(&PathBuf::from("."))
        );

        let matches_update = cli()
            .try_get_matches_from(["crate-indices", "update_refdefs", "tokio"])
            .expect("arguments should parse");
        let sub_update = matches_update
            .subcommand_matches("update_refdefs")
            .expect("subcommand should match");
        assert_eq!(
            sub_update.get_one::<PathBuf>("file_path"),
            Some(&PathBuf::from("./src/refs/crate-refs.md"))
        );
    }

    /// Verifies custom values for `--book-root`, `--file`, and `--directory`.
    #[test]
    fn test_custom_arguments() {
        let matches_alpha = cli()
            .try_get_matches_from([
                "crate-indices",
                "alphabetical_page",
                "--book-root",
                "my_book",
                "serde",
            ])
            .expect("arguments should parse");
        let sub_alpha = matches_alpha
            .subcommand_matches("alphabetical_page")
            .expect("subcommand should match");
        assert_eq!(
            sub_alpha.get_one::<PathBuf>("book_root"),
            Some(&PathBuf::from("my_book"))
        );

        let matches_update = cli()
            .try_get_matches_from([
                "crate-indices",
                "update_refdefs",
                "-f",
                "custom/refs.md",
                "tokio",
            ])
            .expect("arguments should parse");
        let sub_update = matches_update
            .subcommand_matches("update_refdefs")
            .expect("subcommand should match");
        assert_eq!(
            sub_update.get_one::<PathBuf>("file_path"),
            Some(&PathBuf::from("custom/refs.md"))
        );

        let matches_list = cli()
            .try_get_matches_from(["crate-indices", "list_crates", "-d", "custom/dir"])
            .expect("arguments should parse");
        let sub_list = matches_list
            .subcommand_matches("list_crates")
            .expect("subcommand should match");
        assert_eq!(
            sub_list.get_one::<PathBuf>("directory"),
            Some(&PathBuf::from("custom/dir"))
        );
    }

    /// Verifies all single-letter visible aliases (`c`, `a`, `p`, `l`, `u`, `e`).
    #[test]
    fn test_subcommand_aliases() {
        let matches_c = cli()
            .try_get_matches_from(["crate-indices", "c", "serde"])
            .expect("alias 'c' should parse");
        assert_eq!(matches_c.subcommand_name(), Some("category_page"));

        let matches_a = cli()
            .try_get_matches_from(["crate-indices", "a", "serde"])
            .expect("alias 'a' should parse");
        assert_eq!(matches_a.subcommand_name(), Some("alphabetical_page"));

        let matches_p = cli()
            .try_get_matches_from(["crate-indices", "p", "serde"])
            .expect("alias 'p' should parse");
        assert_eq!(matches_p.subcommand_name(), Some("crate_page"));

        let matches_l = cli()
            .try_get_matches_from(["crate-indices", "l"])
            .expect("alias 'l' should parse");
        assert_eq!(matches_l.subcommand_name(), Some("list_crates"));

        let matches_u = cli()
            .try_get_matches_from(["crate-indices", "u", "serde"])
            .expect("alias 'u' should parse");
        assert_eq!(matches_u.subcommand_name(), Some("update_refdefs"));

        let matches_e = cli()
            .try_get_matches_from(["crate-indices", "e"])
            .expect("alias 'e' should parse");
        assert_eq!(matches_e.subcommand_name(), Some("expand_crate_blocks"));
    }

    /// Verifies error handling when required arguments are missing or invalid combinations are
    /// given.
    #[test]
    fn test_invalid_argument_errors() {
        // Missing required crate name in category_page.
        let err = cli()
            .try_get_matches_from(["crate-indices", "category_page"])
            .expect_err("category_page without crate name should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);

        // Missing required crate name in crate_page.
        let err = cli()
            .try_get_matches_from(["crate-indices", "crate_page"])
            .expect_err("crate_page without crate name should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);

        // Missing required crate name in update_refdefs.
        let err = cli()
            .try_get_matches_from(["crate-indices", "update_refdefs"])
            .expect_err("update_refdefs without crate name should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);

        // Missing required metadata option in category_page_from_metadata.
        let err = cli()
            .try_get_matches_from(["crate-indices", "category_page_from_metadata", "serde"])
            .expect_err("category_page_from_metadata without metadata should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);

        // Unknown subcommand.
        let err = cli()
            .try_get_matches_from(["crate-indices", "nonexistent"])
            .expect_err("unknown subcommand should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::InvalidSubcommand);

        // Unexpected flag in category_page.
        let err = cli()
            .try_get_matches_from(["crate-indices", "category_page", "--unknown-flag", "serde"])
            .expect_err("unexpected flag should fail");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }
}
