//! Demonstrates wrapping arithmetic with `std::num::Wrapping`.

// ANCHOR: example
use std::num::Wrapping;

/// Adds two wrapped values and checks the result.
fn main() {
    let x = Wrapping(255_u8);
    let y = x + Wrapping(1);
    assert_eq!(y.0, 0);
}

/// Runs this example for its parent module.
pub(super) fn run() {
    main();
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Verifies that `Wrapping` arithmetic rolls over at the type boundary.
    #[test]
    fn test_wrapping_type_rolls_over() {
        super::main();
    }
}
