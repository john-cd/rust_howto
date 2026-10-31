// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The `'static` lifetime.
//!
//! `'static` means a reference can live for the whole program, like a string
//! literal baked into the binary. A `T: 'static` bound only says that the type
//! owns its data or holds `'static` references; it does not mean the value
//! lives forever.

/// Returns a string literal, which lives for the whole program.
fn my_string() -> &'static str {
    // This string literal is stored directly in the binary,
    // and therefore has a `'static` lifetime.
    let s: &'static str = "I have a static lifetime.";
    s
}

/// Prints a `'static` string.
fn main() {
    println!("{}", my_string());
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the static lifetime example through its entry point.
    #[test]
    fn test_static_lifetime_example_entry_point() {
        main();
    }

    /// The function returns a `'static` string.
    #[test]
    fn test_my_string() {
        let s: &'static str = my_string();
        assert_eq!(s, "I have a static lifetime.");
    }
}
