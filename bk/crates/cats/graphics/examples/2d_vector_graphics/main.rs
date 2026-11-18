// TODO Reviewed ??-??-2026.

//! Runnable entry point for the 2d vector graphics example.
/// Provides vector graphics 2d behavior for the 2d vector graphics component.
mod vector_graphics_2d;

/// Runs the example demonstrated by this module.
fn main() -> anyhow::Result<()> {
    vector_graphics_2d::run()
}
