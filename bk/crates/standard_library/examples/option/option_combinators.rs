// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Combining `Option` values with `map`, `and_then` and other combinators.

/// Parses an even integer and halves it, returning `None` for invalid or odd

/// input.

/// # Arguments
/// * `input` - Input data whose fields determine the requested option combinators operation.
///
/// # Returns
/// `Some` when the requested value is available; otherwise returns `None`.
fn parse_even_half(input: &str) -> Option<u32> {
    input
        .parse::<u32>()
        // Convert a `Result` to `Option`.
        .ok()
        // `and_then` applies a function that can itself return `None`.
        .and_then(|number| (number % 2 == 0).then_some(number))
        // `map` transforms the value without another fallible step.
        .map(|number| number / 2)
}
/// Runs the example.
/// Parses a number and transforms it with `Option` combinators.

fn main() {
    let half = parse_even_half("24").unwrap_or(0);
    println!("Half of the even number is {half}.");
}

// ANCHOR_END: example
/// Runs the example entry point.
/// Runs the example.

pub fn run() {
    main();
}

/// options.
#[cfg(test)]
mod tests {
    use super::*;

    /// options.
    #[test]
    fn test_parse_even_half() {
        assert_eq!(parse_even_half("24"), Some(12));
        assert_eq!(parse_even_half("25"), None);
        assert_eq!(parse_even_half("not a number"), None);
    }

    /// Verifies option combinators example entry point.
    #[test]
    fn test_option_combinators_example_entry_point() {
        main();
    }
}
