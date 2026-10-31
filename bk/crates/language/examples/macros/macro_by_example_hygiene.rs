// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Macro hygiene: variables in a macro do not clash with variables outside it.

/// Declares a variable `x`, which is not visible where the macro is called.
macro_rules! foo {
    () => {
        let _x = 3; // `_x` is local to the macro.
    };
}

// To use a local variable at the macro invocation site, pass it to the macro:
/// Declares a variable with the name passed as an argument, so it is visible
/// where the macro is called.
macro_rules! bar {
    ($v:ident) => {
        let $v = 3;
    };
}

// Other symbols are available at the invocation site.
// You may define a function, `impl` block, etc... in a macro.
/// Defines a function `f`, which is visible where the macro is called.
macro_rules! baz {
    () => {
        /// Defines the function emitted by the macro.
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

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the macro by example hygiene example through its entry point.
    #[test]
    fn test_macro_by_example_hygiene_example_entry_point() {
        main();
    }

    /// The macro-defined `x` is invisible, but the caller-named binding works.
    #[test]
    fn test_hygiene() {
        foo!();
        bar!(y);
        assert_eq!(y, 3);
        baz!(); // Items are not hygienic: `f` is visible.
        f();
    }
}
