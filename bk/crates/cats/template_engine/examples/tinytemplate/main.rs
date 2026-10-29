//! Runs the `tinytemplate` template example.
//!
//! TODO Reviewed ??-??-2026.

mod tinytemplate;

/// Runs the tinytemplate example.
fn main() {
    tinytemplate::run();
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
