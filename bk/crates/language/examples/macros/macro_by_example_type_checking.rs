// TODO Reviewed ??-??-2026.

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

    /// Runs the macro by example type checking example through its entry point.
    #[test]
    fn test_macro_by_example_type_checking_example_entry_point() {
        main();
    }

    /// The parentheses make `double!` safe with low-precedence expressions.
    #[test]
    fn test_double_precedence() {
        assert_eq!(double!(5), 10);
        assert_eq!(double!(1 + 2), 6); // Would be 5 without the parentheses.
        assert_eq!(double!(-3), -6);
    }
}
