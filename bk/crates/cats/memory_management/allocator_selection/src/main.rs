//! Application entry point for allocator selection.
/// Provides global allocator behavior for the src component.
mod global_allocator;

/// Runs the example demonstrated by this module.
fn main() {
    global_allocator::run();
}
