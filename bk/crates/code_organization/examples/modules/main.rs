//! Runs the module visibility example.
//!
//! TODO Reviewed ??-??-2026.

mod module_paths;
mod module_visibility;

/// Runs the modules example.
fn main() {
    module_paths::run();
    module_visibility::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example; each module's `run()` asserts its own results, so a
    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
