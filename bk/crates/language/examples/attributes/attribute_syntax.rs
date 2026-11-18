//! Shows legal attribute forms and lint levels.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: forms
#[doc = "A documented constant."]
const COUNT: u8 = 2;

/// Defines the old count value used by this code.
#[deprecated(note = "Use COUNT instead.")]
const OLD_COUNT: u8 = 2;

/// Data carried by label operations.
#[derive(Debug)]
struct Label;

/// Data carried by number operations.
#[derive(PartialEq, Eq)]
struct Number(
    /// Numeric value stored by the Number wrapper.
    u8,
);
// ANCHOR_END: forms

// ANCHOR: lint_levels
// Inner attributes configure the entire module.
/// Demonstrates module-wide lint levels alongside individual item exceptions.
mod lint_settings {
    // ANCHOR: inner_lint
    #![warn(missing_docs)]
    // ANCHOR_END: inner_lint
    #![deny(unsafe_code)]

    /// Handles the unused helper behavior for this component.
    #[allow(dead_code, reason = "Demonstrates an unused item.")]
    fn unused_helper() {}

    #[allow(unused_imports, reason = "Demonstrates an unused import.")]
    use std::fmt::Debug;

    /// Returns the module status.
    #[allow(unused_variables, reason = "Demonstrates an unused binding.")]
    pub(super) fn message() -> &'static str {
        let unused = "ignored";
        "ready"
    }
}
// ANCHOR_END: lint_levels

/// The attributes apply to the annotated items without changing their values.
#[test]
#[allow(
    deprecated,
    reason = "The example deliberately refers to a deprecated item."
)]
fn attributes_apply_to_items() {
    assert_eq!(COUNT, OLD_COUNT);
    assert_eq!(format!("{Label:?}"), "Label");
    assert!(Number(2) == Number(COUNT));
    assert_eq!(lint_settings::message(), "ready");
}
