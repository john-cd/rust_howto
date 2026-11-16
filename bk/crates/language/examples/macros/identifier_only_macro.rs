//! Shows a macro that accepts identifiers and rejects other tokens.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: definition
macro_rules! must_be_an_identifier {
    ($name:ident) => {
        stringify!($name)
    };
    ($_:tt) => {
        compile_error!("This macro only accepts an identifier.")
    };
}
// ANCHOR_END: definition

/// A bare identifier is accepted and returned as text.
#[test]
fn accepts_an_identifier() {
    // ANCHOR: usage
    assert_eq!(must_be_an_identifier!(foo), "foo");
    // ANCHOR_END: usage
}
