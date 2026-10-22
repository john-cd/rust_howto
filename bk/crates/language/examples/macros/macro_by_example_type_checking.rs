//! Type checking of the expressions a declarative macro expands to.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
/// Returns twice the expression `x`, whose type is not checked by the macro.
macro_rules! double {
    ($x:expr) => {
        ($x) * 2
    };
}

/// Calls a macro that doubles its argument.
fn main() {
    println!("{}", double!(5));
    // ERROR cannot multiply `&str` by `{integer}`
    // println!("{}", double!("hello"));
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
