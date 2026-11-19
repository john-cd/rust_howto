//! Checks that rustc invokes Watt during compilation.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use watt_round_trip::parenthesize;

/// Parentheses preserve the precedence of the whole input expression.
#[test]
fn watt_produces_an_expression() {
    assert_eq!(2 * parenthesize!(20 + 1), 42);
}
// ANCHOR_END: example
