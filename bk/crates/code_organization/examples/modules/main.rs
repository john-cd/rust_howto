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

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
