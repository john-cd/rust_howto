// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Inspects a borrowed label and retains it as an owned string.

use std::borrow::Cow;

/// Reads the label through a string slice without consuming it.
fn label_width(label: &str) -> usize {
    label.chars().count()
}

/// Converts a label into a string that can outlive the input buffer.
fn retain_label(label: Cow<'_, str>) -> String {
    label.into_owned()
}

/// Demonstrates reading a borrowed label before taking ownership.
fn main() {
    let input = String::from("status: ready");
    let label = Cow::Borrowed(input.trim_start_matches("status: "));
    assert_eq!(label_width(&label), 5);

    let saved = retain_label(label);
    drop(input);
    assert_eq!(saved, "ready");
}
// ANCHOR_END: example

/// Runs the label example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the saved label outlives its source buffer.
    #[test]
    fn retains_borrowed_label() {
        main();
    }

    /// Verifies that retaining an owned label keeps its allocation.
    #[test]
    fn moves_owned_label() {
        let label = String::from("ready");
        let address = label.as_ptr();
        let saved = retain_label(Cow::Owned(label));
        assert_eq!(saved.as_ptr(), address);
    }
}
