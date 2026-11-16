//! Shows named capture and positional formatting arguments.
//!
//! TODO Reviewed ??-??-2026.

/// Both forms interpolate their values into the same format string.
#[test]
#[allow(
    clippy::uninlined_format_args,
    reason = "Shows a positional argument alongside named capture."
)]
fn interpolates_named_and_positional_arguments() {
    // ANCHOR: example
    let name = "Alice";
    let age = 30;
    let message = format!("My name is {name} and my age is {}", age);
    println!("{message}");
    // ANCHOR_END: example

    assert_eq!(message, "My name is Alice and my age is 30");
}
