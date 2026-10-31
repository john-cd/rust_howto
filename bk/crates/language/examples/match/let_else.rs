// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `let ... else` statement.
//!
//! `let ... else` binds a pattern and, if it does not match, runs a block that
//! must diverge (return, break or panic). It keeps the main path unindented
//! compared with a nested `if let`.

/// Parses `input` as a number, or panics.
fn parse_number(input: &str) -> u32 {
    // Try to parse the input string.
    let Ok(num) = input.parse::<u32>() else {
        // If the binding fails, the `else` block is triggered and the function
        // panics.
        panic!("Failed to parse number: {input}");
    };
    // If it succeeds, `num` is available in the surrounding scope.
    num
}

/// Parses a number with `let ... else`.
fn main() {
    let value = parse_number("42");
    println!("Parsed value: {value}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the let else example through its entry point.
    #[test]
    fn test_let_else_example_entry_point() {
        main();
    }

    /// A valid number is parsed.
    #[test]
    fn test_parse_number() {
        assert_eq!(parse_number("42"), 42);
        assert_eq!(parse_number("0"), 0);
    }

    /// An invalid number hits the `else` branch, which diverges.
    #[test]
    #[should_panic(expected = "Failed to parse number: abc")]
    fn test_parse_number_invalid() {
        parse_number("abc");
    }

    /// Negative numbers do not fit in a `u32`.
    #[test]
    #[should_panic(expected = "Failed to parse number: -1")]
    fn test_parse_number_negative() {
        parse_number("-1");
    }
}
