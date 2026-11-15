//! Runs the `tinytemplate` template example.
//!
//! TODO Reviewed ??-??-2026.

/// Example showcasing `tinytemplate`.
mod tinytemplate;
/// Runs the example.
/// Runs the tinytemplate example.
fn main() {
    tinytemplate::run();
}

/// Verifies that the `tinytemplate` template example.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs() {
        assert!(std::panic::catch_unwind(main).is_ok());
    }
}
