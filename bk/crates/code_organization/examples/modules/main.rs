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
