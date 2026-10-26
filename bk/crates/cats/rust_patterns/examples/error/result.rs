#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Demonstrates returning and handling a recoverable error with `Result`.

/// Divides `numerator` by `denominator`, returning an error when it is zero.
fn divide(numerator: i32, denominator: i32) -> Result<i32, &'static str> {
    if denominator == 0 {
        Err("division by zero")
    } else {
        Ok(numerator / denominator)
    }
}

/// Handles both success and failure results from `divide`.
fn main() {
    for (numerator, denominator) in [(12, 3), (12, 0)] {
        match divide(numerator, denominator) {
            Ok(quotient) => {
                println!("{numerator} / {denominator} = {quotient}");
            }
            Err(error) => eprintln!(
                "could not divide {numerator} by {denominator}: {error}"
            ),
        }
    }
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::divide;

    /// Returns the quotient for a nonzero denominator.
    #[test]
    fn division_returns_a_success_value() {
        assert_eq!(divide(12, 3), Ok(4));
    }

    /// Returns an error instead of panicking for a zero denominator.
    #[test]
    fn division_reports_a_zero_denominator() {
        assert_eq!(divide(12, 0), Err("division by zero"));
    }
}
