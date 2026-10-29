//! Runs the `yew` web framework example.
//!
//! TODO Reviewed ??-??-2026.

mod yew;

/// Runs the yew example.
fn main() {
    yew::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point (output-only: the modules
    /// test their own results).
    // output-only: the module prints a hint; its own tests render the
    // component.
    #[test]
    fn test_main_entry_point() {
        main();
    }
}
