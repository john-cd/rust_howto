// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Repetitions in declarative macros.

/// Returns the sum of the expressions passed as arguments.
macro_rules! sum {
    // `$x:expr` matches an expression.
    // `$($x:expr),*` matches zero or more occurrences of `$x:expr` separated by commas.
    //
    // To allow for an optional trailing comma, add `$(,)?`. The `?` makes the comma pattern optional.
    ( $( $x:expr ),* ) => {
        {
            let mut temp = 0;
            $(
                temp += $x;
            )*
            temp
        }
    };
}
// Note the double set of { } above.
// One is part of the macro-by-example syntax,
// the other is a block that scopes the `temp` variable
// and avoid name collisions.

/// Sums a list of numbers with a macro.
fn main() {
    let s = sum!(1, 2, 3, 4);
    println!("{s}");
    assert_eq!(s, 10);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the macro by example repetitions example through its entry point.
    #[test]
    fn test_macro_by_example_repetitions_example_entry_point() {
        main();
    }
}
