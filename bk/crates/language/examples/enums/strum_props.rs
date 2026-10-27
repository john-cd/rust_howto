#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Attach arbitrary constant properties to variants with `EnumProperty`.

use strum::EnumIter;
use strum::EnumProperty;
use strum::IntoEnumIterator;

/// Editor commands with an icon and a keyboard shortcut each.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumProperty)]
enum Command {
    #[strum(props(icon = "save", shortcut = "Ctrl+S", confirm = false))]
    Save,
    #[strum(props(
        icon = "trash",
        shortcut = "Ctrl+D",
        confirm = true,
        order = 9
    ))]
    Delete,
    // A variant without properties.
    Noop,
}

/// Reads the properties.
fn main() {
    assert_eq!(Command::Save.get_str("icon"), Some("save"));
    assert_eq!(Command::Delete.get_str("shortcut"), Some("Ctrl+D"));
    // Missing properties are `None`.
    assert_eq!(Command::Noop.get_str("icon"), None);
    assert_eq!(Command::Save.get_str("tooltip"), None);

    // Integer and boolean literals are read with `get_int` and `get_bool`.
    assert_eq!(Command::Delete.get_int("order"), Some(9));
    assert_eq!(Command::Delete.get_bool("confirm"), Some(true));
    assert_eq!(Command::Save.get_bool("confirm"), Some(false));

    for command in Command::iter() {
        println!("{command:?}: {:?}", command.get_str("shortcut"));
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that string, integer, and boolean properties are accessible.
    /// Runs the strum props example through its entry point.
    #[test]
    fn test_strum_props_example_entry_point() {
        main();
    }
}
