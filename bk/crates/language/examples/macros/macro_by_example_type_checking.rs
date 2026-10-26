#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Type checking of the expressions a declarative macro expands to.

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
