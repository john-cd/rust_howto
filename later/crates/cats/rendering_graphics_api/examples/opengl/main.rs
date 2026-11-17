// TODO Reviewed ??-??-2026.

//! Runnable entry point for the opengl example.
/// Provides glow context behavior for the opengl component.
mod glow_context;

/// Runs the example demonstrated by this module.
fn main() {
    glow_context::run();
}
