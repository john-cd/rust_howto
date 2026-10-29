// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This is an example of a `while` loop.

/// This is an example of a `while` loop.
fn main() {
    // Initialize a mutable variable.
    let mut number = 5;
    // The loop continues as long as the condition is true.
    while number != 0 {
        print!("{number} ");
        // Decrement the number.
        number -= 1;
    }
    // Prints: 5 4 3 2 1
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the while1 example through its entry point.
    #[test]
    fn test_while1_example_entry_point() {
        main();
    }

    // Output-only example: the countdown is only printed.
}
