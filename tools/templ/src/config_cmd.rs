//! The configuration and possible commands of the application
use std::path::PathBuf;

/// The configuration of the application
pub(crate) struct Config {
    /// If true, the application will be more verbose
    pub verbose: bool,
}

/// The command that the end user selected.
#[derive(Default, Debug)]
pub(crate) enum Cmd {
    #[default]
    None,
    Badges(CmdArgs2),
    Rbe(CmdArgs),
    CategoriesForCrateBadge(CmdArgs),
    CategoryBadges(CmdArgs),
    Info(CmdArgs),
}

/// The command arguments for multiple commands.
///
/// Contains the list of strings passed as arguments.
#[derive(Debug)]
pub(crate) struct CmdArgs {
    pub args: Vec<String>,
}

/// The command arguments for the `Badges` command.
#[derive(Debug)]
pub(crate) struct CmdArgs2 {
    pub args: Vec<String>,
    pub file: Option<PathBuf>,
}
