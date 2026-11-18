//! A minimal command registry shared with linked extension crates.

use std::collections::BTreeMap;

/// A statically linked command declaration.
pub struct Command {
    /// Stable command name.
    pub name: &'static str,
    /// Handler that returns a small demonstration result.
    pub run: fn() -> &'static str,
}

inventory::collect!(Command);

/// Returns the help text for the registry's built-in help command.
fn help() -> &'static str {
    "help text"
}
inventory::submit! { Command { name: "help", run: help } }

/// Commands indexed by name for deterministic lookup and output.
pub type Commands = BTreeMap<&'static str, fn() -> &'static str>;

/// Sort entries and reject duplicate names before dispatch.
pub fn registry<'a>(
    commands: impl IntoIterator<Item = &'a Command>,
) -> Result<Commands, &'static str> {
    let mut sorted = BTreeMap::new();
    for command in commands {
        if sorted.insert(command.name, command.run).is_some() {
            return Err(command.name);
        }
    }
    Ok(sorted)
}
