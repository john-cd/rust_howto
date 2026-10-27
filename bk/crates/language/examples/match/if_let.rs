#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The `if let` syntax allows you to combine `if` and `let`
//! to handle values that match a specific pattern, while ignoring others.

/// The `if let` syntax allows you to combine `if` and `let`
/// to handle values that match a specific pattern, while ignoring others.
fn main() {
    let config_max: Option<u8> = Some(3u8);

    // If `config_max` is `Some`, bind the inner value to `max`
    if let Some(max) = config_max {
        // `max` is available here.
        println!("The maximum is configured to be {max}.");
    }

    // This is equivalent to the following `match` expression:
    #[allow(clippy::single_match)]
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}."),
        _ => {} // Required because `match` is exhaustive.
    };

    /// Specifies one of the supported movement directions.
    enum Direction {
        Left,
        Right,
    }

    let left = Direction::Left;

    // `matches!` can test a pattern against a non-parameterized, unit-like
    // enum variant, even if the enum does not implement `PartialEq`.
    if matches!(left, Direction::Left) {
        println!("Going left.");
    } else {
        // The `else` branch handles the case where the pattern does not match.
        println!("Going right.");
    }
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the if let example through its entry point.
    #[test]
    fn test_if_let_example_entry_point() {
        main();
    }
}
