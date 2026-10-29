// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Warning about an ignored result with `#[must_use]`.

/// The `#[must_use]` attribute indicates that the return value of a function
/// must be used. This attribute can also be applied to traits, structs, enums,
/// etc.
#[must_use]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Uses the result of a `#[must_use]` function.
fn main() {
    // Use the return value of the function.
    println!("{}", add(1, 2));

    // Uncomment the following to see the warning:
    // "unused return value of `attributes_must_use::add` that must be used"
    // WARNING: add(2, 3);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the attributes must use example through its entry point.
    #[test]
    fn test_attributes_must_use_example_entry_point() {
        main();
    }

    /// `add` returns the sum, including with negative numbers.
    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(-5, 5), 0);
        assert_eq!(add(i32::MAX - 1, 1), i32::MAX);
    }
}
