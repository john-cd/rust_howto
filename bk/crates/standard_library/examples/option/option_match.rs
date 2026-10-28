// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Handling an `Option` with `match`.

/// Divides `numerator` by `denominator`; returns `None` if the denominator is
/// zero.
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

/// Prints the result of divisions.
fn main() {
    let result1 = divide(10.0, 2.0);
    match result1 {
        // Handles the `Some` case.
        Some(value) => println!("Result: {value}"),
        // Handles the `None` case.
        None => println!("Cannot divide by zero!"),
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the option match example through its entry point.
    #[test]
    fn test_option_match_example_entry_point() {
        main();
    }
}
