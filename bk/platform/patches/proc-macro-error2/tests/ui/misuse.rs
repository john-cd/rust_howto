// TODO Reviewed ??-??-2026.

use proc_macro_error2::abort;

/// Data carried by foo operations.
struct Foo;

/// Handles the foo behavior for this component.
#[allow(unused)]
fn foo() {
    abort!(Foo, "BOOM");
}

/// Runs the example demonstrated by this module.
fn main() {}
