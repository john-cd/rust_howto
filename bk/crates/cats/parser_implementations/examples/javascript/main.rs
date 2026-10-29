//! Runs the JavaScript parser example.
//!
//! TODO Reviewed ??-??-2026.

mod swc_ecma_parser;

/// Runs the javascript example.
fn main() {
    swc_ecma_parser::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
