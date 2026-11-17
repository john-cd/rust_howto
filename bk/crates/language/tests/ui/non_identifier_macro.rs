//! Checks the compile_error! branch of the displayed macro.

#[macro_use]
#[path = "../../examples/macros/identifier_only_macro.rs"]
mod identifier_only_macro;

/// Runs the example demonstrated by this module.
fn main() {
    let _ = must_be_an_identifier!(42);
}
