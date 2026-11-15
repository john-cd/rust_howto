//! Runs the module visibility example.
//!
//! TODO Reviewed ??-??-2026.

/// Resolve paths between modules and re-export a public module.
mod module_paths;
/// Visibility of items in modules.
mod module_visibility;
/// Runs the example.
/// Runs the modules example.

fn main() {
    module_paths::run();
    module_visibility::run();
}

/// panic fails the test.
#[cfg(test)]
mod tests {
    use super::*;

    /// panic fails the test.
    #[test]
    fn test_main_runs() {
        main();
    }
}
