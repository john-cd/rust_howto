//! The configuration and possible commands of the application
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

/// The configuration of the application
pub(crate) struct Config {
    /// If true, the application will be more verbose
    pub verbose: bool,
}

/// The command that the end user selected.
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    /// No command was selected.
    #[default]
    None,

    /// Create the badges, and optionally the reference definitions, of crates.
    Badges(
        /// Arguments for creating badges and optional reference definitions.
        CmdArgs2,
    ),

    /// Create a Rust By Example badge.
    Rbe(
        /// Arguments for creating the Rust By Example badge.
        CmdArgs,
    ),

    /// Create the category badges of the crates.
    CategoriesForCrateBadge(
        /// Arguments for building a crate badge from its categories.
        CmdArgs,
    ),

    /// Create the badges of the categories that match the given words.
    CategoryBadges(
        /// Arguments for filtering and creating category badges.
        CmdArgs,
    ),

    /// Get information about a crate.
    Info(
        /// Arguments selecting the crate whose metadata is displayed.
        CmdArgs,
    ),
}

/// The command arguments for multiple commands.
///
/// Contains the list of strings passed as arguments.
#[derive(Debug)]
pub(crate) struct CmdArgs {
    /// The arguments.
    pub args: Vec<String>,
}

/// The command arguments for the `Badges` command.
#[derive(Debug)]
pub(crate) struct CmdArgs2 {
    /// The arguments.
    pub args: Vec<String>,

    /// The file to update, if any.
    pub file: Option<PathBuf>,
}
