// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates wrapping arithmetic with `std::num::Wrapping`.

use std::num::Wrapping;

/// Demonstrates wrapping arithmetic with `std::num::Wrapping`.
/// Adds two wrapped values and checks the result.
fn main() {
    let x = Wrapping(255_u8);
    let y = x + Wrapping(1);
    assert_eq!(y.0, 0);
}
// ANCHOR_END: example

/// Runs this example for its parent module.
/// Demonstrates wrapping arithmetic with `std::num::Wrapping`.
pub(super) fn run() {
    main();
}

/// Verifies that `Wrapping` arithmetic rolls over at the type boundary.
#[cfg(test)]
mod tests {

    /// Verifies that `Wrapping` arithmetic rolls over at the type boundary.
    #[test]
    fn test_wrapping_type_rolls_over() {
        super::main();
    }
}
