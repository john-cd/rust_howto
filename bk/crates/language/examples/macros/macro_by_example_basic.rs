// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A basic declarative macro.

/// A basic macro-by-example (from the Rust By Example book).
macro_rules! say_hello {
    // The matcher `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        let w = "World";
        println!("Hello {w}!");
    };
}

/// Calls the macro.
fn main() {
    say_hello!();
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the macro by example basic example through its entry point.
    #[test]
    fn test_macro_by_example_basic_example_entry_point() {
        main();
    }

    /// The macro expands to statements that bind a local variable.
    #[test]
    fn test_say_hello_expands_to_statements() {
        say_hello!();
        say_hello!(); // Hygiene: expanding twice does not clash.
    }

    // Output-only example: the macro expansion only prints.
}
