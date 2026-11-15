//! Runs the Unicode grapheme example.
//!
//! TODO Reviewed ??-??-2026.

/// Builds or traverses unicode graph data while preserving relationships needed by downstream
/// analysis. Workspace and rendering code use this module to obtain a consistent view of the graph.
mod graphemes;
/// Runs the example.
/// Runs the unicode example.
fn main() {
    graphemes::run();
}

/// Verifies that the Unicode grapheme example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
