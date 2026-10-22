//! Macro hygiene: variables in a macro do not clash with variables outside it.
//!
//! TODO Reviewed ??-??-2026

#![allow(unused)]

// ANCHOR: example
macro_rules! foo {
    () => {
        let x = 3; // `x` is local to the macro.
    };
}

// To use a local variable at the macro invocation site, pass it to the macro:
macro_rules! bar {
    ($v:ident) => {
        let $v = 3;
    };
}

// Other symbols are available at the invocation site.
// You may define a function, `impl` block, etc... in a macro.
macro_rules! baz {
    () => {
        fn f() {}
    };
}

/// Calls the macros and shows which variables are visible.
fn main() {
    foo!();
    // ERROR println!("{x}");

    bar!(x);
    println!("{x}");

    baz!();
    // The function defined in the macro is available at the invocation site:
    f();
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
}
