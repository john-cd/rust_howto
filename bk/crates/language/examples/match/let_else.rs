#![allow(dead_code)]
// TODO Reviewed ??-??-2026
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
