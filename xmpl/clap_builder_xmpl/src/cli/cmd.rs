//! The commands of the application.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;

/// Internal representation of a command passed as an argument to the
/// command-line.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum Cmd {
    /// No (implicit or explicit) command was given.
    #[default]
    None,

    /// Explicit `open` command or implicit open (given a list of
    /// files).
    Open(
        /// Filesystem paths opened by the command.
        Vec<PathBuf>,
    ),

    /// `query` command, storing query words.
    Query(
        /// Query terms searched by the command.
        Vec<String>,
    ),

    /// A hidden command that does nothing.
    Test,
}
