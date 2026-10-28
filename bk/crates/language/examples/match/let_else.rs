// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `let ... else` statement.

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
}
