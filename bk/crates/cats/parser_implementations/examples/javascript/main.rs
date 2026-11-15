//! Runs the JavaScript parser example.
//!
//! TODO Reviewed ??-??-2026.

/// Turns javascript input into structured syntax data while preserving source locations and
/// reporting malformed input. Analysis and validation code use these routines before applying
/// domain-specific rules.
mod swc_ecma_parser;
/// Runs the example.
/// Runs the javascript example.
fn main() {
    swc_ecma_parser::run();
}

/// Verifies that the JavaScript parser example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
