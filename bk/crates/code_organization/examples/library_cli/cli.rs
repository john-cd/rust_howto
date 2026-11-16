//! Reads an optional name.

use code_organization::default_name;

/// Uses the provided name or the library default.
pub(super) fn name(argument: Option<String>) -> String {
    argument.unwrap_or_else(|| default_name().to_owned())
}
