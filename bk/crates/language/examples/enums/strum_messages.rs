#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Labels and tooltips for a settings menu with `strum`'s `EnumMessage`, and
//! the variant list with `IntoEnumIterator`, `VariantNames` and `EnumCount`.

use strum::AsRefStr;
use strum::Display;
use strum::EnumCount;
use strum::EnumIter;
use strum::EnumMessage;
use strum::IntoEnumIterator;
use strum::VariantNames;

/// How a boolean cell is displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumIter, EnumMessage, EnumCount, VariantNames, AsRefStr)]
enum BoolDisplayMode {
    /// `message` is a short label; `detailed_message` is a tooltip.
    #[strum(message = "True / False", detailed_message = "Shows the words True and False")]
    Words,
    #[strum(message = "1 / 0", detailed_message = "Shows 1 for true and 0 for false")]
    Digits,
    #[strum(message = "Checkbox", detailed_message = "Shows a checkbox")]
    Checkbox,
}

/// Builds the menu entries from the enum itself.
fn menu() -> Vec<(String, &'static str, &'static str)> {
    BoolDisplayMode::iter()
        .map(|mode| {
            (
                mode.to_string(),
                mode.get_message().unwrap_or_default(),
                mode.get_detailed_message().unwrap_or_default(),
            )
        })
        .collect()
}

/// Prints the menu.
fn main() {
    for (name, label, tooltip) in menu() {
        println!("{name}: {label} ({tooltip})");
    }

    // The number of variants and their names are available as constants.
    assert_eq!(BoolDisplayMode::COUNT, 3);
    assert_eq!(BoolDisplayMode::VARIANTS, ["Words", "Digits", "Checkbox"]);

    // `AsRefStr` borrows a `&'static str`; `Display` goes through a formatter
    // (and `to_string` allocates).
    let name: &str = BoolDisplayMode::Digits.as_ref();
    assert_eq!(name, "Digits");
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }

    #[test]
    fn test_menu() {
        let entries = menu();
        assert_eq!(entries.len(), BoolDisplayMode::COUNT);
        assert_eq!(entries[0].1, "True / False");
        assert_eq!(entries[2].2, "Shows a checkbox");
    }
}
