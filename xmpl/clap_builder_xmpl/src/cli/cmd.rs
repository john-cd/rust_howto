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
    Open(Vec<PathBuf>),
    /// `query` command, storing query words.
    Query(Vec<String>),
    Test,
}
